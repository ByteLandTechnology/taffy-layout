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
- 它支持 `Auto` 值以使内容居中（类似于 `margin: auto`）。

## 下一步

- [Padding (内边距)](./padding.md)
- [Border (边框)](./border.md)
