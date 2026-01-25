---
title: Grid Placement（网格放置）
sidebar_position: 18
---

# Grid Placement（网格放置）

使用列和行索引控制子元素在网格中的放置位置。

## 示例：基本放置

```tsx live
const tree = new TaffyTree();

const style = new Style();
style.gridColumn = { start: 1, end: { span: 3 } };
style.size = { width: "auto", height: "auto" };
const child = tree.newLeaf(style);

const rootStyle = new Style();
rootStyle.display = Display.Grid;
rootStyle.gridTemplateColumns = [
  { min: 60, max: 60 },
  { min: 60, max: 60 },
  { min: 60, max: 60 },
];
rootStyle.gridTemplateRows = [
  { min: 40, max: 40 },
  { min: 40, max: 40 },
];
rootStyle.gap = { width: 8, height: 8 };
rootStyle.size = { width: 220, height: 100 };
rootStyle.padding = { left: 8, right: 8, top: 8, bottom: 8 };

const root = tree.newWithChildren(rootStyle, [child]);

tree.computeLayout(root, {
  width: 220,
  height: 100,
});

console.log(`Column span: 3`);

return <TaffyTreePreview tree={tree} root={root} />;
```

## 示例：跨列的 Header

```tsx live
const tree = new TaffyTree();

const style = new Style();
style.gridColumn = { start: 1, end: { span: 3 } };
style.size = { width: "auto", height: "auto" };
const child = tree.newLeaf(style);

const rootStyle = new Style();
rootStyle.display = Display.Grid;
rootStyle.gridTemplateColumns = [
  { min: 60, max: 60 },
  { min: 60, max: 60 },
  { min: 60, max: 60 },
];
rootStyle.gridTemplateRows = [
  { min: 40, max: 40 },
  { min: 40, max: 40 },
];
rootStyle.gap = { width: 8, height: 8 };
rootStyle.size = { width: 220, height: 100 };
rootStyle.padding = { left: 8, right: 8, top: 8, bottom: 8 };

const root = tree.newWithChildren(rootStyle, [child]);

tree.computeLayout(root, {
  width: 220,
  height: 100,
});

console.log(`Header span: 3`);

return <TaffyTreePreview tree={tree} root={root} />;
```

## 快速说明

- `start` 和 `end` 定义网格线（例如索引 `1` 或 `{ span: 3 }`）
- `min` 和 `max` 定义轨道尺寸（min-max）

## 后续步骤

- [Grid Templates（网格模板）](./grid-templates.md)
- [Grid Auto Flow（网格自动流向）](./grid-auto-flow.md)
