import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./css/App.css";

function App(): JSX.Element {
  const [name, setName] = useState<string>("");
  const [reply, setReply] = useState<string | null>(null);
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);

  const handleClick = async () => {
    setLoading(true);
    setError(null);
    try {
      const res = await invoke<string>("greet", { name });
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
      <input
        value={name}
        onChange={(e) => setName(e.target.value)}
        placeholder="Your name"
      />
      <button onClick={handleClick} disabled={loading}>
        {loading ? "Loading..." : "PRESS ME TO TRY IT OUT"}
      </button>

      {reply && <div className="reply">Reply: {reply}</div>}
      {error && <div className="error">Error: {error}</div>}

      <footer>Phone number, Email, Location, Author</footer>
    </div>
  );
}

export default App;
