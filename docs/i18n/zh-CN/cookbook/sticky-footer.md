---
title: 粘性页脚
sidebar_position: 4
---

# 粘性页脚

**当内容较短时，页脚粘在底部；当内容增长时，页脚自然向下移动。**

```text
┌────────────────────────────┐
│ Header                     │
├────────────────────────────┤
│ Content (flex: 1)          │
├────────────────────────────┤
│ Footer                     │
└────────────────────────────┘
```

## 关键规则

- 父元素: `flexDirection: column`
- 页面高度为 `auto`，用 `minHeight` 设置视口的最小高度
- 内容: `flexGrow: 1`

## 代码

```tsx live
const tree = new TaffyTree();

// 页面容器
const pageStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Column,
  size: { width: 300, height: "auto" },
  minHeight: 300, // 内容较少时至少填满视口，内容增多时允许页面增高
});

const header = tree.newLeaf(
  new Style({ size: { width: "100%", height: 50 }, marginBottom: 10 }),
);
const footer = tree.newLeaf(
  new Style({ size: { width: "100%", height: 50 }, marginTop: 10 }),
);

// 内容增长以填充空间
const content = tree.newLeaf(
  new Style({
    flexGrow: 1,
    size: { width: "100%", height: "auto" },
  }),
);

const root = tree.newWithChildren(pageStyle, [header, content, footer]);

tree.computeLayout(root, { width: 300, height: 300 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## 相关指南

- **[Flex 伸缩](../styling/flex-grow.md)**
- **[尺寸](../styling/size.md)**
