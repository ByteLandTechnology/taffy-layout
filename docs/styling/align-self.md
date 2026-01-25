---
title: Align Self
sidebar_position: 7
---

# 🕴️ Align Self

**Override the parent's `alignItems` for a specific item.**

The `alignSelf` property allows the default alignment (or the one specified by `alignItems`) to be overridden for individual flex items.

> [!TIP]
> 🔗 **MDN Documentation**: [align-self](https://developer.mozilla.org/en-US/docs/Web/CSS/align-self)

## 🎛️ Values

| Value           | Description                                            |
| :-------------- | :----------------------------------------------------- |
| **`Auto`**      | **Default**. Inherits the parent's `alignItems` value. |
| **`Stretch`**   | Item stretches to fill the container's cross size.     |
| **`FlexStart`** | Item aligns to the start edge.                         |
| **`FlexEnd`**   | Item aligns to the end edge.                           |
| **`Center`**    | Item aligns in the center.                             |
| **`Baseline`**  | Item aligns based on its baseline.                     |

## 💻 Example

```tsx live
const tree = new TaffyTree();

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  alignItems: AlignItems.FlexStart, // Default alignment is Top
  size: { width: 300, height: 100 },
  padding: { left: 10, right: 10, top: 10, bottom: 10 },
  gap: { width: 10, height: 0 },
});

const standardItem = new Style({ size: { width: 50, height: 40 } });

// This item overrides the parent's FlexStart alignment
const selfAlignedItem = new Style({
  size: { width: 50, height: 40 },
  alignSelf: AlignSelf.FlexEnd,
});

const child1 = tree.newLeaf(standardItem);
const child2 = tree.newLeaf(selfAlignedItem); // Will appear at the bottom
const child3 = tree.newLeaf(standardItem);

const root = tree.newWithChildren(rootStyle, [child1, child2, child3]);

tree.computeLayout(root, { width: 300, height: 100 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## ⏭️ Next Steps

- **[Align Items](./align-items.md)** - Set the default alignment for the container.
