---
title: Gap
sidebar_position: 7
---

# Gap

**Define gutters between rows and columns.**

The `gap` property defines the size of the gap (gutter) between rows and between columns in Flexbox and Grid layouts. It is a shorthand for `row-gap` and `column-gap` in CSS, represented as a `Size<LengthPercentage>` in Taffy.

## Values

`gap` takes a Size object with `width` (column gap) and `height` (row gap), usually in pixels or percent.

| Property     | Description                                |
| :----------- | :----------------------------------------- |
| **`width`**  | Space between items in a row (Column Gap). |
| **`height`** | Space between lines/rows (Row Gap).        |

## Example

```tsx live
const tree = new TaffyTree();

const itemStyle = new Style({ size: { width: 60, height: 40 } });

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  flexWrap: FlexWrap.Wrap,
  size: { width: 200, height: 100 },

  // Gap adds space strictly BETWEEN items, not on the outside edges
  gap: { width: 10, height: 10 },
});

const children = Array.from({ length: 6 }).map(() => tree.newLeaf(itemStyle));

const root = tree.newWithChildren(rootStyle, children);

tree.computeLayout(root, { width: 200, height: 100 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## Next Steps

- [Flex Direction](./flex-direction.md)
- [Margin](./margin.md)
