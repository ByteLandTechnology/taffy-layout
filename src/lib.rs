//! # Taffy Layout: WebAssembly Bindings for Taffy Layout Engine
//!
//! This crate provides high-performance WebAssembly bindings for the [Taffy](https://github.com/DioxusLabs/taffy)
//! layout engine, enabling JavaScript and TypeScript applications to leverage Flexbox and CSS Grid
//! layout algorithms with native-like performance.
//!
//! ## Overview
//!
//! Taffy Layout bridges the gap between Rust's powerful layout computation capabilities and the JavaScript
//! ecosystem. It exposes a clean, idiomatic API that feels natural to JavaScript developers while
//! maintaining the performance benefits of WebAssembly.
//!
//! ## Architecture
//!
//! The crate is organized into the following modules:
//!
//! - **[`enums`]**: CSS layout property enums (Display, Position, FlexDirection, etc.)
//! - **[`error`]**: Error handling types and result mapping utilities
//! - **[`layout`]**: Computed layout result wrapper (`Layout` class)
//! - **[`style`]**: Style configuration object (`Style` class)
//! - **[`tree`]**: Layout tree management (`TaffyTree` class)
//! - **[`types`]**: Data transfer objects and TypeScript type definitions
//! - **[`utils`]**: Utility functions for serialization and logging
//!
//! @example
//!
//! ```typescript
//! import { loadTaffy, TaffyTree, Style, Display, FlexDirection } from 'taffy-layout';
//!
//! await loadTaffy();
//!
//! const tree = new TaffyTree();
//! const style = new Style();
//! style.display = Display.Flex;
//! style.flexDirection = FlexDirection.Column;
//!
//! const root = tree.newLeaf(style);
//! tree.computeLayout(root, { width: "max-content", height: "max-content" });
//!
//! const layout = tree.getLayout(root);
//! console.log(`Width: ${layout.width}, Height: ${layout.height}`);
//! ```
//!
//! ## Features
//!
//! - **Flexbox Layout**: Full CSS Flexbox implementation
//! - **CSS Grid Layout**: Complete CSS Grid support
//! - **Custom Measure Functions**: Support for custom text measurement callbacks
//! - **Tree-based API**: Efficient tree structure for complex layouts
//! - **TypeScript Support**: Full TypeScript type definitions included
//!
//! ## Performance
//!
//! Taffy Layout leverages WebAssembly to achieve near-native performance for layout calculations.
//! The library is particularly well-suited for:
//!
//! - Terminal UI frameworks (like Ink)
//! - Canvas-based applications
//! - Custom rendering engines
//! - Any scenario requiring fast, accurate CSS layout computation

pub mod enums;
pub mod error;
pub mod layout;
pub mod style;
pub mod tree;
pub mod types;
pub mod typescript;
pub mod utils;

// Re-export all public types for convenient access
pub use enums::*;
pub use error::JsTaffyError;
pub use layout::JsLayout;
pub use style::JsStyle;
pub use tree::JsTaffyTree;
pub use types::*;
