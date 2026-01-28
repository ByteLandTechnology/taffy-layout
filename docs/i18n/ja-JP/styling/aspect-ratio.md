---
title: アスペクト比 (Aspect Ratio)
sidebar_position: 3
---

# アスペクト比 (Aspect Ratio)

**幅と高さの特定の比率を維持します。**

`aspectRatio` プロパティは、アイテムの寸法の推奨比率を設定します。一方の寸法（例：幅）が設定され、もう一方（高さ）が `auto` の場合、Taffy は比率を満たすように欠落している寸法を計算します。

## 使用法

`width / height` を表す単一の数値を渡します。

- `1.0` = 正方形 (1:1)
- `1.5` = 横長 (3:2)
- `1.77` ≈ 16:9
- `0.56` ≈ 9:16

## 例

```tsx live
const tree = new TaffyTree();

const container = new Style({
  display: Display.Flex,
  size: { width: 300, height: 300 },
  alignItems: AlignItems.Center,
  justifyContent: JustifyContent.Center,
});

const imagePlaceholder = tree.newLeaf(
  new Style({
    // 幅を固定し、高さを比率で決定
    size: { width: 100, height: "auto" },
    aspectRatio: 16 / 9, // ワイド
  }),
);

const root = tree.newWithChildren(container, [imagePlaceholder]);

tree.computeLayout(root, { width: 300, height: 300 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## 次のステップ

- [マージン (Margin)](./margin.md)
- [サイジング (Sizing)](./size.md)
