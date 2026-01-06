[**Taffy-JS API Documentation**](../index.md)

---

[Taffy-JS API Documentation](../index.md) / TaffyError

# Class: TaffyError

Error class thrown when a Taffy operation fails, containing a human-readable error message.

This class wraps the native [`taffy::TaffyError`] type and exposes it to JavaScript
with a readable error message. It is thrown as a JavaScript exception on failure.

**JavaScript Interface**

```typescript
class TaffyError {
  readonly message: string; // Human-readable error description
}
```

**Error Types**

The underlying Taffy errors include:

- `InvalidInputNode`: Node ID doesn't exist in the tree
- `InvalidParentNode`: Specified parent node doesn't exist
- `ChildIndexOutOfBounds`: Child index exceeds available children

## Properties

| Property                       | Modifier   | Type     | Description                                                                                                                                                                                                 |
| ------------------------------ | ---------- | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| <a id="message"></a> `message` | `readonly` | `string` | Gets the human-readable error message # Returns A string describing what went wrong. Examples: - "Node with id 1234 is not present in the Taffy tree" - "Index 5 is out of bounds for node with 3 children" |

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
