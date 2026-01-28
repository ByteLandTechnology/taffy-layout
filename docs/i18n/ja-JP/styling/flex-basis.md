---
title: フレックス基準値 (Flex Basis)
sidebar_position: 10
---

# Flex Basis

残りのスペースが分配される前の要素のデフォルトサイズを定義します。

**`flexBasis`** プロパティは、flex アイテムの初期のメインサイズを指定します。

## 例

```tsx live
const tree = new TaffyTree();

const style1 = new Style();
style1.flexBasis = 100;
style1.size = { width: "auto", height: "100%" };
const child1 = tree.newLeaf(style1);

const style2 = new Style();
style2.flexBasis = 200;
style2.size = { width: "auto", height: "100%" };
const child2 = tree.newLeaf(style2);

const rootStyle = new Style();
rootStyle.display = Display.Flex;
rootStyle.flexDirection = FlexDirection.Row;
rootStyle.size = { width: 350, height: 60 };
rootStyle.gap = { width: 8, height: 0 };
rootStyle.padding = { left: 6, right: 6, top: 6, bottom: 6 };

const root = tree.newWithChildren(rootStyle, [child1, child2]);

tree.computeLayout(root, {
  width: 350,
  height: 60,
});

console.log(`子 1: flexBasis 100, 子 2: flexBasis 200`);

return <TaffyTreePreview tree={tree} root={root} />;
```

## クイックノート

- `flexBasis` は `auto` のサイジングに影響を与えます。
- `width`（または `flexDirection` によっては `height`）に似ていますが、flex アイテムに固有のものです。

## 次のステップ

- [フレックス成長 (Flex Grow)](./flex-grow.md)
- [フレックス収縮 (Flex Shrink)](./flex-shrink.md)
