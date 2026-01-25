---
title: Holy Grail 布局
---

# 🏆 Holy Grail 布局

**带有页眉和页脚的经典三列布局。**

```text
┌──────────────────────────────┐
│ Header                       │
├──────────┬──────────┬────────┤
│ Left     │ Main     │ Right  │
├──────────┴──────────┴────────┤
│ Footer                       │
└──────────────────────────────┘
```

## 💻 代码

```tsx live
const tree = new TaffyTree();

const pageStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Column,
  size: { width: 600, height: 400 },
  gap: { width: 0, height: 10 },
});

const header = tree.newLeaf(new Style({ size: { height: 50 } }));
const footer = tree.newLeaf(new Style({ size: { height: 50 } }));

const bodyRowStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  flexGrow: 1, // 填充页眉/页脚之间的垂直空间
  gap: { width: 10, height: 0 },
});

const left = tree.newLeaf(new Style({ size: { width: 100, height: "100%" } }));
const right = tree.newLeaf(new Style({ size: { width: 100, height: "100%" } }));
const main = tree.newLeaf(
  new Style({
    flexGrow: 1,
    size: { width: "auto", height: "100%" },
  }),
);

const bodyRow = tree.newWithChildren(bodyRowStyle, [left, main, right]);
const root = tree.newWithChildren(pageStyle, [header, bodyRow, footer]);

tree.computeLayout(root, { width: 600, height: 400 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## ⏭️ 相关指南

- **[弹性方向](../styling/flex-direction.md)**
- **[定位](../styling/position.md)**
