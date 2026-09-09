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
- 親の高さ: `auto`、`minSize.height` はビューポートの高さ
- コンテンツ: `flexGrow: 1`
- ヘッダー、コンテンツ、フッター: `flexShrink: 0`

## コード

```tsx live
const tree = new TaffyTree();

// ページコンテナ
const pageStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Column,
  size: { width: 300, height: "auto" },
  minSize: { width: "auto", height: 300 }, // 少なくともビューポートの高さ
});

const header = tree.newLeaf(
  new Style({
    size: { width: "100%", height: 50 },
    marginBottom: 10,
    flexShrink: 0,
  }),
);
const footer = tree.newLeaf(
  new Style({
    size: { width: "100%", height: 50 },
    marginTop: 10,
    flexShrink: 0,
  }),
);

// コンテンツがスペースを埋めるために成長
const content = tree.newLeaf(
  new Style({
    flexGrow: 1,
    flexShrink: 0,
    size: { width: "100%", height: "auto" },
  }),
);

const root = tree.newWithChildren(pageStyle, [header, content, footer]);

tree.computeLayout(root, { width: 300, height: 300 });

return <TaffyTreePreview tree={tree} root={root} />;
```

この空のコンテンツではフッターの `y` は `250` です。コンテンツの高さを `400` にすると、ページは `520`、フッターの `y` は `470` になり、内容を押し縮めずに下へ伸びます。この例は Flexbox による配置で、CSS の `position: sticky` を実装するものではありません。

## 関連ガイド

- **[Flex Grow/Shrink](../styling/flex-grow.md)**
- **[サイジング](../styling/size.md)**
