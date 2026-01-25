---
title: 构建节点树
sidebar_position: 3
---

# 🏗️ 构建节点树

Taffy 中的布局以节点树的形式表示。每个节点都有一个 `Style`，父节点控制其子节点的排列方式。

## 🔑 关键操作

| 操作             | 方法                                            | 描述                                       |
| :--------------- | :---------------------------------------------- | :----------------------------------------- |
| **创建叶子节点** | `tree.newLeaf(style)`                           | 创建一个没有子节点的节点（如文本、图像）。 |
| **创建父节点**   | `tree.newWithChildren(style, children[])`       | 创建一个带有初始子节点的节点。             |
| **添加子节点**   | `tree.addChild(parent, child)`                  | 将子节点追加到父节点。                     |
| **插入子节点**   | `tree.insertChildAtIndex(parent, index, child)` | 在特定位置插入子节点。                     |
| **删除子节点**   | `tree.removeChild(parent, child)`               | 删除特定的子节点。                         |
| **获取样式**     | `tree.getStyle(node)`                           | 检索节点的样式对象。                       |
| **设置样式**     | `tree.setStyle(node, style)`                    | 更新节点的样式。                           |

## 🍃 创建节点

### 叶子节点

叶子节点是布局树的端点，通常表示文本、图像或按钮等内容。

```ts
const tree = new TaffyTree();

// 先创建样式
const style = new Style({
  display: Display.Flex,
  size: { width: 100, height: 50 },
});

// 创建节点
const leafNode = tree.newLeaf(style);
```

### 容器节点

容器节点将其他节点组合在一起。为了方便起见，您可以在创建容器时附带现有子节点。

```ts
const tree = new TaffyTree();
const containerStyle = new Style({ display: Display.Flex });

const child1 = tree.newLeaf(new Style());
const child2 = tree.newLeaf(new Style());

// 创建容器并立即附加子节点
const containerNode = tree.newWithChildren(containerStyle, [child1, child2]);
```

## 🌲 管理子节点

使用这些方法动态操作树结构。

```ts
const tree = new TaffyTree();
const parent = tree.newLeaf(new Style());
const child = tree.newLeaf(new Style());

// 在列表末尾添加子节点
tree.addChild(parent, child); // 索引: 0

// 在开头插入新子节点
const firstChild = tree.newLeaf(new Style());
tree.insertChildAtIndex(parent, 0, firstChild); // 索引: 0，之前的子节点变为索引 1

// 替换子节点
const newChild = tree.newLeaf(new Style());
tree.replaceChildAtIndex(parent, 1, newChild);

// 删除子节点
tree.removeChild(parent, firstChild);
```

## 🎨 更新样式

样式可以随时更新。这会将节点标记为"脏"状态，需要重新计算布局。

```ts
const tree = new TaffyTree();
const node = tree.newLeaf(new Style({ flexGrow: 1 }));

// 稍后更新样式
const newStyle = new Style({ flexGrow: 2, width: 100 });
tree.setStyle(node, newStyle);
```

> [!NOTE]
> **性能提示**: 尽量为静态内容初始化重用 `Style` 对象，但请记住 `tree.setStyle` 会将样式数据复制到内部引擎中，因此在将样式传递给树**之后**修改 JS `Style` 对象不会影响树，直到您再次调用 `setStyle`。

## ⏭️ 下一步

- 📐 **[计算布局](./computing-layouts.md)** - 计算位置和尺寸。
- ⚙️ **[配置](./configuration.md)** - 全局设置。
