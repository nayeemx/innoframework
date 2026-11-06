import { describe, it, expect } from "vitest";
import { csp, DEFAULT_CSP, securityHeaders } from "./csp.js";

describe("csp — default preset", () => {
  it("contains no unsafe-inline", () => {
    expect(DEFAULT_CSP).not.toContain("unsafe-inline");
    expect(DEFAULT_CSP).not.toContain("unsafe-eval");
  });

  it("blocks framing, objects, and base tags", () => {
    expect(DEFAULT_CSP).toContain("frame-ancestors 'none'");
    expect(DEFAULT_CSP).toContain("object-src 'none'");
    expect(DEFAULT_CSP).toContain("base-uri 'self'");
  });

  it("extends with app directives", () => {
    const policy = csp({ fontSrc: ["https://fonts.gstatic.com"] });
    expect(policy).toContain("font-src 'self' https://fonts.gstatic.com");
  });

  it("securityHeaders sets six headers", () => {
    const headers: Record<string, string> = {};
    const res = {
      setHeader: (k: string, v: string) => (headers[k] = v),
    };
    securityHeaders()({} as any, res as any, () => {});
    expect(headers["Content-Security-Policy"]).toBe(DEFAULT_CSP);
    expect(headers["X-Frame-Options"]).toBe("DENY");
    expect(headers["X-Content-Type-Options"]).toBe("nosniff");
    expect(headers["Referrer-Policy"]).toBe(
      "strict-origin-when-cross-origin",
    );
    expect(headers["Strict-Transport-Security"]).toContain("max-age=31536000");
  });
});