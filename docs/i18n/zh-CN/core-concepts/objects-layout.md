---
title: Layout 对象
sidebar_position: 3
---

# Layout 对象

**`Layout`** 对象包含布局过程完成后节点的最终计算结果。

## 关键属性

- **`position`**：节点相对于其父节点左上角的 `x` 和 `y` 坐标。
- **`size`**：节点的计算 `width` 和 `height`（以像素为单位）。
- **`margin` / `padding` / `border`**：解析后的边缘尺寸。

LTR 根节点的坐标通常为 `(0, 0)`；RTL 根节点在可用宽度确定时，其 `x` 为可用宽度减去根节点宽度，`y` 仍为 `0`。子节点的 `x` / `y` 始终是相对于直接父节点的物理坐标，绘制嵌套树时需累加祖先偏移。

`contentSize`（以及 `contentWidth` / `contentHeight`）表示从滚动原点开始可到达的内容范围，可能超出节点自身尺寸。它不是减去 padding 和 border 后的 CSS 内容框尺寸。LTR 的滚动原点位于 padding box 左上角，RTL 位于右上角；起始侧不可滚动到的负向溢出不计入这些值。

## 用法

计算指定子树后，可以读取该子树中节点的布局。`getLayout()` 返回独立快照；重新计算不会更新已取得的 `Layout`，需要再次调用 `getLayout()`。使用完毕后调用 `layout.free()`。

```typescript
const tree = new TaffyTree();
const node = tree.newLeaf(new Style());
tree.computeLayout(node, { width: 100, height: 100 });

const layout = tree.getLayout(node);

console.log(`位置: (${layout.position.x}, ${layout.position.y})`);
console.log(`尺寸: ${layout.size.width}x${layout.size.height}`);
```

## Grid 详细结果

`tree.detailedLayoutInfo(node)` 读取该节点最近存储的 Grid 详细结果，直接返回含 `rows`、`columns`、`items` 的对象，没有 `.Grid` 包装层。从未生成 Grid 详细结果时返回 `null`。

样式或子节点关系变化不会自动清除旧详情。例如，Grid 改为 Flex 并重新计算后，仍可能读到此前的 Grid 数据。应在节点当前作为有子节点的 Grid 容器完成布局后读取；不要仅凭返回值非 `null` 判断当前布局类型或数据是否最新。

每个轴包含 `sizes`、`positions` 和 `gutters`。`positions` 中的 `{ start, end }` 是相对于容器边框框的物理坐标：列的左/右边缘或行的上/下边缘。数组按逻辑轨道顺序排列，因此 RTL 列坐标可以递减，而每个轨道仍满足 `start <= end`。

`gutters` 是相邻轨道之间的实际间距，包含内容对齐分配的额外空间；长度为 `sizes.length + 1`，首尾为 `0`，无轨道时为 `[0]`。容器 padding 和边缘对齐留白反映在 `positions` 中。

## 下一步

- [尺寸、空间与单位](./size-and-space.md)
- [测量函数 (Measure Functions)](./measure-functions.md)
