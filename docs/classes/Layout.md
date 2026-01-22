[**Taffy Layout API Documentation**](../index.md)

---

[Taffy Layout API Documentation](../index.md) / Layout

# Class: Layout

Computed layout result containing position, size, and spacing values for a node.

This class wraps the native [`taffy::Layout`] and provides read-only access
to all computed layout values. All dimensions are in pixels.

## Example

```typescript
const tree = new TaffyTree();
const rootId = tree.newLeaf(new Style());
const node = rootId;

tree.computeLayout(rootId, { width: 800, height: 600 });
const layout = tree.getLayout(node);

console.log("Position:", layout.x, layout.y);
console.log("Size:", layout.width, layout.height);
console.log("Content:", layout.contentWidth, layout.contentHeight);
console.log(
  "Padding:",
  layout.paddingTop,
  layout.paddingRight,
  layout.paddingBottom,
  layout.paddingLeft,
);
console.log(
  "Border:",
  layout.borderTop,
  layout.borderRight,
  layout.borderBottom,
  layout.borderLeft,
);
console.log(
  "Margin:",
  layout.marginTop,
  layout.marginRight,
  layout.marginBottom,
  layout.marginLeft,
);
console.log("Scrollbar:", layout.scrollbarWidth, layout.scrollbarHeight);
console.log("Order:", layout.order);
```

## Properties

