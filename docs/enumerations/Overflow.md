[**Taffy-JS API Documentation**](../index.md)

---

[Taffy-JS API Documentation](../index.md) / Overflow

# Enumeration: Overflow

Overflow handling enumeration

Defines how content that exceeds the container boundaries is handled.
This corresponds to the CSS `overflow` property.

## Example

```typescript
import { Overflow } from "taffy-js";

style.overflow = { x: Overflow.Hidden, y: Overflow.Scroll };
```

## Enumeration Members

| Enumeration Member             | Value | Description                                                                |
| ------------------------------ | ----- | -------------------------------------------------------------------------- |
| <a id="auto"></a> `Auto`       | `3`   | Display scrollbars only when content overflows (internally maps to Scroll) |
| <a id="hidden"></a> `Hidden`   | `1`   | Content is clipped at the container boundary                               |
| <a id="scroll"></a> `Scroll`   | `2`   | Always display scrollbars for scrollable content                           |
| <a id="visible"></a> `Visible` | `0`   | Content is not clipped and may render outside the container                |
