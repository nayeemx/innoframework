# Package Capability Catalog (in progress)

npm blocks most page fetches (403). Entries below are from training knowledge; flagged ones need verification.

## Email / messaging
- @emailjs/browser: client-side email sending via EmailJS (no backend). send/sendForm, rate limiting, blocklists, headless detection. Verified via npm page.

## Cloud / storage (AWS)
- @aws-sdk/client-s3: AWS S3 client (put/get/list/delete objects, multipart).
- @aws-sdk/s3-request-presigner: presigned URLs for S3.

## Build / tooling (Babel)
- @babel/core: JS transpiler core.
- @babel/preset-env: targets-based transpilation.
- @babel/plugin-*: syntax transforms.

## Drag and drop
- @dnd-kit/core + @dnd-kit/sortable: accessible DnD (sensors, collision detection, sortable lists).
- @hello-pangea/dnd: fork of react-beautiful-dnd.

## Fingerprinting
- @fingerprintjs/fingerprintjs: browser fingerprinting (visitor ID).

## Firebase
- @firebase/firestore: Firestore client SDK.
- @firebase/rules-unit-testing: security rules tests.
- firebase: Firebase SDK (auth, firestore, storage, messaging, etc.).
- firebase-admin: server-side Firebase Admin SDK.
- firebase-functions: Cloud Functions.
- firebase-tools: CLI.

## AI
- @google/generative-ai: Gemini API client (text, vision, function calling, streaming).
- @langchain/community, @langchain/openai: LangChain JS (chains, agents, RAG, vector stores).
- langchain: core LangChain.

