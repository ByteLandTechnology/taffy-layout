[**Taffy-JS API Documentation**](../index.md)

---

[Taffy-JS API Documentation](../index.md) / Position

# Enumeration: Position

Position mode enumeration

Controls how an element is positioned within its parent container.
This corresponds to the CSS `position` property.

# JavaScript Usage

```javascript
import { Position } from "taffy-js";

style.position = Position.Relative; // Normal document flow
style.position = Position.Absolute; // Removed from flow, uses inset values
```

# Variants

| Variant    | Value | Description                                                      |
| ---------- | ----- | ---------------------------------------------------------------- |
| `Relative` | 0     | Element stays in normal document flow                            |
| `Absolute` | 1     | Element is removed from flow and positioned via inset properties |

## Enumeration Members

| Enumeration Member               | Value | Description                                                       |
| -------------------------------- | ----- | ----------------------------------------------------------------- |
| <a id="absolute"></a> `Absolute` | `1`   | Element is positioned relative to its nearest positioned ancestor |
| <a id="relative"></a> `Relative` | `0`   | Element participates in normal document flow                      |
