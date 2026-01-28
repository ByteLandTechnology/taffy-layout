---
title: Size, Space, and Units
sidebar_position: 4
---

# Size, Space, and Units

Taffy layout is driven by available space plus size constraints. Understanding this model keeps layouts predictable.

## Available Space

The second argument to `computeLayout` defines the available space:

```tsx live
// Fixed
const fixedTree = new TaffyTree();
const fixedStyle = new Style();
fixedStyle.size = { width: 120, height: 40 };
const fixedChild = fixedTree.newLeaf(fixedStyle);

const fixedRootStyle = new Style();
fixedRootStyle.display = Display.Flex;
fixedRootStyle.flexDirection = FlexDirection.Row;
fixedRootStyle.size = { width: 200, height: 80 };
fixedRootStyle.padding = { left: 10, right: 10, top: 10, bottom: 10 };

const fixedRoot = fixedTree.newWithChildren(fixedRootStyle, [fixedChild]);

fixedTree.computeLayout(fixedRoot, { width: 200, height: 80 });
console.log(fixedTree.printTree(fixedRoot));

// Flexible
const flexibleTree = new TaffyTree();
const flexibleStyle = new Style();
flexibleStyle.size = { width: "auto", height: 40 };
const flexibleChild = flexibleTree.newLeaf(flexibleStyle);

const flexibleRootStyle = new Style();
flexibleRootStyle.display = Display.Flex;
flexibleRootStyle.flexDirection = FlexDirection.Row;
flexibleRootStyle.size = { width: 200, height: 80 };
flexibleRootStyle.padding = { left: 10, right: 10, top: 10, bottom: 10 };

const flexibleRoot = flexibleTree.newWithChildren(flexibleRootStyle, [
  flexibleChild,
]);

flexibleTree.computeLayout(flexibleRoot, {
  width: "max-content",
  height: 80,
});
console.log(flexibleTree.printTree(flexibleRoot));

return (
  <div style={{ display: "flex", gap: 12, flexWrap: "wrap" }}>
    <div>
      <div style={{ marginBottom: 6, fontSize: 12 }}>Fixed space</div>
      <TaffyTreePreview tree={fixedTree} root={fixedRoot} />
    </div>
    <div>
      <div style={{ marginBottom: 6, fontSize: 12 }}>Max-content width</div>
      <TaffyTreePreview tree={flexibleTree} root={flexibleRoot} />
    </div>
  </div>
);
```

### Allowed Values

- `number`: absolute size (usually pixels)
- `"min-content"`: minimum content size
- `"max-content"`: maximum content size
- `"auto"`: let layout decide

## Box Model

Taffy behaves like `box-sizing: border-box`:

```text
┌─────────────────────────┐
│  Margin                 │
│  ┌───────────────────┐  │
│  │ Border            │  │
│  │  ┌─────────────┐  │  │
│  │  │ Padding     │  │  │
│  │  │  Content    │  │  │
│  │  └─────────────┘  │  │
│  └───────────────────┘  │
└─────────────────────────┘
```

- `size` includes padding + border
- margin is external spacing

## Percentages

Percent sizes are relative to the parent content box:

```tsx live
const tree = new TaffyTree();

const style = new Style();
style.size = { width: "50%", height: "100%" };
const child = tree.newLeaf(style);

const rootStyle = new Style();
rootStyle.size = { width: 260, height: 160 };
rootStyle.padding = { left: 16, right: 16, top: 16, bottom: 16 };

const root = tree.newWithChildren(rootStyle, [child]);

tree.computeLayout(root, {
  width: 260,
  height: 160,
});

console.log(tree.printTree(root));

return <TaffyTreePreview tree={tree} root={root} />;
```

## Common Pitfalls

- A root without size often results in 0 width/height for children
- `auto` inside Flex means content-sized
- `max-content` triggers measurement callbacks

## Next Steps

- [Measure Functions](./measure-functions.md)
- [Styling Guide](../styling/index.md)
