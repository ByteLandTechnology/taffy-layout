---
title: 自己配置 (Align Self)
sidebar_position: 15
---

# 自己配置 (Align Self)

**特定のアイテムに対して親の `alignItems` を上書きします。**

`alignSelf` プロパティを使用すると、個々のフレックスアイテムに対してデフォルトの配置（`alignItems` で指定されたもの）を上書きできます。

## 値

| 値              | 説明                                                     |
| :-------------- | :------------------------------------------------------- |
| **`Auto`**      | **デフォルト**。親の `alignItems` 値を継承します。       |
| **`Stretch`**   | アイテムを引き伸ばしてコンテナの交差軸サイズを埋めます。 |
| **`FlexStart`** | アイテムを交差軸の始端に配置します。                     |
| **`FlexEnd`**   | アイテムを交差軸の終端に配置します。                     |
| **`Center`**    | アイテムを中央に配置します。                             |
| **`Baseline`**  | アイテムをベースラインに配置します。                     |

`Start` / `End` はコンテナの論理方向、`SelfStart` / `SelfEnd` はアイテム自身の `direction` に従って配置します。安全な配置には `SafeStart`、`SafeEnd`、`SafeFlexStart`、`SafeFlexEnd`、`SafeCenter`、`SafeSelfStart`、`SafeSelfEnd` があり、内容が収まらない場合は始端配置に戻して始端側のはみ出しを防ぎます。

Grid の `justifySelf` も同じ `AlignSelf` 列挙型を使い、親の `justifyItems` を上書きします。`alignSelf` または `justifySelf` に `AlignSelf.Auto` または `undefined` を設定すると上書きが解除されます。未設定または解除後の直接プロパティアクセスは `AlignSelf.Auto`、`get("alignSelf")` / `get("justifySelf")` は `undefined` を返します。

## 例

```tsx live
const tree = new TaffyTree();

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  alignItems: AlignItems.FlexStart, // デフォルトの配置は上揃え
  size: { width: 300, height: 100 },
  padding: { left: 10, right: 10, top: 10, bottom: 10 },
  gap: { width: 10, height: 0 },
});

const standardItem = new Style({ size: { width: 50, height: 40 } });

// このアイテムは親の FlexStart 配置を上書きします
const selfAlignedItem = new Style({
  size: { width: 50, height: 40 },
  alignSelf: AlignSelf.FlexEnd,
});

const child1 = tree.newLeaf(standardItem);
const child2 = tree.newLeaf(selfAlignedItem); // 下部に表示されます
const child3 = tree.newLeaf(standardItem);

const root = tree.newWithChildren(rootStyle, [child1, child2, child3]);

tree.computeLayout(root, { width: 300, height: 100 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## 次のステップ

- [行揃え (Align Content)](./align-content.md)
- [交差軸揃え (Align Items)](./align-items.md)
