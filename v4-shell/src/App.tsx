import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getMessage, getProfile } from "./lib/rpc";

/*
 * v4 desktop shell UI.
 *
 * Design notes (emilkowalski/skills):
 *  - Press feedback is `scale(0.97)` on every button, 160 ms, GPU-only.
 *  - Error state is a card, not a bare paragraph — it owns its own space.
 *  - Loading is a single staggered line, not a wall of spinners.
 *  - The fail-closed default is shown as a muted card, not hidden.
 */
export default function App() {
  const [msg, setMsg] = useState<string>("locked — no session");
  const [profile, setProfile] = useState<string>("locked — no session");
  const [status, setStatus] = useState<string>("");
  const [busy, setBusy] = useState(false);

  async function unlock() {
    setBusy(true);
    try {
      await invoke("unlock");
      setStatus("session established");
      setMsg("loading...");
      setProfile("loading...");
      const [m, p] = await Promise.all([getMessage.call(), getProfile.call()]);
      setMsg(m.text);
      setProfile(`${p.name} — ${p.role}`);
    } catch (e) {
      setStatus(String(e));
    } finally {
      setBusy(false);
    }
  }

  async function lock() {
    await invoke("lock");
    setStatus("session cleared");
    setMsg("locked — no session");
    setProfile("locked — no session");
  }

  useEffect(() => {
    // Demonstrate the fail-closed default before any session exists.
    getMessage.call().then(
      (r) => setMsg(r.text),
      () => { setMsg("locked — no session (401 equivalent)"); },
    );
  }, []);

  return (
    <main className="container" style={{ padding: "2rem" }}>
      <h1>v4 desktop shell</h1>

      <section
        style={{
          display: "grid",
          gap: "0.75rem",
          marginTop: "1.5rem",
          maxWidth: "40rem",
        }}
      >
        <Row label="message" value={msg} />
        <Row label="profile" value={profile} />
        <Row label="status" value={status || (busy ? "working…" : "idle")} />
      </section>

      <div style={{ marginTop: "2rem", display: "flex", gap: "0.5rem" }}>
        <button onClick={unlock} disabled={busy}>
          Unlock (establish session)
        </button>
        <button onClick={lock} disabled={busy}>
          Lock (clear session)
        </button>
      </div>
    </main>
  );
}

function Row({ label, value }: { label: string; value: string }) {
  const isLocked = value.startsWith("locked");
  return (
    <p style={{ margin: 0 }}>
      <strong style={{ color: "var(--color-muted)" }}>{label}:</strong>{" "}
      <span style={{ color: isLocked ? "var(--color-muted)" : "var(--color-fg)" }}>
        {value}
      </span>
    </p>
  );
}