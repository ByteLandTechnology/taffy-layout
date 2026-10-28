---
title: 定位 (Positioning)
sidebar_position: 22
---

# 定位 (Positioning)

**控制元素在文档中的放置方式。**

`position` 属性决定元素是参与正常布局流程，还是从流程中移除并手动定位。

## 取值

| 值             | 描述                                                                                     |
| :------------- | :--------------------------------------------------------------------------------------- |
| **`Static`**   | **默认值**。元素保持在文档流中，忽略 `inset`。它不会成为绝对定位后代的包含块。           |
| **`Relative`** | 元素保持在文档流中。`inset` 偏移量在视觉上移动它，但它仍在原位置占据空间。               |
| **`Absolute`** | 元素**从流中移除**，相对于其包含块定位：最近的非 `Static` 祖先，或根节点。               |
| **`Fixed`**    | 元素**从流中移除**，相对于根节点定位。                                                   |
| **`Sticky`**   | 与 `Static` 一样布局，但会成为绝对定位后代的包含块。依赖滚动的 sticky 偏移由调用方应用。 |

`Relative`、`Absolute`、`Fixed` 和 `Sticky` 元素都是绝对定位后代的包含块。`Absolute` / `Fixed` 元素的 `Layout.x` / `Layout.y` 相对于其包含块的 border box，而包含块不一定是直接父节点。

## 示例

```tsx live
const tree = new TaffyTree();

const rootStyle = new Style({
  display: Display.Flex,
  position: Position.Relative, // 绝对定位子节点的包含块
  size: { width: 300, height: 120 },
  padding: { left: 10, right: 10, top: 10, bottom: 10 },
  gap: { width: 10, height: 0 },
});

const standardItem = new Style({
  size: { width: 60, height: 60 },
  display: Display.Flex,
  justifyContent: JustifyContent.Center,
  alignItems: AlignItems.Center,
});

const absoluteItem = new Style({
  position: Position.Absolute,
  size: { width: 50, height: 50 },
  inset: { top: 0, right: 0, left: "auto", bottom: "auto" },
});

const child1 = tree.newLeaf(standardItem);
const child2 = tree.newLeaf(standardItem);

// 这个子元素浮动在其他元素之上
const childAbs = tree.newLeaf(absoluteItem);

const root = tree.newWithChildren(rootStyle, [child1, child2, childAbs]);

tree.computeLayout(root, { width: 300, height: 120 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## 典型用途

- 覆盖层和弹出框
- 堆叠式 UI
- 自定义拖拽层

## 后续步骤

- [内嵌偏移 (Inset)](./inset.md)
- [显示模式 (Display)](./display.md)
