---
title: 配置 (Positioning)
sidebar_position: 22
---

# 配置 (Positioning)

**要素がドキュメント内でどのように配置されるかを制御します。**

`position` プロパティは、要素が通常のレイアウトフローに従うか、またはフローから除外されて手動で配置されるかを決定します。

## 値

| 值             | 説明                                                                                                                               |
| :------------- | :--------------------------------------------------------------------------------------------------------------------------------- |
| **`Relative`** | **デフォルト**。要素はドキュメントフロー内に留まります。`inset` オフセットで視覚的に移動しますが、元の位置でスペースを占めます。   |
| **`Absolute`** | 要素は**フローから削除されます**。最も近い配置された祖先（デフォルト以外の配置を持つ親）またはルートに対して相対的に配置されます。 |

## 例

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
