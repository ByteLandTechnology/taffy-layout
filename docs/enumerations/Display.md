[**Taffy-JS API Documentation**](../index.md)

---

[Taffy-JS API Documentation](../index.md) / Display

# Enumeration: Display

Display mode enumeration

Controls the layout algorithm type for an element. This corresponds to the CSS `display` property
and determines how an element and its children are laid out.

# JavaScript Usage

```javascript
import { Display } from "taffy-js";

style.display = Display.Flex; // Enable flexbox layout
style.display = Display.Grid; // Enable grid layout
style.display = Display.None; // Hide element from layout
```

# Variants

| Variant | Value | Description                                          |
| ------- | ----- | ---------------------------------------------------- |
| `Block` | 0     | Block-level layout, element takes full width         |
| `Flex`  | 1     | Flexbox layout, one-dimensional layout model         |
| `Grid`  | 2     | CSS Grid layout, two-dimensional layout model        |
| `None`  | 3     | Element is hidden and does not participate in layout |

## Enumeration Members

| Enumeration Member         | Value | Description                                                     |
| -------------------------- | ----- | --------------------------------------------------------------- |
| <a id="block"></a> `Block` | `0`   | Block-level layout where element takes the full available width |
| <a id="flex"></a> `Flex`   | `1`   | Flexbox layout for one-dimensional item arrangement             |
| <a id="grid"></a> `Grid`   | `2`   | CSS Grid layout for two-dimensional item arrangement            |
| <a id="none"></a> `None`   | `3`   | Element is removed from layout calculation entirely             |
