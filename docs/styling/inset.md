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

- **For `Position.Absolute`**: Offsets use the containing block established by the nearest ancestor that is not `Static`, or the initial containing block if there is no such ancestor. Grid placement can define a containing area within a Grid containing block.
- **For `Position.Fixed`**: Offsets use the initial containing block. Its definite dimensions come from `availableSpace`, not the root's size; see [Position](./position.md) for indefinite dimensions and returned coordinates.
- **For `Position.Relative`**: Offsets move the item relative to its _normal position_ in the flow.
- **For `Position.Static` and `Position.Sticky`**: Offsets are ignored.

The edge names remain physical in both LTR and RTL. The `inset` object requires all four fields; use `"auto"` for unspecified edges, or set individual properties such as `right`.

## Example

```tsx live
const tree = new TaffyTree();

const containerStyle = new Style({
  size: { width: 200, height: 100 },
  display: Display.Flex,
  position: Position.Relative, // containing block of the absolute item
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
