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
| **`Absolute`** | 要素は**フローから除外され**、包含ブロック（`Static` ではない最も近い祖先、または初期包含ブロック）を基準に配置されます。                     |
| **`Fixed`**    | 要素は**フローから除外され**、配置された祖先に関係なく初期包含ブロック内に配置されます。                                                      |
| **`Sticky`**   | `Static` と同様に配置されますが、絶対配置された子孫の包含ブロックになります。スクロールに依存する sticky オフセットは呼び出し側が適用します。 |

`Relative`、`Absolute`、`Fixed`、`Sticky` の要素は、絶対配置された子孫の包含ブロックになります。オフセットとパーセントサイズは通常その祖先の padding box を基準とし、Grid の配置指定によって領域がさらに限定される場合があります。

**初期包含ブロック**はキャンバス原点を起点とします。各軸のサイズは `computeLayout(root, availableSpace)` に渡した確定値からルートのスクロールバー用予約領域を引いたものです。その軸が `"min-content"` または `"max-content"` の場合は、スクロールバー用予約領域を除いたルートの padding box にフォールバックします。したがって、ルートのサイズや位置と同じとは限りません。例えば、静的なルートが 100×100、利用可能なスペースが 300×200 の場合、10×10 の `Fixed` 子、または配置された祖先を持たない `Absolute` 子に `right: 0, bottom: 0` を設定すると、キャンバス座標は `(290, 190)` になります。

返される座標の基準は別に考えます。通常フローのノードの `Layout.x` / `Layout.y` は親の border box、配置された祖先を持つ絶対配置の子孫はその祖先の border box を基準とします。固定配置の子孫と配置された祖先を持たない絶対配置の子孫は、初期包含ブロックでレイアウトされますが、座標は**ルートの border box**を基準に返されます。描画時には対応する基準ノードのキャンバス座標を加算してください。ルート自身の座標は margin、方向、有効な inset を含むキャンバス座標です。対応する走査例は[可視化デバッグ](../advanced/debugging.md)を参照してください。

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
