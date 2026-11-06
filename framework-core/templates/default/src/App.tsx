import { useState } from "react";
import { getMessage, getProfile } from "@/routes/index";

export default function App() {
  const [message, setMessage] = useState<string>("");
  const [profile, setProfile] = useState<{ name: string; role: string } | null>(null);
  const [error, setError] = useState<string | null>(null);

  const loadMessage = async () => {
    try {
      setMessage((await getMessage.call()).text);
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    }
  };

  const loadProfile = async () => {
    try {
      setProfile(await getProfile.call());
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    }
  };

  return (
    <div style={{ padding: "2rem", fontFamily: "system-ui, sans-serif", maxWidth: "40rem", margin: "0 auto" }}>
      <h1 style={{ fontSize: "1.5rem", marginBottom: "1rem" }}>{{NAME}}</h1>
      <div style={{ display: "flex", gap: "0.5rem", marginBottom: "1rem" }}>
        <button onClick={loadMessage} disabled={!!message || !!error}>
          {message ? "Reload Message" : "Load Message"}
        </button>
        <button onClick={loadProfile} disabled={!!profile || !!error}>
          {profile ? "Reload Profile" : "Load Profile"}
        </button>
      </div>
      {error && <p style={{ color: "#dc2626", marginTop: "0.5rem" }}>Error: {error}</p>}
      {message && <p style={{ marginTop: "0.5rem" }}><strong>Message:</strong> {message}</p>}
      {profile && (
        <p style={{ marginTop: "0.5rem" }}><strong>Profile:</strong> {profile.name} — {profile.role}</p>
      )}
      <p style={{ marginTop: "1rem", fontSize: "0.875rem", color: "#64748b" }}>
        Calls a typed server function with an auth guard. Unauthenticated requests fail with 401.
      </p>
    </div>
  );
}