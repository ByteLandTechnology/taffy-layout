[**Taffy-JS API Documentation**](../index.md)

---

[Taffy-JS API Documentation](../index.md) / TaffyError

# Class: TaffyError

Error class thrown when a Taffy operation fails, containing a human-readable error message.

This class wraps the native [`taffy::TaffyError`] type and exposes it to JavaScript
with a readable error message. It is thrown as a JavaScript exception on failure.

## Example

```typescript
try {
  tree.remove(node);
} catch (e) {
  if (e instanceof TaffyError) {
    console.error(e.message);
  }
}
```

## Remarks

The underlying Taffy errors include:

- `InvalidInputNode`: Node ID doesn't exist in the tree
- `InvalidParentNode`: Specified parent node doesn't exist
- `ChildIndexOutOfBounds`: Child index exceeds available children

## Properties

| Property                       | Modifier   | Type     | Description                                                                                                                                                              |
| ------------------------------ | ---------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| <a id="message"></a> `message` | `readonly` | `string` | Gets the human-readable error message **Remarks** Examples: - "Node with id 1234 is not present in the Taffy tree" - "Index 5 is out of bounds for node with 3 children" |

## Methods

### \[dispose\]()

```ts
dispose: void;
```

#### Returns

`void`

---

### free()

```ts
free(): void;
```

#### Returns

`void`
