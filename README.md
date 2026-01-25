# ![Taffy Layout Logo](docs/taffy.svg) Taffy Layout

[English](README.md) | [简体中文](docs/i18n/zh-CN/README.md) | [日本語](docs/i18n/ja-JP/README.md)

[![npm version](https://badge.fury.io/js/taffy-layout.svg)](https://www.npmjs.com/package/taffy-layout)
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
```

## 📖 API Reference

### TaffyTree

The main class for managing layout trees.

[View Documentation](./docs/api/classes/TaffyTree.md)

### Style

Configuration object for node layout properties.

[View Documentation](./docs/api/classes/Style.md)

### Layout

Read-only computed layout result.

[View Documentation](./docs/api/classes/Layout.md)

### Enums

[View Documentation](./docs/api/index.md#enumerations)

### Types

[View Documentation](./docs/api/index.md#type-aliases)

## 📐 Custom Text Measurement

For text nodes or other content that needs dynamic measurement:

```typescript
const tree = new TaffyTree();
const textStyle = new Style();
const rootNode = tree.newLeaf(new Style());
const measureTextWidth = (text: string) => text.length * 8;
const measureTextHeight = (text: string, width: number) => 20;

const textNode = tree.newLeafWithContext(textStyle, { text: "Hello, World!" });

tree.computeLayoutWithMeasure(
  rootNode,
  { width: 800, height: "max-content" },
  (known, available, node, context, style) => {
    if (context?.text) {
      // Your text measurement logic here
      const width = measureTextWidth(context.text);
      const height = measureTextHeight(context.text, available.width as number);
      return { width, height };
    }
    return { width: 0, height: 0 };
  },
);
```

## 🔧 Error Handling

Methods that can fail throw a `TaffyError` as a JavaScript exception. Use try-catch to handle errors:

```typescript
try {
  const tree = new TaffyTree();
  const style = new Style();
  const nodeId = tree.newLeaf(style);
  console.log("Created node:", nodeId);
} catch (e) {
  if (e instanceof TaffyError) {
    console.error("Error:", e.message);
  }
}
```

## 🌐 Browser Support

Taffy Layout works in all modern browsers that support WebAssembly:

- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+

## 📚 Examples

### Flexbox Row Layout

```typescript
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
  ["content-end", "footer-start"],
  ["footer-end"],
];
```

### Absolute Positioning

```typescript
const absoluteStyle = new Style();
absoluteStyle.position = Position.Absolute;
absoluteStyle.inset = { left: 10, top: 10, right: "auto", bottom: "auto" };
absoluteStyle.size = { width: 100, height: 50 };
```

### Percentage Sizing

```typescript
const percentStyle = new Style();
percentStyle.size = {
  width: "50%", // 50% of parent
  height: "100%", // 100% of parent
};
```

### Block Layout with Replaced Elements

```typescript
const imgStyle = new Style();
imgStyle.itemIsReplaced = true;
imgStyle.aspectRatio = 16 / 9; // 16:9 aspect ratio
imgStyle.size = { width: "100%", height: "auto" };
```

## 🏗️ Building from Source

```bash
# Clone the repository
git clone https://github.com/ByteLandTechnology/taffy-layout.git
cd taffy-layout

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
