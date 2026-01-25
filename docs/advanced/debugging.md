# 🐞 Debugging

**Tools to inspect and troubleshoot layouts.**

## 🖨️ Print Tree

The most powerful tool at your disposal is `tree.printTree(node)`. It generates a string representation of the tree structure, style configuration, and computed layout.

```ts
const tree = new TaffyTree();
const root = tree.newLeaf(new Style());
tree.computeLayout(root, { width: 100, height: 100 });

console.log(tree.printTree(root));
```

**Example Output:**

```text
DIV [x: 0    y: 0    w: 100  h: 100  content_w: 100  content_h: 100  border: l:0 r:0 t:0 b:0, padding: l:0 r:0 t:0 b:0] (1)
└── LEAF [x: 0    y: 0    w: 50   h: 50   content_w: 50   content_h: 50   border: l:0 r:0 t:0 b:0, padding: l:0 r:0 t:0 b:0] (2)
```

> **Note**: The actual output format may vary slightly by version but will always show the hierarchy and key constraints.

## 📏 Visual Debugging

If you are rendering to a canvas or screen:

1.  **Draw Borders**: Draw a colored 1px border around every computed layout rect.
2.  **Color Code**: Use different colors for different `display` types (e.g., Blue for Flex, Red for Grid).

```ts
// Mock renderer
const renderer = {
  strokeRect: (x: number, y: number, w: number, h: number, c: string) => {},
};
const tree = new TaffyTree();
const root = tree.newLeaf(new Style());
tree.computeLayout(root, { width: 100, height: 100 });

// Visual debugger function
function debugDraw(node: any) {
  const layout = tree.getLayout(node);
  renderer.strokeRect(layout.x, layout.y, layout.width, layout.height, "red");

  for (const child of tree.children(node)) {
    debugDraw(child);
  }
}
debugDraw(root);
```

## 🔍 Isolation

If a specific sub-tree is misbehaving:

1.  Create a fresh `TaffyTree`.
2.  Replicate _only_ that sub-tree structure.
3.  Hard-code the input constraints (width/height provided to the sub-tree).
4.  Run `computeLayout` and inspect.

This isolates the problem from external parent constraints.
