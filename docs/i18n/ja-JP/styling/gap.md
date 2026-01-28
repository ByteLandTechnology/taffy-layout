---
title: ギャップ (Gap)
sidebar_position: 7
---

# ギャップ (Gap)

**行と列の間の溝（ガター）を定義します。**

`gap` プロパティは、Flexbox および Grid レイアウトにおける行間、列間のギャップ（ガター）のサイズを定義します。これは CSS の `row-gap` と `column-gap` の短縮形で、Taffy では `Size<LengthPercentage>` として表現されます。

## 値

`gap` は `width`（列ギャップ）と `height`（行ギャップ）を持つ Size オブジェクトを受け取ります。通常はピクセルまたはパーセントで指定します。

| プロパティ   | 説明                                       |
| :----------- | :----------------------------------------- |
| **`width`**  | 行内のアイテム間のスペース（列ギャップ）。 |
| **`height`** | 行と行の間のスペース（行ギャップ）。       |

## 例

```tsx live
const tree = new TaffyTree();

const itemStyle = new Style({ size: { width: 60, height: 40 } });

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  flexWrap: FlexWrap.Wrap,
  size: { width: 200, height: 100 },

  // Gap はアイテム間のスペースを厳密に追加し、外縁には追加しません
  gap: { width: 10, height: 10 },
});

const children = Array.from({ length: 6 }).map(() => tree.newLeaf(itemStyle));

const root = tree.newWithChildren(rootStyle, children);

tree.computeLayout(root, { width: 200, height: 100 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## 次のステップ

- [フレックス方向 (Flex Direction)](./flex-direction.md)
- [マージン (Margin)](./margin.md)
