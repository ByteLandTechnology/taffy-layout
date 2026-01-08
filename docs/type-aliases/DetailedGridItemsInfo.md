[**Taffy Layout API Documentation**](../index.md)

---

[Taffy Layout API Documentation](../index.md) / DetailedGridItemsInfo

# Type Alias: DetailedGridItemsInfo

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
Line numbers are 1-indexed, with 1 being the first line.

## Properties

| Property                               | Type     | Description                             |
| -------------------------------------- | -------- | --------------------------------------- |
| <a id="columnend"></a> `columnEnd`     | `number` | Ending column line number (exclusive)   |
| <a id="columnstart"></a> `columnStart` | `number` | Starting column line number (1-indexed) |
| <a id="rowend"></a> `rowEnd`           | `number` | Ending row line number (exclusive)      |
| <a id="rowstart"></a> `rowStart`       | `number` | Starting row line number (1-indexed)    |
