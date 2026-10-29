---
title: 内嵌偏移 (Inset)
sidebar_position: 23
---

# 内嵌偏移 (Inset)

**相对于元素的边缘对元素进行定位。**

`inset` 属性（历史上称为 `top`、`right`、`bottom`、`left`）定义了定位元素的偏移量。其行为取决于 `position` 属性。

## 属性

`inset` 是 Taffy（以及 CSS 简写）中的辅助属性，用于设置：

| 属性         | 描述             |
| :----------- | :--------------- |
| **`top`**    | 距离顶部的距离。 |
| **`bottom`** | 距离底部的距离。 |
| **`left`**   | 距离左侧的距离。 |
| **`right`**  | 距离右侧的距离。 |

## 行为

- **对于 `Position.Absolute`**：由最近的非 `Static` 祖先建立的包含块定位，通常相对于其 padding box 的边缘；没有此类祖先时使用初始包含块。Grid 中指定网格线时还可限定定位区域。
- **对于 `Position.Fixed`**：使用初始包含块，确定尺寸来自 `availableSpace`，不取决于根节点的尺寸；不确定尺寸及返回坐标的规则见[定位](./position.md)。
- **对于 `Position.Relative`**：偏移量相对于元素在流中的*正常位置*。
- **对于 `Position.Static` 和 `Position.Sticky`**：偏移量被忽略。

## 示例

```tsx live
const tree = new TaffyTree();

const containerStyle = new Style({
  size: { width: 200, height: 100 },
  display: Display.Flex,
  position: Position.Relative, // 绝对定位项的包含块
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

## 后续步骤

- [溢出控制 (Overflow)](./overflow.md)
- [定位 (Position)](./position.md)
