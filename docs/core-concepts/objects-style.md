---
title: The Style Object
sidebar_position: 1
---

# The Style Object

The **`Style`** object defines the layout rules for a single node. It contains properties that dictate how the node should be sized, positioned, and how it should arrange its children.

## Key Responsibilities

- **Layout Mode**: Determines if the node uses Flexbox, Grid, or absolute positioning.
- **Dimensions**: Defines width, height, aspect ratio, and min/max constraints.
- **Spacing**: Controls margins, paddings, borders, and gaps.
- **Alignment**: Specifies how children are aligned along the main and cross axes.

## Usage

Styles are typically created and passed to a node during creation.

```typescript
const style = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  size: { width: 100, height: 100 },
  padding: { left: 10, right: 10, top: 10, bottom: 10 },
});
```

## Next Steps

- [The TaffyTree Object](./objects-taffy-tree.md)
- [The Layout Object](./objects-layout.md)
