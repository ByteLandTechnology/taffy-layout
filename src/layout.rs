//! # Layout Result Module
//!
//! This module provides the [`JsLayout`] struct, which represents the computed layout
//! for a node after calling `TaffyTree.computeLayout()`. All values are in pixels.
//!
//! ## Overview
//!
//! After computing the layout for a tree, you can retrieve the layout for each node
//! to get its final position, size, and spacing values. The `Layout` object is read-only
//! and contains all the information needed to render the node.
//!
//! @example
//! ```typescript
//! // Compute layout first
//! tree.computeLayout(rootNode, { width: 800, height: 600 });
//!
//! // Get layout for a specific node
//! const layout = tree.getLayout(nodeId);
//!
//! // Access layout properties
//! console.log(`Position: (${layout.x}, ${layout.y})`);
//! console.log(`Size: ${layout.width} x ${layout.height}`);
//! console.log(`Padding: top=${layout.paddingTop}, left=${layout.paddingLeft}`);
//! ```
//!
//! ## Coordinate System
//!
//! - `x` and `y` are relative to the node's parent
//! - Positive `x` is to the right
//! - Positive `y` is downward
//! - For the root node, `x` and `y` are always 0

use taffy;
use wasm_bindgen::prelude::*;

// =============================================================================
// Layout Result Struct
// =============================================================================

/// Computed layout result containing position, size, and spacing values for a node.
///
/// This class wraps the native [`taffy::Layout`] and provides read-only access
/// to all computed layout values. All dimensions are in pixels.
///
/// @example
/// ```typescript
/// const layout = tree.getLayout(node);
///
/// console.log("Position:", layout.x, layout.y);
/// console.log("Size:", layout.width, layout.height);
/// console.log("Content:", layout.contentWidth, layout.contentHeight);
/// console.log("Padding:", layout.paddingTop, layout.paddingRight, layout.paddingBottom, layout.paddingLeft);
/// console.log("Border:", layout.borderTop, layout.borderRight, layout.borderBottom, layout.borderLeft);
/// console.log("Margin:", layout.marginTop, layout.marginRight, layout.marginBottom, layout.marginLeft);
/// console.log("Scrollbar:", layout.scrollbarWidth, layout.scrollbarHeight);
/// console.log("Order:", layout.order);
/// ```
#[wasm_bindgen(js_name = Layout)]
#[derive(Clone, Debug)]
pub struct JsLayout {
    /// The internal Taffy layout object (crate-visible for tree module)
    pub(crate) inner: taffy::Layout,
}

#[wasm_bindgen(js_class = "Layout")]
impl JsLayout {
    // =========================================================================
    // Rendering Order
    // =========================================================================

    /// Gets the rendering order of the node
    ///
    /// This value determines the z-order for overlapping elements.
    /// Lower values are rendered first (behind higher values).
    ///
    /// @returns - The rendering order as an unsigned 32-bit integer
    #[wasm_bindgen(getter)]
    pub fn order(&self) -> u32 {
        self.inner.order
    }

    // =========================================================================
    // Position
    // =========================================================================

    /// Gets the X coordinate of the node's top-left corner
    ///
    /// This value is relative to the node's parent. For the root node,
    /// this is always 0.
    ///
    /// @returns - The horizontal position in pixels
    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f32 {
        self.inner.location.x
    }

    /// Gets the Y coordinate of the node's top-left corner
    ///
    /// This value is relative to the node's parent. For the root node,
    /// this is always 0.
    ///
    /// @returns - The vertical position in pixels
    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f32 {
        self.inner.location.y
    }

    // =========================================================================
    // Size
    // =========================================================================

    /// Gets the computed width of the node
    ///
    /// This is the final width after layout computation, including
    /// any constraints from min/max size or flex properties.
    ///
    /// @returns - The width in pixels
    #[wasm_bindgen(getter)]
    pub fn width(&self) -> f32 {
        self.inner.size.width
    }

    /// Gets the computed height of the node
    ///
    /// This is the final height after layout computation, including
    /// any constraints from min/max size or flex properties.
    ///
    /// @returns - The height in pixels
    #[wasm_bindgen(getter)]
    pub fn height(&self) -> f32 {
        self.inner.size.height
    }

    // =========================================================================
    // Content Size (for scrollable content)
    // =========================================================================

    /// Gets the width of the scrollable content
    ///
    /// If the node has overflow content, this represents the total
    /// width of all content (may exceed `width`).
    ///
    /// @returns - The content width in pixels
    #[wasm_bindgen(getter, js_name = contentWidth)]
    pub fn content_width(&self) -> f32 {
        self.inner.content_size.width
    }

