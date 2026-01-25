---
title: Flex Direction（弹性方向）
sidebar_position: 2
---

# ➡️ Flex Direction（弹性方向）

**定义主轴的方向。**

`flexDirection` 属性确立了主轴，使子元素可以按水平方向（行）或垂直方向（列）排列。

> [!TIP]
> 🔗 **MDN 文档**: [flex-direction](https://developer.mozilla.org/zh-CN/docs/Web/CSS/flex-direction)

## 🎛️ 取值

| 值                  | 描述                             |
| :------------------ | :------------------------------- |
| **`Row`**           | **默认值**。子元素从左到右排列。 |
| **`Column`**        | 子元素从上到下排列。             |
| **`RowReverse`**    | 子元素从右到左排列。             |
| **`ColumnReverse`** | 子元素从下到上排列。             |

## 📐 图示

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

## 💻 示例

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
  // 修改这里: Row, Column, RowReverse, ColumnReverse
  flexDirection: FlexDirection.Row,
  size: { width: 250, height: 150 },
  padding: { left: 10, right: 10, top: 10, bottom: 10 },
});

const root = tree.newWithChildren(rootStyle, [child1, child2, child3]);

tree.computeLayout(root, { width: 250, height: 150 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## ⏭️ 后续步骤

- **[Flex Wrap（换行）](./flex-wrap.md)** - 处理子元素超出容器时的换行行为。
- **[Justify Content（主轴对齐）](./justify-content.md)** - 沿主轴方向对齐子元素。
- [Align Items（交叉轴对齐）](./align-items.md)
