---
title: Measure Functions
sidebar_position: 5
---

# Measure Functions

When a leaf node's size depends on its content (e.g. text, images, or platform-specific widgets), Taffy cannot calculate the size purely from style properties. In these cases, you must provide a **Measure Function**.

## When to Use

Use `computeLayoutWithMeasure()` instead of the standard `computeLayout()` when your tree contains nodes that need custom measurement. Taffy invokes your callback when a leaf needs content measurements, such as intrinsic sizing for an auto-sized text node. The callback may run more than once with different constraints, or be skipped when layout can use known dimensions or cached results.

## How it Works

The measure function is a callback that Taffy invokes during the layout process. It asks you: "Given these constraints, how big is this content?"

### Arguments

1. **`knownDimensions`**: Dimension hints already determined by the engine for this measurement. Each axis is a number or `undefined`. This is not a copy of the style: even a node with a fixed width can receive `undefined` during a measurement pass.
2. **`availableSpace`**: The space available for the content after layout accounts for padding, borders, and scrollbars. Each axis is a number, `"min-content"`, or `"max-content"`; handle these keywords before using the value in arithmetic.
3. **`node`**: The `bigint` ID of the node being measured.
4. **`context`**: The user value attached through `newLeafWithContext()` or `setNodeContext()`, or `undefined` when no context is attached.
5. **`style`**: An owned copy of the node's current `Style`. Changing it does not update the tree. Call `style.free()` when finished with the copy.

### Return Value

The function must return `{ width, height }` with the measured content dimensions in pixels. Preserve any supplied known dimension when measuring that axis. Taffy then applies the node's padding, borders, box sizing, and size constraints; do not add those edges to the returned content measurement.

The callback must return synchronously. A thrown error or a return value that cannot be decoded as a numeric `{ width, height }` is treated as a zero content measurement by the binding. Handle measurement failures inside the callback; see [Error Handling](../advanced/error-handling.md#measurement-errors).

## Example

```tsx live
const tree = new TaffyTree();

const style = new Style();
// This node has no fixed size, so Taffy will ask the measure function
style.size = { width: "auto", height: "auto" };

const measuredNode = tree.newLeaf(style);

const rootStyle = new Style();
rootStyle.display = Display.Flex;
rootStyle.size = { width: 300, height: 100 };
rootStyle.alignItems = AlignItems.Center;
rootStyle.justifyContent = JustifyContent.Center;

const root = tree.newWithChildren(rootStyle, [measuredNode]);

// We use computeLayoutWithMeasure instead of computeLayout
tree.computeLayoutWithMeasure(
  root,
  { width: 300, height: 100 },
  (knownDims, availableSpace, node, context, measuredStyle) => {
    measuredStyle.free();
    // 1. Preserve dimensions already known for this measurement
    // 2. Otherwise, calculate based on available space or content intrinsic size
    const width =
      knownDims.width ??
      (typeof availableSpace.width === "number"
        ? Math.min(availableSpace.width, 150)
        : 150);

    const height = knownDims.height ?? 50;

    return { width, height };
  },
);

return (
  <div style={{ display: "flex", gap: 10 }}>
    <TaffyTreePreview tree={tree} root={root} />
    <div style={{ padding: 10, background: "#f0f0f0", borderRadius: 4 }}>
      <strong>Measured Size:</strong>
      <br />
      {tree.getLayout(measuredNode).width} x{" "}
      {tree.getLayout(measuredNode).height}
    </div>
  </div>
);
```

## Typical Use Cases

- **Text Layout**: Calculating width/height based on font size, text content, and wrapping width.
- **Images**: Returning the intrinsic dimensions of an image.
- **Native UI Widgets**: wrapping platform-specific controls that have their own sizing logic.

## Performance Tips

- **Cache Results**: Measurement can be expensive. Cache the result based on the inputs (`knownDimensions`, `availableSpace`, content string, etc.) to avoid re-calculating identical measures.
- **Avoid Side Effects**: The measure function should be pure. Do not modify the DOM or external state inside it.

## Cache Invalidation

Taffy caches measured results. Mutating an attached context object in place or changing data read by the callback does not notify the tree. Call `tree.markDirty(node)` for each affected measured node, or `tree.setNodeContext(node, updatedContext)`, before recomputing. A different callback passed to `computeLayoutWithMeasure()` also does not invalidate existing caches by itself.

## Next Steps

- [Styling Guide](../styling/index.md)
- [Layout Cookbook](../cookbook/)
