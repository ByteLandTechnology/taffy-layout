---
title: 主轴对齐 (Justify Content)
sidebar_position: 13
---

# 主轴对齐 (Justify Content)

**沿主轴对齐子元素。**

`justifyContent` 属性沿**主轴**对齐子元素（如果 `flexDirection` 是 `Row`，则为水平方向；如果是 `Column`，则为垂直方向）。

## 取值

| 值                 | 描述                                                     |
| :----------------- | :------------------------------------------------------- |
| **`FlexStart`**    | **默认值**。子元素向行的起始端对齐。                     |
| **`FlexEnd`**      | 子元素向行的末端对齐。                                   |
| **`Center`**       | 子元素在行中居中对齐。                                   |
| **`SpaceBetween`** | 子元素均匀分布。首元素在起始端，末元素在末端。           |
| **`SpaceAround`**  | 子元素均匀分布，每个子元素周围的空间相等。               |
| **`SpaceEvenly`**  | 子元素均匀分布，任意两个子元素（和边缘）之间的空间相等。 |

## 示例

```tsx live
const tree = new TaffyTree();

const style = new Style({
  size: { width: 50, height: 50 },
});

const child1 = tree.newLeaf(style);
const child2 = tree.newLeaf(style);
const child3 = tree.newLeaf(style);

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  // 修改这里以测试不同值
  justifyContent: JustifyContent.SpaceBetween,
  size: { width: 300, height: 80 },
  padding: { left: 10, right: 10, top: 10, bottom: 10 },
});

const root = tree.newWithChildren(rootStyle, [child1, child2, child3]);

tree.computeLayout(root, { width: 300, height: 80 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## API 参考

- [JustifyContent 枚举](../../api/enumerations/JustifyContent.md)

## 后续步骤

- [交叉轴对齐 (Align Items)](./align-items.md)
- [Flex 方向 (Flex Direction)](./flex-direction.md)
