---
title: 网格行 (Grid Row)
sidebar_position: 20
---

# 网格行 (Grid Row)

控制项目在网格中的垂直放置。

**`gridRow`** 属性定义了项目应占据网格中的哪些行。

## 示例

```tsx live
const tree = new TaffyTree();

const childStyle = new Style();
// 占据从第 1 到第 3 条网格线的行
childStyle.gridRow = { start: 1, end: { span: 2 } };
childStyle.size = { width: "auto", height: "auto" };
const child = tree.newLeaf(childStyle);

const rootStyle = new Style();
rootStyle.display = Display.Grid;
rootStyle.gridTemplateColumns = [{ min: 60, max: 60 }];
rootStyle.gridTemplateRows = [
  { min: 40, max: 40 },
  { min: 40, max: 40 },
  { min: 40, max: 40 },
];
rootStyle.gap = { width: 8, height: 8 };
rootStyle.size = { width: 100, height: 200 };
rootStyle.padding = { left: 8, right: 8, top: 8, bottom: 8 };

const root = tree.newWithChildren(rootStyle, [child]);

tree.computeLayout(root, {
  width: 100,
  height: 200,
});

console.log(`行跨度: 2`);

return <TaffyTreePreview tree={tree} root={root} />;
```

## 快速记事

- `start` 和 `end` 定义了网格线。
- 您可以使用绝对索引（从 1 开始）或相对跨度（例如 `{ span: 2 }`）。

## 下一步

- [网格自动流向 (Grid Auto Flow)](./grid-auto-flow.md)
- [网格模板 (Grid Templates)](./grid-templates.md)
