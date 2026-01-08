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
//! } from 'taffy-layout';
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
//!   width: 200,
//!   height: 100
//! };
//! style.size = size;
//!
//! const padding: Rect<LengthPercentage> = {
//!   left: 10,
//!   right: 10,
//!   top: 5,
//!   bottom: 5
//! };
//! style.padding = padding;
//!
//! const tree = new TaffyTree();
//! const nodeId: bigint = tree.newLeaf(style);
//! ```
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
//! - **Dimension**: `number`, `"{number}%"`, or `"auto"`
//! - **LengthPercentage**: `number` or `"{number}%"`
//! - **LengthPercentageAuto**: `number`, `"{number}%"`, or `"auto"`

use crate::enums::*;
use crate::types::*;
use crate::utils::log;
use crate::utils::serialize;
use taffy::style::{self as TaffyStyle};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

// =============================================================================
// Style Struct
// =============================================================================

/// CSS layout configuration for a node, including flexbox, sizing, spacing, and alignment properties.
///
/// This class holds all CSS layout properties for a node. Create an instance with
/// `new Style()` and configure properties before passing to `TaffyTree.newLeaf()`.
///
/// @defaultValue
/// When created, all properties are set to their CSS default values:
/// - `display`: `Display.Block`
/// - `position`: `Position.Relative`
/// - `flexDirection`: `FlexDirection.Row`
/// - `flexWrap`: `FlexWrap.NoWrap`
/// - `flexGrow`: `0`
/// - `flexShrink`: `1`
/// - All alignment properties: `undefined` (use default behavior)
/// - All dimensions: `"auto"`
/// - All spacing: `0`
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
    /// @returns - A new `Style` object with all properties set to CSS defaults
    ///
    /// @example
    /// ```typescript
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
    /// @returns - The current [`Display`](JsDisplay) value
    ///
    /// @defaultValue - `Display.Block`
    #[wasm_bindgen(getter)]
    pub fn display(&self) -> JsDisplay {
        self.inner.display.into()
    }

    /// Sets the display mode
    ///
    /// @param val - The new display mode
    ///
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.display = Display.Flex;
    /// ```
    #[wasm_bindgen(setter)]
    pub fn set_display(&mut self, val: JsDisplay) {
        self.inner.display = val.into();
    }

    /// Gets the position mode
    ///
    /// Determines how the element is positioned within its parent.
    ///
    /// @returns - The current [`Position`](JsPosition) value
    ///
    /// @defaultValue - `Position.Relative`
    #[wasm_bindgen(getter)]
    pub fn position(&self) -> JsPosition {
        self.inner.position.into()
    }

    /// Sets the position mode
    ///
    /// @param val - The new position mode
    ///
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.position = Position.Absolute;
    /// style.inset = { left: 10, top: 10, right: "auto", bottom: "auto" };
    /// ```
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
    /// @returns - The current [`FlexDirection`](JsFlexDirection) value
    ///
    /// @defaultValue - `FlexDirection.Row`
    #[wasm_bindgen(getter, js_name = flexDirection)]
    pub fn flex_direction(&self) -> JsFlexDirection {
        self.inner.flex_direction.into()
    }

    /// Sets the flex direction
    ///
    /// @param val - The new flex direction
    ///
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.flexDirection = FlexDirection.Column;
    /// ```
    #[wasm_bindgen(setter, js_name = flexDirection)]
    pub fn set_flex_direction(&mut self, val: JsFlexDirection) {
        self.inner.flex_direction = val.into();
    }

    /// Gets the flex wrap mode
    ///
    /// Controls whether flex items wrap to new lines.
    ///
    /// @returns - The current [`FlexWrap`](JsFlexWrap) value
    ///
    /// @defaultValue - `FlexWrap.NoWrap`
    #[wasm_bindgen(getter, js_name = flexWrap)]
    pub fn flex_wrap(&self) -> JsFlexWrap {
        self.inner.flex_wrap.into()
    }

    /// Sets the flex wrap mode
    ///
    /// @param val - The new flex wrap mode
    ///
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.flexWrap = FlexWrap.Wrap;
    /// ```
    #[wasm_bindgen(setter, js_name = flexWrap)]
    pub fn set_flex_wrap(&mut self, val: JsFlexWrap) {
        self.inner.flex_wrap = val.into();
    }

    /// Gets the flex grow factor
    ///
    /// Determines how much the item grows relative to siblings when
    /// there is extra space available.
    ///
    /// @returns - The flex grow factor (default: 0)
    #[wasm_bindgen(getter, js_name = flexGrow)]
    pub fn flex_grow(&self) -> f32 {
        self.inner.flex_grow
    }

    /// Sets the flex grow factor
    ///
    /// @param val - The new flex grow factor (must be >= 0)
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.flexGrow = 2;
    /// ```
    #[wasm_bindgen(setter, js_name = flexGrow)]
    pub fn set_flex_grow(&mut self, val: f32) {
        self.inner.flex_grow = val;
    }

    /// Gets the flex shrink factor
    ///
    /// Determines how much the item shrinks relative to siblings when
    /// there is insufficient space.
    ///
    /// @returns - The flex shrink factor (default: 1)
    #[wasm_bindgen(getter, js_name = flexShrink)]
    pub fn flex_shrink(&self) -> f32 {
        self.inner.flex_shrink
    }

    /// Sets the flex shrink factor
    ///
    /// @param val - The new flex shrink factor (must be >= 0)
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.flexShrink = 2;
    /// ```
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
    /// @returns - The current [`AlignItems`](JsAlignItems) value, or `undefined` if not set
    #[wasm_bindgen(getter, js_name = alignItems)]
    pub fn align_items(&self) -> Option<JsAlignItems> {
        self.inner.align_items.map(JsAlignItems::from)
    }

    /// Sets the align-items property
    ///
    /// @param val - The new align-items value, or `undefined` to use default
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.alignItems = AlignItems.Center;
    /// ```
    #[wasm_bindgen(setter, js_name = alignItems)]
    pub fn set_align_items(&mut self, val: JsOptionAlignItems) {
        let val: JsValue = val.unchecked_into();
        self.inner.align_items = if val.is_undefined() {
            None
        } else if let Some(n) = val.as_f64() {
            Some(unsafe { std::mem::transmute::<u8, JsAlignItems>(n as u8) }.into())
        } else {
            None
        };
    }

    /// Gets the align-self property
    ///
    /// Overrides the parent's align-items for this specific element.
    ///
    /// @returns - The current [`AlignSelf`](JsAlignSelf) value (returns `Auto` if not set)
    #[wasm_bindgen(getter, js_name = alignSelf)]
    pub fn align_self(&self) -> Option<JsAlignSelf> {
        match self.inner.align_self {
            Some(v) => Some(JsAlignSelf::from(v)),
            None => Some(JsAlignSelf::Auto),
        }
    }

    /// Sets the align-self property
    ///
    /// @param val - The new align-self value, or `undefined`/`Auto` to inherit from parent
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.alignSelf = AlignSelf.Stretch;
    /// ```
    #[wasm_bindgen(setter, js_name = alignSelf)]
    pub fn set_align_self(&mut self, val: JsOptionAlignSelf) {
        let val: JsValue = val.unchecked_into();
        self.inner.align_self = if val.is_undefined() {
            None
        } else if let Some(n) = val.as_f64() {
            let js_val = unsafe { std::mem::transmute::<u8, JsAlignSelf>(n as u8) };
            match js_val {
                JsAlignSelf::Auto => None,
                _ => Some(js_val.into()),
            }
        } else {
            None
        };
    }

    /// Gets the align-content property
    ///
    /// Controls distribution of space between lines in a multi-line flex container.
    ///
    /// @returns - The current [`AlignContent`](JsAlignContent) value, or `undefined` if not set
    #[wasm_bindgen(getter, js_name = alignContent)]
    pub fn align_content(&self) -> Option<JsAlignContent> {
        self.inner.align_content.map(JsAlignContent::from)
    }

    /// Sets the align-content property
    ///
    /// @param val - The new align-content value, or `undefined` to use default
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.alignContent = AlignContent.SpaceBetween;
    /// ```
    #[wasm_bindgen(setter, js_name = alignContent)]
    pub fn set_align_content(&mut self, val: JsOptionAlignContent) {
        let val: JsValue = val.unchecked_into();
        self.inner.align_content = if val.is_undefined() {
            None
        } else if let Some(n) = val.as_f64() {
            Some(unsafe { std::mem::transmute::<u8, JsAlignContent>(n as u8) }.into())
        } else {
            None
        };
    }

    /// Gets the justify-content property
    ///
    /// Defines alignment and spacing of items along the main axis.
    ///
    /// @returns - The current [`JustifyContent`](JsJustifyContent) value, or `undefined` if not set
    #[wasm_bindgen(getter, js_name = justifyContent)]
    pub fn justify_content(&self) -> Option<JsJustifyContent> {
        self.inner.justify_content.map(JsJustifyContent::from)
    }

    /// Sets the justify-content property
    ///
    /// @param val - The new justify-content value, or `undefined` to use default
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.justifyContent = JustifyContent.Center;
    /// ```
    #[wasm_bindgen(setter, js_name = justifyContent)]
    pub fn set_justify_content(&mut self, val: JsOptionJustifyContent) {
        let val: JsValue = val.unchecked_into();
        self.inner.justify_content = if val.is_undefined() {
            None
        } else if let Some(n) = val.as_f64() {
            Some(unsafe { std::mem::transmute::<u8, JsJustifyContent>(n as u8) }.into())
        } else {
            None
        };
    }

    // =========================================================================
    // Sizing Properties
    // =========================================================================

    /// Gets the aspect ratio
    ///
    /// The ratio of width to height. Used to maintain proportions.
    ///
    /// @returns - The aspect ratio value, or `undefined` if not set
    #[wasm_bindgen(getter, js_name = aspectRatio)]
    pub fn aspect_ratio(&self) -> Option<f32> {
        self.inner.aspect_ratio
    }

    /// Sets the aspect ratio
    ///
    /// @param val - The new aspect ratio (width/height), or `undefined` to clear
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.aspectRatio = 16 / 9;
    /// ```
    #[wasm_bindgen(setter, js_name = aspectRatio)]
    pub fn set_aspect_ratio(&mut self, val: JsOptionNumber) {
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
    /// @returns - A `Point<Overflow>` with `x` and `y` overflow settings
    #[wasm_bindgen(getter)]
    pub fn overflow(&self) -> JsPointOverflow {
        let s = PointOverflowDto {
            x: self.inner.overflow.x as u8,
            y: self.inner.overflow.y as u8,
        };
        serialize(&s).unchecked_into()
    }

    /// Sets the overflow behavior
    ///
    /// @param val - An object with `x` and `y` overflow values
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.overflow = { x: Overflow.Hidden, y: Overflow.Scroll };
    /// ```
    #[wasm_bindgen(setter)]
    pub fn set_overflow(&mut self, val: JsPointOverflow) {
        let val: JsValue = val.unchecked_into();
        if let Ok(s) = serde_wasm_bindgen::from_value::<PointOverflowDto>(val) {
            self.inner.overflow = s.into();
        }
    }

    /// Gets the box sizing mode
    ///
    /// Determines whether padding and border are included in dimensions.
    ///
    /// @returns - The current [`BoxSizing`](JsBoxSizing) value
    ///
    /// @defaultValue `BoxSizing.BorderBox`
    #[wasm_bindgen(getter, js_name = boxSizing)]
    pub fn box_sizing(&self) -> JsBoxSizing {
        self.inner.box_sizing.into()
    }

    /// Sets the box sizing mode
    ///
    /// @param val - The new box sizing mode
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.boxSizing = BoxSizing.ContentBox;
    /// ```
    #[wasm_bindgen(setter, js_name = boxSizing)]
    pub fn set_box_sizing(&mut self, val: JsBoxSizing) {
        self.inner.box_sizing = val.into();
    }

    /// Gets the flex-basis
    ///
    /// The initial size of a flex item before growing/shrinking.
    ///
    /// @returns - A `Dimension` value (`number`, `\"{number}%\"`, or `\"auto\"`)
    #[wasm_bindgen(getter, js_name = flexBasis)]
    pub fn flex_basis(&self) -> JsDimension {
        let d: DimensionDto = self.inner.flex_basis.into();
        serialize(&d).unchecked_into()
    }

    /// Sets the flex-basis
    ///
    /// @param val - The initial size as a Dimension
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.flexBasis = 100;
    /// ```
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
    /// @returns - A `Size<Dimension>` object with `width` and `height` properties
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
    /// @param val - A Size object with Dimension values
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.size = { width: 200, height: "100%" };
    /// ```
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
    /// @returns - A `Size<Dimension>` object with minimum width and height
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
    /// @param val - A Size object with minimum Dimension values
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.minSize = { width: 100, height: "auto" };
    /// ```
    #[wasm_bindgen(setter, js_name = minSize)]
    pub fn set_min_size(&mut self, val: JsSizeDimension) {
        let val: JsValue = val.unchecked_into();
        if let Ok(s) = serde_wasm_bindgen::from_value::<SizeDto<DimensionDto>>(val) {
            self.inner.min_size = s.into();
        }
    }

    /// Gets the maximum size constraints
    ///
    /// @returns - A `Size<Dimension>` object with maximum width and height
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
    /// @param val - A Size object with maximum Dimension values
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.maxSize = { width: "auto", height: 500 };
    /// ```
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
    /// @returns - A `Rect<LengthPercentageAuto>` with left, right, top, bottom margins
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
    /// @param val - A Rect object with LengthPercentageAuto values
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.margin = { left: 10, right: 10, top: 5, bottom: 5 };
    /// ```
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
    /// @returns - A `Rect<LengthPercentage>` with left, right, top, bottom padding
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
    /// @param val - A Rect object with LengthPercentage values
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.padding = { left: 20, right: 20, top: 10, bottom: 10 };
    /// ```
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
    /// @returns - A `Rect<LengthPercentage>` with left, right, top, bottom border widths
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
    /// @param val - A Rect object with LengthPercentage values
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.border = { left: 1, right: 1, top: 1, bottom: 1 };
    /// ```
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
    /// @returns - A `Size<LengthPercentage>` with column (width) and row (height) gaps
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
    /// @param val - A Size object with LengthPercentage gap values
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.gap = { width: 10, height: 10 };
    /// ```
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
    /// @returns - A `Rect<LengthPercentageAuto>` with left, right, top, bottom offsets
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
    /// @param val - A Rect object with LengthPercentageAuto offset values
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.position = Position.Absolute;
    /// style.inset = { left: 0, top: 0, right: "auto", bottom: "auto" };
    /// ```
    #[wasm_bindgen(setter)]
    pub fn set_inset(&mut self, val: JsRectLengthPercentageAuto) {
        let val: JsValue = val.unchecked_into();
        if let Ok(i) = serde_wasm_bindgen::from_value::<RectDto<LengthPercentageAutoDto>>(val) {
            self.inner.inset = i.into();
        }
    }

    // =========================================================================
    // Block Layout Properties
    // =========================================================================

    /// Gets whether this item is a table
    ///
    /// Table children are handled specially in block layout.
    ///
    /// @returns - Whether the item is treated as a table
    ///
    /// @defaultValue - `false`
    #[wasm_bindgen(getter, js_name = itemIsTable)]
    pub fn item_is_table(&self) -> bool {
        self.inner.item_is_table
    }

    /// Sets whether this item is a table
    ///
    /// @param val - Whether the item should be treated as a table
    #[wasm_bindgen(setter, js_name = itemIsTable)]
    pub fn set_item_is_table(&mut self, val: bool) {
        self.inner.item_is_table = val;
    }

    /// Gets whether this item is a replaced element
    ///
    /// Replaced elements have special sizing behavior (e.g., `<img>`, `<video>`).
    ///
    /// @returns - Whether the item is a replaced element
    ///
    /// @defaultValue - `false`
    #[wasm_bindgen(getter, js_name = itemIsReplaced)]
    pub fn item_is_replaced(&self) -> bool {
        self.inner.item_is_replaced
    }

    /// Sets whether this item is a replaced element
    ///
    /// @param val - Whether the item should be treated as a replaced element
    #[wasm_bindgen(setter, js_name = itemIsReplaced)]
    pub fn set_item_is_replaced(&mut self, val: bool) {
        self.inner.item_is_replaced = val;
    }

    /// Gets the scrollbar width
    ///
    /// The width of the scrollbar gutter when `overflow` is set to `Scroll`.
    ///
    /// @returns - The scrollbar width in pixels
    ///
    /// @defaultValue - `0`
    #[wasm_bindgen(getter, js_name = scrollbarWidth)]
    pub fn scrollbar_width(&self) -> f32 {
        self.inner.scrollbar_width
    }

    /// Sets the scrollbar width
    ///
    /// @param val - The scrollbar width in pixels
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.overflow = { x: Overflow.Scroll, y: Overflow.Scroll };
    /// style.scrollbarWidth = 15;
    /// ```
    #[wasm_bindgen(setter, js_name = scrollbarWidth)]
    pub fn set_scrollbar_width(&mut self, val: f32) {
        self.inner.scrollbar_width = val;
    }

    /// Gets the text-align property
    ///
    /// Used by block layout to implement legacy text alignment behavior.
    ///
    /// @returns - The current [`TextAlign`](JsTextAlign) value
    ///
    /// @defaultValue - `TextAlign.Auto`
    #[wasm_bindgen(getter, js_name = textAlign)]
    pub fn text_align(&self) -> JsTextAlign {
        self.inner.text_align.into()
    }

    /// Sets the text-align property
    ///
    /// @param val - The new text-align value
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.textAlign = TextAlign.LegacyCenter;
    /// ```
    #[wasm_bindgen(setter, js_name = textAlign)]
    pub fn set_text_align(&mut self, val: JsTextAlign) {
        self.inner.text_align = val.into();
    }

    // =========================================================================
    // Additional Alignment Properties
    // =========================================================================

    /// Gets the justify-items property
    ///
    /// Defines the default justify-self for all children in the inline axis.
    /// This is primarily used for CSS Grid layout.
    ///
    /// @returns - The current [`AlignItems`](JsAlignItems) value, or `undefined` if not set
    #[wasm_bindgen(getter, js_name = justifyItems)]
    pub fn justify_items(&self) -> Option<JsAlignItems> {
        self.inner.justify_items.map(JsAlignItems::from)
    }

    /// Sets the justify-items property
    ///
    /// @param val - The new justify-items value, or `undefined` to use default
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.display = Display.Grid;
    /// style.justifyItems = AlignItems.Center;
    /// ```
    #[wasm_bindgen(setter, js_name = justifyItems)]
    pub fn set_justify_items(&mut self, val: JsOptionAlignItems) {
        let val: JsValue = val.unchecked_into();
        self.inner.justify_items = if val.is_undefined() {
            None
        } else if let Some(n) = val.as_f64() {
            Some(unsafe { std::mem::transmute::<u8, JsAlignItems>(n as u8) }.into())
        } else {
            None
        };
    }

    /// Gets the justify-self property
    ///
    /// Overrides the parent's justify-items for this specific element in the inline axis.
    ///
    /// @returns - The current [`AlignSelf`](JsAlignSelf) value (returns `Auto` if not set)
    #[wasm_bindgen(getter, js_name = justifySelf)]
    pub fn justify_self(&self) -> Option<JsAlignSelf> {
        match self.inner.justify_self {
            Some(v) => Some(JsAlignSelf::from(v)),
            None => Some(JsAlignSelf::Auto),
        }
    }

    /// Sets the justify-self property
    ///
    /// @param val - The new justify-self value, or `undefined`/`Auto` to inherit from parent
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.justifySelf = AlignSelf.End;
    /// ```
    #[wasm_bindgen(setter, js_name = justifySelf)]
    pub fn set_justify_self(&mut self, val: JsOptionAlignSelf) {
        let val: JsValue = val.unchecked_into();
        self.inner.justify_self = if val.is_undefined() {
            None
        } else if let Some(n) = val.as_f64() {
            let js_val = unsafe { std::mem::transmute::<u8, JsAlignSelf>(n as u8) };
            match js_val {
                JsAlignSelf::Auto => None,
                _ => Some(js_val.into()),
            }
        } else {
            None
        };
    }

    // =========================================================================
    // Grid Layout Properties
    // =========================================================================

    /// Gets the grid-auto-flow property
    ///
    /// Controls how auto-placed items are inserted into the grid.
    ///
    /// @returns - The current [`GridAutoFlow`](JsGridAutoFlow) value
    ///
    /// @defaultValue - `GridAutoFlow.Row`
    #[wasm_bindgen(getter, js_name = gridAutoFlow)]
    pub fn grid_auto_flow(&self) -> JsGridAutoFlow {
        self.inner.grid_auto_flow.into()
    }

    /// Sets the grid-auto-flow property
    ///
    /// @param val - The new grid-auto-flow value
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.display = Display.Grid;
    /// style.gridAutoFlow = GridAutoFlow.Column;
    /// ```
    #[wasm_bindgen(setter, js_name = gridAutoFlow)]
    pub fn set_grid_auto_flow(&mut self, val: JsGridAutoFlow) {
        self.inner.grid_auto_flow = val.into();
    }

    /// Gets the grid-row property
    ///
    /// Defines which row in the grid the item should start and end at.
    /// Corresponds to CSS `grid-row` shorthand.
    ///
    /// @returns - A `Line<GridPlacement>` with start and end placements
    #[wasm_bindgen(getter, js_name = gridRow)]
    pub fn grid_row(&self) -> JsLineGridPlacement {
        let dto: LineGridPlacementDto = self.inner.grid_row.clone().into();
        serialize(&dto).unchecked_into()
    }

    /// Sets the grid-row property
    ///
    /// @param val - A Line object with start and end GridPlacement values
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.display = Display.Grid;
    /// // CSS: grid-row: 1 / 3
    /// style.gridRow = { start: 1, end: 3 };
    /// // CSS: grid-row: 2 / span 2
    /// style.gridRow = { start: 2, end: { span: 2 } };
    /// ```
    #[wasm_bindgen(setter, js_name = gridRow)]
    pub fn set_grid_row(&mut self, val: JsLineGridPlacement) {
        let val: JsValue = val.unchecked_into();
        if let Ok(dto) = serde_wasm_bindgen::from_value::<LineGridPlacementDto>(val) {
            self.inner.grid_row = dto.into();
        }
    }

    /// Gets the grid-column property
    ///
    /// Defines which column in the grid the item should start and end at.
    /// Corresponds to CSS `grid-column` shorthand.
    ///
    /// @returns - A `Line<GridPlacement>` with start and end placements
    #[wasm_bindgen(getter, js_name = gridColumn)]
    pub fn grid_column(&self) -> JsLineGridPlacement {
        let dto: LineGridPlacementDto = self.inner.grid_column.clone().into();
        serialize(&dto).unchecked_into()
    }

    /// Sets the grid-column property
    ///
    /// @param val - A Line object with start and end GridPlacement values
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.display = Display.Grid;
    /// // CSS: grid-column: 1 / 4
    /// style.gridColumn = { start: 1, end: 4 };
    /// // CSS: grid-column: auto / span 3
    /// style.gridColumn = { start: "auto", end: { span: 3 } };
    /// ```
    #[wasm_bindgen(setter, js_name = gridColumn)]
    pub fn set_grid_column(&mut self, val: JsLineGridPlacement) {
        let val: JsValue = val.unchecked_into();
        if let Ok(dto) = serde_wasm_bindgen::from_value::<LineGridPlacementDto>(val) {
            self.inner.grid_column = dto.into();
        }
    }

    /// Gets the grid-template-rows property
    ///
    /// Defines the track sizing functions (heights) of the grid rows.
    ///
    /// @returns - An array of `GridTrack` values
    #[wasm_bindgen(getter, js_name = gridTemplateRows)]
    pub fn grid_template_rows(&self) -> JsGridTemplateComponents {
        let tracks: Vec<GridTemplateComponentDto> = self
            .inner
            .grid_template_rows
            .iter()
            .cloned()
            .map(|t| t.into())
            .collect();
        serialize(&tracks).unchecked_into()
    }

    /// Sets the grid-template-rows property
    ///
    /// @param val - An array of GridTrack objects
    #[wasm_bindgen(setter, js_name = gridTemplateRows)]
    pub fn set_grid_template_rows(&mut self, val: JsGridTemplateComponents) {
        let val: JsValue = val.unchecked_into();
        if let Ok(tracks) = serde_wasm_bindgen::from_value::<Vec<GridTemplateComponentDto>>(val) {
            self.inner.grid_template_rows = tracks.into_iter().map(|t| t.into()).collect();
        }
    }

    /// Gets the grid-template-columns property
    ///
    /// Defines the track sizing functions (widths) of the grid columns.
    ///
    /// @returns - An array of `GridTrack` values
    #[wasm_bindgen(getter, js_name = gridTemplateColumns)]
    pub fn grid_template_columns(&self) -> JsGridTemplateComponents {
        let tracks: Vec<GridTemplateComponentDto> = self
            .inner
            .grid_template_columns
            .iter()
            .cloned()
            .map(|t| t.into())
            .collect();
        serialize(&tracks).unchecked_into()
    }

    /// Sets the grid-template-columns property
    ///
    /// @param val - An array of GridTrack objects
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.display = Display.Grid;
    /// style.gridTemplateColumns = [
    ///   { min: 200, max: 200 },
    ///   { min: "auto", max: "1fr" },
    ///   { min: "auto", max: "1fr" }
    /// ];
    /// ```
    #[wasm_bindgen(setter, js_name = gridTemplateColumns)]
    pub fn set_grid_template_columns(&mut self, val: JsGridTemplateComponents) {
        let val: JsValue = val.unchecked_into();
        if let Ok(tracks) = serde_wasm_bindgen::from_value::<Vec<GridTemplateComponentDto>>(val) {
            self.inner.grid_template_columns = tracks.into_iter().map(|t| t.into()).collect();
        }
    }

    /// Gets the grid-auto-rows property
    ///
    /// Defines the size of implicitly created rows.
    ///
    /// @returns - An array of track sizing functions
    #[wasm_bindgen(getter, js_name = gridAutoRows)]
    pub fn grid_auto_rows(&self) -> JsTrackSizingFunctions {
        let tracks: Vec<TrackSizingFunctionDto> = self
            .inner
            .grid_auto_rows
            .iter()
            .cloned()
            .map(|t| t.into())
            .collect();
        serialize(&tracks).unchecked_into()
    }

    /// Sets the grid-auto-rows property
    ///
    /// @param val - An array of track sizing functions for implicit rows
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.display = Display.Grid;
    /// style.gridAutoRows = [{ min: "auto", max: "auto" }];
    /// ```
    #[wasm_bindgen(setter, js_name = gridAutoRows)]
    pub fn set_grid_auto_rows(&mut self, val: JsTrackSizingFunctions) {
        let val: JsValue = val.unchecked_into();
        if let Ok(tracks) = serde_wasm_bindgen::from_value::<Vec<TrackSizingFunctionDto>>(val) {
            self.inner.grid_auto_rows = tracks.into_iter().map(|t| t.into()).collect();
        }
    }

    /// Gets the grid-auto-columns property
    ///
    /// Defines the size of implicitly created columns.
    ///
    /// @returns - An array of track sizing functions
    #[wasm_bindgen(getter, js_name = gridAutoColumns)]
    pub fn grid_auto_columns(&self) -> JsTrackSizingFunctions {
        let tracks: Vec<TrackSizingFunctionDto> = self
            .inner
            .grid_auto_columns
            .iter()
            .cloned()
            .map(|t| t.into())
            .collect();
        serialize(&tracks).unchecked_into()
    }

    /// Sets the grid-auto-columns property
    ///
    /// @param val - An array of track sizing functions for implicit columns
    #[wasm_bindgen(setter, js_name = gridAutoColumns)]
    pub fn set_grid_auto_columns(&mut self, val: JsTrackSizingFunctions) {
        let val: JsValue = val.unchecked_into();
        if let Ok(tracks) = serde_wasm_bindgen::from_value::<Vec<TrackSizingFunctionDto>>(val) {
            self.inner.grid_auto_columns = tracks.into_iter().map(|t| t.into()).collect();
        }
    }

    /// Gets the grid-template-areas property
    ///
    /// Defines named grid areas that can be referenced by grid items.
    ///
    /// @returns - An array of `GridArea` values
    #[wasm_bindgen(getter, js_name = gridTemplateAreas)]
    pub fn grid_template_areas(&self) -> JsGridTemplateAreas {
        let areas: Vec<crate::types::GridTemplateAreaDto> = self
            .inner
            .grid_template_areas
            .iter()
            .cloned()
            .map(|a| a.into())
            .collect();
        serialize(&areas).unchecked_into()
    }

    /// Sets the grid-template-areas property
    ///
    /// @param val - An array of named grid area definitions
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.display = Display.Grid;
    /// style.gridTemplateAreas = [
    ///   { name: "header", rowStart: 1, rowEnd: 2, columnStart: 1, columnEnd: 4 },
    ///   { name: "main", rowStart: 2, rowEnd: 4, columnStart: 2, columnEnd: 4 }
    /// ];
    /// ```
    #[wasm_bindgen(setter, js_name = gridTemplateAreas)]
    pub fn set_grid_template_areas(&mut self, val: JsGridTemplateAreas) {
        let val: JsValue = val.unchecked_into();
        if let Ok(areas) =
            serde_wasm_bindgen::from_value::<Vec<crate::types::GridTemplateAreaDto>>(val)
        {
            self.inner.grid_template_areas = areas.into_iter().map(|a| a.into()).collect();
        }
    }

    /// Gets the grid-template-row-names property
    ///
    /// Defines the named lines between the rows.
    ///
    /// @returns - An array of arrays of line names
    #[wasm_bindgen(getter, js_name = gridTemplateRowNames)]
    pub fn grid_template_row_names(&self) -> JsGridLineNames {
        let names: Vec<Vec<String>> = self
            .inner
            .grid_template_row_names
            .iter()
            .map(|v| {
                v.iter()
                    .map(|s| AsRef::<str>::as_ref(s).to_string())
                    .collect()
            })
            .collect();
        serialize(&names).unchecked_into()
    }

    /// Sets the grid-template-row-names property
    ///
    /// @param val - An array of arrays of line names
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.gridTemplateRowNames = [["header-start"], ["header-end", "main-start"], ["main-end"]];
    /// ```
    #[wasm_bindgen(setter, js_name = gridTemplateRowNames)]
    pub fn set_grid_template_row_names(&mut self, val: JsGridLineNames) {
        let val: JsValue = val.unchecked_into();
        if let Ok(names) = serde_wasm_bindgen::from_value::<Vec<Vec<String>>>(val) {
            self.inner.grid_template_row_names = names
                .into_iter()
                .map(|v| v.into_iter().map(|s| s.into()).collect())
                .collect();
        }
    }

    /// Gets the grid-template-column-names property
    ///
    /// Defines the named lines between the columns.
    ///
    /// @returns - An array of arrays of line names
    #[wasm_bindgen(getter, js_name = gridTemplateColumnNames)]
    pub fn grid_template_column_names(&self) -> JsGridLineNames {
        let names: Vec<Vec<String>> = self
            .inner
            .grid_template_column_names
            .iter()
            .map(|v| {
                v.iter()
                    .map(|s| AsRef::<str>::as_ref(s).to_string())
                    .collect()
            })
            .collect();
        serialize(&names).unchecked_into()
    }

    /// Sets the grid-template-column-names property
    ///
    /// @param val - An array of arrays of line names
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.gridTemplateColumnNames = [["sidebar-start"], ["sidebar-end", "main-start"], ["main-end"]];
    /// ```
    #[wasm_bindgen(setter, js_name = gridTemplateColumnNames)]
    pub fn set_grid_template_column_names(&mut self, val: JsGridLineNames) {
        let val: JsValue = val.unchecked_into();
        if let Ok(names) = serde_wasm_bindgen::from_value::<Vec<Vec<String>>>(val) {
            self.inner.grid_template_column_names = names
                .into_iter()
                .map(|v| v.into_iter().map(|s| s.into()).collect())
                .collect();
        }
    }
}
