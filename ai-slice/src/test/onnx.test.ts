import { describe, it, expect, vi, beforeEach } from "vitest";
import { OnnxProvider } from "../onnx";

vi.mock("onnxruntime-web", () => {
  const createSession = vi.fn();
  return {
    InferenceSession: { create: createSession },
    Tensor: class Tensor {
      constructor(
        public type: string,
        public data: Float32Array,
        public dims: number[],
      ) {}
    },
    __createSession: createSession,
  };
});

const mockCreateSession = vi.mocked(
  (await import("onnxruntime-web")).InferenceSession.create,
);

const mockSession = {
  run: vi.fn().mockResolvedValue({ output: "The weather is sunny." }),
};

beforeEach(() => {
  vi.clearAllMocks();
  mockCreateSession.mockResolvedValue(mockSession);
  mockSession.run.mockResolvedValue({ output: "The weather is sunny." });
});

describe("OnnxProvider", () => {
  it("reports name as onnx", () => {
    const provider = new OnnxProvider();
    expect(provider.name).toBe("onnx");
  });

  it("creates an InferenceSession on first generate", async () => {
    const provider = new OnnxProvider({ modelPath: "model.onnx" });
    await provider.generate([{ role: "user", content: "hi" }]);
    expect(mockCreateSession).toHaveBeenCalledTimes(1);
    expect(mockCreateSession).toHaveBeenCalledWith("model.onnx", {
      executionProviders: ["cpu"],
    });
  });

  it("reuses the same session across calls", async () => {
    const provider = new OnnxProvider({ modelPath: "model.onnx" });
    await provider.generate([{ role: "user", content: "a" }]);
    await provider.generate([{ role: "user", content: "b" }]);
    expect(mockCreateSession).toHaveBeenCalledTimes(1);
  });

  it("returns session output as string", async () => {
    mockSession.run.mockResolvedValue({ output: "Hello from ONNX" });
    const provider = new OnnxProvider();
    const result = await provider.generate([{ role: "user", content: "hi" }]);
    expect(result).toBe("Hello from ONNX");
  });

  it("passes temperature and maxTokens in response when non-string output", async () => {
    mockSession.run.mockResolvedValue({ output: new Float32Array([1]) });
    const provider = new OnnxProvider({
      temperature: 0.5,
      maxTokens: 256,
    });
    const result = await provider.generate([{ role: "user", content: "hi" }]);
    expect(result).toContain("temperature=0.5");
    expect(result).toContain("maxTokens=256");
  });

  it("passes tools through when provided", async () => {
    mockSession.run.mockResolvedValue({ output: "" });
    const provider = new OnnxProvider();
    await provider.generate(
      [{ role: "user", content: "hi" }],
      { tools: ["getWeather"] },
    );
    expect(mockSession.run).toHaveBeenCalled();
  });

  it("isAvailable returns true when onnxruntime-web loads", async () => {
    const provider = new OnnxProvider();
    const available = await provider.isAvailable();
    expect(available).toBe(true);
  });
});
