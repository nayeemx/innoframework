import { useState } from "react";
import { getMessage, getProfile } from "@/routes/index";

function Button({ className = "", onClick, children, disabled }: { className?: string; onClick?: () => void; children: React.ReactNode; disabled?: boolean }) {
  return (
    <button
      onClick={onClick}
      disabled={disabled}
      className={[
        "inline-flex items-center justify-center rounded-md border border-input bg-background px-4 py-2 text-sm font-medium ring-offset-background transition-colors hover:bg-accent hover:text-accent-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:pointer-events-none disabled:opacity-50",
        className,
      ].join(" ")}
    >
      {children}
    </button>
  );
}

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
    <div className="p-8 font-sans max-w-2xl mx-auto">
      <h1 className="text-2xl font-bold mb-4">{{NAME}}</h1>
      <div className="flex gap-2 mb-4">
        <Button onClick={loadMessage} disabled={!!message || !!error}>
          {message ? "Reload Message" : "Load Message"}
        </Button>
        <Button onClick={loadProfile} disabled={!!profile || !!error} className="bg-accent text-accent-foreground">
          {profile ? "Reload Profile" : "Load Profile"}
        </Button>
      </div>
      {error && <p className="text-destructive mt-2">Error: {error}</p>}
      {message && <p className="mt-2"><strong>Message:</strong> {message}</p>}
      {profile && (
        <p className="mt-2"><strong>Profile:</strong> {profile.name} — {profile.role}</p>
      )}
      <p className="mt-4 text-sm text-muted-foreground">
        Calls a typed server function with an auth guard. Unauthenticated requests fail with 401.
      </p>
    </div>
  );
}