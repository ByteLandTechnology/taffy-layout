---
title: 交叉轴对齐 (Align Items)
sidebar_position: 14
---

# 交叉轴对齐 (Align Items)

**控制子元素沿交叉轴的对齐方式。**

`alignItems` 属性定义了弹性子元素在当前行**交叉轴**上的默认对齐行为。可以将其视为交叉轴方向（垂直于主轴）的 `justifyContent`。

## 取值

| 值              | 描述                                                                    |
| :-------------- | :---------------------------------------------------------------------- |
| **`Stretch`**   | **默认值**。交叉轴尺寸为 `auto` 的子元素可被拉伸，并遵守 min/max 约束。 |
| **`FlexStart`** | 子元素与交叉轴的起始边缘对齐。                                          |
| **`FlexEnd`**   | 子元素与交叉轴的结束边缘对齐。                                          |
| **`Center`**    | 子元素在交叉轴上居中对齐。                                              |
| **`Baseline`**  | 子元素根据布局计算的基线对齐。                                          |

测量回调只返回尺寸，不能提供文本基线；Taffy 不会从文本或字体中测出基线。

`Start` / `End` 按容器的逻辑方向对齐；`SelfStart` / `SelfEnd` 按子元素自身的 `direction` 对齐。安全对齐值包括 `SafeStart`、`SafeEnd`、`SafeFlexStart`、`SafeFlexEnd`、`SafeCenter`、`SafeSelfStart` 和 `SafeSelfEnd`，在内容无法容纳时回退到起始对齐，避免起始边缘溢出。

`alignItems` 也用于 Grid 的块轴对齐；Grid 的行内轴使用 `justifyItems`，同样接受 `AlignItems` 枚举。未设置的属性读回 `undefined`；表中的默认值描述布局行为。

## 示例

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
const child1 = tree.newLeaf(new Style({ size: { width: 50, height: 20 } }));
const child2 = tree.newLeaf(new Style({ size: { width: 50, height: 40 } }));
const child3 = tree.newLeaf(new Style({ size: { width: 50, height: 60 } }));

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

## 后续步骤

- [自身对齐 (Align Self)](./align-self.md)
- [主轴对齐 (Justify Content)](./justify-content.md)
