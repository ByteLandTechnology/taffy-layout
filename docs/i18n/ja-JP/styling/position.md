---
title: 配置 (Positioning)
sidebar_position: 22
---

# 配置 (Positioning)

**要素がドキュメント内でどのように配置されるかを制御します。**

`position` プロパティは、要素が通常のレイアウトフローに従うか、またはフローから除外されて手動で配置されるかを決定します。

## 値

| 値             | 説明                                                                                                                                          |
| :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------- |
| **`Static`**   | **デフォルト**。要素はドキュメントフロー内に留まり、`inset` は無視されます。絶対配置された子孫の包含ブロックにはなりません。                  |
| **`Relative`** | 要素はドキュメントフロー内に留まります。`inset` オフセットで視覚的に移動しますが、元の位置でスペースを占めます。                              |
| **`Absolute`** | 要素は**フローから除外され**、包含ブロック（`Static` ではない最も近い祖先、またはルート）を基準に配置されます。                               |
| **`Fixed`**    | 要素は**フローから除外され**、ルートを基準に配置されます。                                                                                    |
| **`Sticky`**   | `Static` と同様に配置されますが、絶対配置された子孫の包含ブロックになります。スクロールに依存する sticky オフセットは呼び出し側が適用します。 |

`Relative`、`Absolute`、`Fixed`、`Sticky` の要素は、絶対配置された子孫の包含ブロックになります。`Absolute` / `Fixed` 要素の `Layout.x` / `Layout.y` は包含ブロックの border box を基準とする座標で、直接の親とは限りません。

## 例

```tsx live
const tree = new TaffyTree();

const rootStyle = new Style({
  display: Display.Flex,
  position: Position.Relative, // 絶対配置の子の包含ブロック
  size: { width: 300, height: 120 },
  padding: { left: 10, right: 10, top: 10, bottom: 10 },
  gap: { width: 10, height: 0 },
});

const standardItem = new Style({
  size: { width: 60, height: 60 },
  display: Display.Flex,
  justifyContent: JustifyContent.Center,
  alignItems: AlignItems.Center,
});

const absoluteItem = new Style({
  position: Position.Absolute,
  size: { width: 50, height: 50 },
  inset: { top: 0, right: 0, left: "auto", bottom: "auto" },
});

const child1 = tree.newLeaf(standardItem);
const child2 = tree.newLeaf(standardItem);

// この子要素は他の要素の上に重なります
const childAbs = tree.newLeaf(absoluteItem);

const root = tree.newWithChildren(rootStyle, [child1, child2, childAbs]);

tree.computeLayout(root, { width: 300, height: 120 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## 次のステップ

- [オフセット (Inset)](./inset.md)
- [表示モード (Display)](./display.md)
