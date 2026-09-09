---
title: 外边距 (Margin)
sidebar_position: 4
---

# 外边距 (Margin)

控制元素的外部间距。

**`margin`** 在元素周围创建空间，位于任何定义的边框之外。

## 示例

```tsx live
const tree = new TaffyTree();

const childStyle = new Style();
childStyle.size = { width: "100%", height: "100%" };
const innerNode = tree.newLeaf(childStyle);

const boxStyle = new Style();
boxStyle.size = { width: 100, height: 100 };
boxStyle.margin = { left: 20, top: 20, right: 20, bottom: 20 };
const boxNode = tree.newWithChildren(boxStyle, [innerNode]);

const rootStyle = new Style();
rootStyle.size = { width: 200, height: 200 };
rootStyle.display = Display.Flex;

const root = tree.newWithChildren(rootStyle, [boxNode]);

tree.computeLayout(root, { width: 200, height: 200 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## 快速记事

- `margin` 是一个包含 `left`、`right`、`top`、`bottom` 的 `Rect`。
- 每条边接受数字、百分比字符串或 `"auto"`。例如，水平方向两侧的 `"auto"` 外边距可分配剩余空间以居中；效果取决于布局模式和可用空间。

## 下一步

- [Padding (内边距)](./padding.md)
- [Border (边框)](./border.md)
