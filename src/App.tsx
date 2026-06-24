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

  const handleGameLaunch = async (game_name) => {
    setLoading(true);
    setError(null);
    try {
      const res = await invoke<string>("launch_game", {game_name});
        console.log(res);
      } finally {
        setLoading(false);
      }
  };

  const handleLogin = async () => {
    setLoading(true);
    setError(null);
    try {
      const res = await invoke<string>("get_auth_code");
        console.log(res);
      } finally {
        setLoading(false);
      }
  };

  const authenticateCode = async (code) => {
    setLoading(true);
    setError(null);
    try {
      const res = await invoke<string>("log_in", {code})
      console.log(res);
    } finally {
      setLoading(false);
    }
  }



  if (!games) return ( 
  <div className="button-container">
    <button className="login-button" onClick={(e) => {
        e.preventDefault();
        handleLogin();
      }}>Get code
    </button>

      <form onSubmit={(e) => {
        e.preventDefault();
        console.log("Code submitted"); 
      }}>
        <input 
          id="auth-code"
          ref={inputRef}
          type="text" 
          placeholder="enter your code here" 
        />
        <button type="submit" onClick={(e) => { e.preventDefault(); authenticateCode(inputRef.current.value)}}>Log in</button>
      </form>
  </div>
      );

  return (
    <div className="main-container">
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
