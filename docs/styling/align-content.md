---
title: Align Content
sidebar_position: 16
---

# Align Content

**Control alignment of lines in multi-line flex containers.**

The `alignContent` property aligns a flex container's lines within the flex container when there is extra space on the cross-axis. **This property has no effect on single-line flex containers** (i.e. where `flexWrap` is `NoWrap`).

## Values

| Value              | Description                                                                         |
| :----------------- | :---------------------------------------------------------------------------------- |
| **`Stretch`**      | **Default**. Lines stretch to take up the remaining space.                          |
| **`FlexStart`**    | Lines packed to the start of the container.                                         |
| **`FlexEnd`**      | Lines packed to the end of the container.                                           |
| **`Center`**       | Lines packed to the center of the container.                                        |
| **`SpaceBetween`** | Lines evenly distributed; the first line is at the start, the last line at the end. |
| **`SpaceAround`**  | Lines evenly distributed with equal space around them.                              |

## Example

```tsx live
const tree = new TaffyTree();

const itemStyle = new Style({
  size: { width: 80, height: 30 },
  margin: { bottom: 5 },
});

// Create enough children to force wrapping
const children = Array.from({ length: 5 }).map(() => tree.newLeaf(itemStyle));

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  flexWrap: FlexWrap.Wrap, // Required for alignContent to work
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
