/**
 * Taffy Layout: TypeScript wrapper for Taffy WASM bindings
 *
 * This module provides the `loadTaffy` function for initializing the WASM module
 * and re-exports all types and functions from the underlying WASM bindings.
 *
 * @example
 * ```typescript
 * import { loadTaffy, TaffyTree, Style, Display } from 'taffy-layout';
 *
 * await loadTaffy();
 *
 * const tree = new TaffyTree();
 * const style = new Style();
 * style.display = Display.Flex;
 * const node = tree.newLeaf(style);
 * ```
 */

// Re-export everything from the WASM module
export * from "../pkg/taffy_wasm.js";
export type { InitOutput } from "../pkg/taffy_wasm.js";

// Import the init functions for the loader
import init, { initSync } from "../pkg/taffy_wasm.js";
import type { InitOutput } from "../pkg/taffy_wasm.js";

/**
 * Universal initialization function for Taffy WASM module.
 *
 * Automatically detects the environment (Web or Node.js) and loads the WASM accordingly.
 * - In a **Web environment**, it uses `fetch` to load the WASM file.
 * - In a **Node.js environment**, it uses `fs` to read the WASM file.
 *
 * @returns - A promise that resolves to the WASM module exports.
 *
 * @example
 * ```typescript
 * import { loadTaffy } from 'taffy-layout';
 * await loadTaffy();
 * ```
 */
export async function loadTaffy(): Promise<InitOutput> {
  try {
    // Node.js environment - use fs to read the WASM file
    const fs = await import(/* webpackIgnore: true */ "fs");

    // Resolve WASM path relative to this module
    const wasmUrl = new URL("../pkg/taffy_wasm_bg.wasm", import.meta.url);

    const wasmBuffer = fs.readFileSync(wasmUrl);
    return initSync({ module: wasmBuffer });
  } catch {
    // Web environment - use fetch via the default init function
    return await init();
  }
}

// Default export for convenience
export default loadTaffy;
