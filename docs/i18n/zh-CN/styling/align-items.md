---
title: Align Items（交叉轴对齐）
sidebar_position: 6
---

# 🎯 Align Items（交叉轴对齐）

**控制子元素沿交叉轴的对齐方式。**

`alignItems` 属性定义了弹性子元素在当前行**交叉轴**上的默认对齐行为。可以将其视为交叉轴方向（垂直于主轴）的 `justifyContent`。

> [!TIP]
> 🔗 **MDN 文档**: [align-items](https://developer.mozilla.org/zh-CN/docs/Web/CSS/align-items)

## 🎛️ 取值

| 值              | 描述                                                                    |
| :-------------- | :---------------------------------------------------------------------- |
| **`Stretch`**   | **默认值**。子元素被拉伸以填充容器的交叉轴尺寸（需遵守 min/max 约束）。 |
| **`FlexStart`** | 子元素与交叉轴的起始边缘对齐。                                          |
| **`FlexEnd`**   | 子元素与交叉轴的结束边缘对齐。                                          |
| **`Center`**    | 子元素在交叉轴上居中对齐。                                              |
| **`Baseline`**  | 子元素根据文本基线对齐。                                                |

## 💻 示例

```tsx live
const tree = new TaffyTree();

const style = new Style({
  display: Display.Flex,
  size: { width: 50, height: 30 },
  alignItems: AlignItems.Center,
  justifyContent: JustifyContent.Center,
});

const labelStyle = new Style({
  flexGrow: 1,
});

// 创建不同高度的子元素以演示对齐效果
const child1 = tree.newLeaf(new Style({ size: { width: 40, height: 20 } }));
const child2 = tree.newLeaf(new Style({ size: { width: 40, height: 40 } }));
const child3 = tree.newLeaf(new Style({ size: { width: 40, height: 60 } }));

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  size: { width: 300, height: 100 },
  gap: { width: 10, height: 0 },
  padding: { left: 10, right: 10, top: 10, bottom: 10 },

  // 修改这里以测试不同的值
  alignItems: AlignItems.Center,
  // 选项: FlexStart, FlexEnd, Stretch, Baseline
});

const root = tree.newWithChildren(rootStyle, [child1, child2, child3]);

tree.computeLayout(root, { width: 300, height: 100 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## ⏭️ 后续步骤

- **[Align Self（自身对齐）](./align-self.md)** - 为单个子元素覆盖此属性。
- **[Align Content（多行对齐）](./align-content.md)** - 在多行容器中对齐各个行。
