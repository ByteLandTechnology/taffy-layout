---
title: Grid Row
sidebar_position: 20
---

# Grid Row

Control where items are placed vertically in the grid.

The **`gridRow`** property defines which rows an item should occupy within a grid.

## Example

```tsx live
const tree = new TaffyTree();

const childStyle = new Style();
// Occupy rows from grid line 1 to 3
childStyle.gridRow = { start: 1, end: { span: 2 } };
childStyle.size = { width: "auto", height: "auto" };
const child = tree.newLeaf(childStyle);

const rootStyle = new Style();
rootStyle.display = Display.Grid;
rootStyle.gridTemplateColumns = [{ min: 60, max: 60 }];
rootStyle.gridTemplateRows = [
  { min: 40, max: 40 },
  { min: 40, max: 40 },
  { min: 40, max: 40 },
];
rootStyle.gap = { width: 8, height: 8 };
rootStyle.size = { width: 100, height: 200 };
rootStyle.padding = { left: 8, right: 8, top: 8, bottom: 8 };

const root = tree.newWithChildren(rootStyle, [child]);

tree.computeLayout(root, {
  width: 100,
  height: 200,
});

console.log(`Row span: 2`);

return <TaffyTreePreview tree={tree} root={root} />;
```

## Quick Notes

- `start` and `end` define the grid lines.
- You can use absolute indices (1-indexed) or relative spans (e.g., `{ span: 2 }`).

## Next Steps

- [Grid Auto Flow](./grid-auto-flow.md)
- [Grid Templates](./grid-templates.md)
