---
title: フレックス行カード
---

# 🃏 フレックス行カード

**均一サイズのカードの行——統計、ツールバー、パネルに最適です。**

```text
┌────────────────────────────────────┐
│ [Card]  [Card]  [Card]  [Card]     │
└────────────────────────────────────┘
```

## 🔑 キーアイデア

- `display: flex` + `flexDirection: row`
- `flexGrow` で残りのスペースを均等に分割
- `gap` でスペーシングを制御

## 💻 コード

```tsx live
const tree = new TaffyTree();

const style = new Style({
  flexGrow: 1,
  size: { height: "100%", width: "auto" },
});

const children = Array.from({ length: 4 }).map(() => tree.newLeaf(style));

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  gap: { width: 12, height: 0 },
  size: { width: 500, height: 120 },
  padding: { left: 12, right: 12, top: 12, bottom: 12 },
});

const root = tree.newWithChildren(rootStyle, children);

tree.computeLayout(root, { width: 500, height: 120 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## ⏭️ 関連ガイド

- **[Flex Grow/Shrink](../styling/flex-basis-grow-shrink.md)**
- **[スペーシング](../styling/gap.md)**
