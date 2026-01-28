---
title: 表示モード (Display)
sidebar_position: 1
---

# 表示モード (Display)

**ノードのレイアウト動作を定義します。**

`display` プロパティは、ノードの子要素に使用される内部レイアウトアルゴリズムを決定します。

## 値

| 値          | 説明                                                                                                                 |
| :---------- | :------------------------------------------------------------------------------------------------------------------- |
| **`Flex`**  | **Flexbox** アルゴリズムを使用します。アイテムは行または列にレイアウトされます。                                     |
| **`Grid`**  | **CSS Grid** アルゴリズムを使用します。アイテムは 2D グリッドにレイアウトされます。                                  |
| **`Block`** | **Block** アルゴリズムを使用します。（Taffy では現在サポートが制限されており、特定の Flex 設定のように動作します）。 |
| **`None`**  | ノードはレイアウトから削除されます。スペースを占有せず、スキップされます。                                           |

## 例

```tsx live
// Grid デモ
const gridTree = new TaffyTree();
const gridStyle = new Style();
gridStyle.size = { width: 60, height: 40 };
const gridChild1 = gridTree.newLeaf(gridStyle);
const gridChild2 = gridTree.newLeaf(gridStyle);
const gridChild3 = gridTree.newLeaf(gridStyle);
const gridChild4 = gridTree.newLeaf(gridStyle);

const gridRootStyle = new Style();
gridRootStyle.display = Display.Grid;
gridRootStyle.gridTemplateColumns = [
  { min: 0, max: "1fr" },
  { min: 0, max: "1fr" },
];
gridRootStyle.gridTemplateRows = [
  { min: 0, max: "1fr" },
  { min: 0, max: "1fr" },
];
gridRootStyle.gap = { width: 8, height: 8 };
gridRootStyle.padding = { left: 10, right: 10, top: 10, bottom: 10 };
gridRootStyle.size = { width: 140, height: 120 };

const gridRoot = gridTree.newWithChildren(gridRootStyle, [
  gridChild1,
  gridChild2,
  gridChild3,
  gridChild4,
]);

gridTree.computeLayout(gridRoot, {
  width: 140,
  height: 120,
});

console.log(`Flex mode: Flex, Grid columns: 2`);

// Flex デモ設定
const flexTree = new TaffyTree();
const flexStyle = new Style();
flexStyle.size = { width: 60, height: 40 };
const flexChild1 = flexTree.newLeaf(flexStyle);
const flexChild2 = flexTree.newLeaf(flexStyle);

const flexRoot = flexTree.newWithChildren(
  new Style({
    display: Display.Flex,
    gap: { width: 8, height: 8 },
    padding: { left: 10, right: 10, top: 10, bottom: 10 },
    size: { width: 140, height: 120 },
  }),
  [flexChild1, flexChild2],
);
flexTree.computeLayout(flexRoot, { width: 140, height: 120 });

return (
  <div style={{ display: "flex", gap: 16, flexWrap: "wrap" }}>
    <TaffyTreePreview tree={flexTree} root={flexRoot} />
    <TaffyTreePreview tree={gridTree} root={gridRoot} />
  </div>
);
```

## 次のステップ

- [サイジング (Sizing)](./size.md)
- [フレックスボックス (Flexbox)](./flex-direction.md)
- [グリッドレイアウト (Grid)](./grid.md)
