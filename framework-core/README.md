# @innologybd/core

**Innology Framework** — typed fullstack. Authenticated. Fail-closed.

v1 core: typed server functions, signed auth guards, CSP presets, and
build-time secret checks. One package, four surfaces.

```bash
npm i @innologybd/core
```

## Install

```bash
npm i @innologybd/core
# or scaffold a new app:
npx create-innology-app my-app
```

## API

### `createServerFn(name, defaultInput)`

Declare a typed server function. The client call looks local but POSTs to
`/rpc/<name>`, forwarding the session cookie so the auth guard applies.

```ts
import { createServerFn } from "@innologybd/core/rpc";

export const getMessage = createServerFn<{}, { ok: boolean; text: string }>(
  "getMessage", {},
);
const { text } = await getMessage.call();
```

### `makeSession` / `validateSession` / `authGuard` / `requireRole`

Signed tokens: `<base64url(payload)>.<base64url(hmac-sha256(payload, secret))>`.
Verification is constant-time; expired sessions are rejected.

```ts
import { makeSession, validateSession, authGuard, requireRole } from "@innologybd/core/auth";

const token = makeSession(userId, "admin");
const session = validateSession(token);   // Session | null
authGuard(session);                        // throws 401
requireRole("admin")(session);             // throws 403
```

### `securityHeaders()` / `DEFAULT_CSP`

Express middleware that sets the OWASP-recommended headers on every response.

```ts
import { securityHeaders } from "@innologybd/core";
app.use(securityHeaders());
```

Default policy — strict, no `unsafe-inline`:

```
default-src 'self'; script-src 'self'; style-src 'self'; img-src 'self' data:; connect-src 'self'; frame-ancestors 'none'; base-uri 'self'; object-src 'none'; form-action 'self';
```

### `check-secrets`

Fails the build if server-only env/secrets appear in client source.

```bash
node dist/check-secrets.js src
```

## Scaffolding

```bash
npx create-innology-app my-app
cd my-app
npm install
npm run dev        # client 8080 + server 3001
npm run build      # vite build + secret check
npm test           # vitest
```

## Layout of a generated app

```
my-app/
  index.html
  package.json
  tsconfig.json
  vite.config.ts
  vitest.config.ts
  server/index.ts        # Express: login, /rpc/* guarded routes, health
  scripts/check-secrets.js
  src/
    main.tsx
    App.tsx
    lib/rpc.ts            # createServerFn client stub
    routes/index.ts       # typed server function declarations
```

## Security invariants

| # | Check |
|---|---|
| 1 | Every `/rpc/*` route is wrapped in `rpcGuard(validateSession, authGuard)` |
| 2 | Session tokens are HMAC-signed; tampered, forged, expired, or malformed tokens are rejected |
| 3 | CSP has no `unsafe-inline`, no `unsafe-eval`, `frame-ancestors: none`, `object-src: none` |
| 4 | Build fails if `SERVER_*` / `SECRET_*` / `PRIVATE_*` appear in client code |

## License

MIT