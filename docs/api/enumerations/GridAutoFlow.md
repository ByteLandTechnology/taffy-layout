# GridAutoFlow

Grid auto flow enumeration

Controls whether grid items are placed row-wise or column-wise, and whether
the sparse or dense packing algorithm is used.

## Example

```typescript
import { Style, GridAutoFlow } from "taffy-layout";

const style = new Style();
style.gridAutoFlow = GridAutoFlow.Row; // Fill rows first
style.gridAutoFlow = GridAutoFlow.Column; // Fill columns first
style.gridAutoFlow = GridAutoFlow.RowDense; // Fill rows, pack densely
```

## Enumeration Members

| Enumeration Member                     | Value | Description                                                                      |
| -------------------------------------- | ----- | -------------------------------------------------------------------------------- |
| <a id="column"></a> `Column`           | `1`   | Items are placed by filling each column in turn, adding new columns as necessary |
| <a id="columndense"></a> `ColumnDense` | `3`   | Combines `Column` with the dense packing algorithm                               |
| <a id="row"></a> `Row`                 | `0`   | Items are placed by filling each row in turn, adding new rows as necessary       |
| <a id="rowdense"></a> `RowDense`       | `2`   | Combines `Row` with the dense packing algorithm                                  |
