---
title: Grid
sidebar_position: 17
---

# Grid

Taffy’s Grid API mirrors CSS Grid and is best for two-dimensional layouts. Define tracks for rows and columns, then place items by line or area.

## Core Concepts

- **Track**: row or column sizing definition
- **Line**: grid line used for placement
- **Area**: named regions (if you use them)

```text
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
  // Define 2 columns of equal width (1fr)
  gridTemplateColumns: [
    { min: 0, max: "1fr" },
    { min: 0, max: "1fr" },
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

## Inspecting Computed Tracks

After computing layout, `tree.detailedLayoutInfo(node)` returns Grid details, or `null` for a node without Grid details. The returned `rows` and `columns` objects contain:

- `negativeImplicitTracks`, `explicitTracks`, and `positiveImplicitTracks`: the counts before, within, and after the explicit grid.
- `positions`: each track's `{ start, end }` coordinates relative to the container's border box. They include border, padding, gaps, and content alignment offsets.
- `sizes`: each track's `end - start` size.
- `gutters`: actual distances between adjacent tracks, including any extra space distributed by content alignment. The array has one more entry than `sizes`, with `0` at both ends; an empty axis has `gutters: [0]`.

All three arrays use logical track order: top-to-bottom for rows, left-to-right for LTR columns, and right-to-left for RTL columns. Each position still stores physical left/top in `start` and physical right/bottom in `end`. Collapsed auto-fit tracks remain in the arrays with zero size.

`items` contains one-based row and column line numbers relative to the full grid, including implicit tracks. These line numbers follow the same logical direction as the track arrays. Use `positions` when drawing a Grid overlay; the first track can begin away from the container edge even though the first gutter is `0`.

Read these details after laying out a current Grid container with children. The method returns the last stored Grid details: changing the node to another display mode or removing its children can leave earlier details in place, even after recomputing. A node that has never produced Grid details returns `null`. Detailed track coordinates remain unrounded when normal layout rounding is enabled.

## Next Steps

- [Grid Templates](./grid-templates.md)
- [Grid Column](./grid-column.md)
- [Grid Row](./grid-row.md)
