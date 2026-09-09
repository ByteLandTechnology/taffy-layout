---
title: オフセット (Inset)
sidebar_position: 23
---

# オフセット (Inset)

**エッジからの相対位置で要素を配置します。**

`inset` プロパティ（従来の `top`、`right`、`bottom`、`left`）は、配置された要素のオフセットを定義します。その動作は `position` プロパティに依存します。

## プロパティ

`inset` は Taffy（および CSS 略記）のヘルパープロパティで、以下を設定します：

| プロパティ   | 説明             |
| :----------- | :--------------- |
| **`top`**    | 上端からの距離。 |
| **`bottom`** | 下端からの距離。 |
| **`left`**   | 左端からの距離。 |
| **`right`**  | 右端からの距離。 |

## 動作

- **`Position.Absolute` の場合**：直接の親の配置領域を基準にします。通常は親の padding box で、Grid の配置指定によってはグリッド領域を使います。
- **`Position.Relative` の場合**：オフセットはフロー内のアイテムの*通常の位置*に対する相対値になります。

各辺はピクセル数、パーセント文字列、`"auto"`（デフォルト）を受け取ります。`top` / `bottom` / `left` / `right` は物理方向のままで、`direction` を変えてもプロパティ名の左右は入れ替わりません。

## 例

```tsx live
const tree = new TaffyTree();

const containerStyle = new Style({
  size: { width: 200, height: 100 },
  display: Display.Flex,
});

// 右下隅に固定された絶対配置アイテム
const absoluteItem = tree.newLeaf(
  new Style({
    position: Position.Absolute,
    size: { width: 50, height: 50 },

    // 右下隅に固定
    inset: { bottom: 10, right: 10, top: "auto", left: "auto" },
  }),
);

const root = tree.newWithChildren(containerStyle, [absoluteItem]);

tree.computeLayout(root, { width: 200, height: 100 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## 次のステップ

- [オーバーフロー (Overflow)](./overflow.md)
- [配置 (Positioning)](./position.md)
