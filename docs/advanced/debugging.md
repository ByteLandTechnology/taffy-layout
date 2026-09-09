---
title: Debugging
sidebar_position: 1
---

# Debugging

**Tools to inspect and troubleshoot layouts.**

## Print Tree

`tree.printTree(node)` returns a string showing the subtree, layout modes, and computed geometry. Pass the result to `console.log()` to display it.

```ts
const tree = new TaffyTree();
const root = tree.newLeaf(new Style({ width: 100, height: 100 }));
tree.computeLayout(root, { width: 100, height: 100 });

console.log(tree.printTree(root));
```

**Example Output:**

```text
└──  LEAF [x: 0    y: 0    w: 100  h: 100  content_w: 0    content_h: 0    border: l:0 r:0 t:0 b:0, padding: l:0 r:0 t:0 b:0] (4294967297)
```

`content_w` and `content_h` use the same reachable overflow extents as `layout.contentWidth` and `layout.contentHeight`. An empty leaf can have a fixed box size and zero content extents. See [The Layout Object](../core-concepts/objects-layout.md).

## Visual Debugging

If you are rendering to a canvas or screen:

1.  **Draw Borders**: Draw a colored 1px border around every computed layout rect.
2.  **Color Code**: Use different colors for different `display` types (e.g., Blue for Flex, Red for Grid).

```ts
// Mock renderer
const renderer = {
  strokeRect: (x: number, y: number, w: number, h: number, c: string) => {},
};
const tree = new TaffyTree();
const root = tree.newLeaf(new Style());
tree.computeLayout(root, { width: 100, height: 100 });

// Visual debugger function
function debugDraw(node: bigint, parentX = 0, parentY = 0) {
  const layout = tree.getLayout(node);
  const x = parentX + layout.x;
  const y = parentY + layout.y;
  renderer.strokeRect(x, y, layout.width, layout.height, "red");
  layout.free();

  for (const child of tree.children(node)) {
    debugDraw(child, x, y);
  }
}
debugDraw(root);
```

Layout positions are relative to each node's parent. Accumulate ancestor offsets when drawing into a shared canvas coordinate system, including for absolutely positioned nodes.

## Isolation

If a specific sub-tree is misbehaving:

1.  Create a fresh `TaffyTree`.
2.  Replicate _only_ that sub-tree structure.
3.  Hard-code the input constraints (width/height provided to the sub-tree).
4.  Run `computeLayout` and inspect.

This isolates the problem from external parent constraints.
