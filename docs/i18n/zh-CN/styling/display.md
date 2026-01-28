---
title: 显示模式 (Display)
sidebar_position: 1
---

# 显示模式 (Display)

**定义节点的布局行为。**

`display` 属性确定用于节点子元素的内部布局算法。

## 取值

| 值          | 描述                                                                        |
| :---------- | :-------------------------------------------------------------------------- |
| **`Flex`**  | 使用 **Flexbox** 算法。子元素按行或列排列。                                 |
| **`Grid`**  | 使用 **CSS Grid** 算法。子元素按二维网格排列。                              |
| **`Block`** | 使用 **Block** 算法。（Taffy 中目前支持有限，通常表现为特定的 Flex 配置）。 |
| **`None`**  | 节点从布局中移除。它不占用任何空间并被跳过。                                |

## 示例

```tsx live
// Grid 演示
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

// Flex 演示设置
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

## 下一步

- [尺寸 (Size)](./size.md)
- [Flex 方向 (Flex Direction)](./flex-direction.md)
- [网格布局 (Grid)](./grid.md)
