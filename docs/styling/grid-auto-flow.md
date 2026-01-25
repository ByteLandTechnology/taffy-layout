---
title: Grid Auto Flow
sidebar_position: 19
---

# 🌊 Grid Auto Flow

**Control how auto-placed items flow into the grid.**

If you have more items than explicit cells, or if you don't place items explicitly, `gridAutoFlow` controls how they fill the grid.

> [!TIP]
> 🔗 **MDN Documentation**: [grid-auto-flow](https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-flow)

## 🎛️ Values

| Value             | Description                                                 |
| :---------------- | :---------------------------------------------------------- |
| **`Row`**         | **Default**. Items fill current row, then move to next row. |
| **`Column`**      | Items fill current column, then move to next column.        |
| **`RowDense`**    | Like Row, but attempts to backfill holes in the grid.       |
| **`ColumnDense`** | Like Column, but attempts to backfill holes.                |

## 💻 Example

```tsx live
const tree = new TaffyTree();
const rootStyle = new Style({ display: Display.Grid });
const child1 = tree.newLeaf(new Style());
const child2 = tree.newLeaf(new Style());
const child3 = tree.newLeaf(new Style());
const child4 = tree.newLeaf(new Style());

rootStyle.gridTemplateColumns = [
  { min: 60, max: 60 },
  { min: 60, max: 60 },
];
rootStyle.gridTemplateRows = [
  { min: 40, max: 40 },
  { min: 40, max: 40 },
];
rootStyle.gap = { width: 8, height: 8 };
rootStyle.size = { width: 160, height: 100 };
rootStyle.padding = { left: 8, right: 8, top: 8, bottom: 8 };

const root = tree.newWithChildren(rootStyle, [child1, child2, child3, child4]);

tree.computeLayout(root, {
  width: 160,
  height: 100,
});

console.log(`Auto flow: Row`);

return <TaffyTreePreview tree={tree} root={root} />;
```

## Typical Use Cases

- auto card grids
- responsive dashboards

## API Reference

- [GridAutoFlow Enum](../../api/enumerations/GridAutoFlow.md)

## Next Steps

- **[Grid Templates](./grid-templates.md)** - Define the grid structure.
- [Grid Placement](./grid-placement.md)
