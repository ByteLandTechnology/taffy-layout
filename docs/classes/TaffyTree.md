[**Taffy-JS API Documentation**](../index.md)

---

[Taffy-JS API Documentation](../index.md) / TaffyTree

# Class: TaffyTree

The main layout tree class for creating nodes, computing layouts, and managing a tree of styled elements.

TaffyTree is the entry point for the Taffy layout engine. It manages
a tree of nodes and computes their layouts using CSS Flexbox and Grid algorithms.

## Constructors

### Constructor

```ts
new TaffyTree(): TaffyTree;
```

Creates a new empty TaffyTree

The tree starts with no nodes. Use `newLeaf()` or `newWithChildren()`
to add nodes.

#### Returns

`TaffyTree`

#### Example

```typescript
const tree: TaffyTree = new TaffyTree();
```

## Methods

### \[dispose\]()

```ts
dispose: void;
```

#### Returns

`void`

---

### addChild()

```ts
addChild(parent, child): void;
```

Appends a child node to a parent

The child is added as the last child of the parent.

#### Parameters

| Parameter | Type     | Description              |
| --------- | -------- | ------------------------ |
| `parent`  | `bigint` | The parent node ID       |
| `child`   | `bigint` | The child node ID to add |

#### Returns

`void`

#### Throws

`TaffyError` if the parent or child node does not exist

#### Example

```typescript
tree.addChild(parentId, childId);
```

---

### childCount()

```ts
childCount(parent): number;
```

Gets the number of children of a node

#### Parameters

| Parameter | Type     | Description        |
| --------- | -------- | ------------------ |
| `parent`  | `bigint` | The parent node ID |

#### Returns

`number`

- The number of direct children

#### Throws

`TaffyError` if the node does not exist

#### Example

```typescript
const count: number = tree.childCount(parentId);
```

---

### children()

```ts
children(parent): BigUint64Array;
```

Gets all children of a node

#### Parameters

| Parameter | Type     | Description        |
| --------- | -------- | ------------------ |
| `parent`  | `bigint` | The parent node ID |

#### Returns

`BigUint64Array`

- Array of child node IDs (`BigUint64Array`)

#### Throws

`TaffyError` if the parent node does not exist

#### Example

```typescript
const children: BigUint64Array = tree.children(parentId);
```

---

### clear()

```ts
clear(): void;
```

Removes all nodes from the tree

This clears the entire tree, removing all nodes and their relationships.
Use this to reset the tree for reuse.

#### Returns

`void`

#### Example

```typescript
tree.clear();
console.log(tree.totalNodeCount());
```

---

### computeLayout()

```ts
computeLayout(node, available_space): void;
```

Computes the layout for a subtree

This is the main layout computation method. Call this on the root node
to compute layouts for all nodes in the tree.

#### Parameters

| Parameter         | Type                                                                                     | Description                            |
| ----------------- | ---------------------------------------------------------------------------------------- | -------------------------------------- |
| `node`            | `bigint`                                                                                 | The root node ID to compute layout for |
| `available_space` | [`Size`](../interfaces/Size.md)\<[`AvailableSpace`](../type-aliases/AvailableSpace.md)\> | The available space constraints        |

#### Returns

`void`

#### Examples

```typescript
// Fixed size container
{ width: { Definite: 800 }, height: { Definite: 600 } }

// Flexible width, fixed height
{ width: "MaxContent", height: { Definite: 600 } }

// Minimum content size
{ width: "MinContent", height: "MinContent" }
```

```typescript
tree.computeLayout(rootId, {
  width: { Definite: 800 },
  height: { Definite: 600 },
});
```

#### Throws

`TaffyError` if the node does not exist or available space is invalid

---

### computeLayoutWithMeasure()

```ts
computeLayoutWithMeasure(
   node,
   available_space,
   measure_func): void;
```

Computes layout with a custom measure function for leaf nodes

Use this when you have leaf nodes with dynamic content (like text)
that needs to be measured during layout. The measure function is
called for each leaf node that needs measurement.

#### Parameters

