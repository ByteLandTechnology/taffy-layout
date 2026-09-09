# DetailedGridItemsInfo

```ts
type DetailedGridItemsInfo = {
  columnEnd: number;
  columnStart: number;
  rowEnd: number;
  rowStart: number;
};
```

Information about a grid item's placement.

Specifies which grid lines the item spans on both axes.
Line numbers are 1-indexed from the first generated line, including leading
implicit tracks. They can differ from the explicit grid line numbers used in styles.

## Properties

| Property                               | Type     | Description                             |
| -------------------------------------- | -------- | --------------------------------------- |
| <a id="columnend"></a> `columnEnd`     | `number` | Ending column line number (exclusive)   |
| <a id="columnstart"></a> `columnStart` | `number` | Starting column line number (1-indexed) |
| <a id="rowend"></a> `rowEnd`           | `number` | Ending row line number (exclusive)      |
| <a id="rowstart"></a> `rowStart`       | `number` | Starting row line number (1-indexed)    |
