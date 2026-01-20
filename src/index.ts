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
 * Detects if running in a Node.js environment
 */
function isNode(): boolean {
  return (
    typeof process !== "undefined" &&
    process.versions != null &&
    process.versions.node != null
  );
}

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
  if (isNode()) {
    // Node.js environment - use fs to read the WASM file
    const fs = await import(/* webpackIgnore: true */ "fs");
    const url = await import(/* webpackIgnore: true */ "url");
    const path = await import(/* webpackIgnore: true */ "path");

    // Resolve WASM path relative to this module
    const __filename = url.fileURLToPath(import.meta.url);
    const __dirname = path.dirname(__filename);
    const wasmPath = path.join(__dirname, "..", "pkg", "taffy_wasm_bg.wasm");

    const wasmBuffer = fs.readFileSync(wasmPath);
    return initSync({ module: wasmBuffer });
  } else {
    // Web environment - use fetch via the default init function
    return await init();
  }
}

// Default export for convenience
export default loadTaffy;
