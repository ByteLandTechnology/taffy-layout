---
title: Flex 增长 (Flex Grow)
sidebar_position: 11
---

# Flex 增长 (Flex Grow)

控制 flex 项目如何增长以填充可用空间。

**`flexGrow`** 属性定义了 flex 项目在必要时增长的能力。它接受一个无单位的数值作为比例。它规定了在 flex 容器内部，项目应该占据剩余可用空间的多少。

## 示例

```tsx live
const tree = new TaffyTree();

const fixedStyle = new Style();
fixedStyle.size = { width: 100, height: "100%" };
const child1 = tree.newLeaf(fixedStyle);

const growStyle = new Style();
growStyle.flexGrow = 1;
growStyle.size = { width: "auto", height: "100%" };
const child2 = tree.newLeaf(growStyle);

const rootStyle = new Style();
rootStyle.display = Display.Flex;
rootStyle.flexDirection = FlexDirection.Row;
rootStyle.size = { width: 300, height: 60 };
rootStyle.gap = { width: 8, height: 0 };
rootStyle.padding = { left: 6, right: 6, top: 6, bottom: 6 };

const root = tree.newWithChildren(rootStyle, [child1, child2]);

tree.computeLayout(root, {
  width: 300,
  height: 60,
});

console.log(`子节点 1: 100px, 子节点 2: 弹性增长 (grow: 1)`);

return <TaffyTreePreview tree={tree} root={root} />;
```

## 快速记事

- 如果没有 `flexGrow`，项目将根据其内容或定义的 `size` 确定大小。
- 如果所有项目都具有 `flexGrow: 1`，容器中的剩余空间将平均分配给所有子节点。

## 下一步

- [Flex 收缩 (Flex Shrink)](./flex-shrink.md)
- [Flex 基准 (Flex Basis)](./flex-basis.md)
