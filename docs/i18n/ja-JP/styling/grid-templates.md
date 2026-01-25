---
title: Grid Template（グリッドテンプレート）
sidebar_position: 17
---

# ▦ Grid Templates（グリッドテンプレート）

**グリッドの行と列を定義します。**

`gridTemplateColumns` と `gridTemplateRows` プロパティは、グリッドのトラックサイズ関数を定義します。

> [!TIP]
> 🔗 **MDN ドキュメント**: [grid-template-columns](https://developer.mozilla.org/ja/docs/Web/CSS/grid-template-columns), [grid-template-rows](https://developer.mozilla.org/ja/docs/Web/CSS/grid-template-rows)

## 🎛️ トラックサイズ

各トラックは `min` と `max` のサイズ関数を使って定義します：

| 値                               | 説明                                           | 例 (JS)                               |
| :------------------------------- | :--------------------------------------------- | :------------------------------------ |
| **Points（ポイント）**           | ピクセル単位の固定サイズ。                     | `{ min: 100, max: 100 }`              |
| **Percent（パーセント）**        | コンテナサイズの割合。                         | `{ min: 0, max: '50%' }`              |
| **Flex (fr)（フレックス）**      | 残りスペースのシェア（分数単位）。             | `{ min: 0, max: '1fr' }`              |
| **Auto（自動）**                 | コンテンツと利用可能なスペースに基づくサイズ。 | `{ min: 'auto', max: 'auto' }`        |
| **MinContent（最小コンテンツ）** | コンテンツに適合する最小のサイズ。             | `{ min: 'min-content', max: 'auto' }` |
| **MaxContent（最大コンテンツ）** | コンテンツに適合する最大のサイズ。             | `{ min: 'auto', max: 'max-content' }` |

## 💻 例

```tsx live
const tree = new TaffyTree();

const rootStyle = new Style({
  display: Display.Grid,
  size: { width: 260, height: 140 },
  gridTemplateColumns: [
    { min: 60, max: 60 },
    { min: 0, max: "1fr" },
    { min: 60, max: 60 },
  ],
  gridTemplateRows: [{ min: 0, max: "1fr" }],
  padding: { left: 10, right: 10, top: 10, bottom: 10 },
  gap: { width: 10, height: 10 },
});

const childStyle = new Style({
  alignSelf: AlignSelf.Center,
  justifySelf: AlignSelf.Center,
});
const child1 = tree.newLeaf(childStyle);
const child2 = tree.newLeaf(childStyle);
const child3 = tree.newLeaf(childStyle);
const child4 = tree.newLeaf(childStyle);
const child5 = tree.newLeaf(childStyle);
const child6 = tree.newLeaf(childStyle);

const root = tree.newWithChildren(rootStyle, [
  child1,
  child2,
  child3,
  child4,
  child5,
  child6,
]);

tree.computeLayout(root, {
  width: 260,
  height: 140,
});

console.log(`Columns: 3`);

return <TaffyTreePreview tree={tree} root={root} />;
```

## API リファレンス

- [GridTemplateComponent](../../api/type-aliases/GridTemplateComponent.md)

## 次のステップ

- [Grid Placement（グリッド配置）](./grid-placement.md)
- [Grid Auto Flow（グリッド自動流）](./grid-auto-flow.md)
