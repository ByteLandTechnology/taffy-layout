---
title: Computing Layouts
sidebar_position: 4
---

# Computing Layouts

**Turn your styles and tree structure into concrete pixel coordinates.**

Once your tree is built, you call `computeLayout` to calculate the final positions and sizes of every node.

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
//    The engine has now populated the layout data for every node.
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

Taffy employs intelligent caching. If you modify a specific node's style or content, only the affected parts of the tree are recomputed in the next pass.

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
//    Taffy skips recalculating unaffected branches.
tree.computeLayout(root, { width: 800, height: 600 });
```

## Rounding & Precision

By default, Taffy rounds all output coordinates to the nearest pixel (integer) to align with standard display grids.

### Disabling Rounding

For scenarios like high-DPI rendering or vector graphics where sub-pixel precision matters, you can disable rounding.

```ts
const tree = new TaffyTree();

// Enable sub-pixel precision
tree.disableRounding();

// ... compute layout ...
const node = tree.newLeaf(new Style());
const layout = tree.getLayout(node);
console.log(layout.width); // Might be 33.33333... instead of 33
```

## Debug Tips

- 🖨️ **`printTree(root)`**: Prints a text representation of your entire tree depth, styles, and computed layout. Essential for debugging.
- 🔒 **Isolate**: If a complex tree behaves oddly, create a small reproduction with just the problematic nodes to isolate the issue.

## Next Steps

- ⚙️ **[Configuration](./configuration.md)** - Adjust engine settings.
- 🔍 **[Debugging Layouts](../advanced/debugging.md)** - Learn how to troubleshoot.
