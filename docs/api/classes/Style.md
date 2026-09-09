# Style

CSS layout configuration for a node, including flexbox, sizing, spacing, and alignment properties.

This class holds the supported layout properties for a node. Create an instance with
`new Style()` and configure properties before passing to `TaffyTree.newLeaf()`.

## Default Value

A new style uses these layout defaults:

- `display`: `Display.Flex`
- `position`: `Position.Relative`
- `direction`: `Direction.Ltr`; `float`: `Float.None`; `clear`: `Clear.None`
- `boxSizing`: `BoxSizing.BorderBox`; both overflow axes: `Overflow.Visible`
- `flexDirection`: `FlexDirection.Row`
- `flexWrap`: `FlexWrap.NoWrap`
- `flexGrow`: `0`
- `flexShrink`: `1`
- `flexBasis`: `"auto"`; `aspectRatio`: `undefined`
- `alignSelf` and `justifySelf`: `AlignSelf.Auto` (`get()` returns `undefined`)
- Other alignment properties: `undefined` (use default behavior)
- Preferred, minimum, and maximum dimensions: `"auto"`
- Margin, padding, border, and gap: `0`; inset: `"auto"`
- `scrollbarWidth`: `0`; `itemIsTable` and `itemIsReplaced`: `false`
- `textAlign`: `TextAlign.Auto`; `gridAutoFlow`: `GridAutoFlow.Row`
- Grid row/column placements: `"auto"`; track, area, and line-name arrays: `[]`
- Grid template area row and column counts: `0`

## Constructors

### Constructor

```ts
new Style(props?): Style;
```

Creates a new Style instance with default values

#### Parameters

| Parameter | Type  | Description                                   |
| --------- | ----- | --------------------------------------------- |
| `props?`  | `any` | Optional object with initial style properties |

#### Returns

`Style`

- A new `Style` object using the layout defaults and supplied overrides

#### Example

```typescript
// Create with defaults
const style = new Style();
console.log(style.display); // Display.Flex

// Create with initial properties
const style2 = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Column,
  width: 200,
  marginLeft: 10,
});
```

## Properties

