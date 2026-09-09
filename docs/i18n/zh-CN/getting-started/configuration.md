---
title: 配置
sidebar_position: 5
---

# 配置

**针对您的特定用例优化 Taffy。**

Taffy 开箱即用，但您可以根据性能、精度或资源约束进行调优。

## 容量预分配

如果您大约知道需要多少节点，可以使用容量初始化树以减少内存重新分配并提高启动性能。

```tsx live
// 初始化为可容纳 1,000 个节点
const tree = TaffyTree.withCapacity(1000);
console.log(`Initial Node Count: ${tree.totalNodeCount()}`); // 0 个实际节点

const style = new Style({
  display: Display.Flex,
  size: { width: 200, height: 40 },
  alignItems: AlignItems.Center,
  justifyContent: JustifyContent.Center,
});

const root = tree.newLeaf(style);
tree.computeLayout(root, { width: 200, height: 40 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## 舍入设置

控制布局值是吸附到整数（像素）还是保持精确浮点数。

| 设置         | 函数                     | 描述                                                     |
| :----------- | :----------------------- | :------------------------------------------------------- |
| **启用舍入** | `tree.enableRounding()`  | **默认**。将值舍入到最近的像素。防止 UI 中出现模糊边框。 |
| **禁用舍入** | `tree.disableRounding()` | 使用高精度浮点数。最适合矢量图形或可缩放 UI。            |

```tsx live
const tree = new TaffyTree();

// 创建两个总和为 101px 的项目
// 50.5 + 50.5 = 101
const style = new Style({
  size: { width: 50.5, height: 50 },
  display: Display.Flex,
  justifyContent: JustifyContent.Center,
  alignItems: AlignItems.Center,
});
const child1 = tree.newLeaf(style);
const child2 = tree.newLeaf(style);

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  size: { width: 150, height: 60 },
  alignItems: AlignItems.Center,
});
const root = tree.newWithChildren(rootStyle, [child1, child2]);

// 1. 默认（启用舍入）
tree.computeLayout(root, { width: 150, height: 60 });
let layout1 = tree.getLayout(child1);
// layout1.width 可能根据算法舍入为 51 或 50

// 2. 禁用舍入
tree.disableRounding();
tree.computeLayout(root, { width: 150, height: 60 });
layout1 = tree.getLayout(child1);
// layout1.width 将精确为 50.5

console.log(`Precise Width: ${layout1.width}`);

return <TaffyTreePreview tree={tree} root={root} />;
```

## 内存管理

在支持 `FinalizationRegistry` 的环境中，绑定会在 `TaffyTree`、`Style`、`Layout` 和 `TaffyError` 对象被回收后清理它们持有的 WASM 分配。回收时机不确定；没有 `FinalizationRegistry` 时不会自动执行这一步。

为了防止 WASM 堆中的内存峰值或泄漏，您应该显式管理内存：

- **重用：** 使用 `.clear()` 清空节点结构并保留节点存储容量，减少重建树时的部分分配开销。
- **销毁：** 如果您完全完成了树的使用并希望立即释放其内存，请使用 `.free()`。

`clear()` 重置节点结构并保留容量，不是释放所有资源的替代品。当前引擎在 `clear()` / `remove()` 时可能仍保留已关联的 context 引用；需要释放整棵树持有的这些引用时，使用 `tree.free()`。

创建节点或调用 `setStyle()` 后，树已经复制了样式，可以释放不再使用的输入 `Style`。`getStyle()`、`getLayout()` 和测量回调中的样式都是独立对象，也应在使用完毕后调用 `.free()`；释放树不会释放这些仍由 JavaScript 持有的副本。释放后的对象不能再次使用。

```ts
const tree = new TaffyTree();

// ... 使用树 ...

// 选项 1：重用树（推荐）
// 清除所有节点并保留节点存储容量
tree.clear();

// 选项 2：完全释放
tree.free();
```

## 下一步

- **[样式指南](../styling/index.md)** - 了解 Flexbox 和 Grid。
- **[高级主题](../advanced/index.md)** - 调试和内部原理。
