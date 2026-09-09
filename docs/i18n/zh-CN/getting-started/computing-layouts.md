---
title: 计算布局
sidebar_position: 4
---

# 计算布局

**将您的样式和树结构转换为具体的像素坐标。**

一旦您的树构建完成，调用 `computeLayout` 来计算指定根节点及其子树的位置和尺寸。树中未连接到该根节点的其他节点不会被本次计算更新。

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

Taffy 缓存布局结果，输入约束相同且缓存仍有效时可复用。调用 `setStyle()` 或修改父子关系会使相关缓存失效；父节点尺寸变化也可能影响未直接修改的子节点。原地修改测量内容或替换测量回调后，需要对受影响的叶子调用 `markDirty()`；详见[测量函数](../core-concepts/measure-functions.md)。

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
//    Taffy 可复用输入约束未变且缓存仍有效的结果。
tree.computeLayout(root, { width: 800, height: 600 });
```

## 舍入与精度

默认情况下，Taffy 将布局的位置和边缘吸附到整数像素。宽高由舍入后的边缘计算，避免相邻元素之间出现累积误差；这不表示每个字段都独立四舍五入。Grid 详细结果中的轨道位置和尺寸保留未舍入值。

### 禁用舍入

对于高 DPI 渲染或矢量图形等需要亚像素精度的场景，您可以禁用舍入。

```ts
const tree = new TaffyTree();

// 启用亚像素精度
tree.disableRounding();

const style = new Style({ width: 100 / 3, height: 20 });
const node = tree.newLeaf(style);
style.free();
tree.computeLayout(node, { width: 100, height: 100 });
const layout = tree.getLayout(node);
console.log(layout.width); // 约为 33.33333，而不是 33
layout.free();
tree.free();
```

## 调试技巧

- **`printTree(root)`**: 返回指定子树的层次、布局类型与计算结果字符串；使用 `console.log(tree.printTree(root))` 打印它。
- **隔离**: 如果复杂的树表现异常，创建一个仅包含有问题的节点的小型复现来隔离问题。

## 下一步

- **[配置](./configuration.md)** - 调整引擎设置。
- **[调试布局](../advanced/debugging.md)** - 学习如何排查问题。
