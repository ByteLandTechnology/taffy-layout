---
title: 定位 (Positioning)
sidebar_position: 22
---

# 定位 (Positioning)

**控制元素在文档中的放置方式。**

`position` 属性决定元素是参与正常布局流程，还是从流程中移除并手动定位。

## 取值

| 值             | 描述                                                                                     |
| :------------- | :--------------------------------------------------------------------------------------- |
| **`Static`**   | **默认值**。元素保持在文档流中，忽略 `inset`。它不会成为绝对定位后代的包含块。           |
| **`Relative`** | 元素保持在文档流中。`inset` 偏移量在视觉上移动它，但它仍在原位置占据空间。               |
| **`Absolute`** | 元素**从流中移除**，相对于其包含块定位：最近的非 `Static` 祖先，或初始包含块。           |
| **`Fixed`**    | 元素**从流中移除**，在初始包含块中定位，不受定位祖先影响。                               |
| **`Sticky`**   | 与 `Static` 一样布局，但会成为绝对定位后代的包含块。依赖滚动的 sticky 偏移由调用方应用。 |

`Relative`、`Absolute`、`Fixed` 和 `Sticky` 元素会为绝对定位后代建立包含块。偏移和百分比尺寸通常以该祖先的 padding box 为基准；Grid 放置还可能进一步限定定位区域。

**初始包含块**以画布原点为起点。每个轴的尺寸取自 `computeLayout(root, availableSpace)` 传入的确定值，并扣除根节点为滚动条预留的空间。如果该轴为 `"min-content"` 或 `"max-content"`，则在该轴回退到根节点的 padding box，同样扣除滚动条预留空间。因此，它不一定与根节点的尺寸或位置相同。例如，静态根节点为 100×100、可用空间为 300×200 时，10×10 的 `Fixed` 子节点或没有定位祖先的 `Absolute` 子节点设置 `right: 0, bottom: 0`，会位于画布坐标 `(290, 190)`。

返回坐标的基准需要单独区分：流内节点的 `Layout.x` / `Layout.y` 相对于直接父节点的 border box；有定位祖先的绝对定位后代相对于该祖先的 border box。固定定位后代和没有定位祖先的绝对定位后代，虽然在初始包含块中布局，返回坐标仍相对于**根节点的 border box**。绘制时应加上对应基准节点的画布坐标。根节点自身返回画布坐标，其中包含 margin、方向及生效的 inset。完整遍历示例见[可视化调试](../advanced/debugging.md)。

## 示例

```tsx live
const tree = new TaffyTree();

const rootStyle = new Style({
  display: Display.Flex,
  position: Position.Relative, // 绝对定位子节点的包含块
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

// 这个子元素浮动在其他元素之上
const childAbs = tree.newLeaf(absoluteItem);

const root = tree.newWithChildren(rootStyle, [child1, child2, childAbs]);

tree.computeLayout(root, { width: 300, height: 120 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## 典型用途

- 覆盖层和弹出框
- 堆叠式 UI
- 自定义拖拽层

## 后续步骤

- [内嵌偏移 (Inset)](./inset.md)
- [显示模式 (Display)](./display.md)
