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
    <div className="container">
      <div className="header">
        <div className="proton-config-bar">
          <span style={{ color: protonPath ? "#4CAF50" : "#FF5722" }}>
            {protonPath ? "Proton Linked" : "Path Unset"}
          </span>
          <button onClick={handleSelectProton} className="proton-btn">Change</button>
        </div>
      </div>

      <div className="grid">
        {filtered.map((game) => (
          <div key={game.id} className="card">
            <div className="card-top">
              <div>
                <p className="game-name">{game.name}</p>
                <small className="platform">{game.platform.toUpperCase()}</small>
              </div>
              <div className="hours-badge">
                🕒 {((stats.find(s => s.game_name === game.name)?.total_playtime_minutes || 0) / 60).toFixed(1)} hrs
              </div>
            </div>

            <div className="card-footer">
              <div style={{ display: "flex", gap: "2px" }}>
                {[1, 2, 3, 4, 5].map((s) => (
                  <button 
                    key={s} 
                    onClick={() => handleRate(game, s)} 
                    className="star"
                    style={{ color: s <= game.rating ? "#FFD700" : "#555" }}
                  >★</button>
                ))}
              </div>
              
              <div style={{ display: "flex", gap: "5px", flex: 1 }}>
                {game.is_installed && (
                  <button onClick={() => handleUninstall(game)} className="uninstall-btn">🗑️</button>
                )}
                <button 
                  onClick={() => game.is_installed ? handleLaunch(game) : handleInstall(game)} 
                  className={game.is_installed ? "launch-btn" : "install-btn"}
                >
                  {downloadingGame === game.id ? "..." : (game.is_installed ? "Launch" : "Install")}
                </button>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

export default App;
