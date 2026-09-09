---
title: 自身对齐 (Align Self)
sidebar_position: 15
---

# 自身对齐 (Align Self)

**为特定子元素覆盖父级的 `alignItems` 设置。**

`alignSelf` 属性允许单个弹性子元素覆盖默认的对齐方式（即 `alignItems` 指定的方式）。

## 取值

| 值              | 描述                                                     |
| :-------------- | :------------------------------------------------------- |
| **`Auto`**      | **默认值**。继承父级的 `alignItems` 值。                 |
| **`Stretch`**   | 交叉轴尺寸为 `auto` 时可拉伸，并遵守 min/max 约束。      |
| **`FlexStart`** | 子元素与交叉轴的起始边缘对齐。                           |
| **`FlexEnd`**   | 子元素与交叉轴的结束边缘对齐。                           |
| **`Center`**    | 子元素居中对齐。                                         |
| **`Baseline`**  | 子元素根据布局计算的基线对齐；测量回调不能提供文本基线。 |

`Start` / `End` 按容器的逻辑方向对齐；`SelfStart` / `SelfEnd` 按子元素自身的 `direction` 对齐。安全对齐值包括 `SafeStart`、`SafeEnd`、`SafeFlexStart`、`SafeFlexEnd`、`SafeCenter`、`SafeSelfStart` 和 `SafeSelfEnd`，在内容无法容纳时回退到起始对齐，避免起始边缘溢出。

Grid 中的 `justifySelf` 使用相同的 `AlignSelf` 枚举，覆盖父级的 `justifyItems`。将 `alignSelf` 或 `justifySelf` 设为 `AlignSelf.Auto` 或 `undefined` 会移除覆盖。未设置或移除覆盖后，直接属性访问返回 `AlignSelf.Auto`，而 `get("alignSelf")` / `get("justifySelf")` 返回 `undefined`。

## 示例

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

## 后续步骤

- [多行对齐 (Align Content)](./align-content.md)
- [交叉轴对齐 (Align Items)](./align-items.md)
