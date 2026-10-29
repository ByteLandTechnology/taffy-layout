---
title: The Layout Object
sidebar_position: 3
---

# The Layout Object

The **`Layout`** object contains the final, computed results for a node after the layout process has finished.

## Key Properties

- **`position`**: The node's `x` and `y` coordinates, using the reference box described below.
- **`size`**: The computed `width` and `height` of the node in pixels.
- **`margin` / `padding` / `border`**: The resolved sizes of the edges.
- **`contentSize`**: The reachable content extents, also available as `contentWidth` and `contentHeight`.

In-flow nodes use their parent's border box as the coordinate origin. Absolute descendants use the nearest non-static ancestor's border box; without such an ancestor, they use the root's border box. Fixed descendants also report root-relative coordinates, although their positioning area is the initial containing block defined by `availableSpace`. Root coordinates are canvas coordinates and account for margins, direction and applicable insets; they are not necessarily `(0, 0)`. When drawing, add the corresponding reference node's canvas position rather than always adding the direct parent's position. See [Position](../styling/position.md) and the [drawing example](../advanced/debugging.md).

`contentWidth` and `contentHeight` expose the `right` and `bottom` extents of Taffy's scrollable overflow rectangle. They are measured from the scroll origin: the padding box's top-left corner in LTR and top-right corner in RTL. They describe the reachable scrolling direction and exclude overflow before that origin. They are not the full bounding rectangle of all overflowing content, and can exceed the node's size.

## Usage

After calling `computeLayout(root, ...)`, retrieve the layout for nodes within that computed subtree. `getLayout()` returns an owned snapshot: later layout passes do not update an existing `Layout` object. Retrieve a fresh snapshot after recomputing and call `layout.free()` when finished.

```typescript
const tree = new TaffyTree();
const node = tree.newLeaf(new Style());
tree.computeLayout(node, { width: 100, height: 100 });

const layout = tree.getLayout(node);

console.log(`Position: (${layout.position.x}, ${layout.position.y})`);
console.log(`Size: ${layout.size.width}x${layout.size.height}`);
layout.free();
```

## Next Steps

- [Size, Space, and Units](./size-and-space.md)
- [Measure Functions](./measure-functions.md)
