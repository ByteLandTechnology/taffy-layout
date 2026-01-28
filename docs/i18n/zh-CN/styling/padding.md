---
title: 内边距 (Padding)
sidebar_position: 5
---

# 内边距 (Padding)

控制元素的内部间距。

**`padding`** 在元素内容与其边框之间创建空间。

## 示例

```tsx live
const tree = new TaffyTree();

const childStyle = new Style();
childStyle.flexGrow = 1;
const innerNode = tree.newLeaf(childStyle);

const boxStyle = new Style();
boxStyle.size = { width: 150, height: 100 };
boxStyle.display = Display.Flex;
boxStyle.padding = { left: 20, top: 20, right: 20, bottom: 20 };
const boxNode = tree.newWithChildren(boxStyle, [innerNode]);

const rootStyle = new Style();
rootStyle.size = { width: 200, height: 200 };

const root = tree.newWithChildren(rootStyle, [boxNode]);

tree.computeLayout(root, { width: 200, height: 200 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## 快速记事

- `padding` 是一个包含 `left`、`right`、`top`、`bottom` 的 `Rect`。
- 它会影响提供给子节点的内部可用尺寸。

## 下一步

- [边框 (Border)](./border.md)
- [间距 (Gap)](./gap.md)
