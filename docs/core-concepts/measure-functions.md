---
title: Measure Functions
sidebar_position: 5
---

# Measure Functions

When a leaf node's size depends on its content (e.g. text, images, or platform-specific widgets), Taffy cannot calculate the size purely from style properties. In these cases, you must provide a **Measure Function**.

## When to Use

Use `computeLayoutWithMeasure()` instead of the standard `computeLayout()` when your tree contains nodes that need custom measurement. Taffy will invoke your callback for leaf nodes that require content-based sizing (e.g. `width: auto` or `measure` mode).

## How it Works

The measure function is a callback that Taffy invokes during the layout process. It asks you: "Given these constraints, how big is this content?"

### Arguments

1.  **`knownDimensions`**: Dimensions that are explicitly defined in the node's style (e.g. if `width: 100` is set, `knownDimensions.width` will be `100`).
2.  **`availableSpace`**: The space offered by the parent node. This constraints how large the content can be.

### Return Value

The function **must** return a `Size` object containing the measured `width` and `height` in pixels.

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
  (knownDims, availableSpace) => {
    // 1. Check if we have known dimensions (style overrides)
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

## Next Steps

- [Styling Guide](../styling/index.md)
- [Layout Cookbook](../cookbook/)
