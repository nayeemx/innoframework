# v1 Slice

Minimal demonstration of framework v1: TypeScript fullstack slice with
typed server functions, an auth guard, CSP presets, and a build-time
secret check.

## Stack
- Vite + React SWC (client)
- Express (server functions)
- zod (validation)
- vitest + jsdom (tests)

## Scripts
- `npm run dev` — client (5173) + server (3001)
- `npm run build` — client build + secret check
- `npm test` — unit + negative security tests
- `npm run check:secrets` — standalone secret scan

## Security
- CSP header on every response (see server/index.js)
- `/rpc/*` requires a valid `v1_session` cookie (auth guard)
- Login sets `httpOnly`, `sameSite=lax` cookie (dev only; `secure` in prod)
- Build fails if `process.env.SERVER_*` / `SECRET_` / `PRIVATE_` appear in client code

## Test project note
This slice is a standalone demo. To port to `D:\projects\code\portfolio`:
1. Add Express server functions behind the Vite dev proxy
2. Add the auth guard middleware to `/rpc/*`
3. Add the CSP header to vite dev server / netlify.toml
4. Add `scripts/check-secrets.js` to the portfolio build
