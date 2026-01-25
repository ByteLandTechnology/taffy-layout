---
title: Flex Grow、Shrink 和 Basis
sidebar_position: 4
---

# Flex Grow、Shrink 和 Basis

控制弹性子元素如何增长和收缩以适应可用空间。

- **`flexGrow`**：定义弹性子元素在需要时增长的能力。
- **`flexShrink`**：定义弹性子元素在需要时收缩的能力。
- **`flexBasis`**：定义在分配剩余空间之前元素的默认大小。

## 💻 示例

```tsx live
const tree = new TaffyTree();

const fixedStyle = new Style();
fixedStyle.size = { width: 120, height: "100%" };
const child1 = tree.newLeaf(fixedStyle);

const growStyle = new Style();
growStyle.flexGrow = 1;
growStyle.size = { width: "auto", height: "100%" };
const child2 = tree.newLeaf(growStyle);

const rootStyle = new Style();
rootStyle.display = Display.Flex;
rootStyle.flexDirection = FlexDirection.Row;
rootStyle.size = { width: 240, height: 60 };
rootStyle.gap = { width: 8, height: 0 };
rootStyle.padding = { left: 6, right: 6, top: 6, bottom: 6 };

const root = tree.newWithChildren(rootStyle, [child1, child2]);

tree.computeLayout(root, {
  width: 240,
  height: 60,
});

console.log(`Fixed width: 120, grow: 1`);

return <TaffyTreePreview tree={tree} root={root} />;
```

## 快速说明

- 不设置 `flexGrow` 时，子元素按内容大小排列
- `flexBasis` 影响 `auto` 尺寸计算

## ⏭️ 后续步骤

- [Justify Content（主轴对齐）](./justify-content.md)
- [Align Items（交叉轴对齐）](./align-items.md) & [Align Content（多行对齐）](./align-content.md)
