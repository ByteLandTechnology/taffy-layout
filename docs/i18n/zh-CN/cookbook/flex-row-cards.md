---
title: 弹性行卡片
sidebar_position: 1
---

# 弹性行卡片

**一行均匀大小的卡片——非常适合统计数据、工具栏和面板。**

```text
┌────────────────────────────────────┐
│ [Card]  [Card]  [Card]  [Card]     │
└────────────────────────────────────┘
```

## 关键概念

- `display: flex` + `flexDirection: row`
- `flexGrow` 均匀分割剩余空间
- `gap` 控制间距

## 代码

```tsx live
const tree = new TaffyTree();

const style = new Style({
  flexGrow: 1,
  size: { height: "100%", width: "auto" },
});

const children = Array.from({ length: 4 }).map(() => tree.newLeaf(style));

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  gap: { width: 12, height: 0 },
  size: { width: 500, height: 120 },
  padding: { left: 12, right: 12, top: 12, bottom: 12 },
});

const root = tree.newWithChildren(rootStyle, children);

tree.computeLayout(root, { width: 500, height: 120 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## 相关指南

- **[Flex 伸缩](../styling/flex-basis-grow-shrink.md)**
- **[间距](../styling/gap.md)**
