---
title: グリッドレイアウト
sidebar_position: 16
---

# グリッドレイアウト

Taffy の Grid API は CSS Grid に似ており、2次元レイアウトに最適です。行と列のトラックを定義し、線または領域でアイテムを配置します。

## 基本概念

- **トラック（Track）**：行または列のサイジング定義
- **ライン（Line）**：配置に使用されるグリッド線
- **エリア（Area）**：名前付き領域（使用する場合）

```
列:  1fr 2fr
行:  auto 1fr

┌───────────────┐
│ Header        │
├───────┬───────┤
│ Nav   │ Main  │
└───────┴───────┘
```

## 最小限の例

```tsx live
const tree = new TaffyTree();

const rootStyle = new Style({
  display: Display.Grid,
  size: { width: 200, height: 200 },
  // 2つの列を定義（各50%幅）
  gridTemplateColumns: [
    { min: "50%", max: "50%" },
    { min: "50%", max: "50%" },
  ],
  // 2つの行を定義：1行目は固定50px、2行目は残りスペース
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

## ⏭️ 次のステップ

- **[グリッドテンプレート](./grid-templates.md)** - グリッド構造を定義する
- **[グリッド配置](./grid-placement.md)** - アイテムをグリッドに配置する
- **[グリッド自動フロー](./grid-auto-flow.md)** - 自動配置を制御する
