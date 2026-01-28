---
title: 核心概念
sidebar_position: 3
---

# 核心概念

Taffy 的 API 表面积很小，但拥有一套一致的布局模型。本节将向您介绍理解任何布局所需的概念。

## 三个核心对象

Taffy 中的布局由三个主要对象驱动：

1.  **[Style](./objects-style.md)**：定义节点的布局规则（例如，“作为一个 flex 容器”，“具有 10px 的内边距”）。
2.  **[TaffyTree](./objects-taffy-tree.md)**：管理节点层次结构并作为计算的入口点。
3.  **[Layout](./objects-layout.md)**：计算的输出，包含最终的位置和尺寸。

```
Style + Tree  -> computeLayout -> Layout
```

## 基本原则

除了核心对象之外，还有几个关键原则需要理解：

- **[尺寸、空间与单位](./size-and-space.md)**：Taffy 如何处理固定尺寸、百分比和基于内容的尺寸计算。
- **[测量函数 (Measure Functions)](./measure-functions.md)**：为文本或特定平台的部件集成自定义尺寸测量逻辑。
- **树模型**：每个节点都可以有子节点，父节点控制它们沿轴线的放置。

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

- [Style 对象](./objects-style.md)
- [尺寸、空间与单位](./size-and-space.md)
- [布局食谱 (Cookbook)](../cookbook/)
