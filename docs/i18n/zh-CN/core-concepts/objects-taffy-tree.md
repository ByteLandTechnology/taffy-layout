---
title: TaffyTree 对象
sidebar_position: 2
---

# TaffyTree 对象

**`TaffyTree`** 对象代表整个布局树，并作为布局计算的入口点。

## 关键职责

- **节点管理**：创建、添加、移除和插入节点。
- **树层次结构**：维护节点之间的父子关系。
- **计算**：通过 `computeLayout` 或 `computeLayoutWithMeasure` 调用布局算法。
- **结果检索**：存储并提供对每个节点计算结果的访问。

## 用法

```typescript
const tree = new TaffyTree();

// 创建节点
const child = tree.newLeaf(new Style());
const root = tree.newWithChildren(new Style(), [child]);

// 计算布局
tree.computeLayout(root, { width: 500, height: 500 });

// 获取结果
const layout = tree.getLayout(child);
```

## 下一步

- [Layout 对象](./objects-layout.md)
- [尺寸、空间与单位](./size-and-space.md)
