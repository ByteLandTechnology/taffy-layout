---
title: Core Concepts Overview
sidebar_position: 1
---

# Core Concepts Overview

Taffy has a small API surface, but a consistent layout model. This section gives you the concepts to reason about any layout.

## The Three Core Objects

- **Style**: layout rules for a node
- **TaffyTree**: layout tree + compute entry point
- **Layout**: computed results (position, size, margins)

```
Style + Tree  -> computeLayout -> Layout
```

## Tree Model

Every node can have children, and the parent controls how those children are placed along the main and cross axes.

```
Root (flex)
├── Sidebar
└── Content
    ├── Header
    └── Body
```

### Node Creation APIs

- `newLeaf(style)` creates a leaf
- `newWithChildren(style, children)` creates a container
- `addChild` / `insertChildAtIndex` mutate the tree

## Layout Flow

1. Configure root and child styles
2. Call `computeLayout(root, availableSpace)`
3. Read `getLayout(node)` results

```tsx live
const tree = new TaffyTree();

const style = new Style();
style.flexGrow = 1;

const child1 = tree.newLeaf(style);
const child2 = tree.newLeaf(style);

const rootStyle = new Style();
rootStyle.display = Display.Flex;
rootStyle.flexDirection = FlexDirection.Row;
rootStyle.size = { width: 400, height: 200 };

const root = tree.newWithChildren(rootStyle, [child1, child2]);

tree.computeLayout(root, {
  width: 400,
  height: 200,
});

console.log(tree.printTree(root));

return <TaffyTreePreview tree={tree} root={root} />;
```

## What Layout Includes

- `x`, `y`: position relative to parent
- `width`, `height`: computed size
- `margin`, `padding`, `border`: edge sizes

## Next Steps

- [Size, Space, and Units](./size-and-space.md)
- [Measure Functions](./measure-functions.md)
- [Layout Cookbook](../cookbook/)
