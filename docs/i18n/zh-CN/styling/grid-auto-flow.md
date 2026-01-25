---
title: Grid Auto Flow（网格自动流向）
sidebar_position: 19
---

# 🌊 Grid Auto Flow（网格自动流向）

**控制自动放置的子元素如何流入网格。**

如果您有超过显式单元格数量的子元素，或者您没有显式放置子元素，`gridAutoFlow` 将控制它们如何填充网格。

> [!TIP]
> 🔗 **MDN 文档**: [grid-auto-flow](https://developer.mozilla.org/zh-CN/docs/Web/CSS/grid-auto-flow)

## 🎛️ 取值

| 值                | 描述                                           |
| :---------------- | :--------------------------------------------- |
| **`Row`**         | **默认值**。子元素填充当前行，然后移至下一行。 |
| **`Column`**      | 子元素填充当前列，然后移至下一列。             |
| **`RowDense`**    | 类似 Row，但尝试回填网格中的空隙。             |
| **`ColumnDense`** | 类似 Column，但尝试回填空隙。                  |

## 💻 示例

```tsx live
const tree = new TaffyTree();
const rootStyle = new Style({ display: Display.Grid });
const child1 = tree.newLeaf(new Style());
const child2 = tree.newLeaf(new Style());
const child3 = tree.newLeaf(new Style());
const child4 = tree.newLeaf(new Style());

rootStyle.gridTemplateColumns = [
  { min: 60, max: 60 },
  { min: 60, max: 60 },
];
rootStyle.gridTemplateRows = [
  { min: 40, max: 40 },
  { min: 40, max: 40 },
];
rootStyle.gap = { width: 8, height: 8 };
rootStyle.size = { width: 160, height: 100 };
rootStyle.padding = { left: 8, right: 8, top: 8, bottom: 8 };

const root = tree.newWithChildren(rootStyle, [child1, child2, child3, child4]);

tree.computeLayout(root, {
  width: 160,
  height: 100,
});

console.log(`Auto flow: Row`);

return <TaffyTreePreview tree={tree} root={root} />;
```

## 典型用例

- 自动卡片网格
- 响应式仪表板

## API 参考

- [GridAutoFlow 枚举](../../api/enumerations/GridAutoFlow.md)

## 后续步骤

- **[Grid Templates（网格模板）](./grid-templates.md)** - 定义网格结构。
- [Grid Placement（网格放置）](./grid-placement.md)
