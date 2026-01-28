---
title: Flex Row Cards
sidebar_position: 1
---

# Flex Row Cards

**A row of evenly sized cards—great for stats, toolbars, and panels.**

```text
┌────────────────────────────────────┐
│ [Card]  [Card]  [Card]  [Card]     │
└────────────────────────────────────┘
```

## Key Ideas

- `display: flex` + `flexDirection: row`
- `flexGrow` splits remaining space evenly
- `gap` controls spacing

## Code

```tsx live
const tree = new TaffyTree();

const style = new Style({
  flexGrow: 1,
  size: { height: "100%", width: "auto" },
});

const children = Array.from({ length: 4 }).map(() => tree.newLeaf(style));

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  gap: { width: 12, height: 0 },
  size: { width: 500, height: 120 },
  padding: { left: 12, right: 12, top: 12, bottom: 12 },
});

const root = tree.newWithChildren(rootStyle, children);

tree.computeLayout(root, { width: 500, height: 120 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## Related Guides

- **[Flex Grow/Shrink](../styling/flex-basis-grow-shrink.md)**
- **[Spacing](../styling/margin-padding-border.md)**
