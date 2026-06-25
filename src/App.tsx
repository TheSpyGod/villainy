import { invoke } from "@tauri-apps/api/core";
import React, { useState, useEffect } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import "./css/App.css";

const cleanGameName = (name) => name?.replace(/\*/g, "").trim() || "";

function App() {
  const [games, setGames] = useState([]);
  const [filter, setFilter] = useState("all");
  const [loading, setLoading] = useState(true);
  const [downloadingGame, setDownloadingGame] = useState(null);
  const [stats, setStats] = useState([]);
  const [activeSessions, setActiveSessions] = useState([]);
  const [protonPath, setProtonPath] = useState("");

  useEffect(() => {
    (async () => {
      const path = await invoke("get_proton_path_command").catch(console.error);
      if (path) setProtonPath(path);
      await loadGamesAndStats();
      checkSessions();
    })();
  }, []);

  const loadGamesAndStats = async () => {
    setLoading(true);
    try {
      const [rawGames, userStats] = await Promise.all([
        invoke("fetch_all_games"),
        invoke("get_all_stats")
      ]);
      setStats(userStats || []);
      setGames(rawGames.map((g) => ({
        ...g,
        name: cleanGameName(g.name),
        rating: userStats?.find(s => s.game_name === g.name && s.platform === g.platform)?.rating || 0,
        is_installed: g.is_installed ?? false
      })));
    } catch (e) {
      console.error("Data sync failed:", e);
    } finally {
      setLoading(false);
    }
  };

  const handleSelectProton = async () => {
    const selected = await open({ multiple: false, directory: false });
    const path = selected && typeof selected === "object" ? selected.path : selected;
    if (path) {
      setProtonPath(path);
      await invoke("save_proton_path_command", { path });
    }
  };

  const handleRate = async (game, rating) => {
    await invoke("rate_game", { gameName: game.name, platform: game.platform, rating });
    loadGamesAndStats();
  };

  const handleLaunch = async (game) => {
    await invoke("launch_game_tracked", { game }).catch(alert);
    setTimeout(loadGamesAndStats, 2000);
  };

  const handleInstall = async (game) => {
    setDownloadingGame(game.id);
    await invoke("install_game_command", { game }).catch(alert);
    setDownloadingGame(null);
    loadGamesAndStats();
  };

  const handleUninstall = async (game) => {
    if (!window.confirm(`Are you sure you want to uninstall ${game.name}?`)) return;
    
    setDownloadingGame(game.id);
    try {
      await invoke("uninstall_game_command", { game });
      await loadGamesAndStats();
    } catch (e) {
      alert("Uninstall failed: " + e);
    } finally {
      setDownloadingGame(null);
    }
  };

  const filtered = filter === "all" ? games : games.filter((g) => g.platform === filter);

  return (
    <div style={styles.container}>
      <div style={styles.header}>
        <div style={styles.protonConfigBar}>
          <span style={{ color: protonPath ? "#4CAF50" : "#FF5722" }}>{protonPath ? "Proton Linked" : "Path Unset"}</span>
          <button onClick={handleSelectProton} style={styles.protonBtn}>Change</button>
        </div>
      </div>

      <div style={styles.grid}>
  {filtered.map((game) => (
    <div key={game.id} style={styles.card}>
      <div style={styles.cardTop}>
        <div>
          <p style={styles.gameName}>{game.name}</p>
          <small style={styles.platform}>{game.platform.toUpperCase()}</small>
        </div>
        <div style={styles.hoursBadge}>
          🕒 {( (stats.find(s => s.game_name === game.name)?.total_playtime_minutes || 0) / 60).toFixed(1)} hrs
        </div>
      </div>

      <div style={styles.cardFooter}>
        <div style={{ display: "flex", gap: "2px" }}>
          {[1, 2, 3, 4, 5].map((s) => (
            <button key={s} onClick={() => handleRate(game, s)} style={{ ...styles.star, color: s <= game.rating ? "#FFD700" : "#555" }}>★</button>
          ))}
        </div>
        
        <div style={{ display: "flex", gap: "5px", flex: 1 }}>
          {game.is_installed && (
            <button onClick={() => handleUninstall(game)} style={styles.uninstallBtn}>
              🗑️
            </button>
          )}
          <button 
            onClick={() => game.is_installed ? handleLaunch(game) : handleInstall(game)} 
            style={game.is_installed ? styles.launchBtn : styles.installBtn}
          >
            {downloadingGame === game.id ? "..." : (game.is_installed ? "Launch" : "Install")}
          </button>
        </div>
      </div>
    </div>
  ))}
</div>    </div>
  );
}

const styles = {
  container: { padding: "20px", color: "#fff", backgroundColor: "#1a1a1a", minHeight: "100vh" },
  header: { display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: "20px", borderBottom: "1px solid #333", paddingBottom: "10px" },
  protonConfigBar: { display: "flex", alignItems: "center", gap: "10px", background: "#2a2a2a", padding: "8px 12px", borderRadius: "6px", border: "1px solid #444", fontSize: "13px" },
  protonPathText: { maxWidth: "250px", overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap", color: "#aaa" },
  protonBtn: { background: "#444", border: "none", color: "#fff", padding: "4px 8px", borderRadius: "4px", cursor: "pointer" },
  content: { display: "flex", flexDirection: "column", gap: "20px" },
  controls: { display: "flex", gap: "10px" },
  btn: { background: "#008CBA", border: "none", color: "#fff", padding: "10px 15px", borderRadius: "4px", cursor: "pointer" },
  filterBtn: { background: "#333", border: "none", color: "#fff", padding: "10px 15px", borderRadius: "4px", cursor: "pointer" },
  sessions: { display: "flex", alignItems: "center", gap: "10px", background: "#222", padding: "10px", borderRadius: "6px" },
  badge: { background: "#4CAF50", padding: "5px 10px", borderRadius: "4px", fontSize: "12px" },
  authBtn: { background: "#555", border: "none", color: "#fff", padding: "6px 12px", borderRadius: "4px", cursor: "pointer", marginLeft: "auto" },
  grid: { display: "grid", gridTemplateColumns: "repeat(auto-fill, minmax(260px, 1fr))", gap: "20px" },
  card: { background: "#252525", padding: "15px", borderRadius: "8px", display: "flex", flexDirection: "column", justifyContent: "space-between", height: "140px", border: "1px solid #333" },
  cardTop: { display: "flex", justifyContent: "space-between", alignItems: "flex-start", gap: "10px" },
  gameName: { margin: "0 0 5px 0", fontWeight: "bold", fontSize: "16px", maxWidth: "150px", overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap" },
  platform: { color: "#008CBA", fontWeight: "bold" },
  hoursBadge: { background: "#1f1f1f", padding: "4px 8px", borderRadius: "6px", fontSize: "12px", color: "#bbb", border: "1px solid #3c3c3c", whiteSpace: "nowrap" },
  buttonGroup: { display: "flex", gap: "10px" },
  launchBtn: { flex: 1, background: "#4CAF50", border: "none", color: "#fff", padding: "8px", borderRadius: "4px", cursor: "pointer", fontWeight: "bold" },
  installBtn: { flex: 1, background: "#E74C3C", border: "none", color: "#fff", padding: "8px", borderRadius: "4px", cursor: "pointer", fontWeight: "bold" },
  progressContainer: { display: "flex", alignItems: "center", gap: "10px" },
  progressBar: { flex: 1, background: "#444", height: "8px", borderRadius: "4px", overflow: "hidden" },
  progressText: { fontSize: "12px", color: "#aaa" }
};

export default App;