| Parameter         | Type                                                                                     | Description                                |
| ----------------- | ---------------------------------------------------------------------------------------- | ------------------------------------------ |
| `node`            | `bigint`                                                                                 | The root node ID to compute layout for     |
| `available_space` | [`Size`](../interfaces/Size.md)\<[`AvailableSpace`](../type-aliases/AvailableSpace.md)\> | The available space constraints            |
| `measure_func`    | [`MeasureFunction`](../type-aliases/MeasureFunction.md)                                  | A function that measures leaf node content |

#### Returns

`void`

#### Throws

`TaffyError` if the node does not exist or available space is invalid

#### Example

```typescript
tree.computeLayoutWithMeasure(
  rootId,
  { width: { Definite: 800 }, height: "MaxContent" },
  (known, available, node, context, style) => {
    if (context?.text) {
      const measured = measureText(context.text, available.width);
      return { width: measured.width, height: measured.height };
    }
    return { width: 0, height: 0 };
  },
);
```

---

### dirty()

```ts
dirty(node): boolean;
```

Checks if a node is dirty (needs re-layout)

A node is dirty if its style or content has changed since the last
layout computation.

#### Parameters

| Parameter | Type     | Description          |
| --------- | -------- | -------------------- |
| `node`    | `bigint` | The node ID to check |

#### Returns

`boolean`

- true if dirty, false otherwise

#### Throws

`TaffyError` if the node does not exist

#### Example

```typescript
if (tree.dirty(nodeId)) {
  tree.computeLayout(rootId, availableSpace);
}
```

---

### disableRounding()

```ts
disableRounding(): void;
```

Disables rounding of layout values

When disabled, computed layout values retain their fractional precision.
Use this when you need sub-pixel accuracy or when performing custom
rounding.

#### Returns

`void`

#### Example

```typescript
tree.disableRounding();
const layout = tree.getLayout(node);
console.log(layout.x);
```

---

### enableRounding()

```ts
enableRounding(): void;
```

Enables rounding of layout values to whole pixels

When enabled (default), computed layout values like position and size
are rounded to the nearest integer. This prevents sub-pixel rendering
issues in most rendering contexts.

#### Returns

`void`

#### Example

```typescript
tree.enableRounding();
```

---

### free()

```ts
free(): void;
```

#### Returns

`void`

---

### getChildAtIndex()

```ts
getChildAtIndex(parent, index): bigint;
```

Gets the child at a specific index

#### Parameters

| Parameter | Type     | Description                      |
| --------- | -------- | -------------------------------- |
| `parent`  | `bigint` | The parent node ID               |
| `index`   | `number` | The index of the child (0-based) |

#### Returns

`bigint`

- The child node ID (`bigint`)

#### Throws

`TaffyError` if the parent node does not exist or index is out of bounds

#### Example

```typescript
const firstChild: bigint = tree.getChildAtIndex(parentId, 0);
```

---

### getDisjointNodeContextMut()

```ts
getDisjointNodeContextMut(children): any[];
```

Gets context values for multiple nodes at once

This is more efficient than calling `getNodeContext()` multiple times
when you need to access contexts for many nodes.

#### Parameters

| Parameter  | Type             | Description       |
| ---------- | ---------------- | ----------------- |
| `children` | `BigUint64Array` | Array of node IDs |

#### Returns

`any`[]

- Array of context values (undefined for nodes without context)

#### Example

```typescript
const nodes = BigUint64Array.from([id1, id2]);
const contexts = tree.getDisjointNodeContextMut(nodes);
```

---

### getLayout()

```ts
getLayout(node): Layout;
```

Gets the computed layout for a node

Call this after `computeLayout()` to retrieve the computed position
and size for a node.

#### Parameters

| Parameter | Type     | Description |
| --------- | -------- | ----------- |
| `node`    | `bigint` | The node ID |

#### Returns

[`Layout`](Layout.md)

- The computed `Layout`

#### Throws

`TaffyError` if the node does not exist

#### Example

```typescript
tree.computeLayout(rootId, {
  width: { Definite: 800 },
  height: { Definite: 600 },
});
const layout: Layout = tree.getLayout(nodeId);
console.log(
  `Position: (${layout.x}, ${layout.y}), Size: ${layout.width}x${layout.height}`,
);
```

