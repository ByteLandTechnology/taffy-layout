---
title: ボーダー (Border)
sidebar_position: 6
---

# ボーダー (Border)

要素の枠線の幅を制御します。

**`border`** は、ボックスの境界の厚さを定義します。

## 例

```tsx live
const tree = new TaffyTree();

const childStyle = new Style();
childStyle.flexGrow = 1;
const innerNode = tree.newLeaf(childStyle);

const boxStyle = new Style();
boxStyle.size = { width: 150, height: 100 };
boxStyle.display = Display.Flex;
boxStyle.border = { left: 10, top: 10, right: 10, bottom: 10 };
const boxNode = tree.newWithChildren(boxStyle, [innerNode]);

const rootStyle = new Style();
rootStyle.size = { width: 200, height: 200 };

const root = tree.newWithChildren(rootStyle, [boxNode]);

tree.computeLayout(root, { width: 200, height: 200 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## クイックノート

- `border` は、`left`、`right`、`top`、`bottom` を含む `Rect` です。
- Taffy はボーダーのための **スペース** のみを計算します。実際のボーダーのレンダリングは、使用しているレンダリングエンジンに依存します。

## 次のステップ

- [ギャップ (Gap)](./gap.md)
- [サイジング (Sizing)](./size.md)
