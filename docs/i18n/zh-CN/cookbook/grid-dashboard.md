---
title: 网格仪表板
---

# 📊 网格仪表板

**带有页眉、导航和主内容区域的仪表板布局。**

```text
┌────────────────────────────┐
│ Header                     │
├──────────┬─────────────────┤
│ Nav      │ Main            │
└──────────┴─────────────────┘
```

## 🔑 关键概念

- 使用 Grid 进行布局结构
- **Header** 跨越所有列
- **Nav** 和 **Main** 位于第二行

## 💻 代码

```tsx live
const tree = new TaffyTree();

const rootStyle = new Style({
  display: Display.Grid,
  size: { width: 600, height: 320 },
  gap: { width: 12, height: 12 },

  // 列：Nav (1fr)，Main (3fr)
  gridTemplateColumns: [
    { type: "Flex", value: 1 },
    { type: "Flex", value: 3 },
  ],
  // 行：Header (60px)，Content (1fr)
  gridTemplateRows: [
    { type: "Length", value: 60 },
    { type: "Flex", value: 1 },
  ],
});

// 创建子节点
const header = tree.newLeaf(
  new Style({
    gridColumn: { start: 1, end: { span: 2 } }, // 跨越两列
    gridRow: { start: 1, end: { span: 1 } },
  }),
);

const nav = tree.newLeaf(
  new Style({
    gridColumn: { start: 1, end: { span: 1 } },
    gridRow: { start: 2, end: { span: 1 } },
  }),
);

const main = tree.newLeaf(
  new Style({
    gridColumn: { start: 2, end: { span: 1 } },
    gridRow: { start: 2, end: { span: 1 } },
  }),
);

const root = tree.newWithChildren(rootStyle, [header, nav, main]);

tree.computeLayout(root, { width: 600, height: 320 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## ⏭️ 相关指南

- **[网格模板](../styling/grid-templates.md)**
- **[网格放置](../styling/grid-placement.md)**
