---
title: 计算布局
sidebar_position: 4
---

# 计算布局

**将您的样式和树结构转换为具体的像素坐标。**

一旦您的树构建完成，调用 `computeLayout` 来计算每个节点的最终位置和尺寸。

## 标准布局计算

计算布局时，您必须提供**可用空间**（根节点的约束条件）。

```tsx live
const tree = new TaffyTree();
const rootStyle = new Style({
  display: Display.Flex,
  alignItems: AlignItems.Center,
  justifyContent: JustifyContent.Center,
  size: { width: 400, height: 100 },
});

const child = tree.newLeaf(
  new Style({
    size: { width: 50, height: 50 },
  }),
);
const root = tree.newWithChildren(rootStyle, [child]);

// 1. 计算布局
//    传入约束条件：width: 400, height: 100
tree.computeLayout(root, { width: 400, height: 100 });

// 2. 读取结果
//    引擎现在已填充每个节点的布局数据。
const rootLayout = tree.getLayout(root);
const childLayout = tree.getLayout(child);

console.log(`Root Size: ${rootLayout.width}x${rootLayout.height}`);
console.log(`Child Pos: ${childLayout.x}, ${childLayout.y}`);

return (
  <div
    style={{
      width: rootLayout.width,
      height: rootLayout.height,
      background: "#f0f0f0",
      position: "relative",
    }}
  >
    <div
      style={{
        width: childLayout.width,
        height: childLayout.height,
        left: childLayout.x,
        top: childLayout.y,
        position: "absolute",
        background: "#007aff",
      }}
    />
    <div
      style={{
        position: "absolute",
        bottom: 5,
        right: 5,
        fontSize: 10,
        color: "#666",
      }}
    >
      Child at ({childLayout.x}, {childLayout.y})
    </div>
  </div>
);
```

## 增量布局

Taffy 采用智能缓存。如果您修改特定节点的样式或内容，在下一轮计算中只有受影响的树部分会被重新计算。

```ts
const tree = new TaffyTree();
const root = tree.newLeaf(new Style());
const childNode = tree.newLeaf(new Style());
tree.addChild(root, childNode);

// 1. 首次布局
tree.computeLayout(root, { width: 800, height: 600 });

// 2. 修改叶子节点
const newStyle = new Style({ width: 250 });
tree.setStyle(childNode, newStyle);

// 3. 重新计算
//    Taffy 会跳过不受影响的分支。
tree.computeLayout(root, { width: 800, height: 600 });
```

## 舍入与精度

默认情况下，Taffy 将所有输出坐标舍入到最近的像素（整数），以与标准显示网格对齐。

### 禁用舍入

对于高 DPI 渲染或矢量图形等需要亚像素精度的场景，您可以禁用舍入。

```ts
const tree = new TaffyTree();

// 启用亚像素精度
tree.disableRounding();

// ... 计算布局 ...
const node = tree.newLeaf(new Style());
const layout = tree.getLayout(node);
console.log(layout.width); // 可能是 33.33333... 而不是 33
```

## 调试技巧

- **`printTree(root)`**: 打印整个树的深度、样式和计算布局的文本表示。对调试至关重要。
- **隔离**: 如果复杂的树表现异常，创建一个仅包含有问题的节点的小型复现来隔离问题。

## 下一步

- **[配置](./configuration.md)** - 调整引擎设置。
- **[调试布局](../advanced/debugging.md)** - 学习如何排查问题。
