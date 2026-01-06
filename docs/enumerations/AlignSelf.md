[**Taffy-JS API Documentation**](../index.md)

---

[Taffy-JS API Documentation](../index.md) / AlignSelf

# Enumeration: AlignSelf

Cross-axis alignment enumeration for a single element

Overrides the parent's `align-items` value for a specific child element.
This corresponds to the CSS `align-self` property.

# JavaScript Usage

```javascript
import { AlignSelf } from "taffy-js";

style.alignSelf = AlignSelf.Auto; // Use parent's align-items
style.alignSelf = AlignSelf.Center; // Override to center this item
```

# Variants

| Variant     | Value | Description                          |
| ----------- | ----- | ------------------------------------ |
| `Auto`      | 0     | Inherit parent's `align-items` value |
| `Start`     | 1     | Align to cross axis start            |
| `End`       | 2     | Align to cross axis end              |
| `FlexStart` | 3     | Align to flex container start        |
| `FlexEnd`   | 4     | Align to flex container end          |
| `Center`    | 5     | Center along cross axis              |
| `Baseline`  | 6     | Align to text baseline               |
| `Stretch`   | 7     | Stretch to fill cross axis           |

## Enumeration Members

| Enumeration Member                 | Value | Description                                         |
| ---------------------------------- | ----- | --------------------------------------------------- |
| <a id="auto"></a> `Auto`           | `0`   | Inherits the parent container's `align-items` value |
| <a id="baseline"></a> `Baseline`   | `6`   | Item aligned to its text baseline                   |
| <a id="center"></a> `Center`       | `5`   | Item centered along the cross axis                  |
| <a id="end"></a> `End`             | `2`   | Item aligned to the end of the cross axis           |
| <a id="flexend"></a> `FlexEnd`     | `4`   | Item aligned to the end of the flex container       |
| <a id="flexstart"></a> `FlexStart` | `3`   | Item aligned to the start of the flex container     |
| <a id="start"></a> `Start`         | `1`   | Item aligned to the start of the cross axis         |
| <a id="stretch"></a> `Stretch`     | `7`   | Item stretched to fill the container                |
