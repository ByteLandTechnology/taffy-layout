# 🚨 Error Handling

**Handling exceptions and invalid states safely.**

Taffy operations generally do not throw, but invalid API usage (like accessing non-existent nodes) can raise `TaffyError`.

## ⚠️ Common Error Scenarios

| Error Type                  | Cause                                                     | Solution                                         |
| :-------------------------- | :-------------------------------------------------------- | :----------------------------------------------- |
| **`InvalidInputNode`**      | Accessing a node ID that has been freed or never existed. | Ensure the node ID matches a valid, active node. |
| **`ChildIndexOutOfBounds`** | Calling `getChildAtIndex` with an index >= `childCount`.  | Check `childCount` before access.                |
| **`InvalidParentNode`**     | Removing a child that isn't attached to the parent.       | Track your tree structure carefully.             |

## 🛡️ Best Practices

Wrap tree operations in `try-catch` blocks if you are dealing with dynamic or user-generated tree structures.

```ts
import { TaffyTree, Style, TaffyError } from "taffy-layout";

const tree = new TaffyTree();
const someNodeId = tree.newLeaf(new Style());

try {
  // Example: Attempting to access a potentially invalid node
  const layout = tree.getLayout(someNodeId);
} catch (e) {
  if (e instanceof TaffyError) {
    console.error(`Taffy Layout Error: ${e.message}`);
  } else {
    throw e;
  }
}
```

## 🔍 Validation Pattern

Instead of relying on catch, validate indices:

```ts
const tree = new TaffyTree();
const parentNode = tree.newLeaf(new Style());
const index = 0;

const count = tree.childCount(parentNode);
if (index < count) {
  const child = tree.getChildAtIndex(parentNode, index);
  // ... safely use child
}
```
