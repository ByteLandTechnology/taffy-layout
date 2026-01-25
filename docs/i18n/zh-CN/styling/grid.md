---
title: 网格布局
sidebar_position: 16
---

# 网格布局

Taffy 的 Grid API 与 CSS Grid 类似，最适合用于二维布局。定义行和列的轨道，然后通过线或区域放置项目。

## 核心概念

- **轨道（Track）**：行或列的尺寸定义
- **线（Line）**：用于定位的网格线
- **区域（Area）**：命名区域（如果使用的话）

```
列:  1fr 2fr
行:  auto 1fr

┌───────────────┐
│ Header        │
├───────┬───────┤
│ Nav   │ Main  │
└───────┴───────┘
```

## 最小示例

```tsx live
const tree = new TaffyTree();

const rootStyle = new Style({
  display: Display.Grid,
  size: { width: 200, height: 200 },
  // 定义 2 列，每列 50% 宽度
  gridTemplateColumns: [
    { min: "50%", max: "50%" },
    { min: "50%", max: "50%" },
  ],
  // 定义 2 行：第一行固定 50px，第二行占剩余空间
  gridTemplateRows: [
    { min: 50, max: 50 },
    { min: 0, max: "1fr" },
  ],
  gap: { width: 5, height: 5 },
});

const itemStyle = new Style({
  alignContent: AlignContent.Center,
  justifyContent: JustifyContent.Center,
});

const child1 = tree.newLeaf(itemStyle); // 0,0
const child2 = tree.newLeaf(itemStyle); // 0,1
const child3 = tree.newLeaf(itemStyle); // 1,0
const child4 = tree.newLeaf(itemStyle); // 1,1

const root = tree.newWithChildren(rootStyle, [child1, child2, child3, child4]);

tree.computeLayout(root, { width: 200, height: 200 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## ⏭️ 后续步骤

- **[网格模板](./grid-templates.md)** - 定义网格结构
- **[网格定位](./grid-placement.md)** - 将项目放置到网格中
- **[网格自动流](./grid-auto-flow.md)** - 控制自动放置
