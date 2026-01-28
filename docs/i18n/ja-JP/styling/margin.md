---
title: マージン (Margin)
sidebar_position: 4
---

# マージン (Margin)

要素の外側の間隔を制御します。

**`margin`** は、定義された境界線の外側に、要素の周囲のスペースを作成します。

## 例

```tsx live
const tree = new TaffyTree();

const childStyle = new Style();
childStyle.size = { width: "100%", height: "100%" };
const innerNode = tree.newLeaf(childStyle);

const boxStyle = new Style();
boxStyle.size = { width: 100, height: 100 };
boxStyle.margin = { left: 20, top: 20, right: 20, bottom: 20 };
const boxNode = tree.newWithChildren(boxStyle, [innerNode]);

const rootStyle = new Style();
rootStyle.size = { width: 200, height: 200 };
rootStyle.display = Display.Flex;

const root = tree.newWithChildren(rootStyle, [boxNode]);

tree.computeLayout(root, { width: 200, height: 200 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## クイックノート

- `margin` は、`left`、`right`、`top`、`bottom` を含む `Rect` です。
- `Auto` を受け入れ、コンテンツを中央に配置します（`margin: auto` のように）。

## 次のステップ

- [パディング (Padding)](./padding.md)
- [ボーダー (Border)](./border.md)