---

### getNodeContext()

```ts
getNodeContext(node): any;
```

Gets the context value for a node

#### Parameters

| Parameter | Type     | Description |
| --------- | -------- | ----------- |
| `node`    | `bigint` | The node ID |

#### Returns

`any`

- The attached context value, or `undefined` if none is set

#### Example

```typescript
interface Context {
  text: string;
}
const context = tree.getNodeContext(nodeId) as Context | undefined;
if (context) {
  console.log(context.text);
}
```

---

### getNodeContextMut()

```ts
getNodeContextMut(node): any;
```

Gets a mutable reference to the context value for a node

In JavaScript, this behaves the same as `getNodeContext()` since
JavaScript objects are always passed by reference.

#### Parameters

| Parameter | Type     | Description |
| --------- | -------- | ----------- |
| `node`    | `bigint` | The node ID |

#### Returns

`any`

- The attached context value, or `undefined` if none is set

---

### getStyle()

```ts
getStyle(node): Style;
```

Gets the style for a node

#### Parameters

| Parameter | Type     | Description |
| --------- | -------- | ----------- |
| `node`    | `bigint` | The node ID |

#### Returns

[`Style`](Style.md)

- The node's `Style`

#### Throws

`TaffyError` if the node does not exist

#### Example

```typescript
const style: Style = tree.getStyle(nodeId);
console.log("Flex grow:", style.flexGrow);
```

---

### insertChildAtIndex()

```ts
insertChildAtIndex(
   parent,
   index,
   child): void;
```

Inserts a child at a specific index

#### Parameters

| Parameter | Type     | Description                         |
| --------- | -------- | ----------------------------------- |
| `parent`  | `bigint` | The parent node ID                  |
| `index`   | `number` | The position to insert at (0-based) |
| `child`   | `bigint` | The child node ID to insert         |

#### Returns

`void`

#### Throws

`TaffyError` if the parent or child node does not exist, or index is out of bounds

#### Example

```typescript
tree.insertChildAtIndex(parentId, 0, childId);
```

---

### markDirty()

```ts
markDirty(node): void;
```

Marks a node as dirty (requiring re-layout)

Use this when a node's content has changed but its style hasn't.
For example, when text content changes and needs remeasuring.

#### Parameters

| Parameter | Type     | Description               |
| --------- | -------- | ------------------------- |
| `node`    | `bigint` | The node ID to mark dirty |

#### Returns

`void`

#### Throws

`TaffyError` if the node does not exist

#### Example

```typescript
// After updating text content
tree.setNodeContext(nodeId, { text: "Updated text" });
tree.markDirty(nodeId);
tree.computeLayout(rootId, availableSpace);
```

---

### newLeaf()

```ts
newLeaf(style): bigint;
```

Creates a new leaf node with the given style

A leaf node has no children. Use this for elements that contain
content (like text) rather than other elements.

#### Parameters

| Parameter | Type                | Description                          |
| --------- | ------------------- | ------------------------------------ |
| `style`   | [`Style`](Style.md) | The style configuration for the node |

#### Returns

`bigint`

- The node ID (`bigint`)

#### Throws

`TaffyError` if the node cannot be created

#### Example

```typescript
const style = new Style();
style.size = { width: { Length: 100 }, height: { Length: 50 } };
const nodeId: bigint = tree.newLeaf(style);
```

---

### newLeafWithContext()

```ts
newLeafWithContext(style, context): bigint;
```

Creates a new leaf node with an attached context value

The context can be any JavaScript value and is passed to the measure
function during layout computation. This is useful for storing
references to text content or other dynamic data.

#### Parameters

| Parameter | Type                | Description                                |
| --------- | ------------------- | ------------------------------------------ |
| `style`   | [`Style`](Style.md) | The style configuration for the node       |
| `context` | `any`               | Any JavaScript value to attach to the node |

#### Returns

`bigint`

- The node ID (`bigint`)

#### Throws

`TaffyError` if the node cannot be created

#### Example

```typescript
interface TextContext {
  text: string;
  isBold: boolean;
}

const style = new Style();
const context: TextContext = { text: "Hello, World!", isBold: true };
const nodeId: bigint = tree.newLeafWithContext(style, context);
```

