**Taffy-JS API Documentation**

---

# Taffy-JS API Documentation

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
| [JustifyContent](enumerations/JustifyContent.md) | Main axis alignment enumeration                       |
| [Overflow](enumerations/Overflow.md)             | Overflow handling enumeration                         |
| [Position](enumerations/Position.md)             | Position mode enumeration                             |

## Classes

| Class                               | Description                                                                                               |
| ----------------------------------- | --------------------------------------------------------------------------------------------------------- |
| [Layout](classes/Layout.md)         | Computed layout result containing position, size, and spacing values for a node.                          |
| [Style](classes/Style.md)           | CSS layout configuration for a node, including flexbox, sizing, spacing, and alignment properties.        |
| [TaffyError](classes/TaffyError.md) | Error class thrown when a Taffy operation fails, containing a human-readable error message.               |
| [TaffyTree](classes/TaffyTree.md)   | The main layout tree class for creating nodes, computing layouts, and managing a tree of styled elements. |

## Interfaces

| Interface                                                      | Description                                         |
| -------------------------------------------------------------- | --------------------------------------------------- |
| [DetailedGridInfo](interfaces/DetailedGridInfo.md)             | Detailed information about a grid layout.           |
| [DetailedGridItemsInfo](interfaces/DetailedGridItemsInfo.md)   | Information about a grid item's placement.          |
| [DetailedGridTracksInfo](interfaces/DetailedGridTracksInfo.md) | Information about grid tracks (rows or columns).    |
| [InitOutput](interfaces/InitOutput.md)                         | -                                                   |
| [Point](interfaces/Point.md)                                   | Point with x and y coordinates/values.              |
| [Rect](interfaces/Rect.md)                                     | Rectangle with left, right, top, and bottom values. |
| [Size](interfaces/Size.md)                                     | Generic size type with width and height.            |

## Type Aliases

| Type Alias                                                   | Description                                                                |
| ------------------------------------------------------------ | -------------------------------------------------------------------------- |
| [AvailableSpace](type-aliases/AvailableSpace.md)             | Available space constraint for layout computation.                         |
| [DetailedLayoutInfo](type-aliases/DetailedLayoutInfo.md)     | Detailed layout information (for grid layouts).                            |
| [Dimension](type-aliases/Dimension.md)                       | Dimension type supporting length, percentage, or auto values.              |
| [InitInput](type-aliases/InitInput.md)                       | -                                                                          |
| [LengthPercentage](type-aliases/LengthPercentage.md)         | Length or percentage value (no auto support).                              |
| [LengthPercentageAuto](type-aliases/LengthPercentageAuto.md) | Length, percentage, or auto value.                                         |
| [MeasureFunction](type-aliases/MeasureFunction.md)           | Custom measure function for leaf nodes with text or other dynamic content. |
| [SyncInitInput](type-aliases/SyncInitInput.md)               | -                                                                          |

## Functions

| Function                          | Description                                                                                                                       |
| --------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| [default](functions/default.md)   | If `module_or_path` is {RequestInfo} or {URL}, makes a request and for everything else, calls `WebAssembly.instantiate` directly. |
| [initSync](functions/initSync.md) | Instantiates the given `module`, which can either be bytes or a precompiled `WebAssembly.Module`.                                 |
