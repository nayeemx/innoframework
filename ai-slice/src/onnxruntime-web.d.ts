declare module "onnxruntime-web" {
  export class Tensor<T extends "float32" | "int64" | "string"> {
    constructor(
      type: T,
      data: Float32Array | BigInt64Array | string[],
      dims: number[],
    );
    readonly type: T;
    readonly data: Float32Array | BigInt64Array | string[];
    readonly dims: number[];
  }

  export class InferenceSession {
    constructor(modelPath: string, options: { executionProviders: string[] });
    static create(
      modelPath: string,
      options: { executionProviders: string[] },
    ): Promise<InferenceSession>;
    run(feeds: Record<string, Tensor<any>>): Promise<Record<string, any>>;
  }
}
