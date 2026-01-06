[**Taffy-JS API Documentation**](../index.md)

---

[Taffy-JS API Documentation](../index.md) / default

# Function: default()

```ts
function default(module_or_path?): Promise<InitOutput>;
```

If `module_or_path` is {RequestInfo} or {URL}, makes a request and
for everything else, calls `WebAssembly.instantiate` directly.

## Parameters

| Parameter         | Type                                                                                                                                                                     | Description                                 |
| ----------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------- |
| `module_or_path?` | \| \{ `module_or_path`: InitInput \| Promise\<InitInput\>; \} \| [`InitInput`](../type-aliases/InitInput.md) \| `Promise`\<[`InitInput`](../type-aliases/InitInput.md)\> | Passing `InitInput` directly is deprecated. |

## Returns

`Promise`\<[`InitOutput`](../interfaces/InitOutput.md)\>
