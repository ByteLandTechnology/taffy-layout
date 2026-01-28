---
title: 主軸揃え (Justify Content)
sidebar_position: 13
---

# 主軸揃え (Justify Content)

**主軸に沿ってアイテムを配置します。**

`justifyContent` プロパティは**主軸**に沿ってアイテムを配置します（`flexDirection` が `Row` の場合は水平方向、`Column` の場合は垂直方向）。

## 値

| 値                 | 説明                                                                                  |
| :----------------- | :------------------------------------------------------------------------------------ |
| **`FlexStart`**    | **デフォルト**。アイテムを行の開始側に詰めて配置。                                    |
| **`FlexEnd`**      | アイテムを行の末尾側に詰めて配置。                                                    |
| **`Center`**       | アイテムを行の中央に配置。                                                            |
| **`SpaceBetween`** | アイテムを均等に分布。最初のアイテムは開始側、最後は末尾に配置。                      |
| **`SpaceAround`**  | アイテムを均等に分布し、各アイテムの周りのスペースを等しくする。                      |
| **`SpaceEvenly`**  | 任意の 2 つのアイテム間（および端）のスペースが等しくなるようにアイテムを均等に分布。 |

## 例

```tsx live
const tree = new TaffyTree();

const style = new Style({
  size: { width: 50, height: 50 },
});

const child1 = tree.newLeaf(style);
const child2 = tree.newLeaf(style);
const child3 = tree.newLeaf(style);

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  // ここを変更して様々な値をテスト
  justifyContent: JustifyContent.SpaceBetween,
  size: { width: 300, height: 80 },
  padding: { left: 10, right: 10, top: 10, bottom: 10 },
});

const root = tree.newWithChildren(rootStyle, [child1, child2, child3]);

tree.computeLayout(root, { width: 300, height: 80 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## API リファレンス

- [JustifyContent 列挙型](../../api/enumerations/JustifyContent.md)

## 次のステップ

- [交差軸揃え (Align Items)](./align-items.md)
- [フレックス方向 (Flex Direction)](./flex-direction.md)
