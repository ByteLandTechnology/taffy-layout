# DetailedLayoutInfo

```ts
type DetailedLayoutInfo = DetailedGridInfo | null;
```

Detailed layout information (for grid layouts).

Returned by `detailedLayoutInfo()` after computing a grid container's layout.
Contains `rows`, `columns`, and `items` directly, or is `null` if no grid details
have been stored. Childless grids use leaf layout and do not generate details.
Previously stored details can remain after changing display mode or removing
children. Read them after laying out a current grid container with children;
a non-null result alone does not establish that the details are current.

## Remarks

This is only available when the `detailed_layout_info` feature is enabled.

## Example

```typescript
import {
  TaffyTree,
  Style,
  Display,
  type DetailedLayoutInfo,
} from "taffy-layout";

const tree = new TaffyTree();
const style = new Style();
style.display = Display.Grid;
const child = tree.newLeaf(new Style());
const gridNode = tree.newWithChildren(style, [child]);
tree.computeLayout(gridNode, { width: 100, height: 100 });

const info: DetailedLayoutInfo = tree.detailedLayoutInfo(gridNode);

if (info !== null) {
  console.log("Rows:", info.rows.sizes);
  console.log("Columns:", info.columns.sizes);
}
```
