---
title: Building Trees
sidebar_position: 3
---

# Building Trees

Layouts in Taffy are represented as a tree of nodes. Each node has a `Style`, and parent nodes control how their children are arranged.

## Key Operations

| Operation         | Method                                          | Description                                         |
| :---------------- | :---------------------------------------------- | :-------------------------------------------------- |
| **Create Leaf**   | `tree.newLeaf(style)`                           | Create a node without children (e.g., text, image). |
| **Create Parent** | `tree.newWithChildren(style, children[])`       | Create a node with initial children.                |
| **Add Child**     | `tree.addChild(parent, child)`                  | Append a child to a parent.                         |
| **Insert**        | `tree.insertChildAtIndex(parent, index, child)` | Insert a child at a specific position.              |
| **Remove**        | `tree.removeChild(parent, child)`               | Remove a specific child.                            |
| **Get Styles**    | `tree.getStyle(node)`                           | Retrieve the style object for a node.               |
| **Set Styles**    | `tree.setStyle(node, style)`                    | Update the style for a node.                        |

## Creating Nodes

### Leaf Nodes

Leaf nodes are the endpoints of your layout tree, typically representing content like text, images, or buttons.

```ts
const tree = new TaffyTree();

// Create styles first
const style = new Style({
  display: Display.Flex,
  size: { width: 100, height: 50 },
});

// Create the node
const leafNode = tree.newLeaf(style);
```

### Container Nodes

Container nodes group other nodes together. You can create them with existing children for convenience.

```ts
const tree = new TaffyTree();
const containerStyle = new Style({ display: Display.Flex });

const child1 = tree.newLeaf(new Style());
const child2 = tree.newLeaf(new Style());

// Create container with children attached immediately
const containerNode = tree.newWithChildren(containerStyle, [child1, child2]);
```

## Managing Children

Manipulate the tree structure dynamically using these methods.

```ts
const tree = new TaffyTree();
const parent = tree.newLeaf(new Style());
const child = tree.newLeaf(new Style());

// Add a child to the end of the list
tree.addChild(parent, child); // Index: 0

// Insert a new child at the beginning
const firstChild = tree.newLeaf(new Style());
tree.insertChildAtIndex(parent, 0, firstChild); // Index: 0, previous child becomes Index 1

// Replace a child
const newChild = tree.newLeaf(new Style());
tree.replaceChildAtIndex(parent, 1, newChild);

// Remove a child
tree.removeChild(parent, firstChild);
```

## Updating Styles

Styles can be updated at any time. This will mark the node as "dirty" and require a new layout computation.

```ts
const tree = new TaffyTree();
const node = tree.newLeaf(new Style({ flexGrow: 1 }));

// Update style later
const newStyle = new Style({ flexGrow: 2, width: 100 });
tree.setStyle(node, newStyle);
```

> **Performance Tip**: Try to reuse `Style` objects for static content initialization, but remember that `tree.setStyle` copies the style data into the internal engine, so modifying the JS `Style` object _after_ passing it to the tree won't affect the tree until you call `setStyle` again.

## Next Steps

- 📐 **[Computing Layouts](./computing-layouts.md)** - Calculate positions and sizes.
- ⚙️ **[Configuration](./configuration.md)** - Global settings.
