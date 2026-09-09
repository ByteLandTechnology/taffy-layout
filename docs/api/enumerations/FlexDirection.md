# FlexDirection

Flex direction enumeration

Defines the main axis direction for flex item layout. This corresponds to the CSS
`flex-direction` property and determines how flex items are placed within the container.

## Example

```typescript
import { Style, FlexDirection } from "taffy-layout";

const style = new Style();
style.flexDirection = FlexDirection.Row; // Horizontal, following direction (LTR by default)
style.flexDirection = FlexDirection.Column; // Vertical, top to bottom
```

## Enumeration Members

| Enumeration Member                                            | Value | Description                                                                 |
| ------------------------------------------------------------- | ----- | --------------------------------------------------------------------------- |
| <a id="enumeration-member-column"></a> `Column`               | `1`   | Main axis runs vertically from top to bottom                                |
| <a id="enumeration-member-columnreverse"></a> `ColumnReverse` | `3`   | Main axis runs vertically from bottom to top                                |
| <a id="enumeration-member-row"></a> `Row`                     | `0`   | Main axis follows the container's horizontal writing direction (LTR or RTL) |
| <a id="enumeration-member-rowreverse"></a> `RowReverse`       | `2`   | Main axis runs opposite to the container's horizontal writing direction     |
