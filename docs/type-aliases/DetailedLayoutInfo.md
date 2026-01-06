[**Taffy-JS API Documentation**](../index.md)

---

[Taffy-JS API Documentation](../index.md) / DetailedLayoutInfo

# Type Alias: DetailedLayoutInfo

```ts
type DetailedLayoutInfo = DetailedGridInfo | null;
```

Detailed layout information (for grid layouts).

Returned by `detailedLayoutInfo()` for nodes using CSS Grid layout.
Contains detailed information about grid tracks and item placement.

## Remarks

This is only available when the `detailed_layout_info` feature is enabled.

## Example

```typescript
import type { DetailedLayoutInfo, DetailedGridInfo } from "taffy-js";

const info: DetailedLayoutInfo = tree.detailedLayoutInfo(gridNode);

if (info !== "None" && typeof info === "object" && "Grid" in info) {
  const grid: DetailedGridInfo = info.Grid;
  console.log("Rows:", grid.rows.sizes);
  console.log("Columns:", grid.columns.sizes);
}
```
