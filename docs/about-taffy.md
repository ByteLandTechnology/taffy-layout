---
title: About Taffy
sidebar_position: 1
---

# About Taffy

Taffy is a high-performance, embeddable layout engine written in Rust and compiled to WebAssembly. It brings CSS Flexbox and Grid layout algorithms to JavaScript with near-native performance.

## What Makes Taffy Different?

### WebAssembly-Powered

Taffy is written in Rust and compiled to WebAssembly, giving it performance characteristics much closer to native code than pure JavaScript implementations.

### Complete CSS Support

Taffy implements both CSS Flexbox and CSS Grid layout engines with full feature parity with web browsers.

### Tree-Based API

Taffy provides an efficient tree structure for managing layout nodes, making it perfect for complex, nested layouts.

### Custom Measurement

Taffy supports custom measurement callbacks for dynamic content like text, enabling layout for terminal UIs, canvas rendering, and more.

## Use Cases

### Terminal UI

Perfect for terminal-based UI frameworks where precise layout computation is critical.

### Canvas Applications

Ideal for canvas-based rendering engines that need efficient layout calculations.

### Custom Rendering Engines

Excellent choice for custom UI frameworks that need CSS-like layout without the DOM.

## Architecture

Taffy is based on the [Taffy Rust library](https://github.com/DioxusLabs/taffy), with WebAssembly bindings specifically designed for JavaScript and TypeScript.

### Core Modules

- **Style**: CSS layout property configuration
- **TaffyTree**: Layout tree management and computation
- **Layout**: Computed layout results (position, size, etc.)
- **Enums**: CSS property enumerations (Display, FlexDirection, etc.)
- **Types**: TypeScript type definitions

## Why Not Just Use CSS?

While CSS is great for web browsers, Taffy fills important gaps:

1. **Terminal UI**: CSS doesn't work in terminal environments
2. **Canvas Rendering**: No DOM means no CSS support
3. **Custom Engines**: Build your own UI framework with CSS-like layouts
4. **Performance**: Efficient WebAssembly implementation suitable for high-performance applications
