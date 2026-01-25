---
title: Grid Placement
sidebar_position: 18
---

# Grid Placement

Control where items are placed in the grid using column and row indices.

## Example: Basic Placement

```tsx live
const tree = new TaffyTree();

const style = new Style();
style.gridColumn = { start: 1, end: { span: 3 } };
style.size = { width: "auto", height: "auto" };
const child = tree.newLeaf(style);

const rootStyle = new Style();
rootStyle.display = Display.Grid;
rootStyle.gridTemplateColumns = [
  { min: 60, max: 60 },
  { min: 60, max: 60 },
  { min: 60, max: 60 },
];
rootStyle.gridTemplateRows = [
  { min: 40, max: 40 },
  { min: 40, max: 40 },
];
rootStyle.gap = { width: 8, height: 8 };
rootStyle.size = { width: 220, height: 100 };
rootStyle.padding = { left: 8, right: 8, top: 8, bottom: 8 };

const root = tree.newWithChildren(rootStyle, [child]);

tree.computeLayout(root, {
  width: 220,
  height: 100,
});

console.log(`Column span: 3`);

return <TaffyTreePreview tree={tree} root={root} />;
```

## Example: Header Spanning Columns

```tsx live
const tree = new TaffyTree();

const style = new Style();
style.gridColumn = { start: 1, end: { span: 3 } };
style.size = { width: "auto", height: "auto" };
const child = tree.newLeaf(style);

const rootStyle = new Style();
rootStyle.display = Display.Grid;
rootStyle.gridTemplateColumns = [
  { min: 60, max: 60 },
  { min: 60, max: 60 },
  { min: 60, max: 60 },
];
rootStyle.gridTemplateRows = [
  { min: 40, max: 40 },
  { min: 40, max: 40 },
];
rootStyle.gap = { width: 8, height: 8 };
rootStyle.size = { width: 220, height: 100 };
rootStyle.padding = { left: 8, right: 8, top: 8, bottom: 8 };

const root = tree.newWithChildren(rootStyle, [child]);

tree.computeLayout(root, {
  width: 220,
  height: 100,
});

console.log(`Header span: 3`);

return <TaffyTreePreview tree={tree} root={root} />;
```

## Quick Notes

- `start` and `end` define the grid lines (e.g., index `1` or `{ span: 3 }`)
- `min` and `max` define track sizing (min-max)

## Next Steps

- [Grid Templates](./grid-templates.md)
- [Grid Auto Flow](./grid-auto-flow.md)
