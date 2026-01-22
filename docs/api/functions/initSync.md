[**Taffy Layout API Documentation**](../index.md)

---

[Taffy Layout API Documentation](../index.md) / initSync

# Function: initSync()

```ts
function initSync(module): InitOutput;
```

Instantiates the given `module`, which can either be bytes or
a precompiled `WebAssembly.Module`.

## Parameters

| Parameter | Type                                                                                                                           | Description                                     |
| --------- | ------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------- |
| `module`  | \| \{ `module`: [`SyncInitInput`](../type-aliases/SyncInitInput.md); \} \| [`SyncInitInput`](../type-aliases/SyncInitInput.md) | Passing `SyncInitInput` directly is deprecated. |

## Returns

[`InitOutput`](../interfaces/InitOutput.md)
