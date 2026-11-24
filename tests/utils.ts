import { loadTaffy, type InitOutput } from "../src/index";

let wasmInstance: InitOutput | undefined;

export async function setupTaffy(): Promise<void> {
  if (!wasmInstance) {
    wasmInstance = await loadTaffy();
  }
}

export function getTaffyWasm(): InitOutput {
  if (!wasmInstance) {
    throw new Error("Taffy WASM has not been initialized");
  }
  return wasmInstance;
}
