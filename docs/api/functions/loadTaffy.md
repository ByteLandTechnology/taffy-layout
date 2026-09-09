# loadTaffy()

```ts
function loadTaffy(): Promise<InitOutput>;
```

Universal initialization function for Taffy WASM module.

Automatically detects the environment (Web or Node.js) and loads the WASM accordingly.

- In a **Web environment**, it uses `fetch` to load the WASM file.
- In a **Node.js environment**, it uses `fs` to read the WASM file.

## Returns

`Promise`\<[`InitOutput`](../interfaces/InitOutput.md)\>

- A promise that resolves to the WASM module exports.

## Example

```typescript
import { loadTaffy } from "taffy-layout";
await loadTaffy();
```
