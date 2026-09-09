---
title: グリッドレイアウト (Grid)
sidebar_position: 17
---

# グリッドレイアウト (Grid)

Taffy の Grid API は CSS Grid に似ており、2次元レイアウトに最適です。行と列のトラックを定義し、線または領域でアイテムを配置します。

## 基本概念

- **トラック（Track）**：行または列のサイジング定義
- **ライン（Line）**：配置に使用されるグリッド線
- **エリア（Area）**：名前付き領域（使用する場合）

```text
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
  // 2つの列を定義（残りのスペースを等分する 1fr）
  gridTemplateColumns: [
    { min: 0, max: "1fr" },
    { min: 0, max: "1fr" },
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

## 名前付き領域とテンプレートのサイズ

`gridTemplateAreas` は名前付き領域の配列です。`rowStart` / `columnStart` は `1` 始まりで、`rowEnd` / `columnEnd` は領域に含まれない終端のグリッド線です。配列だけを設定した場合、各領域の最大終端位置からテンプレートの行列数を推定します。

```typescript
const style = new Style({
  display: Display.Grid,
  gridTemplateAreas: [
    { name: "main", rowStart: 1, rowEnd: 2, columnStart: 1, columnEnd: 2 },
  ],
  gridTemplateAreaRowCount: 2,
  gridTemplateAreaColumnCount: 3,
});
```

このテンプレートは `2` 行 `3` 列で、名前のないセルは CSS の `.` に相当します。明示的な行列数は最小値であり、読み取った実効サイズは現在の名前付き領域より小さくなりません。配列を置換または空にすると領域の範囲を再計算し、2 つの count プロパティで明示的に指定した最小値だけを保持します。count を `0` にすると、その軸の明示的な最小値を解除します。`getStyle()`、`setStyle()`、測定コールバックのスタイルコピーでもこの区別を保持します。

## 次のステップ

- [グリッドテンプレート (Grid Templates)](./grid-templates.md)
- [グリッド列 (Grid Column)](./grid-column.md)
- [グリッド行 (Grid Row)](./grid-row.md)
