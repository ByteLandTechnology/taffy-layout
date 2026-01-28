---
title: パディング (Padding)
sidebar_position: 5
---

# パディング (Padding)

要素の内側の間隔を制御します。

**`padding`** は、要素のコンテンツとその境界線の間にスペースを作成します。

## 例

```tsx live
const tree = new TaffyTree();

const childStyle = new Style();
childStyle.flexGrow = 1;
const innerNode = tree.newLeaf(childStyle);

const boxStyle = new Style();
boxStyle.size = { width: 150, height: 100 };
boxStyle.display = Display.Flex;
boxStyle.padding = { left: 20, top: 20, right: 20, bottom: 20 };
const boxNode = tree.newWithChildren(boxStyle, [innerNode]);

const rootStyle = new Style();
rootStyle.size = { width: 200, height: 200 };

const root = tree.newWithChildren(rootStyle, [boxNode]);

tree.computeLayout(root, { width: 200, height: 200 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## クイックノート

- `padding` は、`left`、`right`、`top`、`bottom` を含む `Rect` です。
- これは、子要素に利用可能な内部サイズに影響します。

## 次のステップ

- [ボーダー (Border)](./border.md)
- [ギャップ (Gap)](./gap.md)
