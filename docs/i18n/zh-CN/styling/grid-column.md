---
title: 网格列 (Grid Column)
sidebar_position: 19
---

# 网格列 (Grid Column)

控制项目在网格中的水平放置。

**`gridColumn`** 属性定义了项目应占据网格中的哪些列。

## 示例

```tsx live
const tree = new TaffyTree();

const childStyle = new Style();
// 占据从第 1 到第 3 条网格线的列
childStyle.gridColumn = { start: 1, end: { span: 2 } };
childStyle.size = { width: "auto", height: "auto" };
const child = tree.newLeaf(childStyle);

const rootStyle = new Style();
rootStyle.display = Display.Grid;
rootStyle.gridTemplateColumns = [
  { min: 60, max: 60 },
  { min: 60, max: 60 },
  { min: 60, max: 60 },
];
rootStyle.gridTemplateRows = [{ min: 40, max: 40 }];
rootStyle.gap = { width: 8, height: 8 };
rootStyle.size = { width: 220, height: 60 };
rootStyle.padding = { left: 8, right: 8, top: 8, bottom: 8 };

const root = tree.newWithChildren(rootStyle, [child]);

tree.computeLayout(root, {
  width: 220,
  height: 60,
});

console.log(`列跨度: 2`);

return <TaffyTreePreview tree={tree} root={root} />;
```

## 快速记事

- `start` 和 `end` 定义了网格线。
- 您可以使用绝对索引（从 1 开始）或相对跨度（例如 `{ span: 2 }`）。

## 下一步

- [网格行 (Grid Row)](./grid-row.md)
- [网格模板 (Grid Templates)](./grid-templates.md)
