---
title: Positioning（定位）
sidebar_position: 13
---

# 📍 Position（定位）

**控制元素在文档中的放置方式。**

`position` 属性决定元素是参与正常布局流程，还是从流程中移除并手动定位。

> [!TIP]
> 🔗 **MDN 文档**: [position](https://developer.mozilla.org/zh-CN/docs/Web/CSS/position)

## 🎛️ 取值

| 值             | 描述                                                                                     |
| :------------- | :--------------------------------------------------------------------------------------- |
| **`Relative`** | **默认值**。元素保持在文档流中。`inset` 偏移量在视觉上移动它，但它仍在原位置占据空间。   |
| **`Absolute`** | 元素**从流中移除**。它相对于最近的已定位祖先（具有非默认定位的父元素）或根元素进行定位。 |

## 💻 示例

```tsx live
const tree = new TaffyTree();

const rootStyle = new Style({
  display: Display.Flex,
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
  size: { width: 40, height: 40 },
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

## ⏭️ 后续步骤

- **[Inset（偏移量）](./inset.md)** - 定义上/右/下/左坐标。
- **[Display（显示模式）](./display.md)** - 在 Flex 和 None 之间切换。
