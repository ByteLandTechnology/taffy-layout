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

| Property                                       | Modifier   | Type     | Description                                                                                                                                              |
| ---------------------------------------------- | ---------- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| <a id="borderbottom"></a> `borderBottom`       | `readonly` | `number` | Gets the bottom border width                                                                                                                             |
| <a id="borderleft"></a> `borderLeft`           | `readonly` | `number` | Gets the left border width                                                                                                                               |
| <a id="borderright"></a> `borderRight`         | `readonly` | `number` | Gets the right border width                                                                                                                              |
| <a id="bordertop"></a> `borderTop`             | `readonly` | `number` | Gets the top border width                                                                                                                                |
| <a id="contentheight"></a> `contentHeight`     | `readonly` | `number` | Gets the height of the scrollable content If the node has overflow content, this represents the total height of all content (may exceed `height`).       |
| <a id="contentwidth"></a> `contentWidth`       | `readonly` | `number` | Gets the width of the scrollable content If the node has overflow content, this represents the total width of all content (may exceed `width`).          |
| <a id="height"></a> `height`                   | `readonly` | `number` | Gets the computed height of the node This is the final height after layout computation, including any constraints from min/max size or flex properties.  |
| <a id="marginbottom"></a> `marginBottom`       | `readonly` | `number` | Gets the bottom margin                                                                                                                                   |
| <a id="marginleft"></a> `marginLeft`           | `readonly` | `number` | Gets the left margin                                                                                                                                     |
| <a id="marginright"></a> `marginRight`         | `readonly` | `number` | Gets the right margin                                                                                                                                    |
| <a id="margintop"></a> `marginTop`             | `readonly` | `number` | Gets the top margin                                                                                                                                      |
| <a id="order"></a> `order`                     | `readonly` | `number` | Gets the rendering order of the node This value determines the z-order for overlapping elements. Lower values are rendered first (behind higher values). |
| <a id="paddingbottom"></a> `paddingBottom`     | `readonly` | `number` | Gets the bottom padding                                                                                                                                  |
| <a id="paddingleft"></a> `paddingLeft`         | `readonly` | `number` | Gets the left padding                                                                                                                                    |
| <a id="paddingright"></a> `paddingRight`       | `readonly` | `number` | Gets the right padding                                                                                                                                   |
| <a id="paddingtop"></a> `paddingTop`           | `readonly` | `number` | Gets the top padding                                                                                                                                     |
| <a id="scrollbarheight"></a> `scrollbarHeight` | `readonly` | `number` | Gets the height of the horizontal scrollbar When overflow is set to scroll, this indicates the space reserved for the horizontal scrollbar.              |
| <a id="scrollbarwidth"></a> `scrollbarWidth`   | `readonly` | `number` | Gets the width of the vertical scrollbar When overflow is set to scroll, this indicates the space reserved for the vertical scrollbar.                   |
| <a id="width"></a> `width`                     | `readonly` | `number` | Gets the computed width of the node This is the final width after layout computation, including any constraints from min/max size or flex properties.    |
| <a id="x"></a> `x`                             | `readonly` | `number` | Gets the X coordinate of the node's top-left corner This value is relative to the node's parent. For the root node, this is always 0.                    |
| <a id="y"></a> `y`                             | `readonly` | `number` | Gets the Y coordinate of the node's top-left corner This value is relative to the node's parent. For the root node, this is always 0.                    |

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
