---
title: Flex Direction
sidebar_position: 2
---

# ➡️ Flex Direction

**Define the main axis direction.**

The `flexDirection` property establishes the main-axis, causing items to layout either horizontally (rows) or vertically (columns).

> [!TIP]
> 🔗 **MDN Documentation**: [flex-direction](https://developer.mozilla.org/en-US/docs/Web/CSS/flex-direction)

## 🎛️ Values

| Value               | Description                                 |
| :------------------ | :------------------------------------------ |
| **`Row`**           | **Default**. Items flow from left to right. |
| **`Column`**        | Items flow from top to bottom.              |
| **`RowReverse`**    | Items flow from right to left.              |
| **`ColumnReverse`** | Items flow from bottom to top.              |

## 📐 Visual Guide

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

## 💻 Example

```tsx live
const tree = new TaffyTree();

const style = new Style({
  size: { width: 40, height: 40 },
  margin: { bottom: 5, right: 5 },
});

const child1 = tree.newLeaf(style);
const child2 = tree.newLeaf(style);
const child3 = tree.newLeaf(style);

const rootStyle = new Style({
  display: Display.Flex,
  // CHANGE THIS: Row, Column, RowReverse, ColumnReverse
  flexDirection: FlexDirection.Row,
  size: { width: 250, height: 150 },
  padding: { left: 10, right: 10, top: 10, bottom: 10 },
});

const root = tree.newWithChildren(rootStyle, [child1, child2, child3]);

tree.computeLayout(root, { width: 250, height: 150 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## ⏭️ Next Steps

- **[Flex Wrap](./flex-wrap.md)** - Handle items overflowing the line.
- **[Justify Content](./justify-content.md)** - Align items along the direction you chose.
- [Align Items](./align-items.md)
