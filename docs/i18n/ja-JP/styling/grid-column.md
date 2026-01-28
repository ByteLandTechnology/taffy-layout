---
title: グリッド列 (Grid Column)
sidebar_position: 19
---

# Grid Column

グリッド内でアイテムを水平方向にどこに配置するかを制御します。

**`gridColumn`** プロパティは、アイテムがグリッド内のどの列を占有するかを定義します。

## 例

```tsx live
const tree = new TaffyTree();

const childStyle = new Style();
// グリッド線 1 から 3 までの列を占有
childStyle.gridColumn = { start: 1, end: { span: 2 } };
childStyle.size = { width: "auto", height: "auto" };
const child = tree.newLeaf(childStyle);

const rootStyle = new Style();
rootStyle.display = Display.Grid;
rootStyle.gridTemplateColumns = [
  { min: 60, max: 60 },
  { min: 60, max: 60 },
  { min: 60, max: 60 },
];
rootStyle.gridTemplateRows = [{ min: 40, max: 40 }];
rootStyle.gap = { width: 8, height: 8 };
rootStyle.size = { width: 220, height: 60 };
rootStyle.padding = { left: 8, right: 8, top: 8, bottom: 8 };

const root = tree.newWithChildren(rootStyle, [child]);

tree.computeLayout(root, {
  width: 220,
  height: 60,
});

console.log(`列の跨ぎ (span): 2`);

return <TaffyTreePreview tree={tree} root={root} />;
```

## クイックノート

- `start` と `end` はグリッド線を定義します。
- 絶対インデックス（1から開始）または相対スパン（例: `{ span: 2 }`）を使用できます。

## 次のステップ

- [グリッド行 (Grid Row)](./grid-row.md)
- [グリッドテンプレート (Grid Template)](./grid-templates.md)
