---
title: Flex Wrap（フレックス折り返し）
sidebar_position: 3
---

# ↩️ Flex Wrap（フレックス折り返し）

**アイテムを 1 行に収めるか、折り返すかを制御します。**

`flexWrap` プロパティは、主軸に沿った 1 行にアイテムが収まらない場合の動作を制御します。

> [!TIP]
> 🔗 **MDN ドキュメント**: [flex-wrap](https://developer.mozilla.org/ja/docs/Web/CSS/flex-wrap)

## 🎛️ 値

| 値                | 説明                                                                                                         |
| :---------------- | :----------------------------------------------------------------------------------------------------------- |
| **`NoWrap`**      | **デフォルト**。すべてのアイテムを 1 行に収めます。`flexShrink` 設定時は縮小、またはコンテナからあふれます。 |
| **`Wrap`**        | 必要に応じて上から下へ複数行に折り返します。                                                                 |
| **`WrapReverse`** | 必要に応じて下から上へ複数行に折り返します。                                                                 |

## 💻 例

```tsx live
const tree = new TaffyTree();

const style = new Style({
  size: { width: 60, height: 40 },
  margin: { bottom: 5, right: 5 },
});

// 折り返しを強制するために多くの子要素を作成
const children = Array.from({ length: 8 }).map(() => tree.newLeaf(style));

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  // ここを変更: NoWrap, Wrap, WrapReverse
  flexWrap: FlexWrap.Wrap,
  size: { width: 200, height: 200 },
  padding: { left: 10, right: 10, top: 10, bottom: 10 },
});

const root = tree.newWithChildren(rootStyle, children);

tree.computeLayout(root, { width: 200, height: 200 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## ⏭️ 次のステップ

- **[Align Content（行揃え）](./align-content.md)** - 複数行間の間隔を制御します。
- **[Flex Basis、Grow、Shrink（フレックスサイズ）](./flex-basis-grow-shrink.md)** - アイテムのサイズ変更を制御します。
