---
title: Positioning（配置）
sidebar_position: 13
---

# 📍 Position（配置）

**要素がドキュメント内でどのように配置されるかを制御します。**

`position` プロパティは、要素が通常のレイアウトフローに参加するか、またはフローから削除されて手動で配置されるかを決定します。

> [!TIP]
> 🔗 **MDN ドキュメント**: [position](https://developer.mozilla.org/ja/docs/Web/CSS/position)

## 🎛️ 値

| 值             | 説明                                                                                                                               |
| :------------- | :--------------------------------------------------------------------------------------------------------------------------------- |
| **`Relative`** | **デフォルト**。要素はドキュメントフロー内に留まります。`inset` オフセットで視覚的に移動しますが、元の位置でスペースを占めます。   |
| **`Absolute`** | 要素は**フローから削除されます**。最も近い配置された祖先（デフォルト以外の配置を持つ親）またはルートに対して相対的に配置されます。 |

## 💻 例

```tsx live
const tree = new TaffyTree();

const rootStyle = new Style({
  display: Display.Flex,
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
  size: { width: 40, height: 40 },
  inset: { top: 0, right: 0, left: "auto", bottom: "auto" },
});

const child1 = tree.newLeaf(standardItem);
const child2 = tree.newLeaf(standardItem);

// この子要素は他の要素の上に浮かびます
const childAbs = tree.newLeaf(absoluteItem);

const root = tree.newWithChildren(rootStyle, [child1, child2, childAbs]);

tree.computeLayout(root, { width: 300, height: 120 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## 典型的な用途

- オーバーレイとポップオーバー
- 積み重ね UI
- カスタムドラッグレイヤー

## ⏭️ 次のステップ

- **[Inset（オフセット）](./inset.md)** - 上/右/下/左の座標を定義します。
- **[Display（表示モード）](./display.md)** - Flex と None を切り替えます。
