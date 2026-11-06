/*
 * @framework/core — CSP preset and security headers.
 *
 * Default policy: strict, no unsafe-inline, no external origins.
 * Apps extend it (e.g. add fonts.googleapis.com) via `csp()`.
 */

export interface CspOptions {
  fontSrc?: string[];
  imgSrc?: string[];
  connectSrc?: string[];
  styleSrc?: string[];
}

const BASE = [
  "default-src 'self'",
  "script-src 'self'",
  "style-src 'self'",
  "img-src 'self' data:",
  "connect-src 'self'",
  "frame-ancestors 'none'",
  "base-uri 'self'",
  "object-src 'none'",
  "form-action 'self'",
];

/**
 * Build a CSP string from the base policy plus app-specific directives.
 * Directives are joined with "; " and never contain `unsafe-inline`.
 */
export function csp(opts: CspOptions = {}): string {
  const lines = [...BASE];
  if (opts.fontSrc) lines[4] = `font-src 'self' ${opts.fontSrc.join(" ")}`.trim();
  if (opts.imgSrc) lines[3] = `img-src 'self' data: ${opts.imgSrc.join(" ")}`.trim();
  if (opts.connectSrc) lines[5] = `connect-src 'self' ${opts.connectSrc.join(" ")}`.trim();
  if (opts.styleSrc) lines[2] = `style-src 'self' ${opts.styleSrc.join(" ")}`.trim();
  return lines.join("; ");
}

/** The default v1 policy. */
export const DEFAULT_CSP = csp();

/**
 * Express middleware that sets CSP + the OWASP-recommended security headers
 * on every response.
 *
 *   app.use(securityHeaders());
 */
export function securityHeaders() {
  return (_req: any, res: any, next: () => void): void => {
    res.setHeader("Content-Security-Policy", DEFAULT_CSP);
    res.setHeader("X-Frame-Options", "DENY");
    res.setHeader("X-Content-Type-Options", "nosniff");
    res.setHeader("Referrer-Policy", "strict-origin-when-cross-origin");
    res.setHeader(
      "Strict-Transport-Security",
      "max-age=31536000; includeSubDomains; preload",
    );
    next();
  };
}