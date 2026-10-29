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
import { Position, Style, TaffyTree } from "taffy-layout";

// Mock renderer
const renderer = {
  strokeRect: (x: number, y: number, w: number, h: number, c: string) => {},
};
const tree = new TaffyTree();
const root = tree.newLeaf(new Style());
tree.computeLayout(root, { width: 100, height: 100 });

// Visual debugger function
function debugDraw(
  node: bigint,
  parentOrigin = { x: 0, y: 0 },
  absoluteOrigin = parentOrigin,
  rootOrigin = parentOrigin,
) {
  const style = tree.getStyle(node);
  const position = style.position;
  style.free();

  const origin =
    position === Position.Fixed
      ? rootOrigin
      : position === Position.Absolute
        ? absoluteOrigin
        : parentOrigin;
  const layout = tree.getLayout(node);
  const point = { x: origin.x + layout.x, y: origin.y + layout.y };
  renderer.strokeRect(point.x, point.y, layout.width, layout.height, "red");
  layout.free();

  const childRootOrigin = node === root ? point : rootOrigin;
  const childAbsoluteOrigin =
    node === root || position !== Position.Static ? point : absoluteOrigin;
  for (const child of tree.children(node)) {
    debugDraw(child, point, childAbsoluteOrigin, childRootOrigin);
  }
}
debugDraw(root);
```

Call `debugDraw(root)` with the same root passed to `computeLayout`. The traversal tracks the canvas positions of the direct parent, the nearest non-static ancestor, and the root separately. In-flow nodes use the parent; absolute nodes use the positioned ancestor, falling back to the root; fixed nodes always use the root. The root fallback is required because Taffy reports initial-containing-block descendants relative to the root's border box, even though their layout area comes from `availableSpace`. See [Position](../styling/position.md). This example draws layout rectangles; clipping, scrolling and sticky offsets belong to the renderer.

## Isolation

If a specific sub-tree is misbehaving:

1.  Create a fresh `TaffyTree`.
2.  Replicate _only_ that sub-tree structure.
3.  Hard-code the input constraints (width/height provided to the sub-tree).
4.  Run `computeLayout` and inspect.

This isolates the problem from external parent constraints.
