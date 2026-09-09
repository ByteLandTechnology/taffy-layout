---
title: Configuration
sidebar_position: 5
---

# Configuration

**Optimize Taffy for your specific use case.**

Taffy works out-of-the-box, but you can tune it for performance, precision, or resource constraints.

## Capacity Pre-allocation

If you know roughly how many nodes you'll need, initializing the tree with a capacity can reduce memory re-allocations and improve startup performance.

```tsx live
// Initialize with capacity for 1,000 nodes
const tree = TaffyTree.withCapacity(1000);
console.log(`Initial Node Count: ${tree.totalNodeCount()}`); // 0 actual nodes

const style = new Style({
  display: Display.Flex,
  size: { width: 200, height: 40 },
  alignItems: AlignItems.Center,
  justifyContent: JustifyContent.Center,
});

const root = tree.newLeaf(style);
tree.computeLayout(root, { width: 200, height: 40 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## Rounding Settings

Control whether computed layout boxes are snapped to integer coordinates or retain fractional values. Sizes are calculated from rounded edges, so adjacent boxes can receive different rounded widths. Margins and detailed Grid track data can remain fractional even with rounding enabled.

| Setting              | Function                 | Description                                          |
| :------------------- | :----------------------- | :--------------------------------------------------- |
| **Enable Rounding**  | `tree.enableRounding()`  | **Default**. Snaps layout boxes to the integer grid. |
| **Disable Rounding** | `tree.disableRounding()` | Retains fractional layout values as 32-bit floats.   |

```tsx live
const tree = new TaffyTree();

// Create two items that would sum to 101px
// 50.5 + 50.5 = 101
const style = new Style({
  size: { width: 50.5, height: 50 },
  display: Display.Flex,
  justifyContent: JustifyContent.Center,
  alignItems: AlignItems.Center,
});
const child1 = tree.newLeaf(style);
const child2 = tree.newLeaf(style);

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  size: { width: 150, height: 60 },
  alignItems: AlignItems.Center,
});
const root = tree.newWithChildren(rootStyle, [child1, child2]);

// 1. Default (Rounding Enabled)
tree.computeLayout(root, { width: 150, height: 60 });
const roundedLayout = tree.getLayout(child1);
console.log(`Rounded Width: ${roundedLayout.width}`); // 51
roundedLayout.free();

// 2. Disable Rounding
tree.disableRounding();
tree.computeLayout(root, { width: 150, height: 60 });
const preciseLayout = tree.getLayout(child1);
// preciseLayout.width is 50.5

console.log(`Precise Width: ${preciseLayout.width}`);
preciseLayout.free();

return <TaffyTreePreview tree={tree} root={root} />;
```

## Memory Management

`TaffyTree`, `Style`, `Layout`, and `TaffyError` objects own WASM allocations. When the environment supports `FinalizationRegistry`, the bindings register automatic cleanup, but its timing is not guaranteed. Use `.free()` for deterministic cleanup, and do not access an object after freeing it.

To prevent memory spikes or leaks in the WASM heap, you should explicitly manage memory:

- **Reuse:** Use `.clear()` to remove all nodes while retaining allocated node capacity. Existing node IDs become invalid.
- **Dispose:** Use `.free()` if you are completely done with a tree and want to release its memory immediately.

Node creation and `setStyle()` copy styles into the tree. `getStyle()`, `getLayout()`, and the fifth measure-callback argument return independent owned objects: free those copies when finished. Freeing the tree does not free copies you already obtained. Ordinary DTOs such as `layout.size` and `detailedLayoutInfo()` are JavaScript data and have no `.free()` method.

```ts
const tree = new TaffyTree();

const style = new Style({ width: 100, height: 50 });
const node = tree.newLeaf(style);
style.free(); // The tree has its own style copy.
tree.computeLayout(node, { width: 100, height: 50 });
const layout = tree.getLayout(node);
console.log(layout.size);
layout.free();

// Reuse the tree if more nodes will be needed.
// Clears all nodes but keeps memory allocated
tree.clear();

// Free the tree when completely finished.
tree.free();
```

## Next Steps

- 🎨 **[Styling Guide](../styling/index.md)** - Learn about Flexbox and Grid.
- 🛠️ **[Advanced Topics](../advanced/index.md)** - Debugging and Internals.
