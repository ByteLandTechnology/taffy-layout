---
title: Grid Template（网格模板）
sidebar_position: 17
---

# ▦ Grid Templates（网格模板）

**定义网格的行和列。**

`gridTemplateColumns` 和 `gridTemplateRows` 属性定义网格的轨道尺寸函数。

> [!TIP]
> 🔗 **MDN 文档**: [grid-template-columns](https://developer.mozilla.org/zh-CN/docs/Web/CSS/grid-template-columns), [grid-template-rows](https://developer.mozilla.org/zh-CN/docs/Web/CSS/grid-template-rows)

## 🎛️ 轨道尺寸

每个轨道使用 `min` 和 `max` 尺寸函数定义：

| 值                         | 描述                         | 示例 (JS)                             |
| :------------------------- | :--------------------------- | :------------------------------------ |
| **Points（点）**           | 像素单位的固定尺寸。         | `{ min: 100, max: 100 }`              |
| **Percent（百分比）**      | 容器尺寸的百分比。           | `{ min: 0, max: '50%' }`              |
| **Flex (fr)（弹性）**      | 剩余空间的份额（分数单位）。 | `{ min: 0, max: '1fr' }`              |
| **Auto（自动）**           | 基于内容和可用空间的尺寸。   | `{ min: 'auto', max: 'auto' }`        |
| **MinContent（最小内容）** | 适合内容的最小可能尺寸。     | `{ min: 'min-content', max: 'auto' }` |
| **MaxContent（最大内容）** | 适合内容的最大可能尺寸。     | `{ min: 'auto', max: 'max-content' }` |

## 💻 示例

```tsx live
const tree = new TaffyTree();

const rootStyle = new Style({
  display: Display.Grid,
  size: { width: 260, height: 140 },
  gridTemplateColumns: [
    { min: 60, max: 60 },
    { min: 0, max: "1fr" },
    { min: 60, max: 60 },
  ],
  gridTemplateRows: [{ min: 0, max: "1fr" }],
  padding: { left: 10, right: 10, top: 10, bottom: 10 },
  gap: { width: 10, height: 10 },
});

const childStyle = new Style({
  alignSelf: AlignSelf.Center,
  justifySelf: AlignSelf.Center,
});
const child1 = tree.newLeaf(childStyle);
const child2 = tree.newLeaf(childStyle);
const child3 = tree.newLeaf(childStyle);
const child4 = tree.newLeaf(childStyle);
const child5 = tree.newLeaf(childStyle);
const child6 = tree.newLeaf(childStyle);

const root = tree.newWithChildren(rootStyle, [
  child1,
  child2,
  child3,
  child4,
  child5,
  child6,
]);

tree.computeLayout(root, {
  width: 260,
  height: 140,
});

console.log(`Columns: 3`);

return <TaffyTreePreview tree={tree} root={root} />;
```

## API 参考

- [GridTemplateComponent](../../api/type-aliases/GridTemplateComponent.md)

## 后续步骤

- [Grid Placement（网格放置）](./grid-placement.md)
- [Grid Auto Flow（网格自动流向）](./grid-auto-flow.md)
