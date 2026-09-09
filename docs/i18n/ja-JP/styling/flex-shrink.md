---
title: フレックス収縮 (Flex Shrink)
sidebar_position: 12
---

# Flex Shrink

flex アイテムが利用可能なスペース内に収まるようにどのように収縮するかを制御します。

**`flexShrink`** プロパティは、必要に応じて flex アイテムが収縮する能力を定義します。これは、負の空きスペースが分配されるときに、他の flex アイテムと比較してどの程度収縮するかを決定する収縮係数を指定します。

## 例

```tsx live
const tree = new TaffyTree();

const noShrinkStyle = new Style();
noShrinkStyle.flexShrink = 0;
noShrinkStyle.size = { width: 200, height: "100%" };
const child1 = tree.newLeaf(noShrinkStyle);

const shrinkStyle = new Style();
shrinkStyle.flexShrink = 1;
shrinkStyle.size = { width: 200, height: "100%" };
const child2 = tree.newLeaf(shrinkStyle);

const rootStyle = new Style();
rootStyle.display = Display.Flex;
rootStyle.flexDirection = FlexDirection.Row;
rootStyle.size = { width: 300, height: 60 }; // 親は子の合計幅より小さい
rootStyle.gap = { width: 8, height: 0 };
rootStyle.padding = { left: 6, right: 6, top: 6, bottom: 6 };

const root = tree.newWithChildren(rootStyle, [child1, child2]);

tree.computeLayout(root, {
  width: 300,
  height: 60,
});

const firstLayout = tree.getLayout(child1);
const secondLayout = tree.getLayout(child2);
console.log(`子 1: ${firstLayout.width}px, 子 2: ${secondLayout.width}px`);
// 子 1: 200px, 子 2: 80px
firstLayout.free();
secondLayout.free();

return <TaffyTreePreview tree={tree} root={root} />;
```

## クイックノート

- デフォルト値は `1` です。収縮量は係数とフレックス基準サイズに応じて分配されます。自動最小サイズや `minSize` に制限されるため、必ずコンテナ内に収まるとは限りません。
- `0` に設定すると、アイテムの収縮を防ぐことができます。

## 次のステップ

- [フレックス基準値 (Flex Basis)](./flex-basis.md)
- [主軸揃え (Justify Content)](./justify-content.md)