## UI headless
- @headlessui/react: unstyled accessible components (dialog, menu, tabs, etc.) by Tailwind team.
- @radix-ui/*: unstyled accessible primitives (shadcn/ui base).
- @hookform/resolvers: zod/yup resolvers for react-hook-form.

## Realtime / media
- @livekit/components-react, @livekit/components-styles, livekit-client, livekit-server-sdk: WebRTC video/audio (LiveKit).
- @metered-ca/realtime: Metered TURN/realtime.
- peerjs: WebRTC peer connections.
- socket.io, socket.io-client: realtime bidirectional.

## Testing
- @playwright/test: E2E browser testing.
- @testing-library/*: component testing.
- @firebase/rules-unit-testing: rules tests.
- vitest, jsdom: unit testing.
- @vitest/coverage-v8: coverage.

## Forms
- react-hook-form: performant forms.
- zod: schema validation.
- @hookform/resolvers: bridges.

## State
- @reduxjs/toolkit, react-redux, redux, redux-persist: Redux state.
- @tanstack/react-query: server state caching.
- @tanstack/react-table: headless tables.
- @tanstack/react-router, @tanstack/router: type-safe routing (Open_Erp uses).

## Styling
- tailwindcss, @tailwindcss/vite, @tailwindcss/typography, @tailwindcss/postcss: Tailwind v4.
- autoprefixer, postcss: CSS processing.
- class-variance-authority, clsx, tailwind-merge: class utilities.
- tw-animate-css, tailwindcss-animate: animations.
- framer-motion: animations.
- sass: SCSS.
- antd, antd-style: Ant Design.
- bootstrap, @popperjs/core: Bootstrap.

## Icons / UI libs
- lucide-react, react-icons, boxicons, heroicons (via): icons.
- @dnd-kit: DnD.
- @tiptap/*: rich text editor.
- cmdk: command palette.
- embla-carousel-react: carousel.
- vaul: drawer.
- sonner: toasts.
- react-toastify: toasts.
- next-themes: theme switching.
- react-day-picker: date picker.
- react-datepicker: date picker.
- react-resizable-panels: resizable.
- react-modal: modals.
- swiper: carousel.
- leaflet, react-leaflet: maps.
- recharts: charts.
- echarts: charts.
- jspdf, jspdf-autotable, pdfjs-dist: PDF.
- xlsx, exceljs, file-saver, jszip: spreadsheets/files.
- react-player: media player.
- react-to-print: printing.
- react-countdown, react-countup: counters.
- qrcode, qrcode.react: QR.
- otpauth: TOTP.
- bwip-js: barcode.
- chrono-node: date parsing.
- dayjs, date-fns, moment: dates.
- numeral: number formatting.
- lodash: utils.
- axios: HTTP.
- @tanstack/react-query: data fetching.
- dexie, localforage: IndexedDB.
- @univerjs/*: spreadsheets (Univer).
- @metered-ca/realtime: realtime.
- univerjs: (see above).

## Routing
- react-router, react-router-dom: routing.
- @tanstack/react-router: type-safe routing.

## Frameworks
- react, react-dom: UI.
- next: Next.js.
- vite: build tool.
- @vitejs/plugin-react, @vitejs/plugin-react-swc: Vite React plugins.
- @vitejs/plugin-basic-ssl: HTTPS dev.
- vite-plugin-pwa, workbox-window: PWA.
- vitepress: docs.

## Desktop / native
- @tauri-apps/*: Tauri desktop.
- (Rust: tauri, serde, tokio, etc. in Cargo.toml — not cataloged here.)

## 3D
- three, @react-three/fiber, @react-three/drei: 3D (Three.js).

## Auth
- @simplewebauthn/browser: WebAuthn.
- otplib: OTP.
- jsonwebtoken, bcryptjs: JWT/bcrypt (backend).

## Backend (Node)
- express, cors, dotenv, helmet, express-rate-limit, cookie-parser, multer, uuid: Express stack.
- pg: Postgres.
- socket.io: realtime.
- nodemailer: email.
- node-cron: cron.
- fs-extra, adm-zip, unzipper: file ops.
- node-pty, xterm, xterm-addon-*: terminal emulation.
- @google/generative-ai, openai: AI APIs.
- stripe, @stripe/*: payments.
- @aws-sdk/*: S3.
- adm-zip: zip.

## AI/ML (VPS)
- @langchain/*: LangChain.
- @tensorflow/tfjs, onnxruntime-node, @xenova/transformers: ML inference.

## DB
- @supabase/supabase-js, supabase: Supabase.
- pg: Postgres.

## Misc
- react-error-boundary: error boundaries.
- react-i18next, i18next, i18next-browser-languagedetector: i18n.
- react-leaflet: maps.
- baseline-browser-mapping, caniuse-lite: browser compat.
- archiver: zip creation.
- to-ico, @resvg/resvg-js: icon generation.
- nodemailer: email.
- pdfjs-dist: PDF rendering.
- js-base64: base64.
- html2canvas: screenshot.
- rc-progress: progress bar.
- suncalc: sun calculations.
- react-type-animation: typing animation.
- react-scroll-to-top: scroll button.
- react-is: React element checks.
- prop-types: propTypes.
- proptypes: (typo package).
- core-js: polyfills.
- react-scripts: CRA (legacy).
- react-hot-toast: toasts.
- nextjs-toploader: loading bar.
- @emailjs/browser: email (see above).
- @fontsource-variable/geist: font.
- @popperjs/core: positioning.
- @babel/*: transpilation.
- eslint, eslint-plugin-*, globals, typescript, typescript-eslint: linting/types.
- @types/*: type defs.
- lovable-tagger: Lovable integration.
- patch-package: patch deps.
- oxlint: fast linter.
- prettier, prettier-plugin-tailwindcss: formatting.
- gulp, webpack, babel-loader, etc.: build (sneat legacy).
- browser-sync: dev server.
- @babel/plugin-proposal-object-rest-spread etc.: babel plugins.
- ansi-colors, fancy-log, del, deepmerge: gulp helpers.
- gulp-*: gulp plugins.
- string-replace-webpack-plugin, uglifyjs-webpack-plugin: webpack plugins.
- css-loader, sass-loader, style-loader, html-loader: webpack loaders.
- @babel/preset-env: babel preset.

## Note
This catalog is from training knowledge, not verified fetches (npm returns 403). For any package where exact capabilities matter, ask and I'll fetch the official docs (often GitHub READMEs work even when npm is blocked).
