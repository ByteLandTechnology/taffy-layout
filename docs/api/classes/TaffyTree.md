[**Taffy Layout API Documentation**](../index.md)

---

[Taffy Layout API Documentation](../index.md) / TaffyTree

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
const tree = new TaffyTree();
const parentId = tree.newLeaf(new Style());
const childId = tree.newLeaf(new Style());
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
const tree = new TaffyTree();
const parentId = tree.newLeaf(new Style());
const count: number = tree.childCount(parentId);
```

---

### children()

```ts
children(parent): bigint[];
```

Gets all children of a node

#### Parameters

| Parameter | Type     | Description        |
| --------- | -------- | ------------------ |
| `parent`  | `bigint` | The parent node ID |

#### Returns

`bigint`[]

- Array of child node IDs

#### Throws

`TaffyError` if the parent node does not exist

#### Example

```typescript
const tree = new TaffyTree();
const parentId = tree.newLeaf(new Style());
const children = tree.children(parentId);
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
const tree = new TaffyTree();
tree.clear();
console.log(tree.totalNodeCount());
```

---

### computeLayout()

```ts
computeLayout(node, availableSpace): void;
```

Computes the layout for a subtree

This is the main layout computation method. Call this on the root node
to compute layouts for all nodes in the tree.

#### Parameters

| Parameter        | Type                                                                                       | Description                            |
| ---------------- | ------------------------------------------------------------------------------------------ | -------------------------------------- |
| `node`           | `bigint`                                                                                   | The root node ID to compute layout for |
| `availableSpace` | [`Size`](../type-aliases/Size.md)\<[`AvailableSpace`](../type-aliases/AvailableSpace.md)\> | The available space constraints        |

#### Returns

`void`

#### Examples

```typescript
const tree = new TaffyTree();
const rootId = tree.newLeaf(new Style());

// Fixed size container
tree.computeLayout(rootId, { width: 800, height: 600 });

// Flexible width, fixed height
tree.computeLayout(rootId, { width: "max-content", height: 600 });

// Minimum content size
tree.computeLayout(rootId, { width: "min-content", height: "min-content" });
```

```typescript
const tree = new TaffyTree();
const rootId = tree.newLeaf(new Style());
tree.computeLayout(rootId, { width: 800, height: 600 });
```

#### Throws

`TaffyError` if the node does not exist or available space is invalid

---

### computeLayoutWithMeasure()

```ts
computeLayoutWithMeasure(
   node,
   availableSpace,
   measureFunc): void;
```

Updates the stored layout of the provided node and its children

The measure function is called for leaf nodes (nodes without children) that
require measurement according to the layout algorithm (Flexbox/Grid).
For example, this is used for text nodes or other content that has intrinsic size.

#### Parameters

| Parameter        | Type                                                                                       | Description                                |
| ---------------- | ------------------------------------------------------------------------------------------ | ------------------------------------------ |
| `node`           | `bigint`                                                                                   | The root node ID to compute layout for     |
| `availableSpace` | [`Size`](../type-aliases/Size.md)\<[`AvailableSpace`](../type-aliases/AvailableSpace.md)\> | The available space constraints            |
| `measureFunc`    | [`MeasureFunction`](../type-aliases/MeasureFunction.md)                                    | A function that measures leaf node content |

#### Returns

`void`

#### Throws

`TaffyError` if the node does not exist or available space is invalid

#### Example

```typescript
const tree = new TaffyTree();
const rootId = tree.newLeaf(new Style());

const measureText = (text: string, width: number) => ({ width: 0, height: 0 });

tree.computeLayoutWithMeasure(
  rootId,
  { width: 800, height: "max-content" },
  (known, available, node, context, style) => {
    if (context?.text) {
      const measured = measureText(context.text, available.width as number);
      return { width: measured.width, height: measured.height };
    }
    return { width: 0, height: 0 };
  },
);
```

---

### detailedLayoutInfo()

```ts
detailedLayoutInfo(node): any;
```

Gets detailed layout information for grid layouts

#### Parameters

| Parameter | Type     | Description |
| --------- | -------- | ----------- |
| `node`    | `bigint` | The node ID |

#### Returns

`any`

- Detailed grid info or "None" for non-grid nodes

#### Note

This method is only available when the `detailed_layout_info`
feature is enabled.

#### Throws

`TaffyError` if the node does not exist

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
const tree = new TaffyTree();
const rootId = tree.newLeaf(new Style());
const nodeId = rootId;
const availableSpace = { width: 100, height: 100 };

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
const tree = new TaffyTree();
const node = tree.newLeaf(new Style());
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
const tree = new TaffyTree();
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
const tree = new TaffyTree();
const parentId = tree.newLeaf(new Style());
const childId = tree.newLeaf(new Style());
tree.addChild(parentId, childId);
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

| Parameter  | Type       | Description       |
| ---------- | ---------- | ----------------- |
| `children` | `bigint`[] | Array of node IDs |

#### Returns

`any`[]

- Array of context values (undefined for nodes without context)

#### Example

```typescript
const tree = new TaffyTree();
const id1 = tree.newLeaf(new Style());
const id2 = tree.newLeaf(new Style());
const nodes = [id1, id2];
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
const tree = new TaffyTree();
const style = new Style();
style.size = { width: 100, height: 100 };
const rootId = tree.newLeaf(style);
const nodeId = rootId;

