[**Taffy-JS API Documentation**](../index.md)

---

[Taffy-JS API Documentation](../index.md) / FlexDirection

# Enumeration: FlexDirection

Flex direction enumeration

Defines the main axis direction for flex item layout. This corresponds to the CSS
`flex-direction` property and determines how flex items are placed within the container.

## Example

```typescript
import { FlexDirection } from "taffy-js";

style.flexDirection = FlexDirection.Row; // Horizontal, left to right
style.flexDirection = FlexDirection.Column; // Vertical, top to bottom
```

## Enumeration Members

| Enumeration Member                         | Value | Description                                    |
| ------------------------------------------ | ----- | ---------------------------------------------- |
| <a id="column"></a> `Column`               | `1`   | Main axis runs vertically from top to bottom   |
| <a id="columnreverse"></a> `ColumnReverse` | `3`   | Main axis runs vertically from bottom to top   |
| <a id="row"></a> `Row`                     | `0`   | Main axis runs horizontally from left to right |
| <a id="rowreverse"></a> `RowReverse`       | `2`   | Main axis runs horizontally from right to left |
