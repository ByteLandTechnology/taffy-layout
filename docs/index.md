**Taffy Layout API Documentation**

---

# Taffy Layout API Documentation

## Enumerations

| Enumeration                                      | Description                                           |
| ------------------------------------------------ | ----------------------------------------------------- |
| [AlignContent](enumerations/AlignContent.md)     | Multi-line content alignment enumeration              |
| [AlignItems](enumerations/AlignItems.md)         | Cross-axis alignment enumeration for all children     |
| [AlignSelf](enumerations/AlignSelf.md)           | Cross-axis alignment enumeration for a single element |
| [BoxSizing](enumerations/BoxSizing.md)           | Box sizing enumeration                                |
| [Display](enumerations/Display.md)               | Display mode enumeration                              |
| [FlexDirection](enumerations/FlexDirection.md)   | Flex direction enumeration                            |
| [FlexWrap](enumerations/FlexWrap.md)             | Flex wrap mode enumeration                            |
| [GridAutoFlow](enumerations/GridAutoFlow.md)     | Grid auto flow enumeration                            |
| [JustifyContent](enumerations/JustifyContent.md) | Main axis alignment enumeration                       |
| [Overflow](enumerations/Overflow.md)             | Overflow handling enumeration                         |
| [Position](enumerations/Position.md)             | Position mode enumeration                             |
| [TextAlign](enumerations/TextAlign.md)           | Text alignment enumeration (for block layout)         |

## Classes

| Class                               | Description                                                                                               |
| ----------------------------------- | --------------------------------------------------------------------------------------------------------- |
| [Layout](classes/Layout.md)         | Computed layout result containing position, size, and spacing values for a node.                          |
| [Style](classes/Style.md)           | CSS layout configuration for a node, including flexbox, sizing, spacing, and alignment properties.        |
| [TaffyError](classes/TaffyError.md) | Error class thrown when a Taffy operation fails, containing a human-readable error message.               |
| [TaffyTree](classes/TaffyTree.md)   | The main layout tree class for creating nodes, computing layouts, and managing a tree of styled elements. |

## Interfaces

| Interface                              | Description |
| -------------------------------------- | ----------- |
| [InitOutput](interfaces/InitOutput.md) | -           |

## Type Aliases

| Type Alias                                                           | Description                                                                                                                                                    |
| -------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [AvailableSpace](type-aliases/AvailableSpace.md)                     | Available space constraint for layout computation.                                                                                                             |
| [DetailedGridInfo](type-aliases/DetailedGridInfo.md)                 | Detailed information about a grid layout.                                                                                                                      |
| [DetailedGridItemsInfo](type-aliases/DetailedGridItemsInfo.md)       | Information about a grid item's placement.                                                                                                                     |
| [DetailedGridTracksInfo](type-aliases/DetailedGridTracksInfo.md)     | Information about grid tracks (rows or columns).                                                                                                               |
| [DetailedLayoutInfo](type-aliases/DetailedLayoutInfo.md)             | Detailed layout information (for grid layouts).                                                                                                                |
| [Dimension](type-aliases/Dimension.md)                               | Dimension type supporting length, percentage, or auto values.                                                                                                  |
| [GridPlacement](type-aliases/GridPlacement.md)                       | Grid placement type for positioning grid items.                                                                                                                |
| [GridTemplateArea](type-aliases/GridTemplateArea.md)                 | Named grid area definition.                                                                                                                                    |
| [GridTemplateComponent](type-aliases/GridTemplateComponent.md)       | Grid track sizing definition.                                                                                                                                  |
| [GridTemplateRepetition](type-aliases/GridTemplateRepetition.md)     | Grid track repetition definition.                                                                                                                              |
| [InitInput](type-aliases/InitInput.md)                               | -                                                                                                                                                              |
| [LengthPercentage](type-aliases/LengthPercentage.md)                 | Length or percentage value (no auto support).                                                                                                                  |
| [LengthPercentageAuto](type-aliases/LengthPercentageAuto.md)         | Length, percentage, or auto value.                                                                                                                             |
| [Line](type-aliases/Line.md)                                         | Line type representing start and end positions.                                                                                                                |
| [MaxTrackSizingFunction](type-aliases/MaxTrackSizingFunction.md)     | Maximum track sizing function.                                                                                                                                 |
| [MeasureFunction](type-aliases/MeasureFunction.md)                   | Custom measure function for leaf nodes with text or other dynamic content.                                                                                     |
| [MinTrackSizingFunction](type-aliases/MinTrackSizingFunction.md)     | Minumum track sizing function.                                                                                                                                 |
| [Point](type-aliases/Point.md)                                       | Point with x and y coordinates/values.                                                                                                                         |
| [Rect](type-aliases/Rect.md)                                         | Rectangle with left, right, top, and bottom values.                                                                                                            |
| [RepetitionCount](type-aliases/RepetitionCount.md)                   | Grid track repetition parameter.                                                                                                                               |
| [Size](type-aliases/Size.md)                                         | Generic size type with width and height.                                                                                                                       |
| [StyleProperty](type-aliases/StyleProperty.md)                       | Valid property keys for Style.get() method.                                                                                                                    |
| [StylePropertyArrayValues](type-aliases/StylePropertyArrayValues.md) | Helper type to convert an array of property keys to an array of their value types. Unlike `TupleToStyleValues`, this returns an array type instead of a tuple. |
| [StylePropertyValues](type-aliases/StylePropertyValues.md)           | Type-safe property values for batch setting.                                                                                                                   |
| [SyncInitInput](type-aliases/SyncInitInput.md)                       | -                                                                                                                                                              |
| [TrackSizingFunction](type-aliases/TrackSizingFunction.md)           | Track sizing function (min/max pair).                                                                                                                          |

## Functions

| Function                          | Description                                                                                                                       |
| --------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| [default](functions/default.md)   | If `module_or_path` is {RequestInfo} or {URL}, makes a request and for everything else, calls `WebAssembly.instantiate` directly. |
| [initSync](functions/initSync.md) | Instantiates the given `module`, which can either be bytes or a precompiled `WebAssembly.Module`.                                 |
