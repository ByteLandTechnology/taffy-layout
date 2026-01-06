[**Taffy-JS API Documentation**](../index.md)

---

[Taffy-JS API Documentation](../index.md) / Layout

# Class: Layout

Computed layout result containing position, size, and spacing values for a node.

This class wraps the native [`taffy::Layout`] and provides read-only access
to all computed layout values. All dimensions are in pixels.

**Properties**

| Property                       | Type  | Description                    |
| ------------------------------ | ----- | ------------------------------ |
| `x`                            | `f32` | X position relative to parent  |
| `y`                            | `f32` | Y position relative to parent  |
| `width`                        | `f32` | Computed width                 |
| `height`                       | `f32` | Computed height                |
| `contentWidth`                 | `f32` | Width of scrollable content    |
| `contentHeight`                | `f32` | Height of scrollable content   |
| `paddingTop/Right/Bottom/Left` | `f32` | Computed padding               |
| `borderTop/Right/Bottom/Left`  | `f32` | Computed border widths         |
| `marginTop/Right/Bottom/Left`  | `f32` | Computed margins               |
| `scrollbarWidth`               | `f32` | Width of vertical scrollbar    |
| `scrollbarHeight`              | `f32` | Height of horizontal scrollbar |
| `order`                        | `u32` | Rendering order for z-indexing |

**JavaScript Interface**

```typescript
class Layout {
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
  readonly paddingTop: number;
  readonly paddingRight: number;
  readonly paddingBottom: number;
  readonly paddingLeft: number;
  // ... and more
}
```

## Properties

| Property                                       | Modifier   | Type     | Description                                                                                                                                                                                                          |
| ---------------------------------------------- | ---------- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| <a id="borderbottom"></a> `borderBottom`       | `readonly` | `number` | Gets the bottom border width # Returns The computed bottom border width in pixels                                                                                                                                    |
| <a id="borderleft"></a> `borderLeft`           | `readonly` | `number` | Gets the left border width # Returns The computed left border width in pixels                                                                                                                                        |
| <a id="borderright"></a> `borderRight`         | `readonly` | `number` | Gets the right border width # Returns The computed right border width in pixels                                                                                                                                      |
| <a id="bordertop"></a> `borderTop`             | `readonly` | `number` | Gets the top border width # Returns The computed top border width in pixels                                                                                                                                          |
| <a id="contentheight"></a> `contentHeight`     | `readonly` | `number` | Gets the height of the scrollable content If the node has overflow content, this represents the total height of all content (may exceed `height`). # Returns The content height in pixels                            |
| <a id="contentwidth"></a> `contentWidth`       | `readonly` | `number` | Gets the width of the scrollable content If the node has overflow content, this represents the total width of all content (may exceed `width`). # Returns The content width in pixels                                |
| <a id="height"></a> `height`                   | `readonly` | `number` | Gets the computed height of the node This is the final height after layout computation, including any constraints from min/max size or flex properties. # Returns The height in pixels                               |
| <a id="marginbottom"></a> `marginBottom`       | `readonly` | `number` | Gets the bottom margin # Returns The computed bottom margin in pixels                                                                                                                                                |
| <a id="marginleft"></a> `marginLeft`           | `readonly` | `number` | Gets the left margin # Returns The computed left margin in pixels                                                                                                                                                    |
| <a id="marginright"></a> `marginRight`         | `readonly` | `number` | Gets the right margin # Returns The computed right margin in pixels                                                                                                                                                  |
| <a id="margintop"></a> `marginTop`             | `readonly` | `number` | Gets the top margin # Returns The computed top margin in pixels                                                                                                                                                      |
| <a id="order"></a> `order`                     | `readonly` | `number` | Gets the rendering order of the node This value determines the z-order for overlapping elements. Lower values are rendered first (behind higher values). # Returns The rendering order as an unsigned 32-bit integer |
| <a id="paddingbottom"></a> `paddingBottom`     | `readonly` | `number` | Gets the bottom padding # Returns The computed bottom padding in pixels                                                                                                                                              |
| <a id="paddingleft"></a> `paddingLeft`         | `readonly` | `number` | Gets the left padding # Returns The computed left padding in pixels                                                                                                                                                  |
| <a id="paddingright"></a> `paddingRight`       | `readonly` | `number` | Gets the right padding # Returns The computed right padding in pixels                                                                                                                                                |
| <a id="paddingtop"></a> `paddingTop`           | `readonly` | `number` | Gets the top padding # Returns The computed top padding in pixels                                                                                                                                                    |
| <a id="scrollbarheight"></a> `scrollbarHeight` | `readonly` | `number` | Gets the height of the horizontal scrollbar When overflow is set to scroll, this indicates the space reserved for the horizontal scrollbar. # Returns The scrollbar height in pixels (0 if no scrollbar)             |
| <a id="scrollbarwidth"></a> `scrollbarWidth`   | `readonly` | `number` | Gets the width of the vertical scrollbar When overflow is set to scroll, this indicates the space reserved for the vertical scrollbar. # Returns The scrollbar width in pixels (0 if no scrollbar)                   |
| <a id="width"></a> `width`                     | `readonly` | `number` | Gets the computed width of the node This is the final width after layout computation, including any constraints from min/max size or flex properties. # Returns The width in pixels                                  |
| <a id="x"></a> `x`                             | `readonly` | `number` | Gets the X coordinate of the node's top-left corner This value is relative to the node's parent. For the root node, this is always 0. # Returns The horizontal position in pixels                                    |
| <a id="y"></a> `y`                             | `readonly` | `number` | Gets the Y coordinate of the node's top-left corner This value is relative to the node's parent. For the root node, this is always 0. # Returns The vertical position in pixels                                      |

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
