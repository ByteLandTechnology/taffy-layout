---
title: Computing Layouts
sidebar_position: 4
---

# Computing Layouts

**Turn your styles and tree structure into concrete pixel coordinates.**

Once your tree is built, call `computeLayout` on a node to calculate positions and sizes for that node and its descendants.

## Standard Layout Computation

You must provide the **Available Space** (the constraints for the root node) when computing the layout.

```tsx live
const tree = new TaffyTree();
const rootStyle = new Style({
  display: Display.Flex,
  alignItems: AlignItems.Center,
  justifyContent: JustifyContent.Center,
  size: { width: 400, height: 100 },
});

const child = tree.newLeaf(
  new Style({
    size: { width: 50, height: 50 },
  }),
);
const root = tree.newWithChildren(rootStyle, [child]);

// 1. Compute layout
//    We pass the constraints: width: 400, height: 100
tree.computeLayout(root, { width: 400, height: 100 });

// 2. Read Results
//    The engine has now populated the layout data for this subtree.
const rootLayout = tree.getLayout(root);
const childLayout = tree.getLayout(child);

console.log(`Root Size: ${rootLayout.width}x${rootLayout.height}`);
console.log(`Child Pos: ${childLayout.x}, ${childLayout.y}`);

return (
  <div
    style={{
      width: rootLayout.width,
      height: rootLayout.height,
      background: "#f0f0f0",
      position: "relative",
    }}
  >
    <div
      style={{
        width: childLayout.width,
        height: childLayout.height,
        left: childLayout.x,
        top: childLayout.y,
        position: "absolute",
        background: "#007aff",
      }}
    />
    <div
      style={{
        position: "absolute",
        bottom: 5,
        right: 5,
        fontSize: 10,
        color: "#666",
      }}
    >
      Child at ({childLayout.x}, {childLayout.y})
    </div>
  </div>
);
```

## Incremental Layouts

Taffy reuses cached results when their inputs still match. Tree operations such as `setStyle()` invalidate affected caches, including ancestor caches. A change can also alter siblings' sizes or positions.

```ts
const tree = new TaffyTree();
const root = tree.newLeaf(new Style());
const childNode = tree.newLeaf(new Style());
tree.addChild(root, childNode);

// 1. First Layout
tree.computeLayout(root, { width: 800, height: 600 });

// 2. Modify a leaf node
const newStyle = new Style({ width: 250 });
tree.setStyle(childNode, newStyle);

// 3. Re-compute
//    Taffy reuses cached results where the layout inputs still match.
tree.computeLayout(root, { width: 800, height: 600 });
```

Changing a `Style` copy does not update the tree until `setStyle()` is called. Mutating a context object in place, changing external content, or passing a different measure function does not invalidate measurement caches automatically. Call `markDirty(node)` for affected measured nodes, or use `setNodeContext(node, context)`, before the next computation. See [Measure Functions](../core-concepts/measure-functions.md#cache-invalidation).

## Rounding & Precision

By default, Taffy snaps computed layout boxes to integer coordinates. Widths and heights come from rounded edges; this can give adjacent equal-sized items different rounded sizes. Margins and detailed Grid track data can still contain fractions.

### Disabling Rounding

For scenarios like high-DPI rendering or vector graphics where sub-pixel precision matters, you can disable rounding.

```ts
const tree = new TaffyTree();

// Enable sub-pixel precision
tree.disableRounding();

const node = tree.newLeaf(new Style({ width: 100 / 3, height: 20 }));
tree.computeLayout(node, { width: 100, height: 100 });
const layout = tree.getLayout(node);
console.log(layout.width); // Approximately 33.33333 (32-bit float)
layout.free();
```

## Debug Tips

- 🖨️ **`printTree(root)`**: Returns a string showing the tree hierarchy and computed layouts; pass it to `console.log()` to display it.
- 🔒 **Isolate**: If a complex tree behaves oddly, create a small reproduction with just the problematic nodes to isolate the issue.

## Next Steps

- ⚙️ **[Configuration](./configuration.md)** - Adjust engine settings.
- 🔍 **[Debugging Layouts](../advanced/debugging.md)** - Learn how to troubleshoot.
