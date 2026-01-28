---
title: 边框 (Border)
sidebar_position: 6
---

# 边框 (Border)

控制元素的边框宽度。

**`border`** 定义盒子的边界厚度。

## 示例

```tsx live
const tree = new TaffyTree();

const childStyle = new Style();
childStyle.flexGrow = 1;
const innerNode = tree.newLeaf(childStyle);

const boxStyle = new Style();
boxStyle.size = { width: 150, height: 100 };
boxStyle.display = Display.Flex;
boxStyle.border = { left: 10, top: 10, right: 10, bottom: 10 };
const boxNode = tree.newWithChildren(boxStyle, [innerNode]);

const rootStyle = new Style();
rootStyle.size = { width: 200, height: 200 };

const root = tree.newWithChildren(rootStyle, [boxNode]);

tree.computeLayout(root, { width: 200, height: 200 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## 快速记事

- `border` 是一个包含 `left`、`right`、`top`、`bottom` 的 `Rect`。
- Taffy 仅计算边框占用的 **空间**；实际边框的渲染由您的渲染引擎决定。

## 下一步

- [Gap (间距)](./gap.md)
- [Size (尺寸)](./size.md)
