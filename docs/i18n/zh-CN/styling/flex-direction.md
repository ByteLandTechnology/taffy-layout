---
title: Flex 方向 (Flex Direction)
sidebar_position: 8
---

# Flex 方向 (Flex Direction)

**定义主轴的方向。**

`flexDirection` 属性确立了主轴，使子元素可以按水平方向（行）或垂直方向（列）排列。

## 取值

| 值                  | 描述                             |
| :------------------ | :------------------------------- |
| **`Row`**           | **默认值**。子元素从左到右排列。 |
| **`Column`**        | 子元素从上到下排列。             |
| **`RowReverse`**    | 子元素从右到左排列。             |
| **`ColumnReverse`** | 子元素从下到上排列。             |

## 图示

```text
Row（行）:
[Item 1] -> [Item 2] -> [Item 3]

Column（列）:
[Item 1]
   |
   v
[Item 2]
   |
   v
[Item 3]
```

## Row (行)

默认行为。子元素从左到右排列。

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

## Row Reverse (反向行)

子元素从右到左排列。

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

## Column (列)

子元素从上到下排列。

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

## Column Reverse (反向列)

子元素从下到上排列。

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

## 后续步骤

- [Flex 换行 (Flex Wrap)](./flex-wrap.md)
- [主轴对齐 (Justify Content)](./justify-content.md)
