//! # Style Configuration Module
//!
//! This module provides the [`JsStyle`] struct, which represents the CSS layout properties
//! for a node. It is the primary configuration object used when creating or updating nodes.
//!
//! ## Overview
//!
//! The `Style` class exposes all CSS Flexbox and Grid layout properties through a
//! JavaScript-friendly API. Properties are accessed and modified using standard
//! JavaScript getter/setter syntax.
//!
//! ## JavaScript Usage
//!
//!
//! <details>
//! <summary><strong>TypeScript Example</strong></summary>
//!
//! ```typescript
//! import init, {
//!   Style,
//!   Display,
//!   FlexDirection,
//!   AlignItems,
//!   TaffyTree,
//!   type Dimension,
//!   type Size,
//!   type Rect,
//!   type LengthPercentage
//! } from 'taffy-js';
//!
//! await init();
//!
//! const style = new Style();
//!
//! // Configure with type-safe enums
//! style.display = Display.Flex;
//! style.flexDirection = FlexDirection.Column;
//! style.alignItems = AlignItems.Center;
//!
//! // Set dimensions with explicit types
//! const size: Size<Dimension> = {
//!   width: { Length: 200 },
//!   height: { Length: 100 }
//! };
//! style.size = size;
//!
//! const padding: Rect<LengthPercentage> = {
//!   left: { Length: 10 },
//!   right: { Length: 10 },
//!   top: { Length: 5 },
//!   bottom: { Length: 5 }
//! };
//! style.padding = padding;
//!
//! const tree = new TaffyTree();
//! const nodeId: bigint = tree.newLeaf(style);
//! ```
//!
//! </details>
//!
//! ## Property Categories
//!
//! | Category | Properties |
//! |----------|------------|
//! | **Layout Mode** | `display`, `position` |
//! | **Flexbox** | `flexDirection`, `flexWrap`, `flexGrow`, `flexShrink`, `flexBasis` |
//! | **Alignment** | `alignItems`, `alignSelf`, `alignContent`, `justifyContent` |
//! | **Sizing** | `size`, `minSize`, `maxSize`, `aspectRatio`, `boxSizing` |
//! | **Spacing** | `margin`, `padding`, `border`, `gap`, `inset` |
//! | **Overflow** | `overflow` |
//!
//! ## Dimension Types
//!
//! Several properties use special dimension types:
//!
//! - **Dimension**: `{ Length: number }`, `{ Percent: number }`, or `"Auto"`
//! - **LengthPercentage**: `{ Length: number }` or `{ Percent: number }`
//! - **LengthPercentageAuto**: Same as Dimension


use crate::enums::*;
use crate::types::*;
use crate::utils::log;
use crate::utils::serialize;
use taffy::style::{self as TaffyStyle};
use wasm_bindgen::prelude::*;

// =============================================================================
// Style Struct
// =============================================================================

/// CSS layout configuration for a node, including flexbox, sizing, spacing, and alignment properties.
///
/// This class holds all CSS layout properties for a node. Create an instance with
/// `new Style()` and configure properties before passing to `TaffyTree.newLeaf()`.
///
/// # Default Values
///
/// When created, all properties are set to their CSS default values:
/// - `display`: `Display.Block`
/// - `position`: `Position.Relative`
/// - `flexDirection`: `FlexDirection.Row`
/// - `flexWrap`: `FlexWrap.NoWrap`
/// - `flexGrow`: `0`
/// - `flexShrink`: `1`
/// - All alignment properties: `undefined` (use default behavior)
/// - All dimensions: `"Auto"`
/// - All spacing: `{ Length: 0 }`
///
#[wasm_bindgen(js_name = Style)]
pub struct JsStyle {
    /// Internal Taffy style object (crate-internal access for tree operations)
    pub(crate) inner: TaffyStyle::Style,
}

#[wasm_bindgen(js_class = "Style")]
impl JsStyle {
    // =========================================================================
    // Constructor
    // =========================================================================

    /// Creates a new Style instance with default values
    ///
    /// # Returns
    ///
    /// A new `Style` object with all properties set to CSS defaults
    ///
    /// # Example
    ///
    /// ```javascript
    /// const style = new Style();
    /// console.log(style.display);  // Display.Block
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsStyle {
        JsStyle {
            inner: TaffyStyle::Style::default(),
        }
    }

    // =========================================================================
    // Layout Mode Properties
    // =========================================================================

