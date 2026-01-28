---
title: グリッド行 (Grid Row)
sidebar_position: 20
---

# Grid Row

グリッド内でアイテムを垂直方向にどこに配置するかを制御します。

**`gridRow`** プロパティは、アイテムがグリッド内のどの行を占有するかを定義します。

## 例

```tsx live
const tree = new TaffyTree();

const childStyle = new Style();
// グリッド線 1 から 3 までの行を占有
childStyle.gridRow = { start: 1, end: { span: 2 } };
childStyle.size = { width: "auto", height: "auto" };
const child = tree.newLeaf(childStyle);

const rootStyle = new Style();
rootStyle.display = Display.Grid;
rootStyle.gridTemplateColumns = [{ min: 60, max: 60 }];
rootStyle.gridTemplateRows = [
  { min: 40, max: 40 },
  { min: 40, max: 40 },
  { min: 40, max: 40 },
];
rootStyle.gap = { width: 8, height: 8 };
rootStyle.size = { width: 100, height: 200 };
rootStyle.padding = { left: 8, right: 8, top: 8, bottom: 8 };

const root = tree.newWithChildren(rootStyle, [child]);

tree.computeLayout(root, {
  width: 100,
  height: 200,
});

console.log(`行の跨ぎ (span): 2`);

return <TaffyTreePreview tree={tree} root={root} />;
```

## クイックノート

- `start` と `end` はグリッド線を定義します。
- 絶対インデックス（1から開始）または相対スパン（例: `{ span: 2 }`）を使用できます。

## 次のステップ

- [グリッド列 (Grid Column)](./grid-column.md)
- [グリッド自動流 (Grid Auto Flow)](./grid-auto-flow.md)
- [グリッドテンプレート (Grid Template)](./grid-templates.md)
