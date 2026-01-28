---
title: Grid Dashboard
sidebar_position: 2
---

# Grid Dashboard

**A dashboard layout with header, navigation, and main content.**

```text
┌────────────────────────────┐
│ Header                     │
├──────────┬─────────────────┤
│ Nav      │ Main            │
└──────────┴─────────────────┘
```

## Key Ideas

- Use Grid for layout structure.
- **Header** spans all columns.
- **Nav** and **Main** sit in the second row.

## Code

```tsx live
const tree = new TaffyTree();

const rootStyle = new Style({
  display: Display.Grid,
  size: { width: 600, height: 320 },
  gap: { width: 12, height: 12 },

  // Columns: Nav (1fr), Main (3fr)
  gridTemplateColumns: [
    { type: "Flex", value: 1 },
    { type: "Flex", value: 3 },
  ],
  // Rows: Header (60px), Content (1fr)
  gridTemplateRows: [
    { type: "Length", value: 60 },
    { type: "Flex", value: 1 },
  ],
});

// Create children
const header = tree.newLeaf(
  new Style({
    gridColumn: { start: 1, end: { span: 2 } }, // Spans both columns
    gridRow: { start: 1, end: { span: 1 } },
  }),
);

const nav = tree.newLeaf(
  new Style({
    gridColumn: { start: 1, end: { span: 1 } },
    gridRow: { start: 2, end: { span: 1 } },
  }),
);

const main = tree.newLeaf(
  new Style({
    gridColumn: { start: 2, end: { span: 1 } },
    gridRow: { start: 2, end: { span: 1 } },
  }),
);

const root = tree.newWithChildren(rootStyle, [header, nav, main]);

tree.computeLayout(root, { width: 600, height: 320 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## Related Guides

- **[Grid Templates](../styling/grid-templates.md)**
- **[Grid Column](../styling/grid-column.md)**
- **[Grid Row](../styling/grid-row.md)**
