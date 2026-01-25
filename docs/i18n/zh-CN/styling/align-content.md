---
title: Align Content（多行对齐）
sidebar_position: 8
---

# 📚 Align Content（多行对齐）

**控制多行弹性容器中行的对齐方式。**

`alignContent` 属性用于在交叉轴上有剩余空间时，对齐弹性容器内的行。**此属性对单行弹性容器（即 `flexWrap` 为 `NoWrap`）无效**。

> [!TIP]
> 🔗 **MDN 文档**: [align-content](https://developer.mozilla.org/zh-CN/docs/Web/CSS/align-content)

## 🎛️ 取值

| 值                 | 描述                                               |
| :----------------- | :------------------------------------------------- |
| **`Stretch`**      | **默认值**。行被拉伸以占据剩余空间。               |
| **`FlexStart`**    | 行紧凑排列在容器起始位置。                         |
| **`FlexEnd`**      | 行紧凑排列在容器结束位置。                         |
| **`Center`**       | 行紧凑排列在容器中间。                             |
| **`SpaceBetween`** | 行均匀分布；第一行在起始位置，最后一行在结束位置。 |
| **`SpaceAround`**  | 行均匀分布，行与行之间有相等的空间。               |

## 💻 示例

```tsx live
const tree = new TaffyTree();

const itemStyle = new Style({
  size: { width: 80, height: 30 },
  margin: { bottom: 5 },
});

// 创建足够的子元素以强制换行
const children = Array.from({ length: 5 }).map(() => tree.newLeaf(itemStyle));

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  flexWrap: FlexWrap.Wrap, // alignContent 生效所必需
  size: { width: 200, height: 200 }, // 必须有额外的垂直空间

  // 修改这里以测试不同的值
  alignContent: AlignContent.Center,
});

const root = tree.newWithChildren(rootStyle, children);

tree.computeLayout(root, { width: 200, height: 200 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## ⏭️ 后续步骤

- **[Flex Wrap（换行）](./flex-wrap.md)** - 启用换行以使用此属性。
- **[Align Items（交叉轴对齐）](./align-items.md)** - 在单行内对齐子元素。
