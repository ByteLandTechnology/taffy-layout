# DetailedLayoutInfo

```ts
type DetailedLayoutInfo = DetailedGridInfo | undefined;
```

Detailed layout information (for grid layouts).

Returned by `detailedLayoutInfo()` for nodes using CSS Grid layout.
Contains detailed information about grid tracks and item placement.

## Remarks

This is only available when the `detailed_layout_info` feature is enabled.

## Example

```typescript
import {
  TaffyTree,
  Style,
  Display,
  type DetailedLayoutInfo,
  type DetailedGridInfo,
} from "taffy-layout";

const tree = new TaffyTree();
const style = new Style();
style.display = Display.Grid;
const gridNode = tree.newLeaf(style);
tree.computeLayout(gridNode, { width: 100, height: 100 });

const info: DetailedLayoutInfo = tree.detailedLayoutInfo(gridNode);

if (info && typeof info === "object" && "Grid" in info) {
  const grid = info.Grid as DetailedGridInfo;
  console.log("Rows:", grid.rows.sizes);
  console.log("Columns:", grid.columns.sizes);
}
```
