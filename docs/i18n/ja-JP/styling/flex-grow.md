---
title: フレックス成長 (Flex Grow)
sidebar_position: 11
---

# Flex Grow

flex アイテムが利用可能なスペースを埋めるためにどのように成長するかを制御します。

**`flexGrow`** プロパティは、必要に応じて flex アイテムが成長する能力を定義します。これは比率として機能する単位なしの値を受け入れます。flex コンテナ内の残りのスペースのうち、アイテムがどの程度占有すべきかを指定します。

## 例

```tsx live
const tree = new TaffyTree();

const fixedStyle = new Style();
fixedStyle.size = { width: 100, height: "100%" };
const child1 = tree.newLeaf(fixedStyle);

const growStyle = new Style();
growStyle.flexGrow = 1;
growStyle.size = { width: "auto", height: "100%" };
const child2 = tree.newLeaf(growStyle);

const rootStyle = new Style();
rootStyle.display = Display.Flex;
rootStyle.flexDirection = FlexDirection.Row;
rootStyle.size = { width: 300, height: 60 };
rootStyle.gap = { width: 8, height: 0 };
rootStyle.padding = { left: 6, right: 6, top: 6, bottom: 6 };

const root = tree.newWithChildren(rootStyle, [child1, child2]);

tree.computeLayout(root, {
  width: 300,
  height: 60,
});

console.log(`子 1: 100px, 子 2: 成長 (grow: 1)`);

return <TaffyTreePreview tree={tree} root={root} />;
```

## クイックノート

- `flexGrow` がない場合、アイテムはコンテンツまたは定義された `size` に基づいてサイズが決定されます。
- すべてのアイテムに `flexGrow: 1` が設定されている場合、コンテナ内の残りのスペースはすべてのアイテムに均等に分配されます。

## 次のステップ

- [フレックス収縮 (Flex Shrink)](./flex-shrink.md)
- [フレックス基準値 (Flex Basis)](./flex-basis.md)
