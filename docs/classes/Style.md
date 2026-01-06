[**Taffy-JS API Documentation**](../index.md)

---

[Taffy-JS API Documentation](../index.md) / Style

# Class: Style

CSS layout configuration for a node, including flexbox, sizing, spacing, and alignment properties.

This class holds all CSS layout properties for a node. Create an instance with
`new Style()` and configure properties before passing to `TaffyTree.newLeaf()`.

**Default Values**

When created, all properties are set to their CSS default values:

- `display`: `Display.Block`
- `position`: `Position.Relative`
- `flexDirection`: `FlexDirection.Row`
- `flexWrap`: `FlexWrap.NoWrap`
- `flexGrow`: `0`
- `flexShrink`: `1`
- All alignment properties: `undefined` (use default behavior)
- All dimensions: `"Auto"`
- All spacing: `{ Length: 0 }`

## Constructors

### Constructor

```ts
new Style(): Style;
```

Creates a new Style instance with default values

# Returns

A new `Style` object with all properties set to CSS defaults

# Example

```javascript
const style = new Style();
console.log(style.display); // Display.Block
```

#### Returns

`Style`

## Properties

| Property                                     | Type                                                                                                 | Description                                                                                                                                                                                          |
| -------------------------------------------- | ---------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| <a id="aligncontent"></a> `alignContent`     | [`AlignContent`](../enumerations/AlignContent.md) \| `undefined`                                     | Gets the align-content property Controls distribution of space between lines in a multi-line flex container. # Returns The current [`AlignContent`](JsAlignContent) value, or `undefined` if not set |
| <a id="alignitems"></a> `alignItems`         | [`AlignItems`](../enumerations/AlignItems.md) \| `undefined`                                         | Gets the align-items property Defines the default alignment for all children on the cross axis. # Returns The current [`AlignItems`](JsAlignItems) value, or `undefined` if not set                  |
| <a id="alignself"></a> `alignSelf`           | [`AlignSelf`](../enumerations/AlignSelf.md) \| `undefined`                                           | Gets the align-self property Overrides the parent's align-items for this specific element. # Returns The current [`AlignSelf`](JsAlignSelf) value (returns `Auto` if not set)                        |
| <a id="aspectratio"></a> `aspectRatio`       | `number` \| `undefined`                                                                              | Gets the aspect ratio The ratio of width to height. Used to maintain proportions. # Returns The aspect ratio value, or `undefined` if not set                                                        |
| <a id="border"></a> `border`                 | [`Rect`](../interfaces/Rect.md)\<[`LengthPercentage`](../type-aliases/LengthPercentage.md)\>         | Gets the border width Width of the element's border on each side. # Returns A `Rect<LengthPercentage>` with left, right, top, bottom border widths                                                   |
| <a id="boxsizing"></a> `boxSizing`           | [`BoxSizing`](../enumerations/BoxSizing.md)                                                          | Gets the box sizing mode Determines whether padding and border are included in dimensions. # Returns The current [`BoxSizing`](JsBoxSizing) value # Default `BoxSizing.BorderBox`                    |
| <a id="display"></a> `display`               | [`Display`](../enumerations/Display.md)                                                              | Gets the display mode Determines the layout algorithm used for this element and its children. # Returns The current [`Display`](JsDisplay) value # Default `Display.Block`                           |
| <a id="flexbasis"></a> `flexBasis`           | [`Dimension`](../type-aliases/Dimension.md)                                                          | Gets the flex-basis The initial size of a flex item before growing/shrinking. # Returns A `Dimension` value (`{ Length: n }`, `{ Percent: n }`, or `"Auto"`)                                         |
| <a id="flexdirection"></a> `flexDirection`   | [`FlexDirection`](../enumerations/FlexDirection.md)                                                  | Gets the flex direction Defines the main axis direction for flex items. # Returns The current [`FlexDirection`](JsFlexDirection) value # Default `FlexDirection.Row`                                 |
| <a id="flexgrow"></a> `flexGrow`             | `number`                                                                                             | Gets the flex grow factor Determines how much the item grows relative to siblings when there is extra space available. # Returns The flex grow factor (default: 0)                                   |
| <a id="flexshrink"></a> `flexShrink`         | `number`                                                                                             | Gets the flex shrink factor Determines how much the item shrinks relative to siblings when there is insufficient space. # Returns The flex shrink factor (default: 1)                                |
| <a id="flexwrap"></a> `flexWrap`             | [`FlexWrap`](../enumerations/FlexWrap.md)                                                            | Gets the flex wrap mode Controls whether flex items wrap to new lines. # Returns The current [`FlexWrap`](JsFlexWrap) value # Default `FlexWrap.NoWrap`                                              |
| <a id="gap"></a> `gap`                       | [`Size`](../interfaces/Size.md)\<[`LengthPercentage`](../type-aliases/LengthPercentage.md)\>         | Gets the gap Spacing between flex/grid items. # Returns A `Size<LengthPercentage>` with column (width) and row (height) gaps                                                                         |
| <a id="inset"></a> `inset`                   | [`Rect`](../interfaces/Rect.md)\<[`LengthPercentageAuto`](../type-aliases/LengthPercentageAuto.md)\> | Gets the inset Positioning offsets for absolutely positioned elements. # Returns A `Rect<LengthPercentageAuto>` with left, right, top, bottom offsets                                                |
| <a id="justifycontent"></a> `justifyContent` | [`JustifyContent`](../enumerations/JustifyContent.md) \| `undefined`                                 | Gets the justify-content property Defines alignment and spacing of items along the main axis. # Returns The current [`JustifyContent`](JsJustifyContent) value, or `undefined` if not set            |
| <a id="margin"></a> `margin`                 | [`Rect`](../interfaces/Rect.md)\<[`LengthPercentageAuto`](../type-aliases/LengthPercentageAuto.md)\> | Gets the margin Outer spacing around the element. # Returns A `Rect<LengthPercentageAuto>` with left, right, top, bottom margins                                                                     |
| <a id="maxsize"></a> `maxSize`               | [`Size`](../interfaces/Size.md)\<[`Dimension`](../type-aliases/Dimension.md)\>                       | Gets the maximum size constraints # Returns A `Size<Dimension>` object with maximum width and height                                                                                                 |
| <a id="minsize"></a> `minSize`               | [`Size`](../interfaces/Size.md)\<[`Dimension`](../type-aliases/Dimension.md)\>                       | Gets the minimum size constraints # Returns A `Size<Dimension>` object with minimum width and height                                                                                                 |
| <a id="overflow"></a> `overflow`             | [`Point`](../interfaces/Point.md)\<[`Overflow`](../enumerations/Overflow.md)\>                       | Gets the overflow behavior Controls how content that exceeds the container is handled. # Returns A `Point<Overflow>` with `x` and `y` overflow settings                                              |
| <a id="padding"></a> `padding`               | [`Rect`](../interfaces/Rect.md)\<[`LengthPercentage`](../type-aliases/LengthPercentage.md)\>         | Gets the padding Inner spacing between the element's border and content. # Returns A `Rect<LengthPercentage>` with left, right, top, bottom padding                                                  |
| <a id="position"></a> `position`             | [`Position`](../enumerations/Position.md)                                                            | Gets the position mode Determines how the element is positioned within its parent. # Returns The current [`Position`](JsPosition) value # Default `Position.Relative`                                |
| <a id="size"></a> `size`                     | [`Size`](../interfaces/Size.md)\<[`Dimension`](../type-aliases/Dimension.md)\>                       | Gets the size (width and height) # Returns A `Size<Dimension>` object with `width` and `height` properties                                                                                           |

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
