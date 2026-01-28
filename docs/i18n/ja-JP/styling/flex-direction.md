---
title: フレックス方向 (Flex Direction)
sidebar_position: 8
---

# フレックス方向 (Flex Direction)

**主軸の方向を定義します。**

`flexDirection` プロパティは主軸を確立し、子要素を水平方向（行）または垂直方向（列）にレイアウトします。

## 値

| 值                  | 説明                             |
| :------------------ | :------------------------------- |
| **`Row`**           | **デフォルト**。左から右に配置。 |
| **`Column`**        | 上から下に配置。                 |
| **`RowReverse`**    | 右から左に配置。                 |
| **`ColumnReverse`** | 下から上に配置。                 |

## 図解

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

## Row（行）

デフォルトの動作です。アイテムは左から右に配置されます。

```tsx live
const tree = new TaffyTree();

const childStyle = new Style({
  size: { width: 50, height: 40 },
  margin: { left: 4, right: 4, top: 4, bottom: 4 },
});

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  size: { width: 200, height: 160 },
  padding: { left: 8, right: 8, top: 8, bottom: 8 },
});

const root = tree.newWithChildren(rootStyle, [
  tree.newLeaf(childStyle),
  tree.newLeaf(childStyle),
  tree.newLeaf(childStyle),
]);

tree.computeLayout(root, { width: 200, height: 160 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## Row Reverse（反向行）

アイテムは右から左に配置されます。

```tsx live
const tree = new TaffyTree();

const childStyle = new Style({
  size: { width: 50, height: 40 },
  margin: { left: 4, right: 4, top: 4, bottom: 4 },
});

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.RowReverse,
  size: { width: 200, height: 160 },
  padding: { left: 8, right: 8, top: 8, bottom: 8 },
});

const root = tree.newWithChildren(rootStyle, [
  tree.newLeaf(childStyle),
  tree.newLeaf(childStyle),
  tree.newLeaf(childStyle),
]);

tree.computeLayout(root, { width: 200, height: 160 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## Column（列）

アイテムは上から下に配置されます。

```tsx live
const tree = new TaffyTree();

const childStyle = new Style({
  size: { width: 50, height: 40 },
  margin: { left: 4, right: 4, top: 4, bottom: 4 },
});

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Column,
  size: { width: 200, height: 160 },
  padding: { left: 8, right: 8, top: 8, bottom: 8 },
});

const root = tree.newWithChildren(rootStyle, [
  tree.newLeaf(childStyle),
  tree.newLeaf(childStyle),
  tree.newLeaf(childStyle),
]);

tree.computeLayout(root, { width: 200, height: 160 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## Column Reverse（反向列）

アイテムは下から上に配置されます。

```tsx live
const tree = new TaffyTree();

const childStyle = new Style({
  size: { width: 50, height: 40 },
  margin: { left: 4, right: 4, top: 4, bottom: 4 },
});

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.ColumnReverse,
  size: { width: 200, height: 160 },
  padding: { left: 8, right: 8, top: 8, bottom: 8 },
});

const root = tree.newWithChildren(rootStyle, [
  tree.newLeaf(childStyle),
  tree.newLeaf(childStyle),
  tree.newLeaf(childStyle),
]);

tree.computeLayout(root, { width: 200, height: 160 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## 次のステップ

- [フレックス折り返し (Flex Wrap)](./flex-wrap.md)
- [主軸揃え (Justify Content)](./justify-content.md)
