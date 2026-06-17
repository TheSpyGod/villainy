import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./css/App.css";

type Game = {
  display_name: string;
  app_name: string;
  version: string;
  is_dependency: boolean;
};

type GamesResponse = {
  total: number;
  games: Game[];
};

function App(): JSX.Element {
  const [reply, setReply] = useState<string | null>(null);
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  
  const [games, setGames] = useState<GamesResponse | null>(null);

  useEffect(() => {
    async function loadInitialData() {
      setLoading(true);
      try {
        const res = await invoke<GamesResponse>('list_games');
        setGames(res);
        console.log(res);
      } finally {
        setLoading(false);
      }
    }
    loadInitialData();
  }, []);

  const handleClick = async () => {
    setLoading(true);
    setError(null);
    try {
      const res = await invoke<string>("startup");
      setReply(res);
    } catch (e: unknown) {
      const msg =
        typeof e === "object" && e !== null && "message" in e
          ? (e as { message?: unknown }).message
          : String(e);
      setError(typeof msg === "string" ? msg : String(msg));
    } finally {
      setLoading(false);
    }
  };

  if (!games) return <div> Loading ...</div>;

  return (
    <div className="main-container">
      <div className="actions">
        <button onClick={handleClick} disabled={loading}>
          {loading ? "Loading..." : "Debug Platforms (Console Output)"}
        </button>
      </div>

        <div className="games-library">
          <h3>Your Library has {games.total} games found</h3>
        </div>

      <div>
    <div>Total: {games.total}</div>
    <ul>
      {games.games.map((g) => (
        <li key={g.app_name}>
          <a
            href="#"
            onClick={(e) => {
              e.preventDefault();
              handleGameLaunch(g.app_name);
            }}
            role="button"
          >
            {g.display_name}
          </a>
        </li>
      ))}
    </ul>
  </div>

    </div>
  );
}

export default App;
