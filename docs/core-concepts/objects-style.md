---
title: The Style Object
sidebar_position: 1
---

# The Style Object

The **`Style`** object defines the layout rules for a single node. It contains properties that dictate how the node should be sized, positioned, and how it should arrange its children.

## Key Responsibilities

- **Layout Mode and Positioning**: `display` chooses Flexbox, Grid, Block, or FlowRoot; `position` independently selects `Static` (default), `Relative`, `Absolute`, `Fixed`, or `Sticky`. See [Position](../styling/position.md) for containing blocks and sticky offsets.
- **Dimensions**: Defines width, height, aspect ratio, and min/max constraints.
- **Spacing**: Controls margins, paddings, borders, and gaps.
- **Alignment**: Specifies how children are aligned along the main and cross axes.

## Usage

Styles are typically created and passed to a node during creation. A new style uses `Display.Flex`, `Position.Static`, `Direction.Ltr`, `Float.None`, and `Clear.None`. See [Display](../styling/display.md) for block layout, floats, and writing direction.

`alignItems`, `justifyItems`, `alignContent`, and `justifyContent` expose Taffy's `normal` alignment as `undefined`; assigning `undefined` restores it. The resulting alignment depends on the layout mode. `alignSelf` and `justifySelf` instead reset to `auto`: their direct getters return `AlignSelf.Auto`, while `get()` returns `undefined` when unset or assigned `Auto`.

```typescript
const style = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  size: { width: 100, height: 100 },
  padding: { left: 10, right: 10, top: 10, bottom: 10 },
});
```

Node creation and `tree.setStyle()` copy the style. Mutating the original style, or a copy returned by `tree.getStyle()`, does not update the node until you pass it to `setStyle()`. `Style` objects own WASM memory; call `.free()` when finished with each copy. See [Memory Management](../getting-started/configuration.md#memory-management).

## Next Steps

- [The TaffyTree Object](./objects-taffy-tree.md)
- [The Layout Object](./objects-layout.md)
