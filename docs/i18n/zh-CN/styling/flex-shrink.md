---
title: Flex 收缩 (Flex Shrink)
sidebar_position: 12
---

# Flex 收缩 (Flex Shrink)

控制 flex 项目如何收缩以适应可用空间。

**`flexShrink`** 定义弹性子元素的收缩因子。当主轴上的尺寸与间距超过容器可用空间时，收缩量按因子与各项 flex 基准尺寸的乘积分配，并受最小尺寸等约束限制。主轴在行布局中是宽度，在列布局中是高度。

## 示例

```tsx live
const tree = new TaffyTree();

const noShrinkStyle = new Style();
noShrinkStyle.flexShrink = 0;
noShrinkStyle.size = { width: 200, height: "100%" };
const child1 = tree.newLeaf(noShrinkStyle);

const shrinkStyle = new Style();
shrinkStyle.flexShrink = 1;
shrinkStyle.size = { width: 200, height: "100%" };
const child2 = tree.newLeaf(shrinkStyle);

const rootStyle = new Style();
rootStyle.display = Display.Flex;
rootStyle.flexDirection = FlexDirection.Row;
rootStyle.size = { width: 300, height: 60 }; // 容器小于子节点总宽度
rootStyle.gap = { width: 8, height: 0 };
rootStyle.padding = { left: 6, right: 6, top: 6, bottom: 6 };

const root = tree.newWithChildren(rootStyle, [child1, child2]);

tree.computeLayout(root, {
  width: 300,
  height: 60,
});

console.log(`子节点 1 (不收缩): 200px, 子节点 2 (收缩: 1): 80px`);

return <TaffyTreePreview tree={tree} root={root} />;
```

## 快速记事

- 默认值为 `1`，允许项目收缩；最小尺寸等约束仍可能导致溢出。
- 设置为 `0` 可以防止项目收缩。

## 下一步

- [Flex 基准 (Flex Basis)](./flex-basis.md)
- [主轴对齐 (Justify Content)](./justify-content.md)
