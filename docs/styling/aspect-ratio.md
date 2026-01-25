---
title: Aspect Ratio
sidebar_position: 15
---

# 🖼️ Aspect Ratio

**Maintain a specific ratio between width and height.**

The `aspectRatio` property sets a preferred ratio for the item's dimensions. If one dimension is set (e.g., width) and the other is `auto` (height), Taffy will calculate the missing dimension to satisfy the ratio.

> [!TIP]
> 🔗 **MDN Documentation**: [aspect-ratio](https://developer.mozilla.org/en-US/docs/Web/CSS/aspect-ratio)

## 🎛️ Usage

Pass a single number representing the ratio `width / height`.

- `1.0` = Square (1:1)
- `1.5` = Landscape (3:2)
- `1.77` ≈ 16:9
- `0.56` ≈ 9:16

## 💻 Example

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

## ⏭️ Next Steps

- **[Width and Height](./size.md)** - Set explicit dimensions.