---

### newWithChildren()

```ts
newWithChildren(style, children): bigint;
```

Creates a new node with the given children

Use this to create container nodes that have child elements.
The children must already exist in the tree.

#### Parameters

| Parameter  | Type                | Description                                 |
| ---------- | ------------------- | ------------------------------------------- |
| `style`    | [`Style`](Style.md) | The style configuration for the node        |
| `children` | `BigUint64Array`    | Array of child node IDs (as BigUint64Array) |

#### Returns

`bigint`

- The node ID (`bigint`)

#### Throws

`TaffyError` if the node cannot be created

#### Example

```typescript
const containerStyle = new Style();
containerStyle.display = Display.Flex;

const child1: bigint = tree.newLeaf(new Style());
const child2: bigint = tree.newLeaf(new Style());

const container: bigint = tree.newWithChildren(
  containerStyle,
  BigUint64Array.from([child1, child2]),
);
```

---

### parent()

```ts
parent(child): bigint | undefined;
```

Gets the parent of a node

#### Parameters

| Parameter | Type     | Description       |
| --------- | -------- | ----------------- |
| `child`   | `bigint` | The child node ID |

#### Returns

`bigint` \| `undefined`

- The parent node ID, or `undefined` if the node has no parent

#### Example

```typescript
const parentId: bigint | undefined = tree.parent(childId);
```

---

### printTree()

```ts
printTree(node): void;
```

Prints the tree structure to the console (for debugging)

Outputs a text representation of the tree structure starting from
the given node. Useful for debugging layout issues.

#### Parameters

| Parameter | Type     | Description                    |
| --------- | -------- | ------------------------------ |
| `node`    | `bigint` | The root node ID to print from |

#### Returns

`void`

#### Example

```typescript
tree.printTree(rootId);
// Output appears in browser console
```

---

### remove()

```ts
remove(node): bigint;
```

Removes a node from the tree

The node and all its descendants are removed. If the node has a parent,
it is automatically removed from the parent's children.

#### Parameters

| Parameter | Type     | Description           |
| --------- | -------- | --------------------- |
| `node`    | `bigint` | The node ID to remove |

#### Returns

`bigint`

- The removed node ID (`bigint`)

#### Throws

`TaffyError` if the node does not exist

#### Example

```typescript
try {
  const removedId: bigint = tree.remove(nodeId);
} catch (e) {
  console.error("Node doesn't exist");
}
```

---

### removeChild()

```ts
removeChild(parent, child): bigint;
```

Removes a specific child from a parent

#### Parameters

| Parameter | Type     | Description                 |
| --------- | -------- | --------------------------- |
| `parent`  | `bigint` | The parent node ID          |
| `child`   | `bigint` | The child node ID to remove |

#### Returns

`bigint`

- The removed child ID (`bigint`)

#### Throws

`TaffyError` if the parent or child node does not exist

#### Example

```typescript
tree.removeChild(parentId, childId);
```

---

### removeChildAtIndex()

```ts
removeChildAtIndex(parent, index): bigint;
```

Removes a child at a specific index

#### Parameters

| Parameter | Type     | Description                                |
| --------- | -------- | ------------------------------------------ |
| `parent`  | `bigint` | The parent node ID                         |
| `index`   | `number` | The index of the child to remove (0-based) |

#### Returns

`bigint`

- The removed child ID (`bigint`)

#### Throws

`TaffyError` if the parent node does not exist or index is out of bounds

#### Example

```typescript
const removedId: bigint = tree.removeChildAtIndex(parentId, 0);
```

---

### removeChildrenRange()

```ts
removeChildrenRange(
   parent,
   start_index,
   end_index): void;
```

Removes a range of children

Removes children from `start_index` (inclusive) to `end_index` (exclusive).

#### Parameters

| Parameter     | Type     | Description                |
| ------------- | -------- | -------------------------- |
| `parent`      | `bigint` | The parent node ID         |
| `start_index` | `number` | Start of range (inclusive) |
| `end_index`   | `number` | End of range (exclusive)   |

