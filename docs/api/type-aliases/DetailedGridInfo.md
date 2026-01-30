# DetailedGridInfo

```ts
type DetailedGridInfo = {
  columns: DetailedGridTracksInfo;
  items: DetailedGridItemsInfo[];
  rows: DetailedGridTracksInfo;
};
```

Detailed information about a grid layout.

Contains information about grid rows, columns, and item placement.

## Properties

| Property                       | Type                                                  | Description                         |
| ------------------------------ | ----------------------------------------------------- | ----------------------------------- |
| <a id="columns"></a> `columns` | [`DetailedGridTracksInfo`](DetailedGridTracksInfo.md) | Information about column tracks     |
| <a id="items"></a> `items`     | [`DetailedGridItemsInfo`](DetailedGridItemsInfo.md)[] | Array of item placement information |
| <a id="rows"></a> `rows`       | [`DetailedGridTracksInfo`](DetailedGridTracksInfo.md) | Information about row tracks        |
