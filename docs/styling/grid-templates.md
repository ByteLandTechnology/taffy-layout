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
  size: { width: 24, height: 24 },
  alignSelf: AlignSelf.Center,
  justifySelf: AlignSelf.Center,
});
const child1 = tree.newLeaf(childStyle);
const child2 = tree.newLeaf(childStyle);
const child3 = tree.newLeaf(childStyle);
// These items create an implicit second row.
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

## Named Areas and Template Dimensions

`gridTemplateAreas` is an array of named rectangles. Their `rowStart`, `rowEnd`, `columnStart`, and `columnEnd` values are one-based grid line numbers, with end lines exclusive.

```typescript
import { Display, Style } from "taffy-layout";

const namedGrid = new Style({
  display: Display.Grid,
  gridTemplateAreas: [
    { name: "header", rowStart: 1, rowEnd: 2, columnStart: 1, columnEnd: 3 },
  ],
  gridTemplateAreaRowCount: 3,
  gridTemplateAreaColumnCount: 4,
});
```

The named area above spans one row and two columns. The count properties extend the template to three rows and four columns, including unnamed cells (the `.` cells in CSS grid-template-areas).

The effective counts are at least the largest named end line minus one. Reading either count returns this effective dimension. Replacing or clearing `gridTemplateAreas` recalculates the inferred dimensions; only explicitly assigned counts persist. Set a count to `0` to remove that axis's explicit minimum. These rules also apply to `Style.set()`, styles returned by `tree.getStyle()`, and style copies passed to measure callbacks.

Use `gridTemplateRowNames` and `gridTemplateColumnNames` for arrays of names attached to grid lines. They are separate from the named area rectangles.

## API Reference

- [GridTemplateComponent](../api/type-aliases/GridTemplateComponent.md)
- [GridTemplateArea](../api/type-aliases/GridTemplateArea.md)

## Next Steps

- [Grid Column](./grid-column.md)
- [Grid Row](./grid-row.md)