    /// Gets the display mode
    ///
    /// Determines the layout algorithm used for this element and its children.
    ///
    /// # Returns
    ///
    /// The current [`Display`](JsDisplay) value
    ///
    /// # Default
    ///
    /// `Display.Block`
    #[wasm_bindgen(getter)]
    pub fn display(&self) -> JsDisplay {
        self.inner.display.into()
    }

    /// Sets the display mode
    ///
    /// # Arguments
    ///
    /// - `val`: The new display mode
    ///
    ///
    /// <details>
    /// <summary><strong>TypeScript Example</strong></summary>
    ///
    /// ```typescript
    /// style.display = Display.Flex;
    /// ```
    ///
    /// </details>
    #[wasm_bindgen(setter)]
    pub fn set_display(&mut self, val: JsDisplay) {
        self.inner.display = val.into();
    }

    /// Gets the position mode
    ///
    /// Determines how the element is positioned within its parent.
    ///
    /// # Returns
    ///
    /// The current [`Position`](JsPosition) value
    ///
    /// # Default
    ///
    /// `Position.Relative`
    #[wasm_bindgen(getter)]
    pub fn position(&self) -> JsPosition {
        self.inner.position.into()
    }

    /// Sets the position mode
    ///
    /// # Arguments
    ///
    /// - `val`: The new position mode
    ///
    ///
    /// <details>
    /// <summary><strong>TypeScript Example</strong></summary>
    ///
    /// ```typescript
    /// style.position = Position.Absolute;
    /// style.inset = { left: { Length: 10 }, top: { Length: 10 }, right: "Auto", bottom: "Auto" };
    /// ```
    ///
    /// </details>
    #[wasm_bindgen(setter)]
    pub fn set_position(&mut self, val: JsPosition) {
        self.inner.position = val.into();
    }

    // =========================================================================
    // Flexbox Properties
    // =========================================================================

    /// Gets the flex direction
    ///
    /// Defines the main axis direction for flex items.
    ///
    /// # Returns
    ///
    /// The current [`FlexDirection`](JsFlexDirection) value
    ///
    /// # Default
    ///
    /// `FlexDirection.Row`
    #[wasm_bindgen(getter, js_name = flexDirection)]
    pub fn flex_direction(&self) -> JsFlexDirection {
        self.inner.flex_direction.into()
    }

    /// Sets the flex direction
    ///
    /// # Arguments
    ///
    /// - `val`: The new flex direction
    ///
    ///
    /// <details>
    /// <summary><strong>TypeScript Example</strong></summary>
    ///
    /// ```typescript
    /// style.flexDirection = FlexDirection.Column;
    /// ```
    ///
    /// </details>
    #[wasm_bindgen(setter, js_name = flexDirection)]
    pub fn set_flex_direction(&mut self, val: JsFlexDirection) {
        self.inner.flex_direction = val.into();
    }

    /// Gets the flex wrap mode
    ///
    /// Controls whether flex items wrap to new lines.
    ///
    /// # Returns
    ///
    /// The current [`FlexWrap`](JsFlexWrap) value
    ///
    /// # Default
    ///
    /// `FlexWrap.NoWrap`
    #[wasm_bindgen(getter, js_name = flexWrap)]
    pub fn flex_wrap(&self) -> JsFlexWrap {
        self.inner.flex_wrap.into()
    }

    /// Sets the flex wrap mode
    ///
    /// # Arguments
    ///
    /// - `val`: The new flex wrap mode
    ///
    ///
    /// <details>
    /// <summary><strong>TypeScript Example</strong></summary>
    ///
    /// ```typescript
    /// style.flexWrap = FlexWrap.Wrap;
    /// ```
    ///
    /// </details>
    #[wasm_bindgen(setter, js_name = flexWrap)]
    pub fn set_flex_wrap(&mut self, val: JsFlexWrap) {
        self.inner.flex_wrap = val.into();
    }

    /// Gets the flex grow factor
    ///
    /// Determines how much the item grows relative to siblings when
    /// there is extra space available.
    ///
    /// # Returns
    ///
    /// The flex grow factor (default: 0)
    #[wasm_bindgen(getter, js_name = flexGrow)]
    pub fn flex_grow(&self) -> f32 {
        self.inner.flex_grow
    }