| Property                                                               | Type                                                                                                   | Description                                                                                                                                                                                                                                                        |
| ---------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| <a id="aligncontent"></a> `alignContent`                               | [`AlignContent`](../enumerations/AlignContent.md) \| `undefined`                                       | Gets the align-content property Distributes wrapped flex lines, aligns grid rows, or vertically aligns block content, depending on the container's display mode.                                                                                                   |
| <a id="alignitems"></a> `alignItems`                                   | [`AlignItems`](../enumerations/AlignItems.md) \| `undefined`                                           | Gets the align-items property Defines the default alignment for all children on the cross axis.                                                                                                                                                                    |
| <a id="alignself"></a> `alignSelf`                                     | [`AlignSelf`](../enumerations/AlignSelf.md) \| `undefined`                                             | Gets the align-self property Overrides the parent's align-items for this specific element. **Remarks** Unlike this accessor, `get("alignSelf")` returns `undefined` when unset or assigned `AlignSelf.Auto`.                                                       |
| <a id="aspectratio"></a> `aspectRatio`                                 | `number` \| `undefined`                                                                                | Gets the aspect ratio The ratio of width to height. Used to maintain proportions.                                                                                                                                                                                  |
| <a id="border"></a> `border`                                           | [`Rect`](../type-aliases/Rect.md)\<[`LengthPercentage`](../type-aliases/LengthPercentage.md)\>         | Gets the border width Width of the element's border on each side.                                                                                                                                                                                                  |
| <a id="borderbottom"></a> `borderBottom`                               | `any`                                                                                                  | Gets the bottom border width                                                                                                                                                                                                                                       |
| <a id="borderleft"></a> `borderLeft`                                   | `any`                                                                                                  | Gets the left border width                                                                                                                                                                                                                                         |
| <a id="borderright"></a> `borderRight`                                 | `any`                                                                                                  | Gets the right border width                                                                                                                                                                                                                                        |
| <a id="bordertop"></a> `borderTop`                                     | `any`                                                                                                  | Gets the top border width                                                                                                                                                                                                                                          |
| <a id="bottom"></a> `bottom`                                           | `any`                                                                                                  | Gets the bottom inset offset                                                                                                                                                                                                                                       |
| <a id="boxsizing"></a> `boxSizing`                                     | [`BoxSizing`](../enumerations/BoxSizing.md)                                                            | Gets the box sizing mode Determines whether padding and border are included in dimensions.                                                                                                                                                                         |
| <a id="clear"></a> `clear`                                             | [`Clear`](../enumerations/Clear.md)                                                                    | Gets which preceding floats this box must clear in block layout.                                                                                                                                                                                                   |
| <a id="columngap"></a> `columnGap`                                     | `any`                                                                                                  | Gets the column gap (horizontal spacing between items)                                                                                                                                                                                                             |
| <a id="direction"></a> `direction`                                     | [`Direction`](../enumerations/Direction.md)                                                            | Gets the writing direction used for logical layout.                                                                                                                                                                                                                |
| <a id="display"></a> `display`                                         | [`Display`](../enumerations/Display.md)                                                                | Gets the display mode Determines the layout algorithm used for this element and its children.                                                                                                                                                                      |
| <a id="flexbasis"></a> `flexBasis`                                     | [`Dimension`](../type-aliases/Dimension.md)                                                            | Gets the flex-basis The initial size of a flex item before growing/shrinking.                                                                                                                                                                                      |
| <a id="flexdirection"></a> `flexDirection`                             | [`FlexDirection`](../enumerations/FlexDirection.md)                                                    | Gets the flex direction Defines the main axis direction for flex items.                                                                                                                                                                                            |
| <a id="flexgrow"></a> `flexGrow`                                       | `number`                                                                                               | Gets the flex grow factor Determines how much the item grows relative to siblings when there is extra space available.                                                                                                                                             |
| <a id="flexshrink"></a> `flexShrink`                                   | `number`                                                                                               | Gets the flex shrink factor Determines how much the item shrinks relative to siblings when there is insufficient space.                                                                                                                                            |
| <a id="flexwrap"></a> `flexWrap`                                       | [`FlexWrap`](../enumerations/FlexWrap.md)                                                              | Gets the flex wrap mode Controls whether flex items wrap to new lines.                                                                                                                                                                                             |
| <a id="float"></a> `float`                                             | [`Float`](../enumerations/Float.md)                                                                    | Gets whether and where this box floats in block layout.                                                                                                                                                                                                            |
| <a id="gap"></a> `gap`                                                 | [`Size`](../type-aliases/Size.md)\<[`LengthPercentage`](../type-aliases/LengthPercentage.md)\>         | Gets the gap Spacing between flex/grid items.                                                                                                                                                                                                                      |
| <a id="gridautocolumns"></a> `gridAutoColumns`                         | [`TrackSizingFunction`](../type-aliases/TrackSizingFunction.md)[]                                      | Gets the grid-auto-columns property Defines the size of implicitly created columns.                                                                                                                                                                                |
| <a id="gridautoflow"></a> `gridAutoFlow`                               | [`GridAutoFlow`](../enumerations/GridAutoFlow.md)                                                      | Gets the grid-auto-flow property Controls how auto-placed items are inserted into the grid.                                                                                                                                                                        |
| <a id="gridautorows"></a> `gridAutoRows`                               | [`TrackSizingFunction`](../type-aliases/TrackSizingFunction.md)[]                                      | Gets the grid-auto-rows property Defines the size of implicitly created rows.                                                                                                                                                                                      |
| <a id="gridcolumn"></a> `gridColumn`                                   | [`Line`](../type-aliases/Line.md)\<[`GridPlacement`](../type-aliases/GridPlacement.md)\>               | Gets the grid-column property Defines which column in the grid the item should start and end at. Corresponds to CSS `grid-column` shorthand.                                                                                                                       |
| <a id="gridcolumnend"></a> `gridColumnEnd`                             | `any`                                                                                                  | Gets the grid-column-end property                                                                                                                                                                                                                                  |
| <a id="gridcolumnstart"></a> `gridColumnStart`                         | `any`                                                                                                  | Gets the grid-column-start property                                                                                                                                                                                                                                |
| <a id="gridrow"></a> `gridRow`                                         | [`Line`](../type-aliases/Line.md)\<[`GridPlacement`](../type-aliases/GridPlacement.md)\>               | Gets the grid-row property Defines which row in the grid the item should start and end at. Corresponds to CSS `grid-row` shorthand.                                                                                                                                |
| <a id="gridrowend"></a> `gridRowEnd`                                   | `any`                                                                                                  | Gets the grid-row-end property                                                                                                                                                                                                                                     |
| <a id="gridrowstart"></a> `gridRowStart`                               | `any`                                                                                                  | Gets the grid-row-start property                                                                                                                                                                                                                                   |
| <a id="gridtemplateareacolumncount"></a> `gridTemplateAreaColumnCount` | `number`                                                                                               | Gets the effective column count of the grid-template-areas template. This may exceed the extent of the named areas when the template has trailing or entirely unnamed (`.`) cells. It is never smaller than the largest named area's ending column line minus one. |
| <a id="gridtemplatearearowcount"></a> `gridTemplateAreaRowCount`       | `number`                                                                                               | Gets the effective row count of the grid-template-areas template. This may exceed the extent of the named areas when the template has trailing or entirely unnamed (`.`) cells. It is never smaller than the largest named area's ending row line minus one.       |
| <a id="gridtemplateareas"></a> `gridTemplateAreas`                     | [`GridTemplateArea`](../type-aliases/GridTemplateArea.md)[]                                            | Gets the grid-template-areas property Defines named grid areas that can be referenced by grid items.                                                                                                                                                               |
| <a id="gridtemplatecolumnnames"></a> `gridTemplateColumnNames`         | `string`[][]                                                                                           | Gets the grid-template-column-names property Defines the named lines between the columns.                                                                                                                                                                          |
| <a id="gridtemplatecolumns"></a> `gridTemplateColumns`                 | [`GridTemplateComponent`](../type-aliases/GridTemplateComponent.md)[]                                  | Gets the grid-template-columns property Defines the track sizing functions (widths) of the grid columns.                                                                                                                                                           |
| <a id="gridtemplaterownames"></a> `gridTemplateRowNames`               | `string`[][]                                                                                           | Gets the grid-template-row-names property Defines the named lines between the rows.                                                                                                                                                                                |
| <a id="gridtemplaterows"></a> `gridTemplateRows`                       | [`GridTemplateComponent`](../type-aliases/GridTemplateComponent.md)[]                                  | Gets the grid-template-rows property Defines the track sizing functions (heights) of the grid rows.                                                                                                                                                                |
| <a id="height"></a> `height`                                           | [`Dimension`](../type-aliases/Dimension.md)                                                            | Gets the height                                                                                                                                                                                                                                                    |
| <a id="inset"></a> `inset`                                             | [`Rect`](../type-aliases/Rect.md)\<[`LengthPercentageAuto`](../type-aliases/LengthPercentageAuto.md)\> | Gets the inset Positioning offsets for absolutely positioned elements.                                                                                                                                                                                             |
| <a id="itemisreplaced"></a> `itemIsReplaced`                           | `boolean`                                                                                              | Gets whether this item is a replaced element Replaced elements have special sizing behavior (e.g., `<img>`, `<video>`).                                                                                                                                            |
| <a id="itemistable"></a> `itemIsTable`                                 | `boolean`                                                                                              | Gets whether this item is a table Table children are handled specially in block layout.                                                                                                                                                                            |
| <a id="justifycontent"></a> `justifyContent`                           | [`JustifyContent`](../enumerations/JustifyContent.md) \| `undefined`                                   | Gets the justify-content property Defines alignment and spacing of items along the main axis.                                                                                                                                                                      |
| <a id="justifyitems"></a> `justifyItems`                               | [`AlignItems`](../enumerations/AlignItems.md) \| `undefined`                                           | Gets the justify-items property Defines the default justify-self for all children in the inline axis. This is primarily used for CSS Grid layout.                                                                                                                  |
| <a id="justifyself"></a> `justifySelf`                                 | [`AlignSelf`](../enumerations/AlignSelf.md) \| `undefined`                                             | Gets the justify-self property Overrides the parent's justify-items for this specific element in the inline axis. **Remarks** Unlike this accessor, `get("justifySelf")` returns `undefined` when unset or assigned `AlignSelf.Auto`.                              |
| <a id="left"></a> `left`                                               | `any`                                                                                                  | Gets the left inset offset                                                                                                                                                                                                                                         |
| <a id="margin"></a> `margin`                                           | [`Rect`](../type-aliases/Rect.md)\<[`LengthPercentageAuto`](../type-aliases/LengthPercentageAuto.md)\> | Gets the margin Outer spacing around the element.                                                                                                                                                                                                                  |
| <a id="marginbottom"></a> `marginBottom`                               | `any`                                                                                                  | Gets the bottom margin                                                                                                                                                                                                                                             |
| <a id="marginleft"></a> `marginLeft`                                   | `any`                                                                                                  | Gets the left margin                                                                                                                                                                                                                                               |
| <a id="marginright"></a> `marginRight`                                 | `any`                                                                                                  | Gets the right margin                                                                                                                                                                                                                                              |
| <a id="margintop"></a> `marginTop`                                     | `any`                                                                                                  | Gets the top margin                                                                                                                                                                                                                                                |
| <a id="maxheight"></a> `maxHeight`                                     | [`Dimension`](../type-aliases/Dimension.md)                                                            | Gets the maximum height                                                                                                                                                                                                                                            |
| <a id="maxsize"></a> `maxSize`                                         | [`Size`](../type-aliases/Size.md)\<[`Dimension`](../type-aliases/Dimension.md)\>                       | Gets the maximum size constraints                                                                                                                                                                                                                                  |
| <a id="maxwidth"></a> `maxWidth`                                       | [`Dimension`](../type-aliases/Dimension.md)                                                            | Gets the maximum width                                                                                                                                                                                                                                             |
| <a id="minheight"></a> `minHeight`                                     | [`Dimension`](../type-aliases/Dimension.md)                                                            | Gets the minimum height                                                                                                                                                                                                                                            |
| <a id="minsize"></a> `minSize`                                         | [`Size`](../type-aliases/Size.md)\<[`Dimension`](../type-aliases/Dimension.md)\>                       | Gets the minimum size constraints                                                                                                                                                                                                                                  |
| <a id="minwidth"></a> `minWidth`                                       | [`Dimension`](../type-aliases/Dimension.md)                                                            | Gets the minimum width                                                                                                                                                                                                                                             |
| <a id="overflow"></a> `overflow`                                       | [`Point`](../type-aliases/Point.md)\<[`Overflow`](../enumerations/Overflow.md)\>                       | Gets the overflow behavior Controls how content that exceeds the container is handled.                                                                                                                                                                             |
| <a id="overflowx"></a> `overflowX`                                     | [`Overflow`](../enumerations/Overflow.md)                                                              | Gets the horizontal overflow behavior                                                                                                                                                                                                                              |
| <a id="overflowy"></a> `overflowY`                                     | [`Overflow`](../enumerations/Overflow.md)                                                              | Gets the vertical overflow behavior                                                                                                                                                                                                                                |
| <a id="padding"></a> `padding`                                         | [`Rect`](../type-aliases/Rect.md)\<[`LengthPercentage`](../type-aliases/LengthPercentage.md)\>         | Gets the padding Inner spacing between the element's border and content.                                                                                                                                                                                           |
| <a id="paddingbottom"></a> `paddingBottom`                             | `any`                                                                                                  | Gets the bottom padding                                                                                                                                                                                                                                            |
| <a id="paddingleft"></a> `paddingLeft`                                 | `any`                                                                                                  | Gets the left padding                                                                                                                                                                                                                                              |
| <a id="paddingright"></a> `paddingRight`                               | `any`                                                                                                  | Gets the right padding                                                                                                                                                                                                                                             |
| <a id="paddingtop"></a> `paddingTop`                                   | `any`                                                                                                  | Gets the top padding                                                                                                                                                                                                                                               |
| <a id="position"></a> `position`                                       | [`Position`](../enumerations/Position.md)                                                              | Gets the position mode Determines how the element is positioned within its parent.                                                                                                                                                                                 |
| <a id="right"></a> `right`                                             | `any`                                                                                                  | Gets the right inset offset                                                                                                                                                                                                                                        |
| <a id="rowgap"></a> `rowGap`                                           | `any`                                                                                                  | Gets the row gap (vertical spacing between items)                                                                                                                                                                                                                  |
| <a id="scrollbarwidth"></a> `scrollbarWidth`                           | `number`                                                                                               | Gets the scrollbar width The width of the scrollbar gutter when `overflow` is set to `Scroll`.                                                                                                                                                                     |
| <a id="size"></a> `size`                                               | [`Size`](../type-aliases/Size.md)\<[`Dimension`](../type-aliases/Dimension.md)\>                       | Gets the size (width and height)                                                                                                                                                                                                                                   |
| <a id="textalign"></a> `textAlign`                                     | [`TextAlign`](../enumerations/TextAlign.md)                                                            | Gets the text-align property Used by block layout to implement legacy text alignment behavior.                                                                                                                                                                     |
| <a id="top"></a> `top`                                                 | `any`                                                                                                  | Gets the top inset offset                                                                                                                                                                                                                                          |
| <a id="width"></a> `width`                                             | [`Dimension`](../type-aliases/Dimension.md)                                                            | Gets the width                                                                                                                                                                                                                                                     |

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
get(): undefined;
```

Reads multiple style properties in a single WASM call.
Supports both object properties and individual flat properties.

##### Returns

`undefined`

A single value for one key, values in key order for multiple keys,
or `undefined` when no keys are supplied

##### Throws

Error if any property key is unknown.

##### Remarks

- Multiple literal keys produce a typed tuple for destructuring
- A dynamic array of keys produces an array of the corresponding value types
- `get("alignSelf")` and `get("justifySelf")` return `undefined` when unset
  or assigned `AlignSelf.Auto`; direct property reads return `AlignSelf.Auto`

##### Example

```typescript
const style = new Style();
style.display = Display.Flex;

