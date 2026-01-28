---
title: 快速开始
sidebar_position: 2
---

# 快速开始

**在几分钟内获得一个可工作的 Taffy 布局。**

## 最小示例

此示例创建一个简单的 Flexbox 布局，包含一个容器和两个子节点。

```ts
import {
  loadTaffy,
  TaffyTree,
  Style,
  Display,
  FlexDirection,
  AlignItems,
} from "taffy-layout";

// 1. 初始化库
await loadTaffy();
const tree = new TaffyTree();

// 2. 创建样式
const containerStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Column,
  alignItems: AlignItems.Center,
  size: { width: 300, height: 200 },
  padding: { left: 10, right: 10, top: 10, bottom: 10 },
});

const childStyle = new Style({
  flexGrow: 1,
  size: { width: "100%", height: "auto" },
});

// 3. 创建节点树
//    （如果使用 newWithChildren，必须先创建叶子节点）
const child1 = tree.newLeaf(childStyle);
const child2 = tree.newLeaf(childStyle);
const container = tree.newWithChildren(containerStyle, [child1, child2]);

// 4. 计算布局
//    传入根节点和可用空间
tree.computeLayout(container, { width: 300, height: 200 });

// 5. 读取计算结果
const containerLayout = tree.getLayout(container);
const child1Layout = tree.getLayout(child1);

console.log(`Container: ${containerLayout.width}x${containerLayout.height}`);
// 输出: Container: 300x200

// 6. 调试：打印整个树结构
console.log(tree.printTree(container));
```

## 下一步

- **[构建节点树](./building-trees.md)** - 学习如何操作节点树。
- **[计算布局](./computing-layouts.md)** - 了解布局计算过程。
- **[核心概念](../core-concepts/index.md)** - 深入了解 Taffy 的模型。
