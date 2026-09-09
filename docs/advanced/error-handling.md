---
title: Error Handling
sidebar_position: 3
---

# Error Handling

**Handling exceptions and invalid states safely.**

Operations that report a recoverable Taffy error throw a `TaffyError` with a readable `message`, for example when a child index is out of bounds.

Node arguments must identify live nodes in the same `TaffyTree`. Keep track of IDs when removing nodes or clearing the tree, and do not fabricate IDs or copy them between trees. An invalid node ID can cause a WebAssembly panic; it is not guaranteed to produce a catchable `TaffyError`.

Validate child ranges before calling `removeChildrenRange(parent, start, end)`: both bounds must be integers and satisfy `0 <= start <= end <= tree.childCount(parent)`. The end is exclusive. An invalid range can cause a WebAssembly panic even when the parent ID is valid.

## Common Error Scenarios

| Error Type                  | Cause                                                    | Solution                                                                          |
| :-------------------------- | :------------------------------------------------------- | :-------------------------------------------------------------------------------- |
| **`ChildIndexOutOfBounds`** | Calling `getChildAtIndex` with an index >= `childCount`. | Check `childCount` before access.                                                 |
| **Invalid node ID**         | Using a removed, fabricated, or other-tree node ID.      | Track live IDs in your application; do not rely on an exception to validate them. |

## Best Practices

Wrap tree operations in `try-catch` blocks if you are dealing with dynamic or user-generated tree structures.

```ts
import { TaffyTree, Style, TaffyError } from "taffy-layout";

const tree = new TaffyTree();
const parentNode = tree.newLeaf(new Style());

try {
  // The parent is valid, but it has no child at index 0.
  tree.getChildAtIndex(parentNode, 0);
} catch (e) {
  if (e instanceof TaffyError) {
    console.error(`Taffy Layout Error: ${e.message}`);
    e.free();
  } else {
    throw e;
  }
}
```

## Measurement Errors

The binding catches exceptions thrown by a measure callback and uses a zero content measurement. It does the same for returns that cannot be decoded as numeric `{ width, height }`, including a Promise from an async callback. A `try-catch` around `computeLayoutWithMeasure()` therefore does not catch these callback failures.

Handle failures inside the callback. If the caller must receive the error, record it there, return a valid fallback size, and rethrow it after `computeLayoutWithMeasure()` returns. Free the callback's owned `Style` copy in a `finally` block when error paths are possible.

## Validation Pattern

Instead of relying on catch, validate indices:

```ts
const tree = new TaffyTree();
const parentNode = tree.newLeaf(new Style());
const index = 0;

const count = tree.childCount(parentNode);
if (Number.isInteger(index) && index >= 0 && index < count) {
  const child = tree.getChildAtIndex(parentNode, index);
  // ... safely use child
}
```
