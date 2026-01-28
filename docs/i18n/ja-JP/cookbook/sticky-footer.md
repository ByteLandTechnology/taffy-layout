---
title: スティッキーフッター
sidebar_position: 4
---

# スティッキーフッター

**コンテンツが短い場合、フッターは下部に固定されます。コンテンツが増えると、フッターは自然に下に移動します。**

```text
┌────────────────────────────┐
│ Header                     │
├────────────────────────────┤
│ Content (flex: 1)          │
├────────────────────────────┤
│ Footer                     │
└────────────────────────────┘
```

## 主要なルール

- 親: `flexDirection: column`
- コンテンツ: `flexGrow: 1`

## コード

```tsx live
const tree = new TaffyTree();

// ページコンテナ
const pageStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Column,
  size: { width: 300, height: 300 }, // ビューポートをシミュレートするための固定高さ
});

const header = tree.newLeaf(
  new Style({ size: { width: "100%", height: 50 }, margin: { bottom: 10 } }),
);
const footer = tree.newLeaf(
  new Style({ size: { width: "100%", height: 50 }, margin: { top: 10 } }),
);

// コンテンツがスペースを埋めるために成長
const content = tree.newLeaf(
  new Style({
    flexGrow: 1,
    size: { width: "100%", height: "auto" },
  }),
);

const root = tree.newWithChildren(pageStyle, [header, content, footer]);

tree.computeLayout(root, { width: 300, height: 300 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## 関連ガイド

- **[Flex Grow/Shrink](../styling/flex-basis-grow-shrink.md)**
- **[サイジング](../styling/size.md)**
