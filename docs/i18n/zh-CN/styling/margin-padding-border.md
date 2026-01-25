---
title: Margin, Padding, and Border（边距、填充和边框）
sidebar_position: 11
---

# 📦 Margin, Padding, and Border（边距、填充和边框）

**控制盒模型的间距。**

Taffy 遵循标准的 CSS 盒模型。

> [!TIP]
> 🔗 **MDN 文档**: [盒模型](https://developer.mozilla.org/zh-CN/docs/Web/CSS/CSS_Box_Model/Introduction_to_the_CSS_box_model)

## 🖼️ 盒模型

1.  **Content（内容）**：实际的项目（图像、文本或子元素）。
2.  **Padding（填充）**：内容与边框之间的空间。
3.  **Border（边框）**：盒子的边界。
4.  **Margin（边距）**：边框之外的空间，将其他元素推开。

## 🎛️ 属性

每个属性都是一个包含 `left`、`right`、`top`、`bottom` 的 `Rect`。

| 属性          | 描述                                                      |
| :------------ | :-------------------------------------------------------- |
| **`margin`**  | 外部间距。接受 `Auto` 来居中内容（类似 `margin: auto`）。 |
| **`padding`** | 内部间距。影响容器的尺寸。                                |
| **`border`**  | 边框宽度。Taffy 只计算边框的*空间*；渲染由你自己完成。    |

## 💻 示例

```tsx live
const tree = new TaffyTree();

// 内部内容
const contentNode = tree.newLeaf(
  new Style({
    flexGrow: 1, // 填充可用空间
  }),
);

// 演示盒模型的容器
const boxNode = tree.newWithChildren(
  new Style({
    size: { width: 200, height: 120 },
    display: Display.Flex,

    // 1. Margin（外部）
    margin: { left: 20, top: 20 },

    // 2. Border（边界）- 仅逻辑宽度
    border: { left: 5, right: 5, top: 5, bottom: 5 },

    // 3. Padding（内部）
    padding: { left: 15, right: 15, top: 15, bottom: 15 },
  }),
  [contentNode],
);

// 保存示例的根节点
const root = tree.newWithChildren(
  new Style({
    size: { width: 300, height: 200 },
  }),
  [boxNode],
);

tree.computeLayout(root, { width: 300, height: 200 });

// 可视化层次结构
console.log(tree.printTree(root));

return <TaffyTreePreview tree={tree} root={root} />;
```

## ⏭️ 后续步骤

- **[Gap（间距）](./gap.md)** - Flex/Grid 子元素之间的间距。
- **[Size（尺寸）](./size.md)** - 控制宽度和高度。
