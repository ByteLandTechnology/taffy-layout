---
title: Positioning
sidebar_position: 13
---

# 📍 Position

**Control how an element is placed in the document.**

The `position` property determines if an element participates in the normal layout flow or if it is removed and positioned manually.

> [!TIP]
> 🔗 **MDN Documentation**: [position](https://developer.mozilla.org/en-US/docs/Web/CSS/position)

## 🎛️ Values

| Value          | Description                                                                                                                                              |
| :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **`Relative`** | **Default**. The element remains in the document flow. `inset` offsets move it visually, but it still takes up space in its original location.           |
| **`Absolute`** | The element is **removed from the flow**. It is positioned relative to its nearest _positioned_ ancestor (parent with non-default position) or the root. |

## 💻 Example

```tsx live
const tree = new TaffyTree();

const rootStyle = new Style({
  display: Display.Flex,
  size: { width: 300, height: 120 },
  padding: { left: 10, right: 10, top: 10, bottom: 10 },
  gap: { width: 10, height: 0 },
});

const standardItem = new Style({
  size: { width: 60, height: 60 },
  display: Display.Flex,
  justifyContent: JustifyContent.Center,
  alignItems: AlignItems.Center,
});

const absoluteItem = new Style({
  position: Position.Absolute,
  size: { width: 40, height: 40 },
  inset: { top: 0, right: 0, left: "auto", bottom: "auto" },
});

const child1 = tree.newLeaf(standardItem);
const child2 = tree.newLeaf(standardItem);

// This child floats over the others
const childAbs = tree.newLeaf(absoluteItem);

const root = tree.newWithChildren(rootStyle, [child1, child2, childAbs]);

tree.computeLayout(root, { width: 300, height: 120 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## ⏭️ Next Steps

- **[Inset](./inset.md)** - Define the top/right/bottom/left coordinates.
- **[Display](./display.md)** - Switch between Flex and None.