| Property                                       | Modifier   | Type     | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| ---------------------------------------------- | ---------- | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| <a id="border"></a> `border`                   | `readonly` | `any`    | Gets the border as a Rect with left, right, top, bottom border widths **Example** `import { TaffyTree, Style } from "taffy-layout"; const tree = new TaffyTree(); const root = tree.newLeaf(new Style()); tree.computeLayout(root, { width: 100, height: 100 }); const layout = tree.getLayout(root); const border = layout.border; console.log(`Border: ${border.left} ${border.right} ${border.top} ${border.bottom}`); tree.free();`                                                                                                               |
| <a id="borderbottom"></a> `borderBottom`       | `readonly` | `number` | Gets the bottom border width                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| <a id="borderleft"></a> `borderLeft`           | `readonly` | `number` | Gets the left border width                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| <a id="borderright"></a> `borderRight`         | `readonly` | `number` | Gets the right border width                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| <a id="bordertop"></a> `borderTop`             | `readonly` | `number` | Gets the top border width                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| <a id="contentheight"></a> `contentHeight`     | `readonly` | `number` | Gets the height of the scrollable content If the node has overflow content, this represents the total height of all content (may exceed `height`).                                                                                                                                                                                                                                                                                                                                                                                                    |
| <a id="contentsize"></a> `contentSize`         | `readonly` | `any`    | Gets the content size as a Size with contentWidth and contentHeight If the node has overflow content, this represents the total size of all content (may exceed the node's width/height). **Example** `import { TaffyTree, Style } from "taffy-layout"; const tree = new TaffyTree(); const root = tree.newLeaf(new Style()); tree.computeLayout(root, { width: 100, height: 100 }); const layout = tree.getLayout(root); const contentSize = layout.contentSize; console.log(`Content: ${contentSize.width} x ${contentSize.height}`); tree.free();` |
| <a id="contentwidth"></a> `contentWidth`       | `readonly` | `number` | Gets the width of the scrollable content If the node has overflow content, this represents the total width of all content (may exceed `width`).                                                                                                                                                                                                                                                                                                                                                                                                       |
| <a id="height"></a> `height`                   | `readonly` | `number` | Gets the computed height of the node This is the final height after layout computation, including any constraints from min/max size or flex properties.                                                                                                                                                                                                                                                                                                                                                                                               |
| <a id="margin"></a> `margin`                   | `readonly` | `any`    | Gets the margin as a Rect with left, right, top, bottom margins **Example** `import { TaffyTree, Style } from "taffy-layout"; const tree = new TaffyTree(); const root = tree.newLeaf(new Style()); tree.computeLayout(root, { width: 100, height: 100 }); const layout = tree.getLayout(root); const margin = layout.margin; console.log(`Margin: ${margin.left} ${margin.right} ${margin.top} ${margin.bottom}`); tree.free();`                                                                                                                     |
| <a id="marginbottom"></a> `marginBottom`       | `readonly` | `number` | Gets the bottom margin                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| <a id="marginleft"></a> `marginLeft`           | `readonly` | `number` | Gets the left margin                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| <a id="marginright"></a> `marginRight`         | `readonly` | `number` | Gets the right margin                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| <a id="margintop"></a> `marginTop`             | `readonly` | `number` | Gets the top margin                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| <a id="order"></a> `order`                     | `readonly` | `number` | Gets the rendering order of the node This value determines the z-order for overlapping elements. Lower values are rendered first (behind higher values).                                                                                                                                                                                                                                                                                                                                                                                              |
| <a id="padding"></a> `padding`                 | `readonly` | `any`    | Gets the padding as a Rect with left, right, top, bottom padding **Example** `import { TaffyTree, Style } from "taffy-layout"; const tree = new TaffyTree(); const root = tree.newLeaf(new Style()); tree.computeLayout(root, { width: 100, height: 100 }); const layout = tree.getLayout(root); const padding = layout.padding; console.log(`Padding: ${padding.left} ${padding.right} ${padding.top} ${padding.bottom}`); tree.free();`                                                                                                             |
| <a id="paddingbottom"></a> `paddingBottom`     | `readonly` | `number` | Gets the bottom padding                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| <a id="paddingleft"></a> `paddingLeft`         | `readonly` | `number` | Gets the left padding                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| <a id="paddingright"></a> `paddingRight`       | `readonly` | `number` | Gets the right padding                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| <a id="paddingtop"></a> `paddingTop`           | `readonly` | `number` | Gets the top padding                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| <a id="position"></a> `position`               | `readonly` | `any`    | Gets the position as a Point with x and y coordinates **Example** `import { TaffyTree, Style } from "taffy-layout"; const tree = new TaffyTree(); const root = tree.newLeaf(new Style()); tree.computeLayout(root, { width: 100, height: 100 }); const layout = tree.getLayout(root); const pos = layout.position; console.log(`Position: (${pos.x}, ${pos.y})`); tree.free();`                                                                                                                                                                       |
| <a id="scrollbarheight"></a> `scrollbarHeight` | `readonly` | `number` | Gets the height of the horizontal scrollbar When overflow is set to scroll, this indicates the space reserved for the horizontal scrollbar.                                                                                                                                                                                                                                                                                                                                                                                                           |
| <a id="scrollbarsize"></a> `scrollbarSize`     | `readonly` | `any`    | Gets the scrollbar size as a Size with scrollbarWidth and scrollbarHeight When overflow is set to scroll, this indicates the space reserved for scrollbars. **Example** `import { TaffyTree, Style } from "taffy-layout"; const tree = new TaffyTree(); const root = tree.newLeaf(new Style()); tree.computeLayout(root, { width: 100, height: 100 }); const layout = tree.getLayout(root); const scrollbarSize = layout.scrollbarSize; console.log(`Scrollbar: ${scrollbarSize.width} x ${scrollbarSize.height}`); tree.free();`                     |
| <a id="scrollbarwidth"></a> `scrollbarWidth`   | `readonly` | `number` | Gets the width of the vertical scrollbar When overflow is set to scroll, this indicates the space reserved for the vertical scrollbar.                                                                                                                                                                                                                                                                                                                                                                                                                |
| <a id="size"></a> `size`                       | `readonly` | `any`    | Gets the size as a Size with width and height **Example** `import { TaffyTree, Style } from "taffy-layout"; const tree = new TaffyTree(); const root = tree.newLeaf(new Style()); tree.computeLayout(root, { width: 100, height: 100 }); const layout = tree.getLayout(root); const size = layout.size; console.log(`Size: ${size.width} x ${size.height}`); tree.free();`                                                                                                                                                                            |
| <a id="width"></a> `width`                     | `readonly` | `number` | Gets the computed width of the node This is the final width after layout computation, including any constraints from min/max size or flex properties.                                                                                                                                                                                                                                                                                                                                                                                                 |
| <a id="x"></a> `x`                             | `readonly` | `number` | Gets the X coordinate of the node's top-left corner This value is relative to the node's parent. For the root node, this is always 0.                                                                                                                                                                                                                                                                                                                                                                                                                 |
| <a id="y"></a> `y`                             | `readonly` | `number` | Gets the Y coordinate of the node's top-left corner This value is relative to the node's parent. For the root node, this is always 0.                                                                                                                                                                                                                                                                                                                                                                                                                 |

## Methods

### \[dispose\]()

```ts
dispose: void;
```

#### Returns

`void`

---

### free()

```ts
free(): void;
```

#### Returns

`void`

---

### get()

#### Call Signature

```ts
get<K>(...keys): LayoutPropertyValues[K];
```

Reads multiple layout properties in a single WASM call.
Supports both object properties and individual flat properties.

##### Type Parameters

| Type Parameter                                                      |
| ------------------------------------------------------------------- |
| `K` _extends_ [`LayoutProperty`](../type-aliases/LayoutProperty.md) |

##### Parameters

| Parameter | Type    |
| --------- | ------- |
| ...`keys` | \[`K`\] |

##### Returns

[`LayoutPropertyValues`](../type-aliases/LayoutPropertyValues.md)\[`K`\]

Single value for one key, tuple for 2-3 keys, array for 4+ keys

##### Throws

Error if any property key is unknown.

##### Remarks

- Single property: returns exact value type
- 2-3 properties: returns typed tuple for destructuring
- 4+ properties: returns array of union types

##### Example

```typescript
import { TaffyTree, Style } from "taffy-layout";

const tree = new TaffyTree();
const root = tree.newLeaf(new Style());
tree.computeLayout(root, { width: 100, height: 100 });
const layout = tree.getLayout(root);

// Single property - returns exact type
const width = layout.get("width"); // number

// Two properties - returns tuple for destructuring
const [pos, size] = layout.get("position", "size");
// pos: Point<number>, size: Size<number>

// Three properties - returns tuple for destructuring
const [x, y, w] = layout.get("x", "y", "width");

// Four or more properties - returns array
const values = layout.get("x", "y", "width", "height");
// values type is: number[]

tree.free();
```

#### Call Signature

```ts
get<K1, K2>(...keys): [LayoutPropertyValues[K1], LayoutPropertyValues[K2]];
```

##### Type Parameters

| Type Parameter                                                       |
| -------------------------------------------------------------------- |
| `K1` _extends_ [`LayoutProperty`](../type-aliases/LayoutProperty.md) |
| `K2` _extends_ [`LayoutProperty`](../type-aliases/LayoutProperty.md) |

##### Parameters

| Parameter | Type           |
| --------- | -------------- |
| ...`keys` | \[`K1`, `K2`\] |

##### Returns

\[[`LayoutPropertyValues`](../type-aliases/LayoutPropertyValues.md)\[`K1`\], [`LayoutPropertyValues`](../type-aliases/LayoutPropertyValues.md)\[`K2`\]\]

#### Call Signature

```ts
get<K1, K2, K3>(...keys): [LayoutPropertyValues[K1], LayoutPropertyValues[K2], LayoutPropertyValues[K3]];
```

##### Type Parameters

| Type Parameter                                                       |
| -------------------------------------------------------------------- |
| `K1` _extends_ [`LayoutProperty`](../type-aliases/LayoutProperty.md) |
| `K2` _extends_ [`LayoutProperty`](../type-aliases/LayoutProperty.md) |
| `K3` _extends_ [`LayoutProperty`](../type-aliases/LayoutProperty.md) |

##### Parameters

| Parameter | Type                 |
| --------- | -------------------- |
| ...`keys` | \[`K1`, `K2`, `K3`\] |

##### Returns

\[[`LayoutPropertyValues`](../type-aliases/LayoutPropertyValues.md)\[`K1`\], [`LayoutPropertyValues`](../type-aliases/LayoutPropertyValues.md)\[`K2`\], [`LayoutPropertyValues`](../type-aliases/LayoutPropertyValues.md)\[`K3`\]\]

#### Call Signature

```ts
get<Keys>(...keys): LayoutPropertyArrayValues<Keys>;
```

##### Type Parameters

| Type Parameter                                                           |
| ------------------------------------------------------------------------ |
| `Keys` _extends_ [`LayoutProperty`](../type-aliases/LayoutProperty.md)[] |

##### Parameters

| Parameter | Type   |
| --------- | ------ |
| ...`keys` | `Keys` |

##### Returns

[`LayoutPropertyArrayValues`](../type-aliases/LayoutPropertyArrayValues.md)\<`Keys`\>
