---
title: Flex Grow、Shrink、Basis
sidebar_position: 4
---

# Flex Grow、Shrink、Basis

フレックスアイテムが利用可能なスペースに合わせてどのように拡大・縮小するかを制御します。

- **`flexGrow`**：必要に応じたフレックスアイテムの拡大能力を定義します。
- **`flexShrink`**：必要に応じたフレックスアイテムの縮小能力を定義します。
- **`flexBasis`**：残りのスペースが分配される前の要素のデフォルトサイズを定義します。

## 💻 例

```tsx live
const tree = new TaffyTree();

const fixedStyle = new Style();
fixedStyle.size = { width: 120, height: "100%" };
const child1 = tree.newLeaf(fixedStyle);

const growStyle = new Style();
growStyle.flexGrow = 1;
growStyle.size = { width: "auto", height: "100%" };
const child2 = tree.newLeaf(growStyle);

const rootStyle = new Style();
rootStyle.display = Display.Flex;
rootStyle.flexDirection = FlexDirection.Row;
rootStyle.size = { width: 240, height: 60 };
rootStyle.gap = { width: 8, height: 0 };
rootStyle.padding = { left: 6, right: 6, top: 6, bottom: 6 };

const root = tree.newWithChildren(rootStyle, [child1, child2]);

tree.computeLayout(root, {
  width: 240,
  height: 60,
});

console.log(`Fixed width: 120, grow: 1`);

return <TaffyTreePreview tree={tree} root={root} />;
```

## クイックノート

- `flexGrow` を設定しない場合、アイテムはコンテンツサイズになります
- `flexBasis` は `auto` サイズに影響します

## ⏭️ 次のステップ

- [Justify Content（主軸揃え）](./justify-content.md)
- [Align Items（交差軸揃え）](./align-items.md) & [Align Content（行揃え）](./align-content.md)
