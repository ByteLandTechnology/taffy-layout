---
title: Flex Direction
sidebar_position: 8
---

# Flex Direction

**Define the main axis direction.**

The `flexDirection` property establishes the main-axis, causing items to layout either horizontally (rows) or vertically (columns).

## Values

| Value               | Description                                 |
| :------------------ | :------------------------------------------ |
| **`Row`**           | **Default**. Items flow from left to right. |
| **`Column`**        | Items flow from top to bottom.              |
| **`RowReverse`**    | Items flow from right to left.              |
| **`ColumnReverse`** | Items flow from bottom to top.              |

## Visual Guide

```text
Row:
[Item 1] -> [Item 2] -> [Item 3]

Column:
[Item 1]
   |
   v
[Item 2]
   |
   v
[Item 3]
```

## Row

Default behavior. Items are placed from left to right.

```tsx live
const tree = new TaffyTree();

const childStyle = new Style({
  size: { width: 50, height: 40 },
  margin: { left: 4, right: 4, top: 4, bottom: 4 },
});

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  size: { width: 200, height: 160 },
  padding: { left: 8, right: 8, top: 8, bottom: 8 },
});

const root = tree.newWithChildren(rootStyle, [
  tree.newLeaf(childStyle),
  tree.newLeaf(childStyle),
  tree.newLeaf(childStyle),
]);

tree.computeLayout(root, { width: 200, height: 160 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## RowReverse

Items are placed from right to left.

```tsx live
const tree = new TaffyTree();

const childStyle = new Style({
  size: { width: 50, height: 40 },
  margin: { left: 4, right: 4, top: 4, bottom: 4 },
});

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.RowReverse,
  size: { width: 200, height: 160 },
  padding: { left: 8, right: 8, top: 8, bottom: 8 },
});

const root = tree.newWithChildren(rootStyle, [
  tree.newLeaf(childStyle),
  tree.newLeaf(childStyle),
  tree.newLeaf(childStyle),
]);

tree.computeLayout(root, { width: 200, height: 160 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## Column

Items are placed from top to bottom.

```tsx live
const tree = new TaffyTree();

const childStyle = new Style({
  size: { width: 50, height: 40 },
  margin: { left: 4, right: 4, top: 4, bottom: 4 },
});

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Column,
  size: { width: 200, height: 160 },
  padding: { left: 8, right: 8, top: 8, bottom: 8 },
});

const root = tree.newWithChildren(rootStyle, [
  tree.newLeaf(childStyle),
  tree.newLeaf(childStyle),
  tree.newLeaf(childStyle),
]);

tree.computeLayout(root, { width: 200, height: 160 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## ColumnReverse

Items are placed from bottom to top.

```tsx live
const tree = new TaffyTree();

const childStyle = new Style({
  size: { width: 50, height: 40 },
  margin: { left: 4, right: 4, top: 4, bottom: 4 },
});

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.ColumnReverse,
  size: { width: 200, height: 160 },
  padding: { left: 8, right: 8, top: 8, bottom: 8 },
});

const root = tree.newWithChildren(rootStyle, [
  tree.newLeaf(childStyle),
  tree.newLeaf(childStyle),
  tree.newLeaf(childStyle),
]);

tree.computeLayout(root, { width: 200, height: 160 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## Next Steps

- [Flex Wrap](./flex-wrap.md)
- [Justify Content](./justify-content.md)
