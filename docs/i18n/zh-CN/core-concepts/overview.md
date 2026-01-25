---
title: 核心概念概述
sidebar_position: 1
---

# 核心概念概述

Taffy 具有较小的 API 表面，但具有一致的布局模型。本节为您提供理解任何布局的概念。

## 三个核心对象

- **Style**: 节点的布局规则
- **TaffyTree**: 布局树 + 计算入口点
- **Layout**: 计算结果（位置、尺寸、边距）

```
Style + Tree  -> computeLayout -> Layout
```

## 树模型

每个节点可以有子节点，父节点控制子节点如何沿主轴和交叉轴放置。

```
Root (flex)
├── Sidebar
└── Content
    ├── Header
    └── Body
```

### 节点创建 API

- `newLeaf(style)` 创建叶子节点
- `newWithChildren(style, children)` 创建容器
- `addChild` / `insertChildAtIndex` 修改树

## 布局流程

1. 配置根节点和子节点样式
2. 调用 `computeLayout(root, availableSpace)`
3. 读取 `getLayout(node)` 结果

```tsx live
const tree = new TaffyTree();

const style = new Style();
style.flexGrow = 1;

const child1 = tree.newLeaf(style);
const child2 = tree.newLeaf(style);

const rootStyle = new Style();
rootStyle.display = Display.Flex;
rootStyle.flexDirection = FlexDirection.Row;
rootStyle.size = { width: 400, height: 200 };

const root = tree.newWithChildren(rootStyle, [child1, child2]);

tree.computeLayout(root, {
  width: 400,
  height: 200,
});

console.log(tree.printTree(root));

return <TaffyTreePreview tree={tree} root={root} />;
```

## 布局包含的内容

- `x`, `y`: 相对于父节点的位置
- `width`, `height`: 计算后的尺寸
- `margin`, `padding`, `border`: 边缘尺寸

## 下一步

- [尺寸、空间和单位](./size-and-space.md)
- [测量函数](./measure-functions.md)
- [布局实例手册](../cookbook/)
