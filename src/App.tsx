import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./css/App.css";

function App(): JSX.Element {
  const [reply, setReply] = useState<string | null>(null);
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);

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

  const handleGameLaunch = async () => {
    setLoading(true);
    setError(null);
    try {
      const res = await invoke<string>("start_game", { title: "Limbo" });
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

  return (
    <div className="main-container">
      <button onClick={handleClick} disabled={loading}>
        {loading ? "Loading..." : "PRESS ME TO TRY IT OUT"}
      </button>

      <button onClick={handleGameLaunch} disabled={loading}>
        {loading ? "Loading..." : "LAUNCH GAME"}
      </button>

      {reply && <div className="reply">Reply: {reply}</div>}
      {error && <div className="error">Error: {error}</div>}

      <footer>Phone number, Email, Location, Author</footer>
    </div>
  );
}

export default App;
