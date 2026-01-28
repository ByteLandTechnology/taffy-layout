---
title: Inset
sidebar_position: 23
---

# Inset

**Position an element relative to its edges.**

The `inset` property (historically `top`, `right`, `bottom`, `left`) defines the offsets for positioned elements. Its behavior depends on the `position` property.

## Properties

`inset` is a helper property in Taffy (and CSS shorthand) that sets:

| Property     | Description                    |
| :----------- | :----------------------------- |
| **`top`**    | Distance from the top edge.    |
| **`bottom`** | Distance from the bottom edge. |
| **`left`**   | Distance from the left edge.   |
| **`right`**  | Distance from the right edge.  |

## Behavior

- **For `Position.Absolute`**: Offsets are relative to the _nearest positioned ancestor_.
- **For `Position.Relative`**: Offsets move the item relative to its _normal position_ in the flow.

## Example

```tsx live
const tree = new TaffyTree();

const containerStyle = new Style({
  size: { width: 200, height: 100 },
  display: Display.Flex,
});

// Absolute item anchored to bottom-right
const absoluteItem = tree.newLeaf(
  new Style({
    position: Position.Absolute,
    size: { width: 50, height: 50 },

    // Anchor to bottom-right
    inset: { bottom: 10, right: 10, top: "auto", left: "auto" },
  }),
);

const root = tree.newWithChildren(containerStyle, [absoluteItem]);

tree.computeLayout(root, { width: 200, height: 100 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## Next Steps

- [Overflow](./overflow.md)
- [Position](./position.md)
