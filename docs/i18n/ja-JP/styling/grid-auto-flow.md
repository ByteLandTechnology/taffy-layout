---
title: Grid Auto Flow（グリッド自動流）
sidebar_position: 19
---

# 🌊 Grid Auto Flow（グリッド自動流）

**自動配置されたアイテムがグリッドに流れ込む方法を制御します。**

明示的なセルよりも多くのアイテムがある場合、またはアイテムを明示的に配置しない場合、`gridAutoFlow` はそれらがグリッドをどのように埋めるかを制御します。

> [!TIP]
> 🔗 **MDN ドキュメント**: [grid-auto-flow](https://developer.mozilla.org/ja/docs/Web/CSS/grid-auto-flow)

## 🎛️ 値

| 値                | 説明                                                               |
| :---------------- | :----------------------------------------------------------------- |
| **`Row`**         | **デフォルト**。アイテムは現在の行を埋めてから次の行に移動します。 |
| **`Column`**      | アイテムは現在の列を埋めてから次の列に移動します。                 |
| **`RowDense`**    | Row に似ていますが、グリッド内の隙間を埋めようとします。           |
| **`ColumnDense`** | Column に似ていますが、隙間を埋めようとします。                    |

## 💻 例

```tsx live
const tree = new TaffyTree();
const rootStyle = new Style({ display: Display.Grid });
const child1 = tree.newLeaf(new Style());
const child2 = tree.newLeaf(new Style());
const child3 = tree.newLeaf(new Style());
const child4 = tree.newLeaf(new Style());

rootStyle.gridTemplateColumns = [
  { min: 60, max: 60 },
  { min: 60, max: 60 },
];
rootStyle.gridTemplateRows = [
  { min: 40, max: 40 },
  { min: 40, max: 40 },
];
rootStyle.gap = { width: 8, height: 8 };
rootStyle.size = { width: 160, height: 100 };
rootStyle.padding = { left: 8, right: 8, top: 8, bottom: 8 };

const root = tree.newWithChildren(rootStyle, [child1, child2, child3, child4]);

tree.computeLayout(root, {
  width: 160,
  height: 100,
});

console.log(`Auto flow: Row`);

return <TaffyTreePreview tree={tree} root={root} />;
```

## 典型的なユースケース

- 自動カードグリッド
- レスポンシブダッシュボード

## API リファレンス

- [GridAutoFlow 列挙型](../../api/enumerations/GridAutoFlow.md)

## 次のステップ

- **[Grid Templates（グリッドテンプレート）](./grid-templates.md)** - グリッド構造を定義します。
- [Grid Placement（グリッド配置）](./grid-placement.md)