tree.computeLayout(rootId, { width: 800, height: 600 });
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
const tree = new TaffyTree();
const nodeId = tree.newLeaf(new Style());
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
const tree = new TaffyTree();
const nodeId = tree.newLeaf(new Style());
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
const tree = new TaffyTree();
const parentId = tree.newLeaf(new Style());
const childId = tree.newLeaf(new Style());
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
const tree = new TaffyTree();
const rootId = tree.newLeaf(new Style());
const nodeId = rootId;
const availableSpace = { width: 100, height: 100 };

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
const tree = new TaffyTree();
const style = new Style();
style.size = { width: 100, height: 50 };
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

const tree = new TaffyTree();
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

| Parameter  | Type                | Description                           |
| ---------- | ------------------- | ------------------------------------- |
| `style`    | [`Style`](Style.md) | The style configuration for the node  |
| `children` | `bigint`[]          | Array of child node IDs (as bigint[]) |

#### Returns

`bigint`

- The node ID (`bigint`)

#### Throws

`TaffyError` if the node cannot be created

#### Example

```typescript
const tree = new TaffyTree();
const containerStyle = new Style();
containerStyle.display = Display.Flex;

const child1: bigint = tree.newLeaf(new Style());
const child2: bigint = tree.newLeaf(new Style());

const container: bigint = tree.newWithChildren(containerStyle, [
  child1,
  child2,
]);
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
const tree = new TaffyTree();
const parentId = tree.newLeaf(new Style());
const childId = tree.newLeaf(new Style());
tree.addChild(parentId, childId);
const parent: bigint | undefined = tree.parent(childId);
```

---

### printTree()

```ts
printTree(node): string;
```

Prints the tree structure to the console (for debugging)

Outputs a text representation of the tree structure starting from
the given node. Useful for debugging layout issues.

#### Parameters

| Parameter | Type     | Description                    |
| --------- | -------- | ------------------------------ |
| `node`    | `bigint` | The root node ID to print from |

#### Returns

`string`

- A string representation of the tree structure

#### Example

```typescript
const tree = new TaffyTree();
const rootId = tree.newLeaf(new Style());
const output = tree.printTree(rootId);
console.log(output);
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
const tree = new TaffyTree();
const nodeId = tree.newLeaf(new Style());
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
const tree = new TaffyTree();
const parentId = tree.newLeaf(new Style());
const childId = tree.newLeaf(new Style());
tree.addChild(parentId, childId);
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
const tree = new TaffyTree();
const parentId = tree.newLeaf(new Style());
const childId = tree.newLeaf(new Style());
tree.addChild(parentId, childId);
const removedId: bigint = tree.removeChildAtIndex(parentId, 0);
```

---

### removeChildrenRange()

```ts
removeChildrenRange(
   parent,
   startIndex,
   endIndex): void;
```

Removes a range of children

Removes children from `start_index` (inclusive) to `end_index` (exclusive).

#### Parameters

| Parameter    | Type     | Description                |
| ------------ | -------- | -------------------------- |
| `parent`     | `bigint` | The parent node ID         |
| `startIndex` | `number` | Start of range (inclusive) |
| `endIndex`   | `number` | End of range (exclusive)   |

#### Returns

`void`

#### Throws

`TaffyError` if the parent node does not exist or range is invalid

#### Example

```typescript
const tree = new TaffyTree();
const parentId = tree.newLeaf(new Style());
const child0 = tree.newLeaf(new Style());
const child1 = tree.newLeaf(new Style());
const child2 = tree.newLeaf(new Style());
const child3 = tree.newLeaf(new Style());
tree.setChildren(parentId, [child0, child1, child2, child3]);

tree.removeChildrenRange(parentId, 1, 3); // Removes child1 and child2
```

---

### replaceChildAtIndex()

```ts
replaceChildAtIndex(
   parent,
   index,
   newChild): bigint;
```

Replaces a child at a specific index

#### Parameters

| Parameter  | Type     | Description                                 |
| ---------- | -------- | ------------------------------------------- |
| `parent`   | `bigint` | The parent node ID                          |
| `index`    | `number` | The index of the child to replace (0-based) |
| `newChild` | `bigint` | The new child node ID                       |

#### Returns

`bigint`

- The replaced (old) child ID (`bigint`)

#### Throws

`TaffyError` if the parent node does not exist or index is out of bounds

#### Example

```typescript
const tree = new TaffyTree();
const parentId = tree.newLeaf(new Style());
const oldChild = tree.newLeaf(new Style());
const newChildId = tree.newLeaf(new Style());
tree.addChild(parentId, oldChild);
const child = tree.newLeaf(new Style()); // filler child at index 0 if needed, but index 1 implies 2 children
tree.insertChildAtIndex(parentId, 0, child);

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

| Parameter  | Type       | Description                 |
| ---------- | ---------- | --------------------------- |
| `parent`   | `bigint`   | The parent node ID          |
| `children` | `bigint`[] | Array of new child node IDs |

#### Returns

`void`

#### Throws

`TaffyError` if the parent node does not exist

#### Example

```typescript
const tree = new TaffyTree();
const parentId = tree.newLeaf(new Style());
const child1 = tree.newLeaf(new Style());
const child2 = tree.newLeaf(new Style());
const child3 = tree.newLeaf(new Style());
const children = [child1, child2, child3];
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
const tree = new TaffyTree();
const nodeId = tree.newLeaf(new Style());
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
const tree = new TaffyTree();
const nodeId = tree.newLeaf(new Style());
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
const tree = new TaffyTree();
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
const tree = new TaffyTree();
const nodeId = tree.newLeaf(new Style());
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
