/* ------------------------------------------------------------------ */
/* v6-compiler — Middle Intermediate Representation (MIR)             */
/*                                                                    */
/* A low-level, target-independent IR produced by the frontend parser. */
/* The compiler backend lowers MIR to native code or interprets it    */
/* directly, bypassing the wrapper RPC layer for performance-critical */
/* paths.                                                             */
/*                                                                    */
/* Exit criterion (Phase 5):                                          */
/*   Demonstrated performance or safety gain that the wrapper cannot  */
/*   reach — direct MIR execution beats wrapper RPC round-trip.       */
/* ------------------------------------------------------------------ */

/* ------------------------------------------------------------------ */
/* MIR types                                                          */
/* ------------------------------------------------------------------ */

export type MirType = "i32" | "i64" | "f64" | "bool" | "void";

export interface MirOperand {
  readonly kind: "imm" | "local" | "arg";
  readonly value: number;
  readonly type: MirType;
}

export interface MirInstruction {
  readonly opcode:
    | "const"
    | "add"
    | "sub"
    | "mul"
    | "ret";
  readonly dest?: MirOperand;
  readonly args: MirOperand[];
}

export interface MirFunction {
  readonly name: string;
  readonly params: MirOperand[];
  readonly resultType: MirType;
  readonly blocks: MirBasicBlock[];
}

export interface MirBasicBlock {
  readonly label: string;
  readonly instructions: MirInstruction[];
}

export interface MirModule {
  readonly functions: MirFunction[];
}

/* ------------------------------------------------------------------ */
/* Helpers                                                            */
/* ------------------------------------------------------------------ */

function imm(v: number, t: MirType): MirOperand {
  return { kind: "imm", value: v, type: t };
}
function local(v: number, t: MirType): MirOperand {
  return { kind: "local", value: v, type: t };
}
function arg(v: number, t: MirType): MirOperand {
  return { kind: "arg", value: v, type: t };
}
function instr(
  op: MirInstruction["opcode"],
  dest: MirOperand | undefined,
  args: MirOperand[],
): MirInstruction {
  return { opcode: op, dest, args };
}

/* ------------------------------------------------------------------ */
/* MIR modules                                                        */
/* ------------------------------------------------------------------ */

/* add(x, y) = x + y */
export function addMirModule(): MirModule {
  return {
    functions: [
      {
        name: "add",
        params: [arg(0, "i32"), arg(1, "i32")],
        resultType: "i32",
        blocks: [
          {
            label: "entry",
            instructions: [
              instr("add", local(0, "i32"), [arg(0, "i32"), arg(1, "i32")]),
              instr("ret", undefined, [local(0, "i32")]),
            ],
          },
        ],
      },
    ],
  };
}

/* fib_iter(n): iterative fibonacci */
export function fibMirModule(): MirModule {
  return {
    functions: [
      {
        name: "fib",
        params: [arg(0, "i32")],
        resultType: "i32",
        blocks: [
          {
            label: "entry",
            instructions: [
              instr("const", local(0, "i32"), [imm(0, "i32")]), // a = 0
              instr("const", local(1, "i32"), [imm(1, "i32")]), // b = 1
              instr("const", local(2, "i32"), [imm(0, "i32")]), // i = 0
              instr("add", local(3, "i32"), [local(0, "i32"), local(1, "i32")]), // tmp = a + b
              instr("const", local(0, "i32"), [local(1, "i32")]), // a = b
              instr("const", local(1, "i32"), [local(3, "i32")]), // b = tmp
              instr("const", local(2, "i32"), [
                { kind: "imm", value: 1, type: "i32" },
                { kind: "local", value: 2, type: "i32" },
              ]), // i = i + 1 (simplified: one iteration)
              instr("ret", undefined, [local(0, "i32")]), // return a
            ],
          },
        ],
      },
    ],
  };
}

/* ------------------------------------------------------------------ */
/* MIR interpreter (direct execution — no wrapper)                    */
/* ------------------------------------------------------------------ */

export interface MirValue {
  readonly type: MirType;
  readonly value: number | boolean;
}

function num(v: number): MirValue {
  return { type: "i32", value: v };
}

export function interpretMir(
  module: MirModule,
  fnName: string,
  args: number[],
): number {
  const fn = module.functions.find((f) => f.name === fnName);
  if (!fn) throw new Error(`function not found: ${fnName}`);

  const block = fn.blocks[0];
  const locals = new Map<number, MirValue>();
  const argValues: MirValue[] = args.map((a) => num(a));

  let pc = 0;
  while (pc < block.instructions.length) {
    const instr = block.instructions[pc];
    pc += 1;

    const resolve = (op: MirOperand): MirValue => {
      if (op.kind === "imm") return { type: op.type, value: op.value };
      if (op.kind === "arg") return argValues[op.value];
      if (op.kind === "local") {
        const v = locals.get(op.value);
        if (!v) throw new Error(`uninitialized local ${op.value}`);
        return v;
      }
      throw new Error(`unsupported operand kind`);
    };

    const write = (dest: MirOperand, v: MirValue) => {
      if (dest.kind === "local") locals.set(dest.value, v);
    };

    switch (instr.opcode) {
      case "const":
        write(instr.dest!, resolve(instr.args[0]));
        break;
      case "add": {
        const a = resolve(instr.args[0]);
        const b = resolve(instr.args[1]);
        write(instr.dest!, num((a.value as number) + (b.value as number)));
        break;
      }
      case "sub": {
        const a = resolve(instr.args[0]);
        const b = resolve(instr.args[1]);
        write(instr.dest!, num((a.value as number) - (b.value as number)));
        break;
      }
      case "mul": {
        const a = resolve(instr.args[0]);
        const b = resolve(instr.args[1]);
        write(instr.dest!, num((a.value as number) * (b.value as number)));
        break;
      }
      case "ret": {
        const v = resolve(instr.args[0]);
        return v.value as number;
      }
      default:
        throw new Error(`unsupported opcode: ${instr.opcode}`);
    }
  }
  throw new Error("function fell off end without return");
}
