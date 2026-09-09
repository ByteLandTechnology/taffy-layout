# ![Taffy Layout Logo](docs/taffy.svg) Taffy Layout

[English](README.md) | [简体中文](docs/i18n/zh-CN/README.md) | [日本語](docs/i18n/ja-JP/README.md)

[![npm version](https://badge.fury.io/js/taffy-layout.svg)](https://www.npmjs.com/package/taffy-layout)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

High-performance WebAssembly bindings for the [Taffy](https://github.com/DioxusLabs/taffy) layout engine, bringing CSS Flexbox and Grid layout algorithms to JavaScript with near-native performance.

## ✨ Features

- **🚀 High Performance**: WebAssembly-powered layout calculations
- **📦 CSS Layout Algorithms**: Flexbox, Grid, and Block layout with float support
- **↔️ Direction and Alignment**: LTR/RTL layout, safe alignment, and self-relative alignment
- **🔧 Custom Measurement**: Support for custom text/content measurement callbacks
- **📝 TypeScript Ready**: Complete type definitions included
- **🌳 Tree-Based API**: Efficient tree structure for complex layouts
- **💡 Familiar API**: CSS-like property names and values

## 📦 Installation

```bash
npm install taffy-layout
```

## 🚀 Quick Start

```typescript
import {
  loadTaffy,
  TaffyTree,
  Style,
  Display,
  FlexDirection,
  AlignItems,
} from "taffy-layout";

// Initialize WebAssembly module
await loadTaffy();

// Create a layout tree
const tree = new TaffyTree();

// Create container style
const containerStyle = new Style();
containerStyle.display = Display.Flex;
containerStyle.flexDirection = FlexDirection.Column;
containerStyle.alignItems = AlignItems.Center;

// You can set size as an object
containerStyle.size = { width: 300, height: 200 };

// Or use individual width/height properties
containerStyle.width = 300;
containerStyle.height = 200;

// Set padding as an object
containerStyle.padding = { left: 10, right: 10, top: 10, bottom: 10 };

// Or use individual padding properties
containerStyle.paddingLeft = 10;
containerStyle.paddingRight = 10;
containerStyle.paddingTop = 10;
containerStyle.paddingBottom = 10;

// Create child styles
const childStyle = new Style();
childStyle.flexGrow = 1;
childStyle.width = "100%";
childStyle.height = "auto";

// Create nodes
const child1 = tree.newLeaf(childStyle);
const child2 = tree.newLeaf(childStyle);
const container = tree.newWithChildren(containerStyle, [child1, child2]);

// Compute layout
tree.computeLayout(container, { width: 300, height: 200 });

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

containerLayout.free();
child1Layout.free();
child2Layout.free();
containerStyle.free();
childStyle.free();
tree.free();
```

## 📚 Documentation

- [Introduction](docs/intro.md)
- [Getting Started](docs/getting-started/installation.md)
- [Core Concepts](docs/core-concepts/index.md)
- [Styling Guide](docs/styling/index.md)
- [Advanced Usage](docs/advanced/index.md)
- [Cookbook](docs/cookbook/index.md)

## 📖 API Reference

### TaffyTree

The main class for managing layout trees.

[View Documentation](./docs/api/classes/TaffyTree.md)

### Style

Configuration object for node layout properties. New styles default to `Display.Flex` and `Direction.Ltr`; use `Display.Block` or `Display.FlowRoot` for block layout.

[View Documentation](./docs/api/classes/Style.md)

### Layout

Read-only computed layout result.

[View Documentation](./docs/api/classes/Layout.md)

### Enums

[View Documentation](./docs/api/index.md#enumerations)

### Types

[View Documentation](./docs/api/index.md#type-aliases)

## 📐 Custom Text Measurement

The following examples assume WebAssembly has been initialized with `loadTaffy()`.
For text nodes or other content that needs dynamic measurement:

```typescript
import { Style, TaffyTree } from "taffy-layout";

const tree = new TaffyTree();
const textStyle = new Style();
const measureTextWidth = (text: string) => text.length * 8;
const measureTextHeight = (text: string, width: number) => 20;

const textNode = tree.newLeafWithContext(textStyle, { text: "Hello, World!" });
const rootNode = tree.newWithChildren(textStyle, [textNode]);

tree.computeLayoutWithMeasure(
  rootNode,
  { width: 800, height: "max-content" },
  (known, available, node, context, style) => {
    style.free(); // The callback receives an owned copy of the node's style
    if (context?.text) {
      // Your text measurement logic here
      const width = known.width ?? measureTextWidth(context.text);
      const wrappingWidth =
        typeof available.width === "number" ? available.width : width;
      const height =
        known.height ?? measureTextHeight(context.text, wrappingWidth);
      return { width, height };
    }
    return { width: 0, height: 0 };
  },
);

textStyle.free();
tree.free();
```

The callback returns content dimensions; Taffy applies padding, borders, and size constraints. See [Measure Functions](docs/core-concepts/measure-functions.md) for the five callback arguments and intrinsic sizing constraints.

## 🔧 Error Handling

Recoverable errors, such as an out-of-bounds child index on a valid parent, throw `TaffyError`. Node IDs must refer to live nodes in the same tree; invalid IDs can cause a WebAssembly panic and are not guaranteed to throw `TaffyError`. See [Error Handling](docs/advanced/error-handling.md).

```typescript
import { Style, TaffyError, TaffyTree } from "taffy-layout";

const tree = new TaffyTree();
const style = new Style();
const parent = tree.newLeaf(style);
try {
  tree.getChildAtIndex(parent, 0); // Valid parent, but no children
} catch (e) {
  if (e instanceof TaffyError) {
    console.error("Error:", e.message);
    e.free();
  } else {
    throw e;
  }
} finally {
  style.free();
  tree.free();
}
```

## 🌐 Browser Support

The browser must support the generated WebAssembly module and its ES-module JavaScript wrapper, including BigInt node IDs, JavaScript/WebAssembly BigInt integration, and WebAssembly reference types. WebAssembly support alone does not establish compatibility.

## 📚 Examples

### Flexbox Row Layout

```typescript
import { Display, FlexDirection, JustifyContent, Style } from "taffy-layout";

const rowStyle = new Style();
rowStyle.display = Display.Flex;
rowStyle.flexDirection = FlexDirection.Row;
rowStyle.justifyContent = JustifyContent.SpaceBetween;
rowStyle.gap = { width: 10, height: 0 };
```

### CSS Grid Layout

```typescript
import { Style, Display, GridAutoFlow } from "taffy-layout";

const gridStyle = new Style();
gridStyle.display = Display.Grid;
gridStyle.gridAutoFlow = GridAutoFlow.Row;
gridStyle.gap = { width: 10, height: 10 };

// Grid item placement
const itemStyle = new Style();
itemStyle.gridRow = { start: 1, end: 3 }; // Spans 2 rows
itemStyle.gridColumn = { start: 1, end: { span: 2 } }; // Spans 2 columns
```

### Grid Template Areas

```typescript
import { Display, Style } from "taffy-layout";

const gridStyle = new Style();
gridStyle.display = Display.Grid;
gridStyle.gridTemplateAreas = [
  { name: "header", rowStart: 1, rowEnd: 2, columnStart: 1, columnEnd: 4 },
  { name: "sidebar", rowStart: 2, rowEnd: 4, columnStart: 1, columnEnd: 2 },
  { name: "main", rowStart: 2, rowEnd: 4, columnStart: 2, columnEnd: 4 },
  { name: "footer", rowStart: 4, rowEnd: 5, columnStart: 1, columnEnd: 4 },
];

// Named grid lines
gridStyle.gridTemplateRowNames = [
  ["header-start"],
  ["header-end", "content-start"],
  [], // Intermediate line within the two-row content area
  ["content-end", "footer-start"],
  ["footer-end"],
];
```

Use `gridTemplateAreaRowCount` and `gridTemplateAreaColumnCount` to include trailing unnamed cells. The counts are at least the extent of the named areas; see [Grid Templates](docs/styling/grid-templates.md#named-areas-and-template-dimensions).

### Absolute Positioning

```typescript
import { Position, Style } from "taffy-layout";

const absoluteStyle = new Style();
absoluteStyle.position = Position.Absolute;
absoluteStyle.inset = { left: 10, top: 10, right: "auto", bottom: "auto" };
absoluteStyle.size = { width: 100, height: 50 };
```

### Percentage Sizing

```typescript
import { Style } from "taffy-layout";

const percentStyle = new Style();
percentStyle.size = {
  width: "50%", // 50% of parent
  height: "100%", // 100% of parent
};
```

### Block Layout with Replaced Elements

Use this style for an image-like child in a block container. Content loading and drawing are handled by your application.

```typescript
import { Style } from "taffy-layout";

const imgStyle = new Style();
imgStyle.itemIsReplaced = true;
imgStyle.aspectRatio = 16 / 9; // 16:9 aspect ratio
imgStyle.size = { width: "100%", height: "auto" };
```

## 🏗️ Building from Source

The repository's development tools require Node.js 22.14 or later on the 22.x line, or Node.js 24.10 or later. Install Rust 1.85 or later and its `wasm32-unknown-unknown` target before building.

```bash
# Clone the repository
git clone https://github.com/ByteLandTechnology/taffy-layout.git
cd taffy-layout

# Install dependencies
npm install

# Install the Rust WebAssembly target
rustup target add wasm32-unknown-unknown

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
