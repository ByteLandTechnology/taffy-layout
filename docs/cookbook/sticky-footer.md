---
title: Sticky Footer
sidebar_position: 4
---

# Sticky Footer

**When content is short, the footer sticks to the bottom; when content grows, the footer moves down naturally.**

```text
┌────────────────────────────┐
│ Header                     │
├────────────────────────────┤
│ Content (flex: 1)          │
├────────────────────────────┤
│ Footer                     │
└────────────────────────────┘
```

## Key Rules

- Parent: `flexDirection: FlexDirection.Column`, auto height, and a viewport-sized `minHeight`
- Content: `flexGrow: 1`
- Header and footer: `flexShrink: 0`

## Code

```tsx live
const tree = new TaffyTree();

// Page container
const pageStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Column,
  width: 300,
  minHeight: 300, // Fill the viewport, while allowing taller content
});

const header = tree.newLeaf(
  new Style({ width: "100%", height: 50, marginBottom: 10, flexShrink: 0 }),
);
const footer = tree.newLeaf(
  new Style({ width: "100%", height: 50, marginTop: 10, flexShrink: 0 }),
);

// Content grows to fill space
const content = tree.newLeaf(
  new Style({
    flexGrow: 1,
    size: { width: "100%", height: "auto" },
  }),
);

const root = tree.newWithChildren(pageStyle, [header, content, footer]);

tree.computeLayout(root, { width: 300, height: 300 });

return <TaffyTreePreview tree={tree} root={root} />;
```

Here the empty content area grows to 180 pixels and the footer starts at `y: 250`. If the content needs 500 pixels of height, the page grows to 620 pixels and the footer moves to `y: 570`. Supply children or a [measure function](../core-concepts/measure-functions.md) for real content.

## Related Guides

- **[Flex Grow](../styling/flex-grow.md)**
- **[Size](../styling/size.md)**
