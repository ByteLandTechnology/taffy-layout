---
title: 显示模式 (Display)
sidebar_position: 1
---

# 显示模式 (Display)

**定义节点的布局行为。**

`display` 属性确定用于节点子元素的内部布局算法。

## 取值

| 值             | 描述                                                   |
| :------------- | :----------------------------------------------------- |
| **`Flex`**     | **默认值**。使用 Flexbox，子元素按行或列排列。         |
| **`Grid`**     | 使用 CSS Grid，子元素按二维网格排列。                  |
| **`Block`**    | 使用块布局，支持普通流、外边距折叠和浮动。             |
| **`FlowRoot`** | 使用块布局，并始终建立独立的块格式化上下文以包含浮动。 |
| **`None`**     | 节点及其子树不参与布局，尺寸为零；树结构仍保留。       |

## 书写方向、浮动与清除

`direction` 使用 `Direction.Ltr`（默认）或 `Direction.Rtl`，控制逻辑方向。每个 `Style` 都有自己的方向；创建子节点样式时需要设置相应的方向。

块布局中的 `float` 接受 `Float.None`（默认）、`Float.Left`、`Float.Right`；`clear` 接受 `Clear.None`（默认）、`Clear.Left`、`Clear.Right`、`Clear.Both`。`Left` 和 `Right` 指物理方向。容器需要包含浮动子节点时可使用 `Display.FlowRoot`。

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
