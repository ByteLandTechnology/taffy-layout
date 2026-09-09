---
title: 间距 (Gap)
sidebar_position: 7
---

# 间距 (Gap)

**定义行与列之间的间隔。**

`gap` 属性定义了 Flexbox 和 Grid 布局中行与行、列与列之间的间距（沟槽）。它是 CSS 中 `row-gap` 和 `column-gap` 的简写，在 Taffy 中表示为 `Size<LengthPercentage>`。

## 取值

`gap` 接受一个包含 `width`（列间距）和 `height`（行间距）的 Size 对象，通常以像素或百分比为单位。

| 属性         | 描述                 |
| :----------- | :------------------- |
| **`width`**  | 水平间距（列间距）。 |
| **`height`** | 垂直间距（行间距）。 |

在 Flex 行布局中，`width` 分隔同一行的子元素，`height` 分隔换行后的行；列布局中，`height` 分隔同一列的子元素，`width` 分隔换行后的列。`justifyContent` / `alignContent` 还可能在 gap 之外分配额外空间。

## 示例

```tsx live
const tree = new TaffyTree();

const itemStyle = new Style({ size: { width: 60, height: 40 } });

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  flexWrap: FlexWrap.Wrap,
  size: { width: 200, height: 100 },

  // Gap 严格在子元素之间添加空间，而不是在外边缘
  gap: { width: 10, height: 10 },
});

const children = Array.from({ length: 6 }).map(() => tree.newLeaf(itemStyle));

const root = tree.newWithChildren(rootStyle, children);

tree.computeLayout(root, { width: 200, height: 100 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## 后续步骤

- [Flex 方向 (Flex Direction)](./flex-direction.md)
- [外边距 (Margin)](./margin.md)
