---
title: Quick Start
sidebar_position: 2
---

# ⚡ Quick Start

**Get a working Taffy layout in minutes.**

## 1. Minimal Example

This example creates a simple Flexbox layout with a container and two children.

```ts
import {
  loadTaffy,
  TaffyTree,
  Style,
  Display,
  FlexDirection,
  AlignItems,
} from "taffy-layout";

// 1. Initialize the library
await loadTaffy();
const tree = new TaffyTree();

// 2. Create styles
const containerStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Column,
  alignItems: AlignItems.Center,
  size: { width: 300, height: 200 },
  padding: { left: 10, right: 10, top: 10, bottom: 10 },
});

const childStyle = new Style({
  flexGrow: 1,
  size: { width: "100%", height: "auto" },
});

// 3. Create the node tree
//    (Leaf nodes must be created before parents if using newWithChildren)
const child1 = tree.newLeaf(childStyle);
const child2 = tree.newLeaf(childStyle);
const container = tree.newWithChildren(containerStyle, [child1, child2]);

// 4. Compute layout
//    Pass the root node and the available space
tree.computeLayout(container, { width: 300, height: 200 });

// 5. Read computed results
const containerLayout = tree.getLayout(container);
const child1Layout = tree.getLayout(child1);

console.log(`Container: ${containerLayout.width}x${containerLayout.height}`);
// Output: Container: 300x200

// 6. Debugging: Print the whole tree structure
console.log(tree.printTree(container));
```

## ⏭️ Next Steps

- 🏗️ **[Building Trees](./building-trees.md)** - Learn how to manipulate the node tree.
- 📐 **[Computing Layouts](./computing-layouts.md)** - Understanding the layout pass.
- 🧠 **[Core Concepts](../core-concepts/overview.md)** - Deep dive into Taffy's model.
