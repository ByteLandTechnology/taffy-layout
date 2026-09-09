---
title: Aspect Ratio
sidebar_position: 3
---

# Aspect Ratio

**Maintain a specific ratio between width and height.**

The `aspectRatio` property sets a preferred ratio for the item's dimensions. It can derive an auto dimension from the other dimension. Explicit sizes, min/max constraints, and the surrounding layout still determine the final dimensions.

## Usage

Pass a single number representing the ratio `width / height`.

- `1.0` = Square (1:1)
- `1.5` = Landscape (3:2)
- `1.77` ≈ 16:9
- `0.56` ≈ 9:16

## Example

```tsx live
const tree = new TaffyTree();

const container = new Style({
  display: Display.Flex,
  size: { width: 300, height: 300 },
  alignItems: AlignItems.Center,
  justifyContent: JustifyContent.Center,
});

const imagePlaceholder = tree.newLeaf(
  new Style({
    // Fix width, let height be determined by ratio
    size: { width: 100, height: "auto" },
    aspectRatio: 16 / 9, // Wide
  }),
);

const root = tree.newWithChildren(container, [imagePlaceholder]);

tree.computeLayout(root, { width: 300, height: 300 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## Next Steps

- [Margin](./margin.md)
- [Size](./size.md)
