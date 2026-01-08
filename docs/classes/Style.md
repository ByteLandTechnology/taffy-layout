[**Taffy Layout API Documentation**](../index.md)

---

[Taffy Layout API Documentation](../index.md) / Style

# Class: Style

CSS layout configuration for a node, including flexbox, sizing, spacing, and alignment properties.

This class holds all CSS layout properties for a node. Create an instance with
`new Style()` and configure properties before passing to `TaffyTree.newLeaf()`.

## Default Value

When created, all properties are set to their CSS default values:

- `display`: `Display.Block`
- `position`: `Position.Relative`
- `flexDirection`: `FlexDirection.Row`
- `flexWrap`: `FlexWrap.NoWrap`
- `flexGrow`: `0`
- `flexShrink`: `1`
- All alignment properties: `undefined` (use default behavior)
- All dimensions: `"auto"`
- All spacing: `0`

## Constructors

### Constructor

```ts
new Style(): Style;
```

Creates a new Style instance with default values

#### Returns

`Style`

- A new `Style` object with all properties set to CSS defaults

#### Example

```typescript
const style = new Style();
console.log(style.display); // Display.Block
```

## Properties

| Property                                                       | Type                                                                                                   | Default value         | Description                                                                                                                                       |
| -------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ | --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| <a id="aligncontent"></a> `alignContent`                       | [`AlignContent`](../enumerations/AlignContent.md) \| `undefined`                                       | `undefined`           | Gets the align-content property Controls distribution of space between lines in a multi-line flex container.                                      |
| <a id="alignitems"></a> `alignItems`                           | [`AlignItems`](../enumerations/AlignItems.md) \| `undefined`                                           | `undefined`           | Gets the align-items property Defines the default alignment for all children on the cross axis.                                                   |
| <a id="alignself"></a> `alignSelf`                             | [`AlignSelf`](../enumerations/AlignSelf.md) \| `undefined`                                             | `undefined`           | Gets the align-self property Overrides the parent's align-items for this specific element.                                                        |
| <a id="aspectratio"></a> `aspectRatio`                         | `number` \| `undefined`                                                                                | `undefined`           | Gets the aspect ratio The ratio of width to height. Used to maintain proportions.                                                                 |
| <a id="border"></a> `border`                                   | [`Rect`](../type-aliases/Rect.md)\<[`LengthPercentage`](../type-aliases/LengthPercentage.md)\>         | `undefined`           | Gets the border width Width of the element's border on each side.                                                                                 |
| <a id="boxsizing"></a> `boxSizing`                             | [`BoxSizing`](../enumerations/BoxSizing.md)                                                            | `BoxSizing.BorderBox` | Gets the box sizing mode Determines whether padding and border are included in dimensions.                                                        |
| <a id="display"></a> `display`                                 | [`Display`](../enumerations/Display.md)                                                                | - `Display.Block`     | Gets the display mode Determines the layout algorithm used for this element and its children.                                                     |
| <a id="flexbasis"></a> `flexBasis`                             | [`Dimension`](../type-aliases/Dimension.md)                                                            | `undefined`           | Gets the flex-basis The initial size of a flex item before growing/shrinking.                                                                     |
| <a id="flexdirection"></a> `flexDirection`                     | [`FlexDirection`](../enumerations/FlexDirection.md)                                                    | - `FlexDirection.Row` | Gets the flex direction Defines the main axis direction for flex items.                                                                           |
| <a id="flexgrow"></a> `flexGrow`                               | `number`                                                                                               | `undefined`           | Gets the flex grow factor Determines how much the item grows relative to siblings when there is extra space available.                            |
| <a id="flexshrink"></a> `flexShrink`                           | `number`                                                                                               | `undefined`           | Gets the flex shrink factor Determines how much the item shrinks relative to siblings when there is insufficient space.                           |
| <a id="flexwrap"></a> `flexWrap`                               | [`FlexWrap`](../enumerations/FlexWrap.md)                                                              | - `FlexWrap.NoWrap`   | Gets the flex wrap mode Controls whether flex items wrap to new lines.                                                                            |
| <a id="gap"></a> `gap`                                         | [`Size`](../type-aliases/Size.md)\<[`LengthPercentage`](../type-aliases/LengthPercentage.md)\>         | `undefined`           | Gets the gap Spacing between flex/grid items.                                                                                                     |
| <a id="gridautocolumns"></a> `gridAutoColumns`                 | [`TrackSizingFunction`](../type-aliases/TrackSizingFunction.md)[]                                      | `undefined`           | Gets the grid-auto-columns property Defines the size of implicitly created columns.                                                               |
| <a id="gridautoflow"></a> `gridAutoFlow`                       | [`GridAutoFlow`](../enumerations/GridAutoFlow.md)                                                      | - `GridAutoFlow.Row`  | Gets the grid-auto-flow property Controls how auto-placed items are inserted into the grid.                                                       |
| <a id="gridautorows"></a> `gridAutoRows`                       | [`TrackSizingFunction`](../type-aliases/TrackSizingFunction.md)[]                                      | `undefined`           | Gets the grid-auto-rows property Defines the size of implicitly created rows.                                                                     |
| <a id="gridcolumn"></a> `gridColumn`                           | [`Line`](../type-aliases/Line.md)\<[`GridPlacement`](../type-aliases/GridPlacement.md)\>               | `undefined`           | Gets the grid-column property Defines which column in the grid the item should start and end at. Corresponds to CSS `grid-column` shorthand.      |
| <a id="gridrow"></a> `gridRow`                                 | [`Line`](../type-aliases/Line.md)\<[`GridPlacement`](../type-aliases/GridPlacement.md)\>               | `undefined`           | Gets the grid-row property Defines which row in the grid the item should start and end at. Corresponds to CSS `grid-row` shorthand.               |
| <a id="gridtemplateareas"></a> `gridTemplateAreas`             | [`GridTemplateArea`](../type-aliases/GridTemplateArea.md)[]                                            | `undefined`           | Gets the grid-template-areas property Defines named grid areas that can be referenced by grid items.                                              |
| <a id="gridtemplatecolumnnames"></a> `gridTemplateColumnNames` | `string`[][]                                                                                           | `undefined`           | Gets the grid-template-column-names property Defines the named lines between the columns.                                                         |
| <a id="gridtemplatecolumns"></a> `gridTemplateColumns`         | [`GridTemplateComponent`](../type-aliases/GridTemplateComponent.md)[]                                  | `undefined`           | Gets the grid-template-columns property Defines the track sizing functions (widths) of the grid columns.                                          |
| <a id="gridtemplaterownames"></a> `gridTemplateRowNames`       | `string`[][]                                                                                           | `undefined`           | Gets the grid-template-row-names property Defines the named lines between the rows.                                                               |
| <a id="gridtemplaterows"></a> `gridTemplateRows`               | [`GridTemplateComponent`](../type-aliases/GridTemplateComponent.md)[]                                  | `undefined`           | Gets the grid-template-rows property Defines the track sizing functions (heights) of the grid rows.                                               |
| <a id="inset"></a> `inset`                                     | [`Rect`](../type-aliases/Rect.md)\<[`LengthPercentageAuto`](../type-aliases/LengthPercentageAuto.md)\> | `undefined`           | Gets the inset Positioning offsets for absolutely positioned elements.                                                                            |
| <a id="itemisreplaced"></a> `itemIsReplaced`                   | `boolean`                                                                                              | - `false`             | Gets whether this item is a replaced element Replaced elements have special sizing behavior (e.g., `<img>`, `<video>`).                           |
| <a id="itemistable"></a> `itemIsTable`                         | `boolean`                                                                                              | - `false`             | Gets whether this item is a table Table children are handled specially in block layout.                                                           |
| <a id="justifycontent"></a> `justifyContent`                   | [`JustifyContent`](../enumerations/JustifyContent.md) \| `undefined`                                   | `undefined`           | Gets the justify-content property Defines alignment and spacing of items along the main axis.                                                     |
| <a id="justifyitems"></a> `justifyItems`                       | [`AlignItems`](../enumerations/AlignItems.md) \| `undefined`                                           | `undefined`           | Gets the justify-items property Defines the default justify-self for all children in the inline axis. This is primarily used for CSS Grid layout. |
| <a id="justifyself"></a> `justifySelf`                         | [`AlignSelf`](../enumerations/AlignSelf.md) \| `undefined`                                             | `undefined`           | Gets the justify-self property Overrides the parent's justify-items for this specific element in the inline axis.                                 |
| <a id="margin"></a> `margin`                                   | [`Rect`](../type-aliases/Rect.md)\<[`LengthPercentageAuto`](../type-aliases/LengthPercentageAuto.md)\> | `undefined`           | Gets the margin Outer spacing around the element.                                                                                                 |
| <a id="maxsize"></a> `maxSize`                                 | [`Size`](../type-aliases/Size.md)\<[`Dimension`](../type-aliases/Dimension.md)\>                       | `undefined`           | Gets the maximum size constraints                                                                                                                 |
| <a id="minsize"></a> `minSize`                                 | [`Size`](../type-aliases/Size.md)\<[`Dimension`](../type-aliases/Dimension.md)\>                       | `undefined`           | Gets the minimum size constraints                                                                                                                 |
| <a id="overflow"></a> `overflow`                               | [`Point`](../type-aliases/Point.md)\<[`Overflow`](../enumerations/Overflow.md)\>                       | `undefined`           | Gets the overflow behavior Controls how content that exceeds the container is handled.                                                            |
| <a id="padding"></a> `padding`                                 | [`Rect`](../type-aliases/Rect.md)\<[`LengthPercentage`](../type-aliases/LengthPercentage.md)\>         | `undefined`           | Gets the padding Inner spacing between the element's border and content.                                                                          |
| <a id="position"></a> `position`                               | [`Position`](../enumerations/Position.md)                                                              | - `Position.Relative` | Gets the position mode Determines how the element is positioned within its parent.                                                                |
| <a id="scrollbarwidth"></a> `scrollbarWidth`                   | `number`                                                                                               | - `0`                 | Gets the scrollbar width The width of the scrollbar gutter when `overflow` is set to `Scroll`.                                                    |
| <a id="size"></a> `size`                                       | [`Size`](../type-aliases/Size.md)\<[`Dimension`](../type-aliases/Dimension.md)\>                       | `undefined`           | Gets the size (width and height)                                                                                                                  |
| <a id="textalign"></a> `textAlign`                             | [`TextAlign`](../enumerations/TextAlign.md)                                                            | - `TextAlign.Auto`    | Gets the text-align property Used by block layout to implement legacy text alignment behavior.                                                    |

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
