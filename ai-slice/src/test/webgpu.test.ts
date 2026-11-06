import { describe, it, expect, vi, beforeEach } from "vitest";
import { WebGpuProvider } from "../webgpu";

function mockGpu(available: boolean): void {
  const mockDevice = { requestDevice: vi.fn().mockResolvedValue({}) };
  const mockAdapter = available
    ? { requestDevice: vi.fn().mockResolvedValue(mockDevice) }
    : null;
  (navigator as any).gpu = available
    ? { requestAdapter: vi.fn().mockResolvedValue(mockAdapter) }
    : undefined;
}

beforeEach(() => {
  vi.restoreAllMocks();
  mockGpu(true);
});

describe("WebGpuProvider", () => {
  it("reports name as webgpu", () => {
    const provider = new WebGpuProvider();
    expect(provider.name).toBe("webgpu");
  });

  it("requests a GPU adapter and device on first generate", async () => {
    const provider = new WebGpuProvider({ modelName: "my-model" });
    const result = await provider.generate([
      { role: "user", content: "hi" },
    ]);
    expect((navigator as any).gpu).toBeDefined();
    expect((navigator as any).gpu.requestAdapter).toHaveBeenCalled();
    expect(result).toContain("WebGPU model");
  });

  it("reuses the same device across calls", async () => {
    const provider = new WebGpuProvider();
    await provider.generate([{ role: "user", content: "a" }]);
    await provider.generate([{ role: "user", content: "b" }]);
    // requestAdapter called once due to device caching
    expect((navigator as any).gpu.requestAdapter).toHaveBeenCalledTimes(1);
  });

  it("includes temperature and maxTokens in response", async () => {
    const provider = new WebGpuProvider({
      temperature: 0.3,
      maxTokens: 128,
    });
    const result = await provider.generate([
      { role: "user", content: "hello" },
    ]);
    expect(result).toContain("temperature=0.3");
    expect(result).toContain("maxTokens=128");
  });

  it("isAvailable returns true when WebGPU is available", async () => {
    const provider = new WebGpuProvider();
    const available = await provider.isAvailable();
    expect(available).toBe(true);
  });

  it("isAvailable returns false when WebGPU is not available", async () => {
    mockGpu(false);
    const provider = new WebGpuProvider();
    const available = await provider.isAvailable();
    expect(available).toBe(false);
  });

  it("throws on generate when WebGPU is not available", async () => {
    mockGpu(false);
    const provider = new WebGpuProvider();
    await expect(
      provider.generate([{ role: "user", content: "hi" }]),
    ).rejects.toThrow("WebGPU is not available");
  });
});
