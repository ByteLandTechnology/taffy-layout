---
title: 媒体卡片
---

# 🖼️ 媒体卡片

**图像 + 文本布局，固定媒体和灵活文本内容。**

```text
┌────────────────────────────┐
│ [Image]  Title             │
│          Subtitle          │
└────────────────────────────┘
```

## 💻 代码

```tsx live
const tree = new TaffyTree();

const cardStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  gap: { width: 12, height: 0 },
  size: { width: 420, height: 120 },
  padding: { left: 12, right: 12, top: 12, bottom: 12 },
});

const image = tree.newLeaf(
  new Style({
    size: { width: 80, height: 80 },
  }),
);

const textContainer = tree.newWithChildren(
  new Style({
    display: Display.Flex,
    flexDirection: FlexDirection.Column,
    flexGrow: 1,
    gap: { width: 0, height: 6 },
  }),
  [
    tree.newLeaf(new Style({ size: { width: "100%", height: 20 } })), // 标题占位符
    tree.newLeaf(new Style({ size: { width: "60%", height: 16 } })), // 副标题占位符
  ],
);

const root = tree.newWithChildren(cardStyle, [image, textContainer]);

tree.computeLayout(root, { width: 420, height: 120 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## ⏭️ 相关指南

- **[弹性方向](../styling/flex-direction.md)**
- **[对齐项](../styling/align-items.md)**
