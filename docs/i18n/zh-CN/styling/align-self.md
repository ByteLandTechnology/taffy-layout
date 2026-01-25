---
title: Align Self（自身对齐）
sidebar_position: 7
---

# 🕴️ Align Self（自身对齐）

**为特定子元素覆盖父级的 `alignItems` 设置。**

`alignSelf` 属性允许单个弹性子元素覆盖默认的对齐方式（即 `alignItems` 指定的方式）。

> [!TIP]
> 🔗 **MDN 文档**: [align-self](https://developer.mozilla.org/zh-CN/docs/Web/CSS/align-self)

## 🎛️ 取值

| 值              | 描述                                     |
| :-------------- | :--------------------------------------- |
| **`Auto`**      | **默认值**。继承父级的 `alignItems` 值。 |
| **`Stretch`**   | 子元素拉伸以填充容器的交叉轴尺寸。       |
| **`FlexStart`** | 子元素与交叉轴的起始边缘对齐。           |
| **`FlexEnd`**   | 子元素与交叉轴的结束边缘对齐。           |
| **`Center`**    | 子元素居中对齐。                         |
| **`Baseline`**  | 子元素根据基线对齐。                     |

## 💻 示例

```tsx live
const tree = new TaffyTree();

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  alignItems: AlignItems.FlexStart, // 默认对齐方式为顶部
  size: { width: 300, height: 100 },
  padding: { left: 10, right: 10, top: 10, bottom: 10 },
  gap: { width: 10, height: 0 },
});

const standardItem = new Style({ size: { width: 50, height: 40 } });

// 这个子元素覆盖了父级的 FlexStart 对齐方式
const selfAlignedItem = new Style({
  size: { width: 50, height: 40 },
  alignSelf: AlignSelf.FlexEnd,
});

const child1 = tree.newLeaf(standardItem);
const child2 = tree.newLeaf(selfAlignedItem); // 将显示在底部
const child3 = tree.newLeaf(standardItem);

const root = tree.newWithChildren(rootStyle, [child1, child2, child3]);

tree.computeLayout(root, { width: 300, height: 100 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## ⏭️ 后续步骤

- **[Align Items（交叉轴对齐）](./align-items.md)** - 为容器设置默认对齐方式。
