---
title: Inset（偏移量）
sidebar_position: 14
---

# 🖼️ Inset（偏移量）

**相对于元素的边缘对元素进行定位。**

`inset` 属性（历史上称为 `top`、`right`、`bottom`、`left`）定义了定位元素的偏移量。其行为取决于 `position` 属性。

> [!TIP]
> 🔗 **MDN 文档**: [inset](https://developer.mozilla.org/zh-CN/docs/Web/CSS/inset)

## 🎛️ 属性

`inset` 是 Taffy（以及 CSS 简写）中的辅助属性，用于设置：

| 属性         | 描述             |
| :----------- | :--------------- |
| **`top`**    | 距离顶部的距离。 |
| **`bottom`** | 距离底部的距离。 |
| **`left`**   | 距离左侧的距离。 |
| **`right`**  | 距离右侧的距离。 |

## 📐 行为

- **对于 `Position.Absolute`**：偏移量相对于*最近的已定位祖先*。
- **对于 `Position.Relative`**：偏移量相对于元素在流中的*正常位置*。

## 💻 示例

```tsx live
const tree = new TaffyTree();

const containerStyle = new Style({
  size: { width: 200, height: 100 },
  display: Display.Flex,
});

// 锚定到右下角的绝对定位元素
const absoluteItem = tree.newLeaf(
  new Style({
    position: Position.Absolute,
    size: { width: 50, height: 50 },

    // 锚定到右下角
    inset: { bottom: 10, right: 10, top: "auto", left: "auto" },
  }),
);

const root = tree.newWithChildren(containerStyle, [absoluteItem]);

tree.computeLayout(root, { width: 200, height: 100 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## ⏭️ 后续步骤

- **[Position（定位）](./position.md)** - 在相对定位和绝对定位之间选择。
