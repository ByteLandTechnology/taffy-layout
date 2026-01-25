---
title: Flex Direction（フレックス方向）
sidebar_position: 2
---

# ➡️ Flex Direction（フレックス方向）

**主軸の方向を定義します。**

`flexDirection` プロパティは主軸を確立し、子要素を水平方向（行）または垂直方向（列）にレイアウトします。

> [!TIP]
> 🔗 **MDN ドキュメント**: [flex-direction](https://developer.mozilla.org/ja/docs/Web/CSS/flex-direction)

## 🎛️ 値

| 值                  | 説明                             |
| :------------------ | :------------------------------- |
| **`Row`**           | **デフォルト**。左から右に配置。 |
| **`Column`**        | 上から下に配置。                 |
| **`RowReverse`**    | 右から左に配置。                 |
| **`ColumnReverse`** | 下から上に配置。                 |

## 📐 図解

```text
Row（行）:
[Item 1] -> [Item 2] -> [Item 3]

Column（列）:
[Item 1]
   |
   v
[Item 2]
   |
   v
[Item 3]
```

## 💻 例

```tsx live
const tree = new TaffyTree();

const style = new Style({
  size: { width: 40, height: 40 },
  margin: { bottom: 5, right: 5 },
});

const child1 = tree.newLeaf(style);
const child2 = tree.newLeaf(style);
const child3 = tree.newLeaf(style);

const rootStyle = new Style({
  display: Display.Flex,
  // ここを変更: Row, Column, RowReverse, ColumnReverse
  flexDirection: FlexDirection.Row,
  size: { width: 250, height: 150 },
  padding: { left: 10, right: 10, top: 10, bottom: 10 },
});

const root = tree.newWithChildren(rootStyle, [child1, child2, child3]);

tree.computeLayout(root, { width: 250, height: 150 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## ⏭️ 次のステップ

- **[Flex Wrap（折り返し）](./flex-wrap.md)** - 行あふれ時の折り返しを制御します。
- **[Justify Content（主軸揃え）](./justify-content.md)** - 主軸方向の揃えを制御します。
- [Align Items（交差軸揃え）](./align-items.md)
