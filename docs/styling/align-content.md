---
title: Align Content
sidebar_position: 16
---

# Align Content

**Control alignment of lines in multi-line flex containers.**

The `alignContent` property aligns a flex container's lines when there is extra space on the cross axis. It has no effect on a single-line flex container (`flexWrap: FlexWrap.NoWrap`). It also aligns Grid rows and content within Block and FlowRoot containers along the block axis.

The property is initially `undefined`, which leaves alignment to the layout algorithm. In Flexbox, the default behavior stretches wrapping lines.

## Values

| Value               | Description                                                                         |
| :------------------ | :---------------------------------------------------------------------------------- |
| **`Stretch`**       | Lines stretch to take up the remaining space.                                       |
| **`Start` / `End`** | Content aligns to the logical start/end edge.                                       |
| **`FlexStart`**     | Lines packed to the start of the container.                                         |
| **`FlexEnd`**       | Lines packed to the end of the container.                                           |
| **`Center`**        | Lines packed to the center of the container.                                        |
| **`SpaceBetween`**  | Lines evenly distributed; the first line is at the start, the last line at the end. |
| **`SpaceAround`**   | Lines evenly distributed with equal space around them.                              |
| **`SpaceEvenly`**   | Equal spacing between lines and at both edges.                                      |

`SafeStart`, `SafeEnd`, `SafeFlexStart`, `SafeFlexEnd`, and `SafeCenter` fall back to start alignment when content would overflow before the start edge. Block content is aligned as one group; the distribution values do not space individual block children apart.

## Example

```tsx live
const tree = new TaffyTree();

const itemStyle = new Style({
  size: { width: 80, height: 30 },
  marginBottom: 5,
});

// Create enough children to force wrapping
const children = Array.from({ length: 5 }).map(() => tree.newLeaf(itemStyle));

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  flexWrap: FlexWrap.Wrap, // Enable line alignment in this Flexbox example
  size: { width: 200, height: 200 }, // Must have extra vertical space

  // CHANGE THIS TO TEST DIFFERENT VALUES
  alignContent: AlignContent.Center,
});

const root = tree.newWithChildren(rootStyle, children);

tree.computeLayout(root, { width: 200, height: 200 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## Next Steps

- [CSS Grid Layout](./grid.md)
- [Align Self](./align-self.md)
