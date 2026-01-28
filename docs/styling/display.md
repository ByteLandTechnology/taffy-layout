---
title: Display
sidebar_position: 1
---

# Display

**Define the layout behavior of a node.**

The `display` property determines the inner layout algorithm used for a node's children.

## Values

| Value       | Description                                                                                                   |
| :---------- | :------------------------------------------------------------------------------------------------------------ |
| **`Flex`**  | Use the **Flexbox** algorithm. Children are laid out in rows or columns.                                      |
| **`Grid`**  | Use the **CSS Grid** algorithm. Children are laid out in a 2D grid.                                           |
| **`Block`** | Use the **Block** algorithm. (Currently limited support in Taffy, often behaves like a specific Flex config). |
| **`None`**  | The node is removed from the layout. It takes up zero space and is skipped.                                   |

## Example

```tsx live
// Grid demo
const gridTree = new TaffyTree();
const gridStyle = new Style();
gridStyle.size = { width: 60, height: 40 };
const gridChild1 = gridTree.newLeaf(gridStyle);
const gridChild2 = gridTree.newLeaf(gridStyle);
const gridChild3 = gridTree.newLeaf(gridStyle);
const gridChild4 = gridTree.newLeaf(gridStyle);

const gridRootStyle = new Style();
gridRootStyle.display = Display.Grid;
gridRootStyle.gridTemplateColumns = [
  { min: 0, max: "1fr" },
  { min: 0, max: "1fr" },
];
gridRootStyle.gridTemplateRows = [
  { min: 0, max: "1fr" },
  { min: 0, max: "1fr" },
];
gridRootStyle.gap = { width: 8, height: 8 };
gridRootStyle.padding = { left: 10, right: 10, top: 10, bottom: 10 };
gridRootStyle.size = { width: 140, height: 120 };

const gridRoot = gridTree.newWithChildren(gridRootStyle, [
  gridChild1,
  gridChild2,
  gridChild3,
  gridChild4,
]);

gridTree.computeLayout(gridRoot, {
  width: 140,
  height: 120,
});

console.log(`Flex mode: Flex, Grid columns: 2`);

// Flex demo setup
const flexTree = new TaffyTree();
const flexStyle = new Style();
flexStyle.size = { width: 60, height: 40 };
const flexChild1 = flexTree.newLeaf(flexStyle);
const flexChild2 = flexTree.newLeaf(flexStyle);

const flexRoot = flexTree.newWithChildren(
  new Style({
    display: Display.Flex,
    gap: { width: 8, height: 8 },
    padding: { left: 10, right: 10, top: 10, bottom: 10 },
    size: { width: 140, height: 120 },
  }),
  [flexChild1, flexChild2],
);
flexTree.computeLayout(flexRoot, { width: 140, height: 120 });

return (
  <div style={{ display: "flex", gap: 16, flexWrap: "wrap" }}>
    <TaffyTreePreview tree={flexTree} root={flexRoot} />
    <TaffyTreePreview tree={gridTree} root={gridRoot} />
  </div>
);
```

## Next Steps

- [Sizing](./size.md)
- [Flexbox Properties](./flex-direction.md)
- [Grid Layout](./grid.md)
