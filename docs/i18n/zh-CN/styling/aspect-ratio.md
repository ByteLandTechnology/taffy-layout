---
title: Aspect Ratio（纵横比）
sidebar_position: 15
---

# 🖼️ Aspect Ratio（纵横比）

**保持宽度和高度之间的特定比例。**

`aspectRatio` 属性为元素尺寸设置首选比例。如果设置了一个维度（例如宽度）而另一个维度为 `auto`（高度），Taffy 将计算缺失的维度以满足该比例。

> [!TIP]
> 🔗 **MDN 文档**: [aspect-ratio](https://developer.mozilla.org/zh-CN/docs/Web/CSS/aspect-ratio)

## 🎛️ 用法

传入一个表示 `width / height` 比率的数字。

- `1.0` = 正方形 (1:1)
- `1.5` = 横向 (3:2)
- `1.77` ≈ 16:9
- `0.56` ≈ 9:16

## 💻 示例

```tsx live
const tree = new TaffyTree();

const container = new Style({
  display: Display.Flex,
  size: { width: 300, height: 300 },
  alignItems: AlignItems.Center,
  justifyContent: JustifyContent.Center,
});

const imagePlaceholder = tree.newLeaf(
  new Style({
    // 固定宽度，让高度由比例决定
    size: { width: 100, height: "auto" },
    aspectRatio: 16 / 9, // 宽屏
  }),
);

const root = tree.newWithChildren(container, [imagePlaceholder]);

tree.computeLayout(root, { width: 300, height: 300 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## ⏭️ 后续步骤

- **[Width and Height（宽度和高度）](./size.md)** - 设置显式尺寸。
