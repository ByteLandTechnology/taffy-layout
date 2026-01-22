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
//! ## Constructor with Initial Properties
//!
//! You can also create a `Style` with initial properties:
//!
//! ```typescript
//! const style = new Style({
//!   display: Display.Flex,
//!   flexDirection: FlexDirection.Column,
//!   "size.width": 200,
//!   "margin.left": 10
//! });
//! ```
//!
//! ## Batch Property Operations
//!
//! The `Style.get()` and `Style.set()` methods allow reading and writing multiple
//! properties in a single WASM call for better performance.
//!
//! @example
//! ```typescript
//! const style = new Style();
//!
//! // Set multiple properties at once
//! style.set({
//!   display: Display.Flex,
//!   flexDirection: FlexDirection.Column,
//!   "size.width": 200,
//!   "margin.left": 10
//! });
//!
//! // Get a single property
//! const d = style.get("display");
//!
//! // Get multiple properties with destructuring
//! const [display, flexDirection, width] = style.get("display", "flexDirection", "size.width");
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
    /// @param props - Optional object with initial style properties
    /// @returns - A new `Style` object with all properties set to CSS defaults
    ///
    /// @example
    /// ```typescript
    /// // Create with defaults
    /// const style = new Style();
    /// console.log(style.display);  // Display.Block
    ///
    /// // Create with initial properties
    /// const style2 = new Style({
    ///   display: Display.Flex,
    ///   flexDirection: FlexDirection.Column,
    ///   "size.width": 200,
    ///   "margin.left": 10
    /// });
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new(props: Option<JsValue>) -> JsStyle {
        let mut style = JsStyle {
            inner: TaffyStyle::Style::default(),
        };

        if let Some(props_value) = props {
            if props_value.is_object() {
                style.set(props_value);
            }
        }

        style
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

    /// Gets the horizontal overflow behavior
    ///
    /// @returns - The current [`Overflow`](JsOverflow) value for the x-axis
    #[wasm_bindgen(getter, js_name = overflowX)]
    pub fn overflow_x(&self) -> JsOverflow {
        self.inner.overflow.x.into()
    }

    /// Sets the horizontal overflow behavior
    ///
    /// @param val - The new overflow value for the x-axis
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.overflowX = Overflow.Hidden;
    /// ```
    #[wasm_bindgen(setter, js_name = overflowX)]
    pub fn set_overflow_x(&mut self, val: JsOverflow) {
        self.inner.overflow.x = val.into();
    }

    /// Gets the vertical overflow behavior
    ///
    /// @returns - The current [`Overflow`](JsOverflow) value for the y-axis
    #[wasm_bindgen(getter, js_name = overflowY)]
    pub fn overflow_y(&self) -> JsOverflow {
        self.inner.overflow.y.into()
    }

    /// Sets the vertical overflow behavior
    ///
    /// @param val - The new overflow value for the y-axis
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.overflowY = Overflow.Scroll;
    /// ```
    #[wasm_bindgen(setter, js_name = overflowY)]
    pub fn set_overflow_y(&mut self, val: JsOverflow) {
        self.inner.overflow.y = val.into();
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

    /// Gets the width
    ///
    /// @returns - The current width as a [`Dimension`](JsDimension)
    #[wasm_bindgen(getter)]
    pub fn width(&self) -> JsDimension {
        let d: DimensionDto = self.inner.size.width.into();
        serialize(&d).unchecked_into()
    }

    /// Sets the width
    ///
    /// @param val - The new width value
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.width = 200;
    /// style.width = "50%";
    /// style.width = "auto";
    /// ```
    #[wasm_bindgen(setter)]
    pub fn set_width(&mut self, val: JsDimension) {
        let val: JsValue = val.unchecked_into();
        if let Ok(d) = serde_wasm_bindgen::from_value::<DimensionDto>(val) {
            self.inner.size.width = d.into();
        }
    }

    /// Gets the height
    ///
    /// @returns - The current height as a [`Dimension`](JsDimension)
    #[wasm_bindgen(getter)]
    pub fn height(&self) -> JsDimension {
        let d: DimensionDto = self.inner.size.height.into();
        serialize(&d).unchecked_into()
    }

    /// Sets the height
    ///
    /// @param val - The new height value
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.height = 100;
    /// style.height = "75%";
    /// style.height = "auto";
    /// ```
    #[wasm_bindgen(setter)]
    pub fn set_height(&mut self, val: JsDimension) {
        let val: JsValue = val.unchecked_into();
        if let Ok(d) = serde_wasm_bindgen::from_value::<DimensionDto>(val) {
            self.inner.size.height = d.into();
        }
    }

    /// Gets the minimum width
    ///
    /// @returns - The current minimum width as a [`Dimension`](JsDimension)
    #[wasm_bindgen(getter, js_name = minWidth)]
    pub fn min_width(&self) -> JsDimension {
        let d: DimensionDto = self.inner.min_size.width.into();
        serialize(&d).unchecked_into()
    }

    /// Sets the minimum width
    ///
    /// @param val - The new minimum width value
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.minWidth = 100;
    /// style.minWidth = "auto";
    /// ```
    #[wasm_bindgen(setter, js_name = minWidth)]
    pub fn set_min_width(&mut self, val: JsDimension) {
        let val: JsValue = val.unchecked_into();
        if let Ok(d) = serde_wasm_bindgen::from_value::<DimensionDto>(val) {
            self.inner.min_size.width = d.into();
        }
    }

    /// Gets the minimum height
    ///
    /// @returns - The current minimum height as a [`Dimension`](JsDimension)
    #[wasm_bindgen(getter, js_name = minHeight)]
    pub fn min_height(&self) -> JsDimension {
        let d: DimensionDto = self.inner.min_size.height.into();
        serialize(&d).unchecked_into()
    }

    /// Sets the minimum height
    ///
    /// @param val - The new minimum height value
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.minHeight = 50;
    /// style.minHeight = "auto";
    /// ```
    #[wasm_bindgen(setter, js_name = minHeight)]
    pub fn set_min_height(&mut self, val: JsDimension) {
        let val: JsValue = val.unchecked_into();
        if let Ok(d) = serde_wasm_bindgen::from_value::<DimensionDto>(val) {
            self.inner.min_size.height = d.into();
        }
    }

    /// Gets the maximum width
    ///
    /// @returns - The current maximum width as a [`Dimension`](JsDimension)
    #[wasm_bindgen(getter, js_name = maxWidth)]
    pub fn max_width(&self) -> JsDimension {
        let d: DimensionDto = self.inner.max_size.width.into();
        serialize(&d).unchecked_into()
    }

    /// Sets the maximum width
    ///
    /// @param val - The new maximum width value
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.maxWidth = 500;
    /// style.maxWidth = "auto";
    /// ```
    #[wasm_bindgen(setter, js_name = maxWidth)]
    pub fn set_max_width(&mut self, val: JsDimension) {
        let val: JsValue = val.unchecked_into();
        if let Ok(d) = serde_wasm_bindgen::from_value::<DimensionDto>(val) {
            self.inner.max_size.width = d.into();
        }
    }

    /// Gets the maximum height
    ///
    /// @returns - The current maximum height as a [`Dimension`](JsDimension)
    #[wasm_bindgen(getter, js_name = maxHeight)]
    pub fn max_height(&self) -> JsDimension {
        let d: DimensionDto = self.inner.max_size.height.into();
        serialize(&d).unchecked_into()
    }

    /// Sets the maximum height
    ///
    /// @param val - The new maximum height value
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.maxHeight = 300;
    /// style.maxHeight = "auto";
    /// ```
    #[wasm_bindgen(setter, js_name = maxHeight)]
    pub fn set_max_height(&mut self, val: JsDimension) {
        let val: JsValue = val.unchecked_into();
        if let Ok(d) = serde_wasm_bindgen::from_value::<DimensionDto>(val) {
            self.inner.max_size.height = d.into();
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

    /// Gets the left margin
    ///
    /// @returns - The current left margin as a [`LengthPercentageAuto`](JsLengthPercentageAuto)
    #[wasm_bindgen(getter, js_name = marginLeft)]
    pub fn margin_left(&self) -> JsValue {
        let d: LengthPercentageAutoDto = self.inner.margin.left.into();
        serialize(&d).unchecked_into()
    }

    /// Sets the left margin
    ///
    /// @param val - The new left margin value
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.marginLeft = 10;
    /// style.marginLeft = "5%";
    /// style.marginLeft = "auto";
    /// ```
    #[wasm_bindgen(setter, js_name = marginLeft)]
    pub fn set_margin_left(&mut self, val: JsValue) {
        let val: JsValue = val.unchecked_into();
        if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageAutoDto>(val) {
            self.inner.margin.left = d.into();
        }
    }

    /// Gets the right margin
    ///
    /// @returns - The current right margin as a [`LengthPercentageAuto`](JsLengthPercentageAuto)
    #[wasm_bindgen(getter, js_name = marginRight)]
    pub fn margin_right(&self) -> JsValue {
        let d: LengthPercentageAutoDto = self.inner.margin.right.into();
        serialize(&d).unchecked_into()
    }

    /// Sets the right margin
    ///
    /// @param val - The new right margin value
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.marginRight = 10;
    /// style.marginRight = "5%";
    /// style.marginRight = "auto";
    /// ```
    #[wasm_bindgen(setter, js_name = marginRight)]
    pub fn set_margin_right(&mut self, val: JsValue) {
        let val: JsValue = val.unchecked_into();
        if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageAutoDto>(val) {
            self.inner.margin.right = d.into();
        }
    }

    /// Gets the top margin
    ///
    /// @returns - The current top margin as a [`LengthPercentageAuto`](JsLengthPercentageAuto)
    #[wasm_bindgen(getter, js_name = marginTop)]
    pub fn margin_top(&self) -> JsValue {
        let d: LengthPercentageAutoDto = self.inner.margin.top.into();
        serialize(&d).unchecked_into()
    }

    /// Sets the top margin
    ///
    /// @param val - The new top margin value
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.marginTop = 5;
    /// style.marginTop = "10%";
    /// style.marginTop = "auto";
    /// ```
    #[wasm_bindgen(setter, js_name = marginTop)]
    pub fn set_margin_top(&mut self, val: JsValue) {
        let val: JsValue = val.unchecked_into();
        if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageAutoDto>(val) {
            self.inner.margin.top = d.into();
        }
    }

    /// Gets the bottom margin
    ///
    /// @returns - The current bottom margin as a [`LengthPercentageAuto`](JsLengthPercentageAuto)
    #[wasm_bindgen(getter, js_name = marginBottom)]
    pub fn margin_bottom(&self) -> JsValue {
        let d: LengthPercentageAutoDto = self.inner.margin.bottom.into();
        serialize(&d).unchecked_into()
    }

    /// Sets the bottom margin
    ///
    /// @param val - The new bottom margin value
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.marginBottom = 5;
    /// style.marginBottom = "10%";
    /// style.marginBottom = "auto";
    /// ```
    #[wasm_bindgen(setter, js_name = marginBottom)]
    pub fn set_margin_bottom(&mut self, val: JsValue) {
        let val: JsValue = val.unchecked_into();
        if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageAutoDto>(val) {
            self.inner.margin.bottom = d.into();
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

    /// Gets the left padding
    ///
    /// @returns - The current left padding as a [`LengthPercentage`](JsLengthPercentage)
    #[wasm_bindgen(getter, js_name = paddingLeft)]
    pub fn padding_left(&self) -> JsValue {
        let d: LengthPercentageDto = self.inner.padding.left.into();
        serialize(&d).unchecked_into()
    }

    /// Sets the left padding
    ///
    /// @param val - The new left padding value
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.paddingLeft = 20;
    /// style.paddingLeft = "10%";
    /// ```
    #[wasm_bindgen(setter, js_name = paddingLeft)]
    pub fn set_padding_left(&mut self, val: JsValue) {
        let val: JsValue = val.unchecked_into();
        if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageDto>(val) {
            self.inner.padding.left = d.into();
        }
    }

    /// Gets the right padding
    ///
    /// @returns - The current right padding as a [`LengthPercentage`](JsLengthPercentage)
    #[wasm_bindgen(getter, js_name = paddingRight)]
    pub fn padding_right(&self) -> JsValue {
        let d: LengthPercentageDto = self.inner.padding.right.into();
        serialize(&d).unchecked_into()
    }

    /// Sets the right padding
    ///
    /// @param val - The new right padding value
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.paddingRight = 20;
    /// style.paddingRight = "10%";
    /// ```
    #[wasm_bindgen(setter, js_name = paddingRight)]
    pub fn set_padding_right(&mut self, val: JsValue) {
        let val: JsValue = val.unchecked_into();
        if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageDto>(val) {
            self.inner.padding.right = d.into();
        }
    }

    /// Gets the top padding
    ///
    /// @returns - The current top padding as a [`LengthPercentage`](JsLengthPercentage)
    #[wasm_bindgen(getter, js_name = paddingTop)]
    pub fn padding_top(&self) -> JsValue {
        let d: LengthPercentageDto = self.inner.padding.top.into();
        serialize(&d).unchecked_into()
    }

    /// Sets the top padding
    ///
    /// @param val - The new top padding value
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.paddingTop = 10;
    /// style.paddingTop = "5%";
    /// ```
    #[wasm_bindgen(setter, js_name = paddingTop)]
    pub fn set_padding_top(&mut self, val: JsValue) {
        let val: JsValue = val.unchecked_into();
        if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageDto>(val) {
            self.inner.padding.top = d.into();
        }
    }

    /// Gets the bottom padding
    ///
    /// @returns - The current bottom padding as a [`LengthPercentage`](JsLengthPercentage)
    #[wasm_bindgen(getter, js_name = paddingBottom)]
    pub fn padding_bottom(&self) -> JsValue {
        let d: LengthPercentageDto = self.inner.padding.bottom.into();
        serialize(&d).unchecked_into()
    }

    /// Sets the bottom padding
    ///
    /// @param val - The new bottom padding value
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.paddingBottom = 10;
    /// style.paddingBottom = "5%";
    /// ```
    #[wasm_bindgen(setter, js_name = paddingBottom)]
    pub fn set_padding_bottom(&mut self, val: JsValue) {
        let val: JsValue = val.unchecked_into();
        if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageDto>(val) {
            self.inner.padding.bottom = d.into();
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

    /// Gets the left border width
    ///
    /// @returns - The current left border width as a [`LengthPercentage`](JsLengthPercentage)
    #[wasm_bindgen(getter, js_name = borderLeft)]
    pub fn border_left(&self) -> JsValue {
        let d: LengthPercentageDto = self.inner.border.left.into();
        serialize(&d).unchecked_into()
    }

    /// Sets the left border width
    ///
    /// @param val - The new left border width value
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.borderLeft = 1;
    /// style.borderLeft = "2%";
    /// ```
    #[wasm_bindgen(setter, js_name = borderLeft)]
    pub fn set_border_left(&mut self, val: JsValue) {
        let val: JsValue = val.unchecked_into();
        if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageDto>(val) {
            self.inner.border.left = d.into();
        }
    }

    /// Gets the right border width
    ///
    /// @returns - The current right border width as a [`LengthPercentage`](JsLengthPercentage)
    #[wasm_bindgen(getter, js_name = borderRight)]
    pub fn border_right(&self) -> JsValue {
        let d: LengthPercentageDto = self.inner.border.right.into();
        serialize(&d).unchecked_into()
    }

    /// Sets the right border width
    ///
    /// @param val - The new right border width value
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.borderRight = 1;
    /// style.borderRight = "2%";
    /// ```
    #[wasm_bindgen(setter, js_name = borderRight)]
    pub fn set_border_right(&mut self, val: JsValue) {
        let val: JsValue = val.unchecked_into();
        if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageDto>(val) {
            self.inner.border.right = d.into();
        }
    }

    /// Gets the top border width
    ///
    /// @returns - The current top border width as a [`LengthPercentage`](JsLengthPercentage)
    #[wasm_bindgen(getter, js_name = borderTop)]
    pub fn border_top(&self) -> JsValue {
        let d: LengthPercentageDto = self.inner.border.top.into();
        serialize(&d).unchecked_into()
    }

    /// Sets the top border width
    ///
    /// @param val - The new top border width value
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.borderTop = 1;
    /// style.borderTop = "2%";
    /// ```
    #[wasm_bindgen(setter, js_name = borderTop)]
    pub fn set_border_top(&mut self, val: JsValue) {
        let val: JsValue = val.unchecked_into();
        if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageDto>(val) {
            self.inner.border.top = d.into();
        }
    }

    /// Gets the bottom border width
    ///
    /// @returns - The current bottom border width as a [`LengthPercentage`](JsLengthPercentage)
    #[wasm_bindgen(getter, js_name = borderBottom)]
    pub fn border_bottom(&self) -> JsValue {
        let d: LengthPercentageDto = self.inner.border.bottom.into();
        serialize(&d).unchecked_into()
    }

    /// Sets the bottom border width
    ///
    /// @param val - The new bottom border width value
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.borderBottom = 1;
    /// style.borderBottom = "2%";
    /// ```
    #[wasm_bindgen(setter, js_name = borderBottom)]
    pub fn set_border_bottom(&mut self, val: JsValue) {
        let val: JsValue = val.unchecked_into();
        if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageDto>(val) {
            self.inner.border.bottom = d.into();
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

    /// Gets the column gap (horizontal spacing between items)
    ///
    /// @returns - The current column gap as a [`LengthPercentage`](JsLengthPercentage)
    #[wasm_bindgen(getter, js_name = columnGap)]
    pub fn column_gap(&self) -> JsValue {
        let d: LengthPercentageDto = self.inner.gap.width.into();
        serialize(&d).unchecked_into()
    }

    /// Sets the column gap (horizontal spacing between items)
    ///
    /// @param val - The new column gap value
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.columnGap = 10;
    /// style.columnGap = "5%";
    /// ```
    #[wasm_bindgen(setter, js_name = columnGap)]
    pub fn set_column_gap(&mut self, val: JsValue) {
        let val: JsValue = val.unchecked_into();
        if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageDto>(val) {
            self.inner.gap.width = d.into();
        }
    }

    /// Gets the row gap (vertical spacing between items)
    ///
    /// @returns - The current row gap as a [`LengthPercentage`](JsLengthPercentage)
    #[wasm_bindgen(getter, js_name = rowGap)]
    pub fn row_gap(&self) -> JsValue {
        let d: LengthPercentageDto = self.inner.gap.height.into();
        serialize(&d).unchecked_into()
    }

    /// Sets the row gap (vertical spacing between items)
    ///
    /// @param val - The new row gap value
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.rowGap = 10;
    /// style.rowGap = "5%";
    /// ```
    #[wasm_bindgen(setter, js_name = rowGap)]
    pub fn set_row_gap(&mut self, val: JsValue) {
        let val: JsValue = val.unchecked_into();
        if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageDto>(val) {
            self.inner.gap.height = d.into();
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

    /// Gets the left inset offset
    ///
    /// @returns - The current left offset as a [`LengthPercentageAuto`](JsLengthPercentageAuto)
    #[wasm_bindgen(getter)]
    pub fn left(&self) -> JsValue {
        let d: LengthPercentageAutoDto = self.inner.inset.left.into();
        serialize(&d).unchecked_into()
    }

    /// Sets the left inset offset
    ///
    /// @param val - The new left offset value
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.position = Position.Absolute;
    /// style.left = 0;
    /// style.left = "10%";
    /// style.left = "auto";
    /// ```
    #[wasm_bindgen(setter)]
    pub fn set_left(&mut self, val: JsValue) {
        let val: JsValue = val.unchecked_into();
        if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageAutoDto>(val) {
            self.inner.inset.left = d.into();
        }
    }

    /// Gets the right inset offset
    ///
    /// @returns - The current right offset as a [`LengthPercentageAuto`](JsLengthPercentageAuto)
    #[wasm_bindgen(getter)]
    pub fn right(&self) -> JsValue {
        let d: LengthPercentageAutoDto = self.inner.inset.right.into();
        serialize(&d).unchecked_into()
    }

    /// Sets the right inset offset
    ///
    /// @param val - The new right offset value
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.position = Position.Absolute;
    /// style.right = 0;
    /// style.right = "10%";
    /// style.right = "auto";
    /// ```
    #[wasm_bindgen(setter)]
    pub fn set_right(&mut self, val: JsValue) {
        let val: JsValue = val.unchecked_into();
        if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageAutoDto>(val) {
            self.inner.inset.right = d.into();
        }
    }

    /// Gets the top inset offset
    ///
    /// @returns - The current top offset as a [`LengthPercentageAuto`](JsLengthPercentageAuto)
    #[wasm_bindgen(getter)]
    pub fn top(&self) -> JsValue {
        let d: LengthPercentageAutoDto = self.inner.inset.top.into();
        serialize(&d).unchecked_into()
    }

    /// Sets the top inset offset
    ///
    /// @param val - The new top offset value
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.position = Position.Absolute;
    /// style.top = 0;
    /// style.top = "10%";
    /// style.top = "auto";
    /// ```
    #[wasm_bindgen(setter)]
    pub fn set_top(&mut self, val: JsValue) {
        let val: JsValue = val.unchecked_into();
        if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageAutoDto>(val) {
            self.inner.inset.top = d.into();
        }
    }

    /// Gets the bottom inset offset
    ///
    /// @returns - The current bottom offset as a [`LengthPercentageAuto`](JsLengthPercentageAuto)
    #[wasm_bindgen(getter)]
    pub fn bottom(&self) -> JsValue {
        let d: LengthPercentageAutoDto = self.inner.inset.bottom.into();
        serialize(&d).unchecked_into()
    }

    /// Sets the bottom inset offset
    ///
    /// @param val - The new bottom offset value
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.position = Position.Absolute;
    /// style.bottom = 0;
    /// style.bottom = "10%";
    /// style.bottom = "auto";
    /// ```
    #[wasm_bindgen(setter)]
    pub fn set_bottom(&mut self, val: JsValue) {
        let val: JsValue = val.unchecked_into();
        if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageAutoDto>(val) {
            self.inner.inset.bottom = d.into();
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

    /// Gets the grid-row-start property
    ///
    /// @returns - The current grid row start placement as a [`GridPlacement`](JsGridPlacement)
    #[wasm_bindgen(getter, js_name = gridRowStart)]
    pub fn grid_row_start(&self) -> JsValue {
        let dto: GridPlacementDto = self.inner.grid_row.start.clone().into();
        serialize(&dto).unchecked_into()
    }

    /// Sets the grid-row-start property
    ///
    /// @param val - The new grid row start placement
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.display = Display.Grid;
    /// style.gridRowStart = 1;
    /// style.gridRowStart = "auto";
    /// style.gridRowStart = { span: 2 };
    /// ```
    #[wasm_bindgen(setter, js_name = gridRowStart)]
    pub fn set_grid_row_start(&mut self, val: JsValue) {
        let val: JsValue = val.unchecked_into();
        if let Ok(dto) = serde_wasm_bindgen::from_value::<GridPlacementDto>(val) {
            self.inner.grid_row.start = dto.into();
        }
    }

    /// Gets the grid-row-end property
    ///
    /// @returns - The current grid row end placement as a [`GridPlacement`](JsGridPlacement)
    #[wasm_bindgen(getter, js_name = gridRowEnd)]
    pub fn grid_row_end(&self) -> JsValue {
        let dto: GridPlacementDto = self.inner.grid_row.end.clone().into();
        serialize(&dto).unchecked_into()
    }

    /// Sets the grid-row-end property
    ///
    /// @param val - The new grid row end placement
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.display = Display.Grid;
    /// style.gridRowEnd = 3;
    /// style.gridRowEnd = "auto";
    /// style.gridRowEnd = { span: 2 };
    /// ```
    #[wasm_bindgen(setter, js_name = gridRowEnd)]
    pub fn set_grid_row_end(&mut self, val: JsValue) {
        let val: JsValue = val.unchecked_into();
        if let Ok(dto) = serde_wasm_bindgen::from_value::<GridPlacementDto>(val) {
            self.inner.grid_row.end = dto.into();
        }
    }

    /// Gets the grid-column-start property
    ///
    /// @returns - The current grid column start placement as a [`GridPlacement`](JsGridPlacement)
    #[wasm_bindgen(getter, js_name = gridColumnStart)]
    pub fn grid_column_start(&self) -> JsValue {
        let dto: GridPlacementDto = self.inner.grid_column.start.clone().into();
        serialize(&dto).unchecked_into()
    }

    /// Sets the grid-column-start property
    ///
    /// @param val - The new grid column start placement
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.display = Display.Grid;
    /// style.gridColumnStart = 1;
    /// style.gridColumnStart = "auto";
    /// style.gridColumnStart = { span: 2 };
    /// ```
    #[wasm_bindgen(setter, js_name = gridColumnStart)]
    pub fn set_grid_column_start(&mut self, val: JsValue) {
        let val: JsValue = val.unchecked_into();
        if let Ok(dto) = serde_wasm_bindgen::from_value::<GridPlacementDto>(val) {
            self.inner.grid_column.start = dto.into();
        }
    }

    /// Gets the grid-column-end property
    ///
    /// @returns - The current grid column end placement as a [`GridPlacement`](JsGridPlacement)
    #[wasm_bindgen(getter, js_name = gridColumnEnd)]
    pub fn grid_column_end(&self) -> JsValue {
        let dto: GridPlacementDto = self.inner.grid_column.end.clone().into();
        serialize(&dto).unchecked_into()
    }

    /// Sets the grid-column-end property
    ///
    /// @param val - The new grid column end placement
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.display = Display.Grid;
    /// style.gridColumnEnd = 4;
    /// style.gridColumnEnd = "auto";
    /// style.gridColumnEnd = { span: 3 };
    /// ```
    #[wasm_bindgen(setter, js_name = gridColumnEnd)]
    pub fn set_grid_column_end(&mut self, val: JsValue) {
        let val: JsValue = val.unchecked_into();
        if let Ok(dto) = serde_wasm_bindgen::from_value::<GridPlacementDto>(val) {
            self.inner.grid_column.end = dto.into();
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

    // =========================================================================
    // Batch Property Reading
    // =========================================================================

    /// Reads multiple style properties in a single WASM call.
    ///
    /// Supports dot notation for nested properties (e.g., `"size.width"`, `"margin.left"`).
    ///
    /// @param keys - Property paths to read
    /// @returns - Single value if one key, array of values if multiple keys
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    /// style.display = Display.Flex;
    /// style.size = { width: 100, height: "50%" };
    ///
    /// // Read single property
    /// const d = style.get("display");
    ///
    /// // Read nested property
    /// const w = style.get("size.width");
    ///
    /// // Read multiple properties with destructuring
    /// const [display, width, margin] = style.get("display", "size.width", "margin.left");
    /// ```
    #[wasm_bindgen(variadic)]
    pub fn get(&self, keys: Vec<String>) -> JsValue {
        if keys.is_empty() {
            return JsValue::UNDEFINED;
        }

        let results: Vec<JsValue> = keys.iter().map(|key| self.get_property(key)).collect();

        if results.len() == 1 {
            results.into_iter().next().unwrap()
        } else {
            js_sys::Array::from_iter(results).into()
        }
    }

    /// Internal helper to get a property value by its path
    fn get_property(&self, path: &str) -> JsValue {
        match path {
            // Layout Mode
            "display" => JsValue::from(self.inner.display as u8),
            "position" => JsValue::from(self.inner.position as u8),
            "boxSizing" => JsValue::from(self.inner.box_sizing as u8),

            // Overflow
            "overflow" => {
                let s = PointOverflowDto {
                    x: self.inner.overflow.x as u8,
                    y: self.inner.overflow.y as u8,
                };
                serialize(&s)
            }
            "overflowX" => JsValue::from(self.inner.overflow.x as u8),
            "overflowY" => JsValue::from(self.inner.overflow.y as u8),

            // Flexbox
            "flexDirection" => JsValue::from(self.inner.flex_direction as u8),
            "flexWrap" => JsValue::from(self.inner.flex_wrap as u8),
            "flexGrow" => JsValue::from(self.inner.flex_grow),
            "flexShrink" => JsValue::from(self.inner.flex_shrink),
            "flexBasis" => {
                let d: DimensionDto = self.inner.flex_basis.into();
                serialize(&d)
            }

            // Alignment
            "alignItems" => match self.inner.align_items {
                Some(v) => JsValue::from(v as u8),
                None => JsValue::UNDEFINED,
            },
            "alignSelf" => match self.inner.align_self {
                Some(v) => JsValue::from(v as u8),
                None => JsValue::UNDEFINED,
            },
            "alignContent" => match self.inner.align_content {
                Some(v) => JsValue::from(v as u8),
                None => JsValue::UNDEFINED,
            },
            "justifyContent" => match self.inner.justify_content {
                Some(v) => JsValue::from(v as u8),
                None => JsValue::UNDEFINED,
            },
            "justifyItems" => match self.inner.justify_items {
                Some(v) => JsValue::from(v as u8),
                None => JsValue::UNDEFINED,
            },
            "justifySelf" => match self.inner.justify_self {
                Some(v) => JsValue::from(v as u8),
                None => JsValue::UNDEFINED,
            },

            // Sizing - aspectRatio
            "aspectRatio" => match self.inner.aspect_ratio {
                Some(v) => JsValue::from(v),
                None => JsValue::UNDEFINED,
            },

            // Sizing - size
            "size" => {
                let s: SizeDto<DimensionDto> = SizeDto {
                    width: self.inner.size.width.into(),
                    height: self.inner.size.height.into(),
                };
                serialize(&s)
            }
            "width" => {
                let d: DimensionDto = self.inner.size.width.into();
                serialize(&d)
            }
            "height" => {
                let d: DimensionDto = self.inner.size.height.into();
                serialize(&d)
            }

            // Sizing - minSize
            "minSize" => {
                let s: SizeDto<DimensionDto> = SizeDto {
                    width: self.inner.min_size.width.into(),
                    height: self.inner.min_size.height.into(),
                };
                serialize(&s)
            }
            "minWidth" => {
                let d: DimensionDto = self.inner.min_size.width.into();
                serialize(&d)
            }
            "minHeight" => {
                let d: DimensionDto = self.inner.min_size.height.into();
                serialize(&d)
            }

            // Sizing - maxSize
            "maxSize" => {
                let s: SizeDto<DimensionDto> = SizeDto {
                    width: self.inner.max_size.width.into(),
                    height: self.inner.max_size.height.into(),
                };
                serialize(&s)
            }
            "maxWidth" => {
                let d: DimensionDto = self.inner.max_size.width.into();
                serialize(&d)
            }
            "maxHeight" => {
                let d: DimensionDto = self.inner.max_size.height.into();
                serialize(&d)
            }

            // Spacing - margin
            "margin" => {
                let r: RectDto<LengthPercentageAutoDto> = RectDto {
                    left: self.inner.margin.left.into(),
                    right: self.inner.margin.right.into(),
                    top: self.inner.margin.top.into(),
                    bottom: self.inner.margin.bottom.into(),
                };
                serialize(&r)
            }
            "marginLeft" => {
                let d: LengthPercentageAutoDto = self.inner.margin.left.into();
                serialize(&d)
            }
            "marginRight" => {
                let d: LengthPercentageAutoDto = self.inner.margin.right.into();
                serialize(&d)
            }
            "marginTop" => {
                let d: LengthPercentageAutoDto = self.inner.margin.top.into();
                serialize(&d)
            }
            "marginBottom" => {
                let d: LengthPercentageAutoDto = self.inner.margin.bottom.into();
                serialize(&d)
            }

            // Spacing - padding
            "padding" => {
                let r: RectDto<LengthPercentageDto> = RectDto {
                    left: self.inner.padding.left.into(),
                    right: self.inner.padding.right.into(),
                    top: self.inner.padding.top.into(),
                    bottom: self.inner.padding.bottom.into(),
                };
                serialize(&r)
            }
            "paddingLeft" => {
                let d: LengthPercentageDto = self.inner.padding.left.into();
                serialize(&d)
            }
            "paddingRight" => {
                let d: LengthPercentageDto = self.inner.padding.right.into();
                serialize(&d)
            }
            "paddingTop" => {
                let d: LengthPercentageDto = self.inner.padding.top.into();
                serialize(&d)
            }
            "paddingBottom" => {
                let d: LengthPercentageDto = self.inner.padding.bottom.into();
                serialize(&d)
            }

            // Spacing - border
            "border" => {
                let r: RectDto<LengthPercentageDto> = RectDto {
                    left: self.inner.border.left.into(),
                    right: self.inner.border.right.into(),
                    top: self.inner.border.top.into(),
                    bottom: self.inner.border.bottom.into(),
                };
                serialize(&r)
            }
            "borderLeft" => {
                let d: LengthPercentageDto = self.inner.border.left.into();
                serialize(&d)
            }
            "borderRight" => {
                let d: LengthPercentageDto = self.inner.border.right.into();
                serialize(&d)
            }
            "borderTop" => {
                let d: LengthPercentageDto = self.inner.border.top.into();
                serialize(&d)
            }
            "borderBottom" => {
                let d: LengthPercentageDto = self.inner.border.bottom.into();
                serialize(&d)
            }

            // Spacing - inset
            "inset" => {
                let r: RectDto<LengthPercentageAutoDto> = RectDto {
                    left: self.inner.inset.left.into(),
                    right: self.inner.inset.right.into(),
                    top: self.inner.inset.top.into(),
                    bottom: self.inner.inset.bottom.into(),
                };
                serialize(&r)
            }
            "left" => {
                let d: LengthPercentageAutoDto = self.inner.inset.left.into();
                serialize(&d)
            }
            "right" => {
                let d: LengthPercentageAutoDto = self.inner.inset.right.into();
                serialize(&d)
            }
            "top" => {
                let d: LengthPercentageAutoDto = self.inner.inset.top.into();
                serialize(&d)
            }
            "bottom" => {
                let d: LengthPercentageAutoDto = self.inner.inset.bottom.into();
                serialize(&d)
            }

            // Spacing - gap
            "gap" => {
                let s: SizeDto<LengthPercentageDto> = SizeDto {
                    width: self.inner.gap.width.into(),
                    height: self.inner.gap.height.into(),
                };
                serialize(&s)
            }
            "columnGap" => {
                let d: LengthPercentageDto = self.inner.gap.width.into();
                serialize(&d)
            }
            "rowGap" => {
                let d: LengthPercentageDto = self.inner.gap.height.into();
                serialize(&d)
            }

            // Block layout
            "itemIsTable" => JsValue::from(self.inner.item_is_table),
            "itemIsReplaced" => JsValue::from(self.inner.item_is_replaced),
            "scrollbarWidth" => JsValue::from(self.inner.scrollbar_width),
            "textAlign" => JsValue::from(self.inner.text_align as u8),

            // Grid layout
            "gridAutoFlow" => JsValue::from(self.inner.grid_auto_flow as u8),

            "gridRow" => {
                let dto: LineGridPlacementDto = self.inner.grid_row.clone().into();
                serialize(&dto)
            }
            "gridRowStart" => {
                let dto: GridPlacementDto = self.inner.grid_row.start.clone().into();
                serialize(&dto)
            }
            "gridRowEnd" => {
                let dto: GridPlacementDto = self.inner.grid_row.end.clone().into();
                serialize(&dto)
            }

            "gridColumn" => {
                let dto: LineGridPlacementDto = self.inner.grid_column.clone().into();
                serialize(&dto)
            }
            "gridColumnStart" => {
                let dto: GridPlacementDto = self.inner.grid_column.start.clone().into();
                serialize(&dto)
            }
            "gridColumnEnd" => {
                let dto: GridPlacementDto = self.inner.grid_column.end.clone().into();
                serialize(&dto)
            }

            "gridTemplateRows" => {
                let tracks: Vec<GridTemplateComponentDto> = self
                    .inner
                    .grid_template_rows
                    .iter()
                    .cloned()
                    .map(|t| t.into())
                    .collect();
                serialize(&tracks)
            }

            "gridTemplateColumns" => {
                let tracks: Vec<GridTemplateComponentDto> = self
                    .inner
                    .grid_template_columns
                    .iter()
                    .cloned()
                    .map(|t| t.into())
                    .collect();
                serialize(&tracks)
            }

            "gridAutoRows" => {
                let tracks: Vec<TrackSizingFunctionDto> = self
                    .inner
                    .grid_auto_rows
                    .iter()
                    .cloned()
                    .map(|t| t.into())
                    .collect();
                serialize(&tracks)
            }

            "gridAutoColumns" => {
                let tracks: Vec<TrackSizingFunctionDto> = self
                    .inner
                    .grid_auto_columns
                    .iter()
                    .cloned()
                    .map(|t| t.into())
                    .collect();
                serialize(&tracks)
            }

            "gridTemplateAreas" => {
                let areas: Vec<crate::types::GridTemplateAreaDto> = self
                    .inner
                    .grid_template_areas
                    .iter()
                    .cloned()
                    .map(|a| a.into())
                    .collect();
                serialize(&areas)
            }

            "gridTemplateRowNames" => {
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
                serialize(&names)
            }

            "gridTemplateColumnNames" => {
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
                serialize(&names)
            }

            // Unknown property path
            _ => {
                log(&format!("Unknown property path: {}", path));
                JsValue::UNDEFINED
            }
        }
    }

    // =========================================================================
    // Batch Property Writing
    // =========================================================================

    /// Sets multiple style properties in a single WASM call.
    ///
    /// Accepts an object where keys are property paths (supporting dot notation)
    /// and values are the new property values.
    ///
    /// @param props - Object with property paths as keys and values to set
    ///
    /// @example
    /// ```typescript
    /// const style = new Style();
    ///
    /// // Set multiple properties at once
    /// style.set({
    ///   display: Display.Flex,
    ///   flexDirection: FlexDirection.Column,
    ///   "size.width": 200,
    ///   "size.height": "50%",
    ///   "margin.left": 10,
    ///   "margin.right": "auto"
    /// });
    /// ```
    #[wasm_bindgen]
    pub fn set(&mut self, props: JsValue) {
        if !props.is_object() {
            log("set() requires an object argument");
            return;
        }

        let obj = js_sys::Object::from(props);
        let entries = js_sys::Object::entries(&obj);

        for i in 0..entries.length() {
            let entry = entries.get(i);
            let arr = js_sys::Array::from(&entry);
            if arr.length() >= 2 {
                let key = arr.get(0);
                let value = arr.get(1);
                if let Some(key_str) = key.as_string() {
                    self.set_property(&key_str, value);
                }
            }
        }
    }

    /// Internal helper to set a property value by its path
    fn set_property(&mut self, path: &str, value: JsValue) {
        match path {
            // Layout Mode
            "display" => {
                if let Some(n) = value.as_f64() {
                    self.inner.display =
                        unsafe { std::mem::transmute::<u8, JsDisplay>(n as u8) }.into();
                }
            }
            "position" => {
                if let Some(n) = value.as_f64() {
                    self.inner.position =
                        unsafe { std::mem::transmute::<u8, JsPosition>(n as u8) }.into();
                }
            }
            "boxSizing" => {
                if let Some(n) = value.as_f64() {
                    self.inner.box_sizing =
                        unsafe { std::mem::transmute::<u8, JsBoxSizing>(n as u8) }.into();
                }
            }

            // Overflow
            "overflow" => {
                if let Ok(s) = serde_wasm_bindgen::from_value::<PointOverflowDto>(value) {
                    self.inner.overflow = s.into();
                }
            }
            "overflowX" => {
                if let Some(n) = value.as_f64() {
                    self.inner.overflow.x =
                        unsafe { std::mem::transmute::<u8, JsOverflow>(n as u8) }.into();
                }
            }
            "overflowY" => {
                if let Some(n) = value.as_f64() {
                    self.inner.overflow.y =
                        unsafe { std::mem::transmute::<u8, JsOverflow>(n as u8) }.into();
                }
            }

            // Flexbox
            "flexDirection" => {
                if let Some(n) = value.as_f64() {
                    self.inner.flex_direction =
                        unsafe { std::mem::transmute::<u8, JsFlexDirection>(n as u8) }.into();
                }
            }
            "flexWrap" => {
                if let Some(n) = value.as_f64() {
                    self.inner.flex_wrap =
                        unsafe { std::mem::transmute::<u8, JsFlexWrap>(n as u8) }.into();
                }
            }
            "flexGrow" => {
                if let Some(n) = value.as_f64() {
                    self.inner.flex_grow = n as f32;
                }
            }
            "flexShrink" => {
                if let Some(n) = value.as_f64() {
                    self.inner.flex_shrink = n as f32;
                }
            }
            "flexBasis" => {
                if let Ok(d) = serde_wasm_bindgen::from_value::<DimensionDto>(value) {
                    self.inner.flex_basis = d.into();
                }
            }

            // Alignment
            "alignItems" => {
                if value.is_undefined() {
                    self.inner.align_items = None;
                } else if let Some(n) = value.as_f64() {
                    self.inner.align_items =
                        Some(unsafe { std::mem::transmute::<u8, JsAlignItems>(n as u8) }.into());
                }
            }
            "alignSelf" => {
                if value.is_undefined() {
                    self.inner.align_self = None;
                } else if let Some(n) = value.as_f64() {
                    let js_val = unsafe { std::mem::transmute::<u8, JsAlignSelf>(n as u8) };
                    self.inner.align_self = match js_val {
                        JsAlignSelf::Auto => None,
                        _ => Some(js_val.into()),
                    };
                }
            }
            "alignContent" => {
                if value.is_undefined() {
                    self.inner.align_content = None;
                } else if let Some(n) = value.as_f64() {
                    self.inner.align_content =
                        Some(unsafe { std::mem::transmute::<u8, JsAlignContent>(n as u8) }.into());
                }
            }
            "justifyContent" => {
                if value.is_undefined() {
                    self.inner.justify_content = None;
                } else if let Some(n) = value.as_f64() {
                    self.inner.justify_content = Some(
                        unsafe { std::mem::transmute::<u8, JsJustifyContent>(n as u8) }.into(),
                    );
                }
            }
            "justifyItems" => {
                if value.is_undefined() {
                    self.inner.justify_items = None;
                } else if let Some(n) = value.as_f64() {
                    self.inner.justify_items =
                        Some(unsafe { std::mem::transmute::<u8, JsAlignItems>(n as u8) }.into());
                }
            }
            "justifySelf" => {
                if value.is_undefined() {
                    self.inner.justify_self = None;
                } else if let Some(n) = value.as_f64() {
                    let js_val = unsafe { std::mem::transmute::<u8, JsAlignSelf>(n as u8) };
                    self.inner.justify_self = match js_val {
                        JsAlignSelf::Auto => None,
                        _ => Some(js_val.into()),
                    };
                }
            }

            // Sizing - aspectRatio
            "aspectRatio" => {
                if value.is_undefined() || value.is_null() {
                    self.inner.aspect_ratio = None;
                } else if let Some(n) = value.as_f64() {
                    self.inner.aspect_ratio = Some(n as f32);
                }
            }

            // Sizing - size
            "size" => {
                if let Ok(s) = serde_wasm_bindgen::from_value::<SizeDto<DimensionDto>>(value) {
                    self.inner.size = s.into();
                }
            }
            "width" => {
                if let Ok(d) = serde_wasm_bindgen::from_value::<DimensionDto>(value) {
                    self.inner.size.width = d.into();
                }
            }
            "height" => {
                if let Ok(d) = serde_wasm_bindgen::from_value::<DimensionDto>(value) {
                    self.inner.size.height = d.into();
                }
            }

            // Sizing - minSize
            "minSize" => {
                if let Ok(s) = serde_wasm_bindgen::from_value::<SizeDto<DimensionDto>>(value) {
                    self.inner.min_size = s.into();
                }
            }
            "minWidth" => {
                if let Ok(d) = serde_wasm_bindgen::from_value::<DimensionDto>(value) {
                    self.inner.min_size.width = d.into();
                }
            }
            "minHeight" => {
                if let Ok(d) = serde_wasm_bindgen::from_value::<DimensionDto>(value) {
                    self.inner.min_size.height = d.into();
                }
            }

            // Sizing - maxSize
            "maxSize" => {
                if let Ok(s) = serde_wasm_bindgen::from_value::<SizeDto<DimensionDto>>(value) {
                    self.inner.max_size = s.into();
                }
            }
            "maxWidth" => {
                if let Ok(d) = serde_wasm_bindgen::from_value::<DimensionDto>(value) {
                    self.inner.max_size.width = d.into();
                }
            }
            "maxHeight" => {
                if let Ok(d) = serde_wasm_bindgen::from_value::<DimensionDto>(value) {
                    self.inner.max_size.height = d.into();
                }
            }

            // Spacing - margin
            "margin" => {
                if let Ok(r) =
                    serde_wasm_bindgen::from_value::<RectDto<LengthPercentageAutoDto>>(value)
                {
                    self.inner.margin = r.into();
                }
            }
            "marginLeft" => {
                if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageAutoDto>(value) {
                    self.inner.margin.left = d.into();
                }
            }
            "marginRight" => {
                if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageAutoDto>(value) {
                    self.inner.margin.right = d.into();
                }
            }
            "marginTop" => {
                if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageAutoDto>(value) {
                    self.inner.margin.top = d.into();
                }
            }
            "marginBottom" => {
                if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageAutoDto>(value) {
                    self.inner.margin.bottom = d.into();
                }
            }

            // Spacing - padding
            "padding" => {
                if let Ok(r) = serde_wasm_bindgen::from_value::<RectDto<LengthPercentageDto>>(value)
                {
                    self.inner.padding = r.into();
                }
            }
            "paddingLeft" => {
                if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageDto>(value) {
                    self.inner.padding.left = d.into();
                }
            }
            "paddingRight" => {
                if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageDto>(value) {
                    self.inner.padding.right = d.into();
                }
            }
            "paddingTop" => {
                if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageDto>(value) {
                    self.inner.padding.top = d.into();
                }
            }
            "paddingBottom" => {
                if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageDto>(value) {
                    self.inner.padding.bottom = d.into();
                }
            }

            // Spacing - border
            "border" => {
                if let Ok(r) = serde_wasm_bindgen::from_value::<RectDto<LengthPercentageDto>>(value)
                {
                    self.inner.border = r.into();
                }
            }
            "borderLeft" => {
                if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageDto>(value) {
                    self.inner.border.left = d.into();
                }
            }
            "borderRight" => {
                if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageDto>(value) {
                    self.inner.border.right = d.into();
                }
            }
            "borderTop" => {
                if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageDto>(value) {
                    self.inner.border.top = d.into();
                }
            }
            "borderBottom" => {
                if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageDto>(value) {
                    self.inner.border.bottom = d.into();
                }
            }

            // Spacing - inset
            "inset" => {
                if let Ok(r) =
                    serde_wasm_bindgen::from_value::<RectDto<LengthPercentageAutoDto>>(value)
                {
                    self.inner.inset = r.into();
                }
            }
            "left" => {
                if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageAutoDto>(value) {
                    self.inner.inset.left = d.into();
                }
            }
            "right" => {
                if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageAutoDto>(value) {
                    self.inner.inset.right = d.into();
                }
            }
            "top" => {
                if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageAutoDto>(value) {
                    self.inner.inset.top = d.into();
                }
            }
            "bottom" => {
                if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageAutoDto>(value) {
                    self.inner.inset.bottom = d.into();
                }
            }

            // Spacing - gap
            "gap" => {
                if let Ok(s) = serde_wasm_bindgen::from_value::<SizeDto<LengthPercentageDto>>(value)
                {
                    self.inner.gap = s.into();
                }
            }
            "columnGap" => {
                if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageDto>(value) {
                    self.inner.gap.width = d.into();
                }
            }
            "rowGap" => {
                if let Ok(d) = serde_wasm_bindgen::from_value::<LengthPercentageDto>(value) {
                    self.inner.gap.height = d.into();
                }
            }

            // Block layout
            "itemIsTable" => {
                if let Some(b) = value.as_bool() {
                    self.inner.item_is_table = b;
                }
            }
            "itemIsReplaced" => {
                if let Some(b) = value.as_bool() {
                    self.inner.item_is_replaced = b;
                }
            }
            "scrollbarWidth" => {
                if let Some(n) = value.as_f64() {
                    self.inner.scrollbar_width = n as f32;
                }
            }
            "textAlign" => {
                if let Some(n) = value.as_f64() {
                    self.inner.text_align =
                        unsafe { std::mem::transmute::<u8, JsTextAlign>(n as u8) }.into();
                }
            }

            // Grid layout
            "gridAutoFlow" => {
                if let Some(n) = value.as_f64() {
                    self.inner.grid_auto_flow =
                        unsafe { std::mem::transmute::<u8, JsGridAutoFlow>(n as u8) }.into();
                }
            }

            "gridRow" => {
                if let Ok(dto) = serde_wasm_bindgen::from_value::<LineGridPlacementDto>(value) {
                    self.inner.grid_row = dto.into();
                }
            }
            "gridRowStart" => {
                if let Ok(dto) = serde_wasm_bindgen::from_value::<GridPlacementDto>(value) {
                    self.inner.grid_row.start = dto.into();
                }
            }
            "gridRowEnd" => {
                if let Ok(dto) = serde_wasm_bindgen::from_value::<GridPlacementDto>(value) {
                    self.inner.grid_row.end = dto.into();
                }
            }

            "gridColumn" => {
                if let Ok(dto) = serde_wasm_bindgen::from_value::<LineGridPlacementDto>(value) {
                    self.inner.grid_column = dto.into();
                }
            }
            "gridColumnStart" => {
                if let Ok(dto) = serde_wasm_bindgen::from_value::<GridPlacementDto>(value) {
                    self.inner.grid_column.start = dto.into();
                }
            }
            "gridColumnEnd" => {
                if let Ok(dto) = serde_wasm_bindgen::from_value::<GridPlacementDto>(value) {
                    self.inner.grid_column.end = dto.into();
                }
            }

            "gridTemplateRows" => {
                if let Ok(tracks) =
                    serde_wasm_bindgen::from_value::<Vec<GridTemplateComponentDto>>(value)
                {
                    self.inner.grid_template_rows = tracks.into_iter().map(|t| t.into()).collect();
                }
            }

            "gridTemplateColumns" => {
                if let Ok(tracks) =
                    serde_wasm_bindgen::from_value::<Vec<GridTemplateComponentDto>>(value)
                {
                    self.inner.grid_template_columns =
                        tracks.into_iter().map(|t| t.into()).collect();
                }
            }

            "gridAutoRows" => {
                if let Ok(tracks) =
                    serde_wasm_bindgen::from_value::<Vec<TrackSizingFunctionDto>>(value)
                {
                    self.inner.grid_auto_rows = tracks.into_iter().map(|t| t.into()).collect();
                }
            }

            "gridAutoColumns" => {
                if let Ok(tracks) =
                    serde_wasm_bindgen::from_value::<Vec<TrackSizingFunctionDto>>(value)
                {
                    self.inner.grid_auto_columns = tracks.into_iter().map(|t| t.into()).collect();
                }
            }

            "gridTemplateAreas" => {
                if let Ok(areas) =
                    serde_wasm_bindgen::from_value::<Vec<crate::types::GridTemplateAreaDto>>(value)
                {
                    self.inner.grid_template_areas = areas.into_iter().map(|a| a.into()).collect();
                }
            }

            "gridTemplateRowNames" => {
                if let Ok(names) = serde_wasm_bindgen::from_value::<Vec<Vec<String>>>(value) {
                    self.inner.grid_template_row_names = names
                        .into_iter()
                        .map(|v| v.into_iter().map(|s| s.into()).collect())
                        .collect();
                }
            }

            "gridTemplateColumnNames" => {
                if let Ok(names) = serde_wasm_bindgen::from_value::<Vec<Vec<String>>>(value) {
                    self.inner.grid_template_column_names = names
                        .into_iter()
                        .map(|v| v.into_iter().map(|s| s.into()).collect())
                        .collect();
                }
            }

            // Unknown property path
            _ => {
                log(&format!("Unknown property path for set: {}", path));
            }
        }
    }
}
