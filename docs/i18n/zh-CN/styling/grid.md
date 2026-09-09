---
title: 网格布局 (Grid)
sidebar_position: 17
---

# 网格布局 (Grid)

Taffy 的 Grid API 与 CSS Grid 类似，最适合用于二维布局。定义行和列的轨道，然后通过线或区域放置项目。

## 核心概念

- **轨道（Track）**：行或列的尺寸定义
- **线（Line）**：用于定位的网格线
- **区域（Area）**：命名区域（如果使用的话）

```text
列:  1fr 2fr
行:  auto 1fr

┌───────────────┐
│ Header        │
├───────┬───────┤
│ Nav   │ Main  │
└───────┴───────┘
```

## 最小示例

```tsx live
const tree = new TaffyTree();

const rootStyle = new Style({
  display: Display.Grid,
  size: { width: 200, height: 200 },
  // 定义 2 列，每列平均分配剩余空间 (1fr)
  gridTemplateColumns: [
    { min: 0, max: "1fr" },
    { min: 0, max: "1fr" },
  ],
  // 定义 2 行：第一行固定 50px，第二行占剩余空间
  gridTemplateRows: [
    { min: 50, max: 50 },
    { min: 0, max: "1fr" },
  ],
  gap: { width: 5, height: 5 },
});

const itemStyle = new Style({
  alignContent: AlignContent.Center,
  justifyContent: JustifyContent.Center,
});

const child1 = tree.newLeaf(itemStyle); // 0,0
const child2 = tree.newLeaf(itemStyle); // 0,1
const child3 = tree.newLeaf(itemStyle); // 1,0
const child4 = tree.newLeaf(itemStyle); // 1,1

const root = tree.newWithChildren(rootStyle, [child1, child2, child3, child4]);

tree.computeLayout(root, { width: 200, height: 200 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## 命名区域与模板尺寸

`gridTemplateAreas` 是命名区域数组。`rowStart` / `columnStart` 从 `1` 开始，`rowEnd` / `columnEnd` 是不包含在区域内的结束网格线。只设置数组时，模板行列数由区域的最大结束位置推断。

```typescript
const style = new Style({
  display: Display.Grid,
  gridTemplateAreas: [
    { name: "main", rowStart: 1, rowEnd: 2, columnStart: 1, columnEnd: 2 },
  ],
  gridTemplateAreaRowCount: 2,
  gridTemplateAreaColumnCount: 3,
});
```

此模板包含 `2` 行 `3` 列，未命名的单元格相当于 CSS 中的 `.`。显式行列数是最小值，读回的有效尺寸不会小于当前命名区域的范围。替换或清空数组会重新推断范围，只有通过两个 count 属性显式设置的最小值会保留；将某个 count 设为 `0` 可清除该轴的显式最小值。`getStyle()`、`setStyle()` 和测量回调的样式副本都保留这一区别。

## 后续步骤

- [网格模板 (Grid Templates)](./grid-templates.md)
- [网格列 (Grid Column)](./grid-column.md)
- [网格行 (Grid Row)](./grid-row.md)