    /// Sets the flex grow factor
    ///
    /// # Arguments
    ///
    /// - `val`: The new flex grow factor (must be >= 0)
    ///
    ///
    /// <details>
    /// <summary><strong>TypeScript Example</strong></summary>
    ///
    /// ```typescript
    /// style.flexGrow = 2;
    /// ```
    ///
    /// </details>
    #[wasm_bindgen(setter, js_name = flexGrow)]
    pub fn set_flex_grow(&mut self, val: f32) {
        self.inner.flex_grow = val;
    }

    /// Gets the flex shrink factor
    ///
    /// Determines how much the item shrinks relative to siblings when
    /// there is insufficient space.
    ///
    /// # Returns
    ///
    /// The flex shrink factor (default: 1)
    #[wasm_bindgen(getter, js_name = flexShrink)]
    pub fn flex_shrink(&self) -> f32 {
        self.inner.flex_shrink
    }

    /// Sets the flex shrink factor
    ///
    /// # Arguments
    ///
    /// - `val`: The new flex shrink factor (must be >= 0)
    ///
    ///
    /// <details>
    /// <summary><strong>TypeScript Example</strong></summary>
    ///
    /// ```typescript
    /// style.flexShrink = 2;
    /// ```
    ///
    /// </details>
    #[wasm_bindgen(setter, js_name = flexShrink)]
    pub fn set_flex_shrink(&mut self, val: f32) {
        self.inner.flex_shrink = val;
    }

    // =========================================================================
    // Alignment Properties
    // =========================================================================

    /// Gets the align-items property
    ///
    /// Defines the default alignment for all children on the cross axis.
    ///
    /// # Returns
    ///
    /// The current [`AlignItems`](JsAlignItems) value, or `undefined` if not set
    #[wasm_bindgen(getter, js_name = alignItems)]
    pub fn align_items(&self) -> Option<JsAlignItems> {
        self.inner.align_items.map(JsAlignItems::from)
    }

    /// Sets the align-items property
    ///
    /// # Arguments
    ///
    /// - `val`: The new align-items value, or `undefined` to use default
    ///
    ///
    /// <details>
    /// <summary><strong>TypeScript Example</strong></summary>
    ///
    /// ```typescript
    /// style.alignItems = AlignItems.Center;
    /// ```
    ///
    /// </details>
    #[wasm_bindgen(setter, js_name = alignItems)]
    pub fn set_align_items(&mut self, val: OneOptAlignItems) {
        let val: JsValue = val.unchecked_into();
        self.inner.align_items = if val.is_undefined() || val.is_null() {
            None
        } else {
            let n = val.as_f64().unwrap_or(0.0) as u32;
            JsAlignItems::try_from(n).ok().map(|x| x.into())
        };
    }

    /// Gets the align-self property
    ///
    /// Overrides the parent's align-items for this specific element.
    ///
    /// # Returns
    ///
    /// The current [`AlignSelf`](JsAlignSelf) value (returns `Auto` if not set)
    #[wasm_bindgen(getter, js_name = alignSelf)]
    pub fn align_self(&self) -> Option<JsAlignSelf> {
        match self.inner.align_self {
            Some(v) => Some(JsAlignSelf::from(v)),
            None => Some(JsAlignSelf::Auto),
        }
    }

    /// Sets the align-self property
    ///
    /// # Arguments
    ///
    /// - `val`: The new align-self value, or `undefined`/`Auto` to inherit from parent
    ///
    ///
    /// <details>
    /// <summary><strong>TypeScript Example</strong></summary>
    ///
    /// ```typescript
    /// style.alignSelf = AlignSelf.Stretch;
    /// ```
    ///
    /// </details>
    #[wasm_bindgen(setter, js_name = alignSelf)]
    pub fn set_align_self(&mut self, val: OneOptAlignSelf) {
        let val: JsValue = val.unchecked_into();
        self.inner.align_self = if val.is_undefined() || val.is_null() {
            None
        } else {
            let n = val.as_f64().unwrap_or(0.0) as u32;
            match JsAlignSelf::try_from(n) {
                Ok(JsAlignSelf::Auto) => None,
                Ok(other) => Some(other.into()),
                Err(_) => None,
            }
        };
    }

    /// Gets the align-content property
    ///
    /// Controls distribution of space between lines in a multi-line flex container.
    ///
    /// # Returns
    ///
    /// The current [`AlignContent`](JsAlignContent) value, or `undefined` if not set
    #[wasm_bindgen(getter, js_name = alignContent)]
    pub fn align_content(&self) -> Option<JsAlignContent> {
        self.inner.align_content.map(JsAlignContent::from)
    }

