#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;

/*
 * v4 Tauri shell backend.
 *
 * Every IPC command runs through `authorize`, which mirrors the v1 auth guard:
 * callers must hold an authenticated session. In a desktop shell the session is
 * established locally (the app owns the process), so `unlock` issues a session
 * token and protected commands fail closed until it has been called.
 *
 * This keeps the same invariant as v1: no data leaves a command without an
 * authenticated session.
 */

#[derive(Clone, Debug)]
struct Session {
    user_id: String,
    role: String,
}

#[derive(Default)]
struct AppState {
    session: std::sync::Mutex<Option<Session>>,
}

#[derive(Serialize)]
struct MessageResult {
    ok: bool,
    text: String,
}

#[derive(Serialize)]
struct ProfileResult {
    ok: bool,
    name: String,
    role: String,
}

#[derive(Serialize)]
struct HealthResult {
    ok: bool,
    session: Option<String>,
}

#[derive(Serialize)]
struct UnlockResult {
    ok: bool,
    role: String,
}

/// Mirror of v1's authGuard: fail closed when there is no session.
fn authorize(state: &AppState) -> Result<Session, String> {
    let guard = state
        .session
        .lock()
        .map_err(|_| "session lock poisoned".to_string())?;
    guard
        .clone()
        .ok_or_else(|| "Unauthenticated".to_string())
}

/// Desktop equivalent of POST /login: establishes a local session.
#[tauri::command]
fn unlock(state: tauri::State<'_, AppState>) -> Result<UnlockResult, String> {
    let session = Session {
        user_id: "00000000-0000-0000-0000-000000000001".to_string(),
        role: "user".to_string(),
    };
    let role = session.role.clone();
    let mut guard = state
        .session
        .lock()
        .map_err(|_| "session lock poisoned".to_string())?;
    *guard = Some(session);
    Ok(UnlockResult { ok: true, role })
}

/// Desktop equivalent of POST /logout.
#[tauri::command]
fn lock(state: tauri::State<'_, AppState>) -> Result<HealthResult, String> {
    let mut guard = state
        .session
        .lock()
        .map_err(|_| "session lock poisoned".to_string())?;
    *guard = None;
    Ok(HealthResult { ok: true, session: None })
}

#[tauri::command]
fn get_message(state: tauri::State<'_, AppState>) -> Result<MessageResult, String> {
    authorize(&state)?;
    Ok(MessageResult {
        ok: true,
        text: "Hello from v4 Tauri shell.".into(),
    })
}

#[tauri::command]
fn get_profile(state: tauri::State<'_, AppState>) -> Result<ProfileResult, String> {
    let session = authorize(&state)?;
    Ok(ProfileResult {
        ok: true,
        name: "Portfolio Developer".into(),
        role: session.role,
    })
}

/// Health probe usable by the smoke test; reports whether a session exists.
#[tauri::command]
fn health(state: tauri::State<'_, AppState>) -> Result<HealthResult, String> {
    let guard = state
        .session
        .lock()
        .map_err(|_| "session lock poisoned".to_string())?;
    Ok(HealthResult {
        ok: true,
        session: guard.as_ref().map(|s| s.user_id.clone()),
    })
}

/// Native fibonacci computation for benchmarking against the JS path.
#[tauri::command]
fn fibonacci_native(n: u32) -> Result<u32, String> {
    if n > 50 {
        return Err("n too large (overflow)".to_string());
    }
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    for _ in 0..n {
        let c = a.wrapping_add(b);
        a = b;
        b = c;
    }
    Ok(a)
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            unlock,
            lock,
            health,
            get_message,
            get_profile,
            fibonacci_native
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guarded_commands_fail_closed_without_session() {
        let state = AppState::default();
        assert_eq!(authorize(&state).unwrap_err(), "Unauthenticated");
    }

    #[test]
    fn authorize_passes_after_unlock() {
        let state = AppState::default();
        *state.session.lock().unwrap() = Some(Session {
            user_id: "00000000-0000-0000-0000-000000000001".into(),
            role: "user".into(),
        });
        assert_eq!(authorize(&state).unwrap().role, "user");
    }

    #[test]
    fn authorize_fails_again_after_lock() {
        let state = AppState::default();
        *state.session.lock().unwrap() = Some(Session {
            user_id: "00000000-0000-0000-0000-000000000001".into(),
            role: "user".into(),
        });
        *state.session.lock().unwrap() = None;
        assert_eq!(authorize(&state).unwrap_err(), "Unauthenticated");
    }
}
