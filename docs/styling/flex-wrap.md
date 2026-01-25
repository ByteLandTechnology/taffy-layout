---
title: Flex Wrap
sidebar_position: 3
---

# ↩️ Flex Wrap

**Control whether items are forced onto one line or can wrap.**

The `flexWrap` property controls what happens when items don't fit in a single line along the main axis.

> [!TIP]
> 🔗 **MDN Documentation**: [flex-wrap](https://developer.mozilla.org/en-US/docs/Web/CSS/flex-wrap)

## 🎛️ Values

| Value             | Description                                                                                                          |
| :---------------- | :------------------------------------------------------------------------------------------------------------------- |
| **`NoWrap`**      | **Default**. All items are forced onto one line. They may shrink (if `flexShrink` is set) or overflow the container. |
| **`Wrap`**        | Items wrap onto multiple lines if needed, from top to bottom.                                                        |
| **`WrapReverse`** | Items wrap onto multiple lines, from bottom to top.                                                                  |

## 💻 Example

```tsx live
const tree = new TaffyTree();

const style = new Style({
  size: { width: 60, height: 40 },
  margin: { bottom: 5, right: 5 },
});

// Create many children to force wrapping
const children = Array.from({ length: 8 }).map(() => tree.newLeaf(style));

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  // CHANGE THIS: NoWrap, Wrap, WrapReverse
  flexWrap: FlexWrap.Wrap,
  size: { width: 200, height: 200 },
  padding: { left: 10, right: 10, top: 10, bottom: 10 },
});

const root = tree.newWithChildren(rootStyle, children);

tree.computeLayout(root, { width: 200, height: 200 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## ⏭️ Next Steps

- **[Align Content](./align-content.md)** - Control how multiple lines are spaced.
- **[Flex Basis, Grow, Shrink](./flex-basis-grow-shrink.md)** - Control how items resize.
