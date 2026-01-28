---
title: The Layout Object
sidebar_position: 3
---

# The Layout Object

The **`Layout`** object contains the final, computed results for a node after the layout process has finished.

## Key Properties

- **`location`**: The `x` and `y` coordinates of the node relative to its parent's top-left corner.
- **`size`**: The computed `width` and `height` of the node in pixels.
- **`margin` / `padding` / `border`**: The resolved sizes of the edges.

## Usage

After calling `computeLayout` on the tree, you can retrieve the layout for any node in the tree.

```typescript
const tree = new TaffyTree();
const node = tree.newLeaf(new Style());
tree.computeLayout(node, { width: 100, height: 100 });

const layout = tree.getLayout(node);

console.log(`Position: (${layout.position.x}, ${layout.position.y})`);
console.log(`Size: ${layout.size.width}x${layout.size.height}`);
```

## Next Steps

- [Size, Space, and Units](./size-and-space.md)
- [Measure Functions](./measure-functions.md)