    /// Sets the align-content property
    ///
    /// # Arguments
    ///
    /// - `val`: The new align-content value, or `undefined` to use default
    ///
    ///
    /// <details>
    /// <summary><strong>TypeScript Example</strong></summary>
    ///
    /// ```typescript
    /// style.alignContent = AlignContent.SpaceBetween;
    /// ```
    ///
    /// </details>
    #[wasm_bindgen(setter, js_name = alignContent)]
    pub fn set_align_content(&mut self, val: OneOptAlignContent) {
        let val: JsValue = val.unchecked_into();
        self.inner.align_content = if val.is_undefined() || val.is_null() {
            None
        } else {
            let n = val.as_f64().unwrap_or(0.0) as u32;
            JsAlignContent::try_from(n).ok().map(|x| x.into())
        };
    }

    /// Gets the justify-content property
    ///
    /// Defines alignment and spacing of items along the main axis.
    ///
    /// # Returns
    ///
    /// The current [`JustifyContent`](JsJustifyContent) value, or `undefined` if not set
    #[wasm_bindgen(getter, js_name = justifyContent)]
    pub fn justify_content(&self) -> Option<JsJustifyContent> {
        self.inner.justify_content.map(JsJustifyContent::from)
    }

    /// Sets the justify-content property
    ///
    /// # Arguments
    ///
    /// - `val`: The new justify-content value, or `undefined` to use default
    ///
    ///
    /// <details>
    /// <summary><strong>TypeScript Example</strong></summary>
    ///
    /// ```typescript
    /// style.justifyContent = JustifyContent.Center;
    /// ```
    ///
    /// </details>
    #[wasm_bindgen(setter, js_name = justifyContent)]
    pub fn set_justify_content(&mut self, val: OneOptJustifyContent) {
        let val: JsValue = val.unchecked_into();
        self.inner.justify_content = if val.is_undefined() || val.is_null() {
            None
        } else {
            let n = val.as_f64().unwrap_or(0.0) as u32;
            JsJustifyContent::try_from(n).ok().map(|x| x.into())
        };
    }

    // =========================================================================
    // Sizing Properties
    // =========================================================================

    /// Gets the aspect ratio
    ///
    /// The ratio of width to height. Used to maintain proportions.
    ///
    /// # Returns
    ///
    /// The aspect ratio value, or `undefined` if not set
    #[wasm_bindgen(getter, js_name = aspectRatio)]
    pub fn aspect_ratio(&self) -> Option<f32> {
        self.inner.aspect_ratio
    }

    /// Sets the aspect ratio
    ///
    /// # Arguments
    ///
    /// - `val`: The new aspect ratio (width/height), or `undefined` to clear
    ///
    ///
    /// <details>
    /// <summary><strong>TypeScript Example</strong></summary>
    ///
    /// ```typescript
    /// style.aspectRatio = 16 / 9;
    /// ```
    ///
    /// </details>
    #[wasm_bindgen(setter, js_name = aspectRatio)]
    pub fn set_aspect_ratio(&mut self, val: OnOptNumber) {
        let val: JsValue = val.unchecked_into();
        self.inner.aspect_ratio = if val.is_undefined() || val.is_null() {
            None
        } else {
            val.as_f64().map(|v| v as f32)
        };
    }

    /// Gets the overflow behavior
    ///
    /// Controls how content that exceeds the container is handled.
    ///
    /// # Returns
    ///
    /// A `Point<Overflow>` with `x` and `y` overflow settings
    #[wasm_bindgen(getter)]
    pub fn overflow(&self) -> JsPointOverflow {
        serialize(&self.inner.overflow).unchecked_into()
    }

    /// Sets the overflow behavior
    ///
    /// # Arguments
    ///
    /// - `val`: An object with `x` and `y` overflow values
    ///
    ///
    /// <details>
    /// <summary><strong>TypeScript Example</strong></summary>
    ///
    /// ```typescript
    /// style.overflow = { x: Overflow.Hidden, y: Overflow.Scroll };
    /// ```
    ///
    /// </details>
    #[wasm_bindgen(setter)]
    pub fn set_overflow(&mut self, val: JsPointOverflow) {
        let val: JsValue = val.unchecked_into();
        if let Ok(o) = serde_wasm_bindgen::from_value(val) {
            self.inner.overflow = o;
        }
    }

