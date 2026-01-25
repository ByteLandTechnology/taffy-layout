---
title: Grid Layout
sidebar_position: 16
---

# Grid Layout

Taffy’s Grid API mirrors CSS Grid and is best for two-dimensional layouts. Define tracks for rows and columns, then place items by line or area.

## Core Concepts

- **Track**: row or column sizing definition
- **Line**: grid line used for placement
- **Area**: named regions (if you use them)

```
Columns:  1fr 2fr
Rows:     auto 1fr

┌───────────────┐
│ Header        │
├───────┬───────┤
│ Nav   │ Main  │
└───────┴───────┘
```

## Minimal Example

```tsx live
const tree = new TaffyTree();

const rootStyle = new Style({
  display: Display.Grid,
  size: { width: 200, height: 200 },
  // Define 2 columns of 50% width each
  gridTemplateColumns: [
    { min: "50%", max: "50%" },
    { min: "50%", max: "50%" },
  ],
  // Define 2 rows: 50px fixed, and "1fr" (remaining space)
  gridTemplateRows: [
    { min: 50, max: 50 },
    { min: 0, max: "1fr" },
  ],
  gap: { width: 5, height: 5 },
});

const itemStyle = new Style({
  alignContent: AlignContent.Center,
  justifyContent: JustifyContent.Center,
});

const child1 = tree.newLeaf(itemStyle); // 0,0
const child2 = tree.newLeaf(itemStyle); // 0,1
const child3 = tree.newLeaf(itemStyle); // 1,0
const child4 = tree.newLeaf(itemStyle); // 1,1

const root = tree.newWithChildren(rootStyle, [child1, child2, child3, child4]);

tree.computeLayout(root, { width: 200, height: 200 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## ⏭️ Next Steps

- **[Grid Templates](./grid-templates.md)** - Define the grid structure.
- **[Grid Placement](./grid-placement.md)** - Place items into the grid.
- **[Grid Auto Flow](./grid-auto-flow.md)** - Control automatic placement.
