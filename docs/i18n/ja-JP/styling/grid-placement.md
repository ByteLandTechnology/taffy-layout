---
title: Grid Placement（グリッド配置）
sidebar_position: 18
---

# Grid Placement（グリッド配置）

列と行のインデックスを使用して、グリッド内のアイテムの配置位置を制御します。

## 例：基本配置

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

## 例：列にまたがる Header

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

## クイックノート

- `start` と `end` はグリッドラインを定義します（例：インデックス `1` または `{ span: 3 }`）
- `min` と `max` はトラックサイズを定義します（min-max）

## 次のステップ

- [Grid Templates（グリッドテンプレート）](./grid-templates.md)
- [Grid Auto Flow（グリッド自動流）](./grid-auto-flow.md)