    /// Gets the box sizing mode
    ///
    /// Determines whether padding and border are included in dimensions.
    ///
    /// # Returns
    ///
    /// The current [`BoxSizing`](JsBoxSizing) value
    ///
    /// # Default
    ///
    /// `BoxSizing.BorderBox`
    #[wasm_bindgen(getter, js_name = boxSizing)]
    pub fn box_sizing(&self) -> JsBoxSizing {
        self.inner.box_sizing.into()
    }

    /// Sets the box sizing mode
    ///
    /// # Arguments
    ///
    /// - `val`: The new box sizing mode
    ///
    ///
    /// <details>
    /// <summary><strong>TypeScript Example</strong></summary>
    ///
    /// ```typescript
    /// style.boxSizing = BoxSizing.ContentBox;
    /// ```
    ///
    /// </details>
    #[wasm_bindgen(setter, js_name = boxSizing)]
    pub fn set_box_sizing(&mut self, val: JsBoxSizing) {
        self.inner.box_sizing = val.into();
    }

    /// Gets the flex-basis
    ///
    /// The initial size of a flex item before growing/shrinking.
    ///
    /// # Returns
    ///
    /// A `Dimension` value (`{ Length: n }`, `{ Percent: n }`, or `"Auto"`)
    #[wasm_bindgen(getter, js_name = flexBasis)]
    pub fn flex_basis(&self) -> JsDimension {
        let d: DimensionDto = self.inner.flex_basis.into();
        serialize(&d).unchecked_into()
    }

    /// Sets the flex-basis
    ///
    /// # Arguments
    ///
    /// - `val`: The initial size as a Dimension
    ///
    ///
    /// <details>
    /// <summary><strong>TypeScript Example</strong></summary>
    ///
    /// ```typescript
    /// style.flexBasis = { Length: 100 };
    /// ```
    ///
    /// </details>
    #[wasm_bindgen(setter, js_name = flexBasis)]
    pub fn set_flex_basis(&mut self, val: JsDimension) {
        let val: JsValue = val.unchecked_into();
        if let Ok(d) = serde_wasm_bindgen::from_value::<DimensionDto>(val) {
            self.inner.flex_basis = d.into();
        }
    }

    // =========================================================================
    // Dimension Properties
    // =========================================================================

    /// Gets the size (width and height)
    ///
    /// # Returns
    ///
    /// A `Size<Dimension>` object with `width` and `height` properties
    #[wasm_bindgen(getter)]
    pub fn size(&self) -> JsSizeDimension {
        let s: SizeDto<DimensionDto> = SizeDto {
            width: self.inner.size.width.into(),
            height: self.inner.size.height.into(),
        };
        serialize(&s).unchecked_into()
    }

    /// Sets the size (width and height)
    ///
    /// # Arguments
    ///
    /// - `val`: A Size object with Dimension values
    ///
    ///
    /// <details>
    /// <summary><strong>TypeScript Example</strong></summary>
    ///
    /// ```typescript
    /// style.size = { width: { Length: 200 }, height: { Percent: 100 } };
    /// ```
    ///
    /// </details>
    #[wasm_bindgen(setter)]
    pub fn set_size(&mut self, val: JsSizeDimension) {
        let val: JsValue = val.unchecked_into();
        match serde_wasm_bindgen::from_value::<SizeDto<DimensionDto>>(val.clone()) {
            Ok(s) => {
                self.inner.size = s.into();
            }
            Err(e) => {
                let json = js_sys::JSON::stringify(&val)
                    .ok()
                    .and_then(|s| s.as_string())
                    .unwrap_or("?".to_string());
                log(&format!("set_size Error: {} | Input: {}", e, json));
            }
        }
    }

    /// Gets the minimum size constraints
    ///
    /// # Returns
    ///
    /// A `Size<Dimension>` object with minimum width and height
    #[wasm_bindgen(getter, js_name = minSize)]
    pub fn min_size(&self) -> JsSizeDimension {
        let s: SizeDto<DimensionDto> = SizeDto {
            width: self.inner.min_size.width.into(),
            height: self.inner.min_size.height.into(),
        };
        serialize(&s).unchecked_into()
    }

