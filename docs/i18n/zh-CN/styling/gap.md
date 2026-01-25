---
title: Gap（间距）
sidebar_position: 9
---

# 🕳️ Gap（间距）

**定义行与列之间的间隔。**

`gap` 属性定义了 Flexbox 和 Grid 布局中行与行、列与列之间的间距（沟槽）。它是 CSS 中 `row-gap` 和 `column-gap` 的简写，在 Taffy 中表示为 `Size<LengthPercentage>`。

> [!TIP]
> 🔗 **MDN 文档**: [gap](https://developer.mozilla.org/zh-CN/docs/Web/CSS/gap)

## 🎛️ 取值

`gap` 接受一个包含 `width`（列间距）和 `height`（行间距）的 Size 对象，通常以像素或百分比为单位。

| 属性         | 描述                             |
| :----------- | :------------------------------- |
| **`width`**  | 行内子元素之间的空间（列间距）。 |
| **`height`** | 行与行之间的空间（行间距）。     |

## 💻 示例

```tsx live
const tree = new TaffyTree();

const itemStyle = new Style({ size: { width: 40, height: 40 } });

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  flexWrap: FlexWrap.Wrap,
  size: { width: 150, height: 100 },

  // Gap 严格在子元素之间添加空间，而不是在外边缘
  gap: { width: 10, height: 10 },
});

const children = Array.from({ length: 6 }).map(() => tree.newLeaf(itemStyle));

const root = tree.newWithChildren(rootStyle, children);

tree.computeLayout(root, { width: 150, height: 100 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## ⏭️ 后续步骤

- **[Margin, Padding, Border（边距、填充、边框）](./margin-padding-border.md)** - 子元素周围的间距。
