[**Taffy-JS API Documentation**](../index.md)

---

[Taffy-JS API Documentation](../index.md) / BoxSizing

# Enumeration: BoxSizing

Box sizing enumeration

Controls how the total width and height of an element is calculated.
This corresponds to the CSS `box-sizing` property.

## Example

```typescript
import { BoxSizing } from "taffy-js";

style.boxSizing = BoxSizing.BorderBox; // Size includes padding and border
style.boxSizing = BoxSizing.ContentBox; // Size is content only
```

## Enumeration Members

| Enumeration Member                   | Value | Description                                                |
| ------------------------------------ | ----- | ---------------------------------------------------------- |
| <a id="borderbox"></a> `BorderBox`   | `0`   | The width and height properties include padding and border |
| <a id="contentbox"></a> `ContentBox` | `1`   | The width and height properties include only the content   |
