---
title: Grid Template
sidebar_position: 18
---

# ▦ Grid Templates

**Define the rows and columns of your grid.**

The `gridTemplateColumns` and `gridTemplateRows` properties define the line names and track sizing functions of the grid.

## Track Sizing

Each track is defined using a `min` and `max` sizing function:

| Value          | Description                                 | Example (JS)                          |
| :------------- | :------------------------------------------ | :------------------------------------ |
| **Points**     | Fixed size in pixels.                       | `{ min: 100, max: 100 }`              |
| **Percent**    | Percentage of container size.               | `{ min: 0, max: '50%' }`              |
| **Flex (fr)**  | Share of remaining space (Fractional unit). | `{ min: 0, max: '1fr' }`              |
| **Auto**       | Size based on content and available space.  | `{ min: 'auto', max: 'auto' }`        |
| **MinContent** | Smallest possible size that fits content.   | `{ min: 'min-content', max: 'auto' }` |
| **MaxContent** | Largest possible size that fits content.    | `{ min: 'auto', max: 'max-content' }` |

## Example

```tsx live
const tree = new TaffyTree();

const rootStyle = new Style({
  display: Display.Grid,
  size: { width: 260, height: 140 },
  gridTemplateColumns: [
    { min: 60, max: 60 },
    { min: 0, max: "1fr" },
    { min: 60, max: 60 },
  ],
  gridTemplateRows: [{ min: 0, max: "1fr" }],
  padding: { left: 10, right: 10, top: 10, bottom: 10 },
  gap: { width: 10, height: 10 },
});

const childStyle = new Style({
  alignSelf: AlignSelf.Center,
  justifySelf: AlignSelf.Center,
});
const child1 = tree.newLeaf(childStyle);
const child2 = tree.newLeaf(childStyle);
const child3 = tree.newLeaf(childStyle);
const child4 = tree.newLeaf(childStyle);
const child5 = tree.newLeaf(childStyle);
const child6 = tree.newLeaf(childStyle);

const root = tree.newWithChildren(rootStyle, [
  child1,
  child2,
  child3,
  child4,
  child5,
  child6,
]);

tree.computeLayout(root, {
  width: 260,
  height: 140,
});

console.log(`Columns: 3`);

return <TaffyTreePreview tree={tree} root={root} />;
```

## API Reference

- [GridTemplateComponent](../../api/type-aliases/GridTemplateComponent.md)

## Next Steps

- [Grid Column](./grid-column.md)
- [Grid Row](./grid-row.md)
