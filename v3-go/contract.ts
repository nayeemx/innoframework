import { z } from "zod";

export const FibonacciInputSchema = z.object({
  n: z.number().int().nonnegative().max(10_000_000),
});

export type FibonacciInput = z.infer<typeof FibonacciInputSchema>;

export const FibonacciOutputSchema = z.object({
  result: z.number().int().nonnegative(),
  n: z.number().int().nonnegative(),
});

export type FibonacciOutput = z.infer<typeof FibonacciOutputSchema>;

export const BASE_URL = "http://localhost:8081";