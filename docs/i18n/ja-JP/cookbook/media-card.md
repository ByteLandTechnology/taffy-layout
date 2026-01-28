---
title: メディアカード
sidebar_position: 5
---

# メディアカード

**画像 + テキストレイアウト、固定メディアとフレキシブルなテキストコンテンツ。**

```text
┌────────────────────────────┐
│ [Image]  Title             │
│          Subtitle          │
└────────────────────────────┘
```

## コード

```tsx live
const tree = new TaffyTree();

const cardStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  gap: { width: 12, height: 0 },
  size: { width: 420, height: 120 },
  padding: { left: 12, right: 12, top: 12, bottom: 12 },
});

const image = tree.newLeaf(
  new Style({
    size: { width: 80, height: 80 },
  }),
);

const textContainer = tree.newWithChildren(
  new Style({
    display: Display.Flex,
    flexDirection: FlexDirection.Column,
    flexGrow: 1,
    gap: { width: 0, height: 6 },
  }),
  [
    tree.newLeaf(new Style({ size: { width: "100%", height: 20 } })), // タイトルプレースホルダー
    tree.newLeaf(new Style({ size: { width: "60%", height: 16 } })), // サブタイトルプレースホルダー
  ],
);

const root = tree.newWithChildren(cardStyle, [image, textContainer]);

tree.computeLayout(root, { width: 420, height: 120 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## 関連ガイド

- **[フレックス方向](../styling/flex-direction.md)**
- **[アラインアイテム](../styling/align-items.md)**
