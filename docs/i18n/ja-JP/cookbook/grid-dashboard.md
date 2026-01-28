---
title: グリッドダッシュボード
sidebar_position: 2
---

# グリッドダッシュボード

**ヘッダー、ナビゲーション、メインコンテンツエリアを持つダッシュボードレイアウト。**

```text
┌────────────────────────────┐
│ Header                     │
├──────────┬─────────────────┤
│ Nav      │ Main            │
└──────────┴─────────────────┘
```

## キーアイデア

- レイアウト構造に Grid を使用
- **Header** はすべての列にまたがる
- **Nav** と **Main** は 2 行目にある

## コード

```tsx live
const tree = new TaffyTree();

const rootStyle = new Style({
  display: Display.Grid,
  size: { width: 600, height: 320 },
  gap: { width: 12, height: 12 },

  // 列：Nav (1fr)、Main (3fr)
  gridTemplateColumns: [
    { type: "Flex", value: 1 },
    { type: "Flex", value: 3 },
  ],
  // 行：Header (60px)、Content (1fr)
  gridTemplateRows: [
    { type: "Length", value: 60 },
    { type: "Flex", value: 1 },
  ],
});

// 子ノードを作成
const header = tree.newLeaf(
  new Style({
    gridColumn: { start: 1, end: { span: 2 } }, // 両列にまたがる
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

## 関連ガイド

- **[グリッドテンプレート](../styling/grid-templates.md)**
- **[グリッド列](../styling/grid-column.md)**
- **[グリッド行](../styling/grid-row.md)**