    /// Sets the minimum size constraints
    ///
    /// # Arguments
    ///
    /// - `val`: A Size object with minimum Dimension values
    ///
    ///
    /// <details>
    /// <summary><strong>TypeScript Example</strong></summary>
    ///
    /// ```typescript
    /// style.minSize = { width: { Length: 100 }, height: "Auto" };
    /// ```
    ///
    /// </details>
    #[wasm_bindgen(setter, js_name = minSize)]
    pub fn set_min_size(&mut self, val: JsSizeDimension) {
        let val: JsValue = val.unchecked_into();
        if let Ok(s) = serde_wasm_bindgen::from_value::<SizeDto<DimensionDto>>(val) {
            self.inner.min_size = s.into();
        }
    }

    /// Gets the maximum size constraints
    ///
    /// # Returns
    ///
    /// A `Size<Dimension>` object with maximum width and height
    #[wasm_bindgen(getter, js_name = maxSize)]
    pub fn max_size(&self) -> JsSizeDimension {
        let s: SizeDto<DimensionDto> = SizeDto {
            width: self.inner.max_size.width.into(),
            height: self.inner.max_size.height.into(),
        };
        serialize(&s).unchecked_into()
    }

    /// Sets the maximum size constraints
    ///
    /// # Arguments
    ///
    /// - `val`: A Size object with maximum Dimension values
    ///
    ///
    /// <details>
    /// <summary><strong>TypeScript Example</strong></summary>
    ///
    /// ```typescript
    /// style.maxSize = { width: "MaxContent", height: { Length: 500 } };
    /// ```
    ///
    /// </details>
    #[wasm_bindgen(setter, js_name = maxSize)]
    pub fn set_max_size(&mut self, val: JsSizeDimension) {
        let val: JsValue = val.unchecked_into();
        if let Ok(s) = serde_wasm_bindgen::from_value::<SizeDto<DimensionDto>>(val) {
            self.inner.max_size = s.into();
        }
    }

    // =========================================================================
    // Spacing Properties
    // =========================================================================

    /// Gets the margin
    ///
    /// Outer spacing around the element.
    ///
    /// # Returns
    ///
    /// A `Rect<LengthPercentageAuto>` with left, right, top, bottom margins
    #[wasm_bindgen(getter)]
    pub fn margin(&self) -> JsRectLengthPercentageAuto {
        let m: RectDto<LengthPercentageAutoDto> = RectDto {
            left: self.inner.margin.left.into(),
            right: self.inner.margin.right.into(),
            top: self.inner.margin.top.into(),
            bottom: self.inner.margin.bottom.into(),
        };
        serialize(&m).unchecked_into()
    }

    /// Sets the margin
    ///
    /// # Arguments
    ///
    /// - `val`: A Rect object with LengthPercentageAuto values
    ///
    ///
    /// <details>
    /// <summary><strong>TypeScript Example</strong></summary>
    ///
    /// ```typescript
    /// style.margin = { left: { Length: 10 }, right: { Length: 10 }, top: { Length: 5 }, bottom: { Length: 5 } };
    /// ```
    ///
    /// </details>
    #[wasm_bindgen(setter)]
    pub fn set_margin(&mut self, val: JsRectLengthPercentageAuto) {
        let val: JsValue = val.unchecked_into();
        if let Ok(m) = serde_wasm_bindgen::from_value::<RectDto<LengthPercentageAutoDto>>(val) {
            self.inner.margin = m.into();
        }
    }

    /// Gets the padding
    ///
    /// Inner spacing between the element's border and content.
    ///
    /// # Returns
    ///
    /// A `Rect<LengthPercentage>` with left, right, top, bottom padding
    #[wasm_bindgen(getter)]
    pub fn padding(&self) -> JsRectLengthPercentage {
        let m: RectDto<LengthPercentageDto> = RectDto {
            left: self.inner.padding.left.into(),
            right: self.inner.padding.right.into(),
            top: self.inner.padding.top.into(),
            bottom: self.inner.padding.bottom.into(),
        };
        serialize(&m).unchecked_into()
    }

    /// Sets the padding
    ///
    /// # Arguments
    ///
    /// - `val`: A Rect object with LengthPercentage values
    ///
    ///
    /// <details>
    /// <summary><strong>TypeScript Example</strong></summary>
    ///
    /// ```typescript
    /// style.padding = { left: { Length: 20 }, right: { Length: 20 }, top: { Length: 10 }, bottom: { Length: 10 } };
    /// ```
    ///
    /// </details>
    #[wasm_bindgen(setter)]
    pub fn set_padding(&mut self, val: JsRectLengthPercentage) {
        let val: JsValue = val.unchecked_into();
        if let Ok(p) = serde_wasm_bindgen::from_value::<RectDto<LengthPercentageDto>>(val) {
            self.inner.padding = p.into();
        }
    }

