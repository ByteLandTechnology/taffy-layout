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

- Parent: `flexDirection: column`
- Content: `flexGrow: 1`

## Code

```tsx live
const tree = new TaffyTree();

// Page container
const pageStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Column,
  size: { width: 300, height: 300 }, // Fixed height to simulate viewport
});

const header = tree.newLeaf(
  new Style({ size: { width: "100%", height: 50 }, margin: { bottom: 10 } }),
);
const footer = tree.newLeaf(
  new Style({ size: { width: "100%", height: 50 }, margin: { top: 10 } }),
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

## Related Guides

- **[Flex Grow](../styling/flex-basis-grow-shrink.md)**
- **[Size](../styling/size.md)**
