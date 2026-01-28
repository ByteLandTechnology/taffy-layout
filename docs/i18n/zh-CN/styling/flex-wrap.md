---
title: Flex 换行 (Flex Wrap)
sidebar_position: 9
---

# Flex 换行 (Flex Wrap)

**控制子元素是保持在一行还是可以换行。**

`flexWrap` 属性控制当子元素在主轴方向的单行中放不下时的行为。

## 取值

| 值                | 描述                                                                                    |
| :---------------- | :-------------------------------------------------------------------------------------- |
| **`NoWrap`**      | **默认值**。所有子元素保持在一行。它们可能会收缩（如果设置了 `flexShrink`）或溢出容器。 |
| **`Wrap`**        | 子元素在需要时换行到多行，从上到下排列。                                                |
| **`WrapReverse`** | 子元素在需要时换行到多行，从下到上排列。                                                |

## 示例

```tsx live
const tree = new TaffyTree();

const style = new Style({
  size: { width: 60, height: 40 },
  margin: { bottom: 5, right: 5 },
});

// 创建多个子元素以强制换行
const children = Array.from({ length: 8 }).map(() => tree.newLeaf(style));

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  // 修改这里: NoWrap, Wrap, WrapReverse
  flexWrap: FlexWrap.Wrap,
  size: { width: 200, height: 200 },
  padding: { left: 10, right: 10, top: 10, bottom: 10 },
});

const root = tree.newWithChildren(rootStyle, children);

tree.computeLayout(root, { width: 200, height: 200 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## 后续步骤

- [Flex 基准 (Flex Basis)](./flex-basis.md)
- [多行对齐 (Align Content)](./align-content.md)