// Single property - returns exact type (includes undefined for optional properties)
const display = style.get("display"); // Display | undefined

// Individual flat property - returns exact type
const width = style.get("width"); // Dimension | undefined

// Optional properties return undefined when not set
const alignItems = style.get("alignItems"); // AlignItems | undefined

// Two properties - returns tuple for destructuring
const [d, w] = style.get("display", "width"); // [Display | undefined, Dimension | undefined]

// Three properties - returns tuple for destructuring
const [d2, w2, f] = style.get("display", "width", "flexGrow");

// Four literal keys also return a typed tuple
const values = style.get("display", "width", "flexGrow", "flexShrink");
// values: [Display | undefined, Dimension | undefined, number | undefined, number | undefined]
```

#### Call Signature

```ts
get<K>(...keys): StylePropertyValues[K];
```

##### Type Parameters

| Type Parameter                                                    |
| ----------------------------------------------------------------- |
| `K` _extends_ [`StyleProperty`](../type-aliases/StyleProperty.md) |

##### Parameters

| Parameter | Type    |
| --------- | ------- |
| ...`keys` | \[`K`\] |

##### Returns

[`StylePropertyValues`](../type-aliases/StylePropertyValues.md)\[`K`\]

#### Call Signature

```ts
get<K1, K2>(...keys): [StylePropertyValues[K1], StylePropertyValues[K2]];
```

##### Type Parameters

| Type Parameter                                                     |
| ------------------------------------------------------------------ |
| `K1` _extends_ [`StyleProperty`](../type-aliases/StyleProperty.md) |
| `K2` _extends_ [`StyleProperty`](../type-aliases/StyleProperty.md) |

##### Parameters

| Parameter | Type           |
| --------- | -------------- |
| ...`keys` | \[`K1`, `K2`\] |

##### Returns

\[[`StylePropertyValues`](../type-aliases/StylePropertyValues.md)\[`K1`\], [`StylePropertyValues`](../type-aliases/StylePropertyValues.md)\[`K2`\]\]

#### Call Signature

```ts
get<K1, K2, K3>(...keys): [StylePropertyValues[K1], StylePropertyValues[K2], StylePropertyValues[K3]];
```

##### Type Parameters

| Type Parameter                                                     |
| ------------------------------------------------------------------ |
| `K1` _extends_ [`StyleProperty`](../type-aliases/StyleProperty.md) |
| `K2` _extends_ [`StyleProperty`](../type-aliases/StyleProperty.md) |
| `K3` _extends_ [`StyleProperty`](../type-aliases/StyleProperty.md) |

##### Parameters

| Parameter | Type                 |
| --------- | -------------------- |
| ...`keys` | \[`K1`, `K2`, `K3`\] |

##### Returns

\[[`StylePropertyValues`](../type-aliases/StylePropertyValues.md)\[`K1`\], [`StylePropertyValues`](../type-aliases/StylePropertyValues.md)\[`K2`\], [`StylePropertyValues`](../type-aliases/StylePropertyValues.md)\[`K3`\]\]

#### Call Signature

```ts
get<Keys>(...keys): StylePropertyArrayValues<Keys>;
```

##### Type Parameters

| Type Parameter                                                         |
| ---------------------------------------------------------------------- |
| `Keys` _extends_ [`StyleProperty`](../type-aliases/StyleProperty.md)[] |

##### Parameters

| Parameter | Type   |
| --------- | ------ |
| ...`keys` | `Keys` |

##### Returns

[`StylePropertyArrayValues`](../type-aliases/StylePropertyArrayValues.md)\<`Keys`\>

---

### set()

```ts
set(props): void;
```

Sets multiple style properties in a single WASM call.
Supports both object properties and individual flat properties.

#### Parameters

| Parameter | Type                                                            | Description                                  |
| --------- | --------------------------------------------------------------- | -------------------------------------------- |
| `props`   | [`StylePropertyValues`](../type-aliases/StylePropertyValues.md) | Object mapping property keys to their values |

#### Returns

`void`

#### Remarks

Only accepts valid property keys with their corresponding value types.

#### Throws

Error if any property key is unknown.

#### Example

```typescript
const style = new Style();
style.set({
  display: Display.Flex,
  width: 200,
  marginLeft: 10,
  marginRight: "auto",
});
```
