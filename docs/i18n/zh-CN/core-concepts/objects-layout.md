---
title: Layout 对象
sidebar_position: 3
---

# Layout 对象

**`Layout`** 对象包含布局过程完成后节点的最终计算结果。

## 关键属性

- **`location`**：节点相对于其父节点左上角的 `x` 和 `y` 坐标。
- **`size`**：节点的计算 `width` 和 `height`（以像素为单位）。
- **`margin` / `padding` / `border`**：解析后的边缘尺寸。

## 用法

在树上调用 `computeLayout` 后，您可以检索树中任何节点的布局。

```typescript
const tree = new TaffyTree();
const node = tree.newLeaf(new Style());
tree.computeLayout(node, { width: 100, height: 100 });

const layout = tree.getLayout(node);

console.log(`位置: (${layout.position.x}, ${layout.position.y})`);
console.log(`尺寸: ${layout.size.width}x${layout.size.height}`);
```

## 下一步

- [尺寸、空间与单位](./size-and-space.md)
- [测量函数 (Measure Functions)](./measure-functions.md)
