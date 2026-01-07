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

| Enumeration Member             | Value | Description                                                                                 |
| ------------------------------ | ----- | ------------------------------------------------------------------------------------------- |
| <a id="clip"></a> `Clip`       | `1`   | Content is clipped at the container boundary, but unlike Hidden, this forbids all scrolling |
| <a id="hidden"></a> `Hidden`   | `2`   | Content is clipped at the container boundary                                                |
| <a id="scroll"></a> `Scroll`   | `3`   | Always display scrollbars for scrollable content                                            |
| <a id="visible"></a> `Visible` | `0`   | Content is not clipped and may render outside the container                                 |
