---
title: Core Concepts
sidebar_position: 3
---

# Core Concepts

Taffy has a small API surface, but a consistent layout model. This section gives you the concepts to reason about any layout.

## The Three Core Objects

Layout in Taffy is driven by three primary objects:

1.  **[Style](./objects-style.md)**: Defines the layout rules for a node (e.g., "be a flex container", "have 10px padding").
2.  **[TaffyTree](./objects-taffy-tree.md)**: Manages the node hierarchy and serves as the entry point for calculations.
3.  **[Layout](./objects-layout.md)**: The output of the computation, containing final positions and sizes.

```
Style + Tree  -> computeLayout -> Layout
```

## Fundamental Principles

Beyond the core objects, there are several key principles to understand:

- **[Size, Space, and Units](./size-and-space.md)**: How Taffy handles fixed sizes, percentages, and content-based sizing.
- **[Measure Functions](./measure-functions.md)**: Integrating custom sizing logic for text or platform-specific widgets.
- **Tree Model**: Every node can have children, and parents control their placement along axes.

## Next Steps

- [The Style Object](./objects-style.md)
- [Size, Space, and Units](./size-and-space.md)
- [Layout Cookbook](../cookbook/)
