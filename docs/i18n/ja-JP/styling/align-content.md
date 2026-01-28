---
title: 行揃え (Align Content)
sidebar_position: 16
---

# 行揃え (Align Content)

**複数行のフレックスコンテナにおける行の配置を制御します。**

`alignContent` プロパティは、交差軸に余分なスペースがある場合に、フレックスコンテナ内の行を配置します。**このプロパティは単一行のフレックスコンテナ（`flexWrap` が `NoWrap`）には効果がありません**。

## 値

| 值                 | 説明                                                               |
| :----------------- | :----------------------------------------------------------------- |
| **`Stretch`**      | **デフォルト**。行は残りのスペースを埋めるように引き伸ばされます。 |
| **`FlexStart`**    | 行はコンテナの始端に詰めて配置されます。                           |
| **`FlexEnd`**      | 行はコンテナの終端に詰めて配置されます。                           |
| **`Center`**       | 行はコンテナの中央に詰めて配置されます。                           |
| **`SpaceBetween`** | 行は均等に配置され、最初の行は始端、最後の行は終端に配置されます。 |
| **`SpaceAround`**  | 行は均等に配置され、各行の周囲に等しいスペースが設けられます。     |

## 例

```tsx live
const tree = new TaffyTree();

const itemStyle = new Style({
  size: { width: 80, height: 30 },
  margin: { bottom: 5 },
});

// 折り返しを強制するために十分な数の子要素を作成
const children = Array.from({ length: 5 }).map(() => tree.newLeaf(itemStyle));

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  flexWrap: FlexWrap.Wrap, // alignContent を動作させるために必須
  size: { width: 200, height: 200 }, // 余分な垂直スペースが必要

  // ここを変更して様々な値をテスト
  alignContent: AlignContent.Center,
});

const root = tree.newWithChildren(rootStyle, children);

tree.computeLayout(root, { width: 200, height: 200 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## 次のステップ

- [グリッドレイアウト (Grid)](./grid.md)
- [自己配置 (Align Self)](./align-self.md)