#### Returns

`void`

#### Throws

`TaffyError` if the parent node does not exist or range is invalid

#### Example

```typescript
tree.removeChildrenRange(parentId, 1, 3);
```

---

### replaceChildAtIndex()

```ts
replaceChildAtIndex(
   parent,
   index,
   new_child): bigint;
```

Replaces a child at a specific index

#### Parameters

| Parameter   | Type     | Description                                 |
| ----------- | -------- | ------------------------------------------- |
| `parent`    | `bigint` | The parent node ID                          |
| `index`     | `number` | The index of the child to replace (0-based) |
| `new_child` | `bigint` | The new child node ID                       |

#### Returns

`bigint`

- The replaced (old) child ID (`bigint`)

#### Throws

`TaffyError` if the parent node does not exist or index is out of bounds

#### Example

```typescript
const oldChildId: bigint = tree.replaceChildAtIndex(parentId, 1, newChildId);
```

---

### setChildren()

```ts
setChildren(parent, children): void;
```

Replaces all children of a node

Any existing children are removed and replaced with the new array.

#### Parameters

| Parameter  | Type             | Description                 |
| ---------- | ---------------- | --------------------------- |
| `parent`   | `bigint`         | The parent node ID          |
| `children` | `BigUint64Array` | Array of new child node IDs |

#### Returns

`void`

#### Throws

`TaffyError` if the parent node does not exist

#### Example

```typescript
const children = BigUint64Array.from([child1, child2, child3]);
tree.setChildren(parentId, children);
```

---

### setNodeContext()

```ts
setNodeContext(node, context): void;
```

Sets a context value for a node

The context can be any JavaScript value and is passed to the measure
function during layout computation.

#### Parameters

| Parameter | Type     | Description                    |
| --------- | -------- | ------------------------------ |
| `node`    | `bigint` | The node ID                    |
| `context` | `any`    | Any JavaScript value to attach |

#### Returns

`void`

#### Throws

`TaffyError` if the node does not exist

#### Example

```typescript
interface Context {
  text: string;
}
tree.setNodeContext(nodeId, { text: "Updated text" } as Context);
```

---

### setStyle()

```ts
setStyle(node, style): void;
```

Sets the style for an existing node

This replaces the node's current style with the provided one.
The node will be marked as dirty and require re-layout.

#### Parameters

| Parameter | Type                | Description                 |
| --------- | ------------------- | --------------------------- |
| `node`    | `bigint`            | The node ID                 |
| `style`   | [`Style`](Style.md) | The new style configuration |

#### Returns

`void`

#### Throws

`TaffyError` if the node does not exist

#### Example

```typescript
const newStyle = new Style();
newStyle.flexGrow = 2;
tree.setStyle(nodeId, newStyle);
```

---

### totalNodeCount()

```ts
totalNodeCount(): number;
```

Gets the total number of nodes in the tree

#### Returns

`number`

- The total count of all nodes

#### Example

```typescript
const count: number = tree.totalNodeCount();
```

---

### unroundedLayout()

```ts
unroundedLayout(node): Layout;
```

Gets the unrounded (fractional) layout for a node

Returns the raw computed values before any rounding is applied.
Useful when you need sub-pixel precision.

#### Parameters

| Parameter | Type     | Description |
| --------- | -------- | ----------- |
| `node`    | `bigint` | The node ID |

#### Returns

[`Layout`](Layout.md)

- The unrounded `Layout`

#### Example

```typescript
const layout: Layout = tree.unroundedLayout(nodeId);
console.log(`Exact width: ${layout.width}`);
```

---

### withCapacity()

```ts
static withCapacity(capacity): TaffyTree;
```

Creates a new TaffyTree with pre-allocated capacity

Use this when you know approximately how many nodes will be in the tree.
This can improve performance by reducing memory reallocations.

#### Parameters

| Parameter  | Type     | Description                                   |
| ---------- | -------- | --------------------------------------------- |
| `capacity` | `number` | The number of nodes to pre-allocate space for |

#### Returns

`TaffyTree`

#### Example

```typescript
const tree: TaffyTree = TaffyTree.withCapacity(1000);
```
