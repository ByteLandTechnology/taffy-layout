# Taffy WebAssembly Bindings

> **High-performance Flexbox and CSS Grid layout for JavaScript/TypeScript, powered by Rust and WebAssembly.**

![License](https://img.shields.io/npm/l/taffy-js?style=flat-square) ![Version](https://img.shields.io/npm/v/taffy-js?style=flat-square) ![WASM](https://img.shields.io/badge/platform-wasm-blueviolet?style=flat-square)

**Taffy** is a generic, high-performance UI layout library written in Rust. This package (`taffy-js`) provides WebAssembly bindings, allowing you to use Taffy's standards-compliant Flexbox and Grid algorithms directly in your web or Node.js applications with near-native performance.

## ✨ Features

- **🚀 High Performance**: Leverages the speed of Rust and WebAssembly for complex layout computations.
- **📦 Tiny Footprint**: Highly optimized WASM binary size.
- **🎨 Modern Layouts**: Full support for **Flexbox** and **CSS Grid** specifications.
- **🛠 Framework Agnostic**: Use it with React, Vue, Svelte, vanilla JS, or even in Node.js for server-side layout calculation.
- **🔒 Type-Safe**: Fully typed API with TypeScript definitions included.
- **🐛 Debug Mode**: Optional verbose logging for debugging layout issues.

## 📦 Installation

```bash
npm install taffy-js
```

> **Note**: This package relies on WebAssembly. Ensure your runtime (modern browser or Node.js) supports WASM.

## 🚀 Quick Start

### Functional API

```typescript
import init, {
  new_leaf,
  new_with_children,
  compute_layout,
  get_layout,
  Display,
  FlexDirection,
  AlignItems,
  JustifyContent,
} from "taffy-js";

async function run() {
  // 1. Initialize the WASM module
  await init();

  // 2. Define Styles
  const boxStyle = {
    display: Display.Flex,
    width: { value: 100, unit: "Pixels" },
    height: { value: 100, unit: "Pixels" },
    justify_content: JustifyContent.Center,
    align_items: AlignItems.Center,
  };

  const rootStyle = {
    display: Display.Flex,
    width: { value: 500, unit: "Pixels" },
    height: { value: 500, unit: "Pixels" },
    flex_direction: FlexDirection.Row,
    justify_content: JustifyContent.SpaceAround,
    align_items: AlignItems.Center,
  };

  // 3. Create Tree Nodes
  const child1 = new_leaf(boxStyle);
  const child2 = new_leaf(boxStyle);
  const root = new_with_children(rootStyle, [child1, child2]);

  // 4. Compute Layout
  compute_layout(root, { width: 500, height: 500 });

  // 5. Retrieve Results
  const rootLayout = get_layout(root);
  const child1Layout = get_layout(child1);
  const child2Layout = get_layout(child2);

  console.log("Root:", rootLayout); // { x: 0, y: 0, width: 500, height: 500 }
  console.log("Child 1:", child1Layout); // { x: ..., y: ..., width: 100, height: 100 }
  console.log("Child 2:", child2Layout); // { x: ..., y: ..., width: 100, height: 100 }
}

run();
```

### Object-Oriented API (Yoga-like)

The library also provides an OO wrapper for a more intuitive, Yoga-style experience:

```typescript
import init, {
  TaffyNode,
  Display,
  FlexDirection,
  Edge,
  Gutter,
} from "taffy-js";

async function run() {
  await init();

  // Create nodes using the OO API
  const root = new TaffyNode({});
  root.setDisplay(Display.Flex);
  root.setFlexDirection(FlexDirection.Column);
  root.setWidth(400);
  root.setHeight(400);
  root.setPadding(Edge.All, 10);
  root.setGap(Gutter.Row, 8);

  const child1 = new TaffyNode({});
  child1.setHeight(100);
  child1.setFlexGrow(1);

  const child2 = new TaffyNode({});
  child2.setHeight(50);

  // Build the tree
  root.addChild(child1);
  root.addChild(child2);

  // Compute layout
  root.computeLayout({ width: 400, height: 400 });

  // Get results
  const layout = root.getLayout();
  console.log("Root layout:", layout);
  console.log("Child1 layout:", child1.getLayout());
  console.log("Child2 layout:", child2.getLayout());

  // Cleanup (optional - nodes are automatically freed when dropped)
  root.free();
}

run();
```

## 📐 Architecture

`taffy-js` acts as a thin wrapper around the [Taffy](https://github.com/DioxusLabs/taffy) Rust crate.

1.  **Node Tree**: You build a flat tree of nodes in the WASM memory space using integer IDs (`u64`).
2.  **Style Transfer**: Styles are serialized from JS objects to Rust structs via `serde-wasm-bindgen`.
3.  **Computation**: Rust runs the layout algorithms (Flexbox/Grid/Block).
4.  **Readout**: You query the final computed geometry (x, y, width, height) back to JS.

## 📚 API Reference

### Lifecycle

| Function  | Description                                                                     |
| :-------- | :------------------------------------------------------------------------------ |
| `init()`  | Initializes the WASM module. Must be awaited before calling any other function. |
| `clear()` | Clears all nodes from the layout tree, resetting to empty state.                |

### Node Management (Functional API)

| Function            | Signature                                      | Description                             |
| :------------------ | :--------------------------------------------- | :-------------------------------------- |
| `new_leaf`          | `(style: Style) -> number`                     | Creates a leaf node (no children).      |
| `new_with_children` | `(style: Style, children: number[]) -> number` | Creates a node containing child nodes.  |
| `add_child`         | `(parent: number, child: number) -> void`      | Appends a child to a parent.            |
| `remove_child`      | `(parent: number, child: number) -> void`      | Removes a specific child from a parent. |
| `set_children`      | `(parent: number, children: number[]) -> void` | Replaces all children of a node.        |
| `remove_node`       | `(node: number) -> void`                       | Deletes a node and frees its memory.    |
| `get_children`      | `(parent: number) -> number[]`                 | Returns an array of child node IDs.     |
| `get_parent`        | `(node: number) -> number \| null`             | Returns the parent node ID, or null.    |

### Layout & Style (Functional API)

| Function         | Signature                                       | Description                                          |
| :--------------- | :---------------------------------------------- | :--------------------------------------------------- |
| `set_style`      | `(node: number, style: Style) -> void`          | Updates the style properties of a node.              |
| `compute_layout` | `(root: number, space: AvailableSpace) -> void` | Triggers the layout calculation algorithm.           |
| `get_layout`     | `(node: number) -> Layout`                      | Returns `{x, y, width, height}` for a node.          |
| `mark_dirty`     | `(node: number) -> void`                        | Manually marks a node as dirty, requiring re-layout. |
| `node_count`     | `() -> number`                                  | Returns the total number of nodes in the tree.       |

### Dimension Helpers

| Function    | Signature                                           | Description                             |
| :---------- | :-------------------------------------------------- | :-------------------------------------- |
| `px`        | `(value: number) -> Dimension`                      | Creates a pixel dimension.              |
| `percent`   | `(value: number) -> Dimension`                      | Creates a percentage dimension.         |
| `auto`      | `() -> Dimension`                                   | Creates an auto dimension.              |
| `dimension` | `(value: number, unit: DimensionUnit) -> Dimension` | Creates a dimension with explicit unit. |

### TaffyNode Class (OO API)

The `TaffyNode` class provides a Yoga-like object-oriented interface:

#### Constructor & Lifecycle

| Method                 | Description                                                   |
| :--------------------- | :------------------------------------------------------------ |
| `new TaffyNode(style)` | Creates a new node with the given style.                      |
| `free()`               | Explicitly frees the node. Nodes are also auto-freed on drop. |

#### Style Methods

| Method                        | Description                                    |
| :---------------------------- | :--------------------------------------------- |
| `setStyle(style)`             | Updates the full style object.                 |
| `setDisplay(display)`         | Sets display mode (Flex, Grid, Block, None).   |
| `setPositionType(position)`   | Sets positioning (Relative, Absolute).         |
| `setFlexDirection(direction)` | Sets flex direction (Row, Column, etc.).       |
| `setFlexWrap(wrap)`           | Sets flex wrap behavior.                       |
| `setAlignItems(align)`        | Sets cross-axis alignment for children.        |
| `setAlignSelf(align)`         | Sets cross-axis alignment for this item.       |
| `setJustifyContent(justify)`  | Sets main-axis alignment.                      |
| `setFlexGrow(value)`          | Sets the flex grow factor.                     |
| `setFlexShrink(value)`        | Sets the flex shrink factor.                   |
| `setFlexBasis(value)`         | Sets flex basis in pixels.                     |
| `setFlexBasisPercent(value)`  | Sets flex basis as percentage.                 |
| `setFlexBasisAuto()`          | Sets flex basis to auto.                       |
| `setWidth(value)`             | Sets width in pixels.                          |
| `setWidthPercent(value)`      | Sets width as percentage.                      |
| `setWidthAuto()`              | Sets width to auto.                            |
| `setHeight(value)`            | Sets height in pixels.                         |
| `setHeightPercent(value)`     | Sets height as percentage.                     |
| `setHeightAuto()`             | Sets height to auto.                           |
| `setMinWidth(value)`          | Sets minimum width in pixels.                  |
| `setMinWidthPercent(value)`   | Sets minimum width as percentage.              |
| `setMinHeight(value)`         | Sets minimum height in pixels.                 |
| `setMinHeightPercent(value)`  | Sets minimum height as percentage.             |
| `setMargin(edge, value)`      | Sets margin for specified edge(s).             |
| `setMarginAuto(edge)`         | Sets margin to auto for specified edge(s).     |
| `setPadding(edge, value)`     | Sets padding for specified edge(s).            |
| `setBorder(edge, value)`      | Sets border for specified edge(s).             |
| `setGap(gutter, value)`       | Sets gap between children.                     |
| `setMeasureFunc(fn)`          | Sets a custom measure function for leaf nodes. |

#### Tree Methods

| Method                      | Description                               |
| :-------------------------- | :---------------------------------------- |
| `addChild(child)`           | Appends a child node.                     |
| `insertChild(child, index)` | Inserts a child at the specified index.   |
| `removeChild(child)`        | Removes a child node.                     |
| `setChildren(childIds)`     | Replaces all children with the given IDs. |

#### Layout Methods

| Method                          | Description                           |
| :------------------------------ | :------------------------------------ |
| `computeLayout(availableSpace)` | Computes layout for this subtree.     |
| `getLayout()`                   | Returns the computed layout.          |
| `markDirty()`                   | Marks this node as needing re-layout. |

### Type Definitions

#### `Style`

A comprehensive object mirroring CSS properties:

```typescript
interface Style {
  // Display & Position
  display?: Display; // Flex, Grid, Block, None
  position?: Position; // Relative, Absolute

  // Dimensions
  width?: Dimension;
  height?: Dimension;
  min_width?: Dimension;
  min_height?: Dimension;
  max_width?: Dimension;
  max_height?: Dimension;

  // Position offsets (for absolute positioning)
  left?: Dimension;
  right?: Dimension;
  top?: Dimension;
  bottom?: Dimension;

  // Flexbox
  flex_direction?: FlexDirection;
  flex_wrap?: FlexWrap;
  flex_grow?: number;
  flex_shrink?: number;
  flex_basis?: Dimension;
  justify_content?: JustifyContent;
  align_items?: AlignItems;
  align_self?: AlignSelf;
  align_content?: AlignContent;

  // Spacing
  margin_left?: Dimension;
  margin_right?: Dimension;
  margin_top?: Dimension;
  margin_bottom?: Dimension;
  padding_left?: Dimension;
  padding_right?: Dimension;
  padding_top?: Dimension;
  padding_bottom?: Dimension;
  row_gap?: Dimension;
  column_gap?: Dimension;

  // Grid
  grid_template_rows?: TrackDefinition[];
  grid_template_columns?: TrackDefinition[];
  grid_auto_rows?: TrackDefinition[];
  grid_auto_columns?: TrackDefinition[];
  grid_auto_flow?: GridAutoFlow;
  grid_row?: Line;
  grid_column?: Line;
}
```

#### `Dimension`

```typescript
interface Dimension {
  value: number;
  unit: "Pixels" | "Percent" | "Auto";
}
```

#### `AvailableSpace`

Used when triggering layout computation:

```typescript
interface AvailableSpace {
  width?: number; // undefined = unlimited/content-based
  height?: number; // undefined = unlimited/content-based
}
```

#### `Layout`

The computed layout result:

```typescript
interface Layout {
  x: number; // X position relative to parent
  y: number; // Y position relative to parent
  width: number; // Computed width
  height: number; // Computed height
}
```

### Enums

#### Display

- `Display.None` - Hidden, takes no space
- `Display.Flex` - Flexbox container
- `Display.Grid` - Grid container
- `Display.Block` - Block layout

#### Position

- `Position.Static` - Normal flow (treated as Relative)
- `Position.Relative` - Normal flow
- `Position.Absolute` - Removed from flow, positioned relative to containing block

#### FlexDirection

- `FlexDirection.Row` - Horizontal, left to right
- `FlexDirection.Column` - Vertical, top to bottom
- `FlexDirection.RowReverse` - Horizontal, right to left
- `FlexDirection.ColumnReverse` - Vertical, bottom to top

#### JustifyContent

- `JustifyContent.Start`, `FlexStart`
- `JustifyContent.End`, `FlexEnd`
- `JustifyContent.Center`
- `JustifyContent.SpaceBetween`
- `JustifyContent.SpaceAround`
- `JustifyContent.SpaceEvenly`

#### AlignItems / AlignSelf

- `Start`, `End`, `FlexStart`, `FlexEnd`
- `Center`, `Baseline`, `Stretch`
- `AlignSelf.Auto` (inherit from parent)

#### Edge

- `Edge.Left`, `Edge.Right`, `Edge.Top`, `Edge.Bottom`
- `Edge.Start`, `Edge.End` (logical)
- `Edge.Horizontal`, `Edge.Vertical`
- `Edge.All`

#### Gutter

- `Gutter.Column` - Gap between columns
- `Gutter.Row` - Gap between rows
- `Gutter.All` - Both directions

## 🐛 Debug Logging

For development, you can enable verbose console logging by building with the `debug` feature:

```bash
# When building from source
wasm-pack build --features debug
```

When enabled, the library outputs detailed logs to the browser console including:

- Node creation and removal
- Style updates
- Layout computation details
- Error diagnostics

**Note**: Debug logging is disabled by default for optimal performance in production.

## 🛠 Building from Source

If you want to contribute or build the WASM binary yourself:

1.  **Prerequisites**: Install Rust and `wasm-pack`.

    ```bash
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
    ```

2.  **Build** (production):

    ```bash
    npm install
    npm run build
    ```

3.  **Build** (with debug logging):
    ```bash
    wasm-pack build --features debug
    ```

The artifacts will be generated in the `pkg/` directory.

## 📄 License

MIT License © 2024 ByteLand Technology

This project wraps [Taffy](https://github.com/DioxusLabs/taffy), which is also MIT licensed.