    /// Gets the border width
    ///
    /// Width of the element's border on each side.
    ///
    /// # Returns
    ///
    /// A `Rect<LengthPercentage>` with left, right, top, bottom border widths
    #[wasm_bindgen(getter)]
    pub fn border(&self) -> JsRectLengthPercentage {
        let m: RectDto<LengthPercentageDto> = RectDto {
            left: self.inner.border.left.into(),
            right: self.inner.border.right.into(),
            top: self.inner.border.top.into(),
            bottom: self.inner.border.bottom.into(),
        };
        serialize(&m).unchecked_into()
    }

    /// Sets the border width
    ///
    /// # Arguments
    ///
    /// - `val`: A Rect object with LengthPercentage values
    ///
    ///
    /// <details>
    /// <summary><strong>TypeScript Example</strong></summary>
    ///
    /// ```typescript
    /// style.border = { left: { Length: 1 }, right: { Length: 1 }, top: { Length: 1 }, bottom: { Length: 1 } };
    /// ```
    ///
    /// </details>
    #[wasm_bindgen(setter)]
    pub fn set_border(&mut self, val: JsRectLengthPercentage) {
        let val: JsValue = val.unchecked_into();
        if let Ok(b) = serde_wasm_bindgen::from_value::<RectDto<LengthPercentageDto>>(val) {
            self.inner.border = b.into();
        }
    }

    /// Gets the gap
    ///
    /// Spacing between flex/grid items.
    ///
    /// # Returns
    ///
    /// A `Size<LengthPercentage>` with column (width) and row (height) gaps
    #[wasm_bindgen(getter)]
    pub fn gap(&self) -> JsSizeLengthPercentage {
        let s: SizeDto<LengthPercentageDto> = SizeDto {
            width: self.inner.gap.width.into(),
            height: self.inner.gap.height.into(),
        };
        serialize(&s).unchecked_into()
    }

    /// Sets the gap
    ///
    /// # Arguments
    ///
    /// - `val`: A Size object with LengthPercentage gap values
    ///
    ///
    /// <details>
    /// <summary><strong>TypeScript Example</strong></summary>
    ///
    /// ```typescript
    /// style.gap = { width: { Length: 10 }, height: { Length: 10 } };
    /// ```
    ///
    /// </details>
    #[wasm_bindgen(setter)]
    pub fn set_gap(&mut self, val: JsSizeLengthPercentage) {
        let val: JsValue = val.unchecked_into();
        if let Ok(g) = serde_wasm_bindgen::from_value::<SizeDto<LengthPercentageDto>>(val) {
            self.inner.gap = g.into();
        }
    }

    /// Gets the inset
    ///
    /// Positioning offsets for absolutely positioned elements.
    ///
    /// # Returns
    ///
    /// A `Rect<LengthPercentageAuto>` with left, right, top, bottom offsets
    #[wasm_bindgen(getter)]
    pub fn inset(&self) -> JsRectLengthPercentageAuto {
        let m: RectDto<LengthPercentageAutoDto> = RectDto {
            left: self.inner.inset.left.into(),
            right: self.inner.inset.right.into(),
            top: self.inner.inset.top.into(),
            bottom: self.inner.inset.bottom.into(),
        };
        serialize(&m).unchecked_into()
    }

    /// Sets the inset
    ///
    /// # Arguments
    ///
    /// - `val`: A Rect object with LengthPercentageAuto offset values
    ///
    ///
    /// <details>
    /// <summary><strong>TypeScript Example</strong></summary>
    ///
    /// ```typescript
    /// style.position = Position.Absolute;
    /// style.inset = { left: { Length: 0 }, top: { Length: 0 }, right: "Auto", bottom: "Auto" };
    /// ```
    ///
    /// </details>
    #[wasm_bindgen(setter)]
    pub fn set_inset(&mut self, val: JsRectLengthPercentageAuto) {
        let val: JsValue = val.unchecked_into();
        if let Ok(i) = serde_wasm_bindgen::from_value::<RectDto<LengthPercentageAutoDto>>(val) {
            self.inner.inset = i.into();
        }
    }
}
