[**Taffy Layout API Documentation**](../index.md)

---

[Taffy Layout API Documentation](../index.md) / Position

# Enumeration: Position

Position mode enumeration

Controls how an element is positioned within its parent container.
This corresponds to the CSS `position` property.

## Example

```typescript
import { Style, Position } from "taffy-layout";

const style = new Style();
style.position = Position.Relative; // Normal document flow
style.position = Position.Absolute; // Removed from flow, uses inset values
```

## Enumeration Members

| Enumeration Member               | Value | Description                                                       |
| -------------------------------- | ----- | ----------------------------------------------------------------- |
| <a id="absolute"></a> `Absolute` | `1`   | Element is positioned relative to its nearest positioned ancestor |
| <a id="relative"></a> `Relative` | `0`   | Element participates in normal document flow                      |
