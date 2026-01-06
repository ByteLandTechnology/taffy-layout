# Taffy-JS

[![npm version](https://badge.fury.io/js/taffy-js.svg)](https://www.npmjs.com/package/taffy-js)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

High-performance WebAssembly bindings for the [Taffy](https://github.com/DioxusLabs/taffy) layout engine, bringing CSS Flexbox and Grid layout algorithms to JavaScript with near-native performance.

## ✨ Features

- **🚀 High Performance**: WebAssembly-powered layout calculations
- **📦 Complete CSS Support**: Full Flexbox and CSS Grid implementation
- **🔧 Custom Measurement**: Support for custom text/content measurement callbacks
- **📝 TypeScript Ready**: Complete type definitions included
- **🌳 Tree-Based API**: Efficient tree structure for complex layouts
- **💡 Familiar API**: CSS-like property names and values

## 📦 Installation

```bash
npm install taffy-js
```

## 🚀 Quick Start

```javascript
import init, {
  TaffyTree,
  Style,
  Display,
  FlexDirection,
  AlignItems,
} from "taffy-js";

async function main() {
  // Initialize WebAssembly module
  await init();

  // Create a layout tree
  const tree = new TaffyTree();

  // Create container style
  const containerStyle = new Style();
  containerStyle.display = Display.Flex;
  containerStyle.flexDirection = FlexDirection.Column;
  containerStyle.alignItems = AlignItems.Center;
  containerStyle.size = { width: { Length: 300 }, height: { Length: 200 } };
  containerStyle.padding = {
    left: { Length: 10 },
    right: { Length: 10 },
    top: { Length: 10 },
    bottom: { Length: 10 },
  };

  // Create child styles
  const childStyle = new Style();
  childStyle.flexGrow = 1;
  childStyle.size = { width: { Percent: 100 }, height: "Auto" };

  // Create nodes
  const child1 = tree.newLeaf(childStyle);
  const child2 = tree.newLeaf(childStyle);
  const container = tree.newWithChildren(
    containerStyle,
    BigUint64Array.from([child1, child2]),
  );

  // Compute layout
  tree.computeLayout(container, {
    width: { Definite: 300 },
    height: { Definite: 200 },
  });

  // Read computed layouts
  const containerLayout = tree.getLayout(container);
  const child1Layout = tree.getLayout(child1);
  const child2Layout = tree.getLayout(child2);

  console.log(`Container: ${containerLayout.width}x${containerLayout.height}`);
  console.log(
    `Child 1: ${child1Layout.width}x${child1Layout.height} at (${child1Layout.x}, ${child1Layout.y})`,
  );
  console.log(
    `Child 2: ${child2Layout.width}x${child2Layout.height} at (${child2Layout.x}, ${child2Layout.y})`,
  );
}

main();
```

## 📖 API Reference

### TaffyTree

The main class for managing layout trees.

```typescript
class TaffyTree {
  // Construction
  constructor();
  static withCapacity(capacity: number): TaffyTree;

  // Node Creation (throws TaffyError on failure)
  newLeaf(style: Style): bigint;
  newLeafWithContext(style: Style, context: any): bigint;
  newWithChildren(style: Style, children: BigUint64Array): bigint;

  // Tree Operations
  clear(): void;
  remove(node: bigint): bigint; // throws TaffyError
  totalNodeCount(): number;

  // Child Management (throws TaffyError on failure)
  addChild(parent: bigint, child: bigint): void;
  removeChild(parent: bigint, child: bigint): bigint;
  setChildren(parent: bigint, children: BigUint64Array): void;
  children(parent: bigint): BigUint64Array;
  childCount(parent: bigint): number;
  parent(child: bigint): bigint | undefined;

  // Style Management (throws TaffyError on failure)
  setStyle(node: bigint, style: Style): void;
  getStyle(node: bigint): Style;

  // Layout Computation (throws TaffyError on failure)
  computeLayout(node: bigint, availableSpace: Size<AvailableSpace>): void;
  computeLayoutWithMeasure(
    node: bigint,
    availableSpace: Size<AvailableSpace>,
    measureFunc: MeasureFunction,
  ): void;

  // Layout Results (throws TaffyError on failure)
  getLayout(node: bigint): Layout;
  unroundedLayout(node: bigint): Layout;

  // Dirty Tracking (throws TaffyError on failure)
  markDirty(node: bigint): void;
  dirty(node: bigint): boolean;

  // Configuration
  enableRounding(): void;
  disableRounding(): void;
}
```

### Style

Configuration object for node layout properties.

```typescript
class Style {
  constructor();

  // Layout Mode
  display: Display; // Block, Flex, Grid, None
  position: Position; // Relative, Absolute

  // Flexbox
  flexDirection: FlexDirection; // Row, Column, RowReverse, ColumnReverse
  flexWrap: FlexWrap; // NoWrap, Wrap, WrapReverse
  flexGrow: number; // Growth factor (default: 0)
  flexShrink: number; // Shrink factor (default: 1)
  flexBasis: Dimension; // Initial size

  // Alignment
  alignItems: AlignItems | undefined;
  alignSelf: AlignSelf | undefined;
  alignContent: AlignContent | undefined;
  justifyContent: JustifyContent | undefined;

  // Sizing
  size: Size<Dimension>; // Width and height
  minSize: Size<Dimension>; // Minimum constraints
  maxSize: Size<Dimension>; // Maximum constraints
  aspectRatio: number | undefined; // Width/height ratio
  boxSizing: BoxSizing; // BorderBox, ContentBox

  // Spacing
  margin: Rect<LengthPercentageAuto>;
  padding: Rect<LengthPercentage>;
  border: Rect<LengthPercentage>;
  gap: Size<LengthPercentage>; // Row and column gap
  inset: Rect<LengthPercentageAuto>; // For absolute positioning

  // Overflow
  overflow: Point<Overflow>;
}
```

### Layout

Read-only computed layout result.

```typescript
class Layout {
  // Position (relative to parent)
  readonly x: number;
  readonly y: number;

  // Size
  readonly width: number;
  readonly height: number;

  // Content size (for scrollable content)
  readonly contentWidth: number;
  readonly contentHeight: number;

  // Spacing
  readonly paddingTop: number;
  readonly paddingRight: number;
  readonly paddingBottom: number;
  readonly paddingLeft: number;

  readonly borderTop: number;
  readonly borderRight: number;
  readonly borderBottom: number;
  readonly borderLeft: number;

  readonly marginTop: number;
  readonly marginRight: number;
  readonly marginBottom: number;
  readonly marginLeft: number;

  // Scrollbars
  readonly scrollbarWidth: number;
  readonly scrollbarHeight: number;

  // Rendering order
  readonly order: number;
}
```

### Enums

```typescript
enum Display {
  Block,
  Flex,
  Grid,
  None,
}
enum Position {
  Relative,
  Absolute,
}
enum FlexDirection {
  Row,
  Column,
  RowReverse,
  ColumnReverse,
}
enum FlexWrap {
  NoWrap,
  Wrap,
  WrapReverse,
}
enum AlignItems {
  Start,
  End,
  FlexStart,
  FlexEnd,
  Center,
  Baseline,
  Stretch,
}
enum AlignSelf {
  Auto,
  Start,
  End,
  FlexStart,
  FlexEnd,
  Center,
  Baseline,
  Stretch,
}
enum AlignContent {
  Start,
  End,
  FlexStart,
  FlexEnd,
  Center,
  Stretch,
  SpaceBetween,
  SpaceAround,
  SpaceEvenly,
}
enum JustifyContent {
  Start,
  End,
  FlexStart,
  FlexEnd,
  Center,
  Stretch,
  SpaceBetween,
  SpaceAround,
  SpaceEvenly,
}
enum Overflow {
  Visible,
  Hidden,
  Scroll,
  Auto,
}
enum BoxSizing {
  BorderBox,
  ContentBox,
}
```

### Types

```typescript
// Dimension values
type Dimension = { Length: number } | { Percent: number } | "Auto";
type LengthPercentage = { Length: number } | { Percent: number };
type LengthPercentageAuto = { Length: number } | { Percent: number } | "Auto";

// Geometry
interface Size<T> {
  width: T;
  height: T;
}
interface Rect<T> {
  left: T;
  right: T;
  top: T;
  bottom: T;
}
interface Point<T> {
  x: T;
  y: T;
}

// Available space for layout computation
type AvailableSpace = { Definite: number } | "MinContent" | "MaxContent";

// Measure function for custom content measurement
type MeasureFunction = (
  knownDimensions: Size<number | null>,
  availableSpace: Size<AvailableSpace>,
  node: bigint,
  context: any,
  style: Style,
) => Size<number>;
```

## 📐 Custom Text Measurement

For text nodes or other content that needs dynamic measurement:

```javascript
const textNode = tree.newLeafWithContext(textStyle, { text: "Hello, World!" });

tree.computeLayoutWithMeasure(
  rootNode,
  { width: { Definite: 800 }, height: "MaxContent" },
  (known, available, node, context, style) => {
    if (context?.text) {
      // Your text measurement logic here
      const width = measureTextWidth(context.text);
      const height = measureTextHeight(context.text, available.width);
      return { width, height };
    }
    return { width: 0, height: 0 };
  },
);
```

## 🔧 Error Handling

Methods that can fail throw a `TaffyError` as a JavaScript exception. Use try-catch to handle errors:

```javascript
try {
  const nodeId = tree.newLeaf(style);
  console.log("Created node:", nodeId);
} catch (error) {
  // error is a TaffyError instance
  console.error("Error:", error.message);
}
```

## 🌐 Browser Support

Taffy-JS works in all modern browsers that support WebAssembly:

- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+

## 📚 Examples

### Flexbox Row Layout

```javascript
const rowStyle = new Style();
rowStyle.display = Display.Flex;
rowStyle.flexDirection = FlexDirection.Row;
rowStyle.justifyContent = JustifyContent.SpaceBetween;
rowStyle.gap = { width: { Length: 10 }, height: { Length: 0 } };
```

### Absolute Positioning

```javascript
const absoluteStyle = new Style();
absoluteStyle.position = Position.Absolute;
absoluteStyle.inset = {
  left: { Length: 10 },
  top: { Length: 10 },
  right: "Auto",
  bottom: "Auto",
};
absoluteStyle.size = { width: { Length: 100 }, height: { Length: 50 } };
```

### Percentage Sizing

```javascript
const percentStyle = new Style();
percentStyle.size = {
  width: { Percent: 50 }, // 50% of parent
  height: { Percent: 100 }, // 100% of parent
};
```

## 🏗️ Building from Source

```bash
# Clone the repository
git clone https://github.com/user/taffy-js.git
cd taffy-js

# Install dependencies
npm install

# Build the WebAssembly module
npm run build

# Run tests
npm test
```

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

## 🙏 Acknowledgments

- [Taffy](https://github.com/DioxusLabs/taffy) - The Rust layout engine this project wraps
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) - Rust/WebAssembly interoperability
