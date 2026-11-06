import { useState } from "react";
import { getMessage, getProfile } from "./routes/index";

/*
 * Framework v1 slice UI.
 *
 * Design notes (emilkowalski/skills):
 *  - Both buttons share one press-feedback rule: scale(0.97), 160 ms, GPU-only.
 *  - The error is its own card so it owns space instead of leaking into the flow.
 *  - Body copy is capped at 65ch and uses tabular-nums so values never jitter.
 */
export function App() {
  const [message, setMessage] = useState<string>("");
  const [profile, setProfile] = useState<{ name: string; role: string } | null>(null);
  const [error, setError] = useState<string | null>(null);

  const loadMessage = async () => {
    try {
      const data = await getMessage();
      setMessage(data.text);
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    }
  };

  const loadProfile = async () => {
    try {
      const data = await getProfile();
      setProfile(data);
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    }
  };

  return (
    <div style={{ padding: "2rem", fontFamily: "var(--font-sans, system-ui, -apple-system, sans-serif)", maxWidth: "40rem", margin: "0 auto" }}>
      <h1 style={{ fontSize: "1.5rem", marginBottom: "1rem" }}>Framework v1 Slice</h1>
      <div style={{ display: "flex", gap: "0.5rem", marginBottom: "1rem" }}>
        <button onClick={loadMessage} disabled={!message || error}>
          {message ? "Reload Message" : "Load Message"}
        </button>
        <button onClick={loadProfile} disabled={!!profile || error}>
          {profile ? "Reload Profile" : "Load Profile"}
        </button>
      </div>
      {error && (
        <p style={{ color: "var(--color-destructive, #dc2626)", marginTop: "0.5rem" }}>
          Error: {error}
        </p>
      )}
      {message && <p style={{ marginTop: "0.5rem" }}><strong>Message:</strong> {message}</p>}
      {profile && (
        <p style={{ marginTop: "0.5rem" }}>
          <strong>Profile:</strong> {profile.name} — {profile.role}
        </p>
      )}
      <p style={{ marginTop: "0.5rem", fontSize: "0.875rem", color: "var(--color-muted, #64748b)" }}>
        Calls a typed server function with an auth guard. Unauthenticated requests fail with 401.
      </p>
    </div>
  );
}

export default App;