---
title: Flex 基准 (Flex Basis)
sidebar_position: 10
---

# Flex 基准 (Flex Basis)

在分配剩余空间之前，定义元素的默认大小。

**`flexBasis`** 属性指定弹性子元素在分配剩余空间之前的初始主轴大小。

## 示例

```tsx live
const tree = new TaffyTree();

const style1 = new Style();
style1.flexBasis = 100;
style1.size = { width: "auto", height: "100%" };
const child1 = tree.newLeaf(style1);

const style2 = new Style();
style2.flexBasis = 200;
style2.size = { width: "auto", height: "100%" };
const child2 = tree.newLeaf(style2);

const rootStyle = new Style();
rootStyle.display = Display.Flex;
rootStyle.flexDirection = FlexDirection.Row;
rootStyle.size = { width: 350, height: 60 };
rootStyle.gap = { width: 8, height: 0 };
rootStyle.padding = { left: 6, right: 6, top: 6, bottom: 6 };

const root = tree.newWithChildren(rootStyle, [child1, child2]);

tree.computeLayout(root, {
  width: 350,
  height: 60,
});

console.log(`子节点 1: flexBasis 100, 子节点 2: flexBasis 200`);

return <TaffyTreePreview tree={tree} root={root} />;
```

## 快速记事

- `flexBasis` 影响 `auto` 尺寸的计算。
- 它类似于 `width`（或 `height`，取决于 `flexDirection`），但专门用于弹性子元素。

## 下一步

- [Flex 增长 (Flex Grow)](./flex-grow.md)
- [Flex 收缩 (Flex Shrink)](./flex-shrink.md)
