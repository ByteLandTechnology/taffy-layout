---
title: The TaffyTree Object
sidebar_position: 2
---

# The TaffyTree Object

The **`TaffyTree`** object represents the entire layout tree and serves as the entry point for layout calculations.

## Key Responsibilities

- **Node Management**: Creating, adding, removing, and inserting nodes.
- **Tree Hierarchy**: Maintaining the parent-child relationships between nodes.
- **Computation**: Invoking the layout algorithm via `computeLayout` or `computeLayoutWithMeasure`.
- **Result Retrieval**: Storing and providing access to the computed results for each node.

## Usage

```typescript
const tree = new TaffyTree();

// Create nodes
const child = tree.newLeaf(new Style());
const root = tree.newWithChildren(new Style(), [child]);

// Compute layout
tree.computeLayout(root, { width: 500, height: 500 });

// Access results
const layout = tree.getLayout(child);
```

## Next Steps

- [The Layout Object](./objects-layout.md)
- [Size, Space, and Units](./size-and-space.md)