    /// Gets the height of the scrollable content
    ///
    /// If the node has overflow content, this represents the total
    /// height of all content (may exceed `height`).
    ///
    /// @returns - The content height in pixels
    #[wasm_bindgen(getter, js_name = contentHeight)]
    pub fn content_height(&self) -> f32 {
        self.inner.content_size.height
    }

    // =========================================================================
    // Scrollbar Size
    // =========================================================================

    /// Gets the width of the vertical scrollbar
    ///
    /// When overflow is set to scroll, this indicates the space
    /// reserved for the vertical scrollbar.
    ///
    /// @returns - The scrollbar width in pixels (0 if no scrollbar)
    #[wasm_bindgen(getter, js_name = scrollbarWidth)]
    pub fn scrollbar_width(&self) -> f32 {
        self.inner.scrollbar_size.width
    }

    /// Gets the height of the horizontal scrollbar
    ///
    /// When overflow is set to scroll, this indicates the space
    /// reserved for the horizontal scrollbar.
    ///
    /// @returns - The scrollbar height in pixels (0 if no scrollbar)
    #[wasm_bindgen(getter, js_name = scrollbarHeight)]
    pub fn scrollbar_height(&self) -> f32 {
        self.inner.scrollbar_size.height
    }

    // =========================================================================
    // Border Widths
    // =========================================================================

    /// Gets the left border width
    ///
    /// @returns - The computed left border width in pixels
    #[wasm_bindgen(getter, js_name = borderLeft)]
    pub fn border_left(&self) -> f32 {
        self.inner.border.left
    }

    /// Gets the right border width
    ///
    /// @returns - The computed right border width in pixels
    #[wasm_bindgen(getter, js_name = borderRight)]
    pub fn border_right(&self) -> f32 {
        self.inner.border.right
    }

    /// Gets the top border width
    ///
    /// @returns - The computed top border width in pixels
    #[wasm_bindgen(getter, js_name = borderTop)]
    pub fn border_top(&self) -> f32 {
        self.inner.border.top
    }

    /// Gets the bottom border width
    ///
    /// @returns - The computed bottom border width in pixels
    #[wasm_bindgen(getter, js_name = borderBottom)]
    pub fn border_bottom(&self) -> f32 {
        self.inner.border.bottom
    }

    // =========================================================================
    // Padding
    // =========================================================================

    /// Gets the left padding
    ///
    /// @returns - The computed left padding in pixels
    #[wasm_bindgen(getter, js_name = paddingLeft)]
    pub fn padding_left(&self) -> f32 {
        self.inner.padding.left
    }

    /// Gets the right padding
    ///
    /// @returns - The computed right padding in pixels
    #[wasm_bindgen(getter, js_name = paddingRight)]
    pub fn padding_right(&self) -> f32 {
        self.inner.padding.right
    }

    /// Gets the top padding
    ///
    /// @returns - The computed top padding in pixels
    #[wasm_bindgen(getter, js_name = paddingTop)]
    pub fn padding_top(&self) -> f32 {
        self.inner.padding.top
    }

    /// Gets the bottom padding
    ///
    /// @returns - The computed bottom padding in pixels
    #[wasm_bindgen(getter, js_name = paddingBottom)]
    pub fn padding_bottom(&self) -> f32 {
        self.inner.padding.bottom
    }

    // =========================================================================
    // Margins
    // =========================================================================

    /// Gets the left margin
    ///
    /// @returns - The computed left margin in pixels
    #[wasm_bindgen(getter, js_name = marginLeft)]
    pub fn margin_left(&self) -> f32 {
        self.inner.margin.left
    }

    /// Gets the right margin
    ///
    /// @returns - The computed right margin in pixels
    #[wasm_bindgen(getter, js_name = marginRight)]
    pub fn margin_right(&self) -> f32 {
        self.inner.margin.right
    }

    /// Gets the top margin
    ///
    /// @returns - The computed top margin in pixels
    #[wasm_bindgen(getter, js_name = marginTop)]
    pub fn margin_top(&self) -> f32 {
        self.inner.margin.top
    }

    /// Gets the bottom margin
    ///
    /// @returns - The computed bottom margin in pixels
    #[wasm_bindgen(getter, js_name = marginBottom)]
    pub fn margin_bottom(&self) -> f32 {
        self.inner.margin.bottom
    }
}

// =============================================================================
// Conversion Implementations
// =============================================================================

impl From<&taffy::Layout> for JsLayout {
    fn from(layout: &taffy::Layout) -> Self {
        JsLayout {
            inner: layout.clone(),
        }
    }
}

impl From<taffy::Layout> for JsLayout {
    fn from(layout: taffy::Layout) -> Self {
        JsLayout { inner: layout }
    }
}
