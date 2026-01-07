//! # CSS Layout Property Enumerations
//!
//! This module defines all CSS layout-related enumerations that are exposed to JavaScript/TypeScript.
//! Each enum maps directly to its corresponding CSS property and provides bidirectional conversion
//! between Rust and JavaScript representations.
//!
//! ## Enum Naming Convention
//!
//! - Rust internal names use the `Js` prefix (e.g., `JsDisplay`)
//! - JavaScript/TypeScript exposed names use the original CSS property names (e.g., `Display`)
//! - This is achieved via `#[wasm_bindgen(js_name = ...)]` attributes
//!
//! ## Conversion Traits
//!
//! Each enum implements:
//! - `From<JsEnum> for taffy::style::Enum` - Convert from JS to Taffy
//! - `From<taffy::style::Enum> for JsEnum` - Convert from Taffy to JS
//! - `TryFrom<u32> for JsEnum` - Convert from raw number (for setter handling)

use wasm_bindgen::prelude::*;

// =============================================================================
// Display Mode
// =============================================================================

/// Display mode enumeration
///
/// Controls the layout algorithm type for an element. This corresponds to the CSS `display` property
/// and determines how an element and its children are laid out.
///
/// @example
/// ```typescript
/// import { Display } from 'taffy-js';
///
/// style.display = Display.Flex;  // Enable flexbox layout
/// style.display = Display.Grid;  // Enable grid layout
/// style.display = Display.None;  // Hide element from layout
/// ```
#[wasm_bindgen(js_name = Display)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum JsDisplay {
    /// Block-level layout where element takes the full available width
    Block = 0,
    /// Flexbox layout for one-dimensional item arrangement
    Flex = 1,
    /// CSS Grid layout for two-dimensional item arrangement
    Grid = 2,
    /// Element is removed from layout calculation entirely
    None = 3,
}

impl From<JsDisplay> for taffy::style::Display {
    fn from(val: JsDisplay) -> Self {
        match val {
            JsDisplay::Block => taffy::style::Display::Block,
            JsDisplay::Flex => taffy::style::Display::Flex,
            JsDisplay::Grid => taffy::style::Display::Grid,
            JsDisplay::None => taffy::style::Display::None,
        }
    }
}

impl From<taffy::style::Display> for JsDisplay {
    fn from(val: taffy::style::Display) -> Self {
        match val {
            taffy::style::Display::Block => JsDisplay::Block,
            taffy::style::Display::Flex => JsDisplay::Flex,
            taffy::style::Display::Grid => JsDisplay::Grid,
            taffy::style::Display::None => JsDisplay::None,
        }
    }
}

// =============================================================================
// Position Mode
// =============================================================================

/// Position mode enumeration
///
/// Controls how an element is positioned within its parent container.
/// This corresponds to the CSS `position` property.
///
/// @example
/// ```typescript
/// import { Position } from 'taffy-js';
///
/// style.position = Position.Relative;  // Normal document flow
/// style.position = Position.Absolute;  // Removed from flow, uses inset values
/// ```
#[wasm_bindgen(js_name = Position)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum JsPosition {
    /// Element participates in normal document flow
    Relative = 0,
    /// Element is positioned relative to its nearest positioned ancestor
    Absolute = 1,
}

impl From<JsPosition> for taffy::style::Position {
    fn from(val: JsPosition) -> Self {
        match val {
            JsPosition::Relative => taffy::style::Position::Relative,
            JsPosition::Absolute => taffy::style::Position::Absolute,
        }
    }
}

impl From<taffy::style::Position> for JsPosition {
    fn from(val: taffy::style::Position) -> Self {
        match val {
            taffy::style::Position::Relative => JsPosition::Relative,
            taffy::style::Position::Absolute => JsPosition::Absolute,
        }
    }
}

// =============================================================================
// Flex Direction
// =============================================================================

/// Flex direction enumeration
///
/// Defines the main axis direction for flex item layout. This corresponds to the CSS
/// `flex-direction` property and determines how flex items are placed within the container.
///
/// @example
/// ```typescript
/// import { FlexDirection } from 'taffy-js';
///
/// style.flexDirection = FlexDirection.Row;     // Horizontal, left to right
/// style.flexDirection = FlexDirection.Column;  // Vertical, top to bottom
/// ```
#[wasm_bindgen(js_name = FlexDirection)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum JsFlexDirection {
    /// Main axis runs horizontally from left to right
    Row = 0,
    /// Main axis runs vertically from top to bottom
    Column = 1,
    /// Main axis runs horizontally from right to left
    RowReverse = 2,
    /// Main axis runs vertically from bottom to top
    ColumnReverse = 3,
}

impl From<JsFlexDirection> for taffy::style::FlexDirection {
    fn from(val: JsFlexDirection) -> Self {
        match val {
            JsFlexDirection::Row => taffy::style::FlexDirection::Row,
            JsFlexDirection::Column => taffy::style::FlexDirection::Column,
            JsFlexDirection::RowReverse => taffy::style::FlexDirection::RowReverse,
            JsFlexDirection::ColumnReverse => taffy::style::FlexDirection::ColumnReverse,
        }
    }
}

impl From<taffy::style::FlexDirection> for JsFlexDirection {
    fn from(val: taffy::style::FlexDirection) -> Self {
        match val {
            taffy::style::FlexDirection::Row => JsFlexDirection::Row,
            taffy::style::FlexDirection::Column => JsFlexDirection::Column,
            taffy::style::FlexDirection::RowReverse => JsFlexDirection::RowReverse,
            taffy::style::FlexDirection::ColumnReverse => JsFlexDirection::ColumnReverse,
        }
    }
}

// =============================================================================
// Flex Wrap
// =============================================================================

/// Flex wrap mode enumeration
///
/// Controls whether flex items wrap onto multiple lines when they overflow the container.
/// This corresponds to the CSS `flex-wrap` property.
///
/// @example
/// ```typescript
/// import { FlexWrap } from 'taffy-js';
///
/// style.flexWrap = FlexWrap.NoWrap;  // All items on single line
/// style.flexWrap = FlexWrap.Wrap;    // Items wrap to new lines
/// ```
#[wasm_bindgen(js_name = FlexWrap)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum JsFlexWrap {
    /// All flex items are placed on a single line
    NoWrap = 0,
    /// Flex items wrap onto multiple lines from top to bottom
    Wrap = 1,
    /// Flex items wrap onto multiple lines from bottom to top
    WrapReverse = 2,
}

impl From<JsFlexWrap> for taffy::style::FlexWrap {
    fn from(val: JsFlexWrap) -> Self {
        match val {
            JsFlexWrap::NoWrap => taffy::style::FlexWrap::NoWrap,
            JsFlexWrap::Wrap => taffy::style::FlexWrap::Wrap,
            JsFlexWrap::WrapReverse => taffy::style::FlexWrap::WrapReverse,
        }
    }
}

impl From<taffy::style::FlexWrap> for JsFlexWrap {
    fn from(val: taffy::style::FlexWrap) -> Self {
        match val {
            taffy::style::FlexWrap::NoWrap => JsFlexWrap::NoWrap,
            taffy::style::FlexWrap::Wrap => JsFlexWrap::Wrap,
            taffy::style::FlexWrap::WrapReverse => JsFlexWrap::WrapReverse,
        }
    }
}

// =============================================================================
// Align Items
// =============================================================================

/// Cross-axis alignment enumeration for all children
///
/// Defines the default alignment for all flex/grid items along the cross axis.
/// This corresponds to the CSS `align-items` property.
///
/// @example
/// ```typescript
/// import { AlignItems } from 'taffy-js';
///
/// style.alignItems = AlignItems.Center;   // Center items on cross axis
/// style.alignItems = AlignItems.Stretch;  // Stretch items to fill container
/// ```
#[wasm_bindgen(js_name = AlignItems)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum JsAlignItems {
    /// Items aligned to the start of the cross axis
    Start = 0,
    /// Items aligned to the end of the cross axis
    End = 1,
    /// Items aligned to the start of the flex container
    FlexStart = 2,
    /// Items aligned to the end of the flex container
    FlexEnd = 3,
    /// Items centered along the cross axis
    Center = 4,
    /// Items aligned to their text baselines
    Baseline = 5,
    /// Items stretched to fill the container
    Stretch = 6,
}

impl From<JsAlignItems> for taffy::style::AlignItems {
    fn from(val: JsAlignItems) -> Self {
        match val {
            JsAlignItems::Start => taffy::style::AlignItems::Start,
            JsAlignItems::End => taffy::style::AlignItems::End,
            JsAlignItems::FlexStart => taffy::style::AlignItems::FlexStart,
            JsAlignItems::FlexEnd => taffy::style::AlignItems::FlexEnd,
            JsAlignItems::Center => taffy::style::AlignItems::Center,
            JsAlignItems::Baseline => taffy::style::AlignItems::Baseline,
            JsAlignItems::Stretch => taffy::style::AlignItems::Stretch,
        }
    }
}

impl From<taffy::style::AlignItems> for JsAlignItems {
    fn from(val: taffy::style::AlignItems) -> Self {
        match val {
            taffy::style::AlignItems::Start => JsAlignItems::Start,
            taffy::style::AlignItems::End => JsAlignItems::End,
            taffy::style::AlignItems::FlexStart => JsAlignItems::FlexStart,
            taffy::style::AlignItems::FlexEnd => JsAlignItems::FlexEnd,
            taffy::style::AlignItems::Center => JsAlignItems::Center,
            taffy::style::AlignItems::Baseline => JsAlignItems::Baseline,
            taffy::style::AlignItems::Stretch => JsAlignItems::Stretch,
        }
    }
}

// =============================================================================
// Align Self
// =============================================================================

/// Cross-axis alignment enumeration for a single element
///
/// Overrides the parent's `align-items` value for a specific child element.
/// This corresponds to the CSS `align-self` property.
///
/// @example
/// ```typescript
/// import { AlignSelf } from 'taffy-js';
///
/// style.alignSelf = AlignSelf.Auto;    // Use parent's align-items
/// style.alignSelf = AlignSelf.Center;  // Override to center this item
/// ```
#[wasm_bindgen(js_name = AlignSelf)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum JsAlignSelf {
    /// Inherits the parent container's `align-items` value
    Auto = 0,
    /// Item aligned to the start of the cross axis
    Start = 1,
    /// Item aligned to the end of the cross axis
    End = 2,
    /// Item aligned to the start of the flex container
    FlexStart = 3,
    /// Item aligned to the end of the flex container
    FlexEnd = 4,
    /// Item centered along the cross axis
    Center = 5,
    /// Item aligned to its text baseline
    Baseline = 6,
    /// Item stretched to fill the container
    Stretch = 7,
}

impl From<JsAlignSelf> for taffy::style::AlignSelf {
    fn from(val: JsAlignSelf) -> Self {
        match val {
            JsAlignSelf::Auto => taffy::style::AlignSelf::Stretch,
            JsAlignSelf::Start => taffy::style::AlignSelf::Start,
            JsAlignSelf::End => taffy::style::AlignSelf::End,
            JsAlignSelf::FlexStart => taffy::style::AlignSelf::FlexStart,
            JsAlignSelf::FlexEnd => taffy::style::AlignSelf::FlexEnd,
            JsAlignSelf::Center => taffy::style::AlignSelf::Center,
            JsAlignSelf::Baseline => taffy::style::AlignSelf::Baseline,
            JsAlignSelf::Stretch => taffy::style::AlignSelf::Stretch,
        }
    }
}

impl From<taffy::style::AlignSelf> for JsAlignSelf {
    fn from(val: taffy::style::AlignSelf) -> Self {
        match val {
            taffy::style::AlignSelf::Start => JsAlignSelf::Start,
            taffy::style::AlignSelf::End => JsAlignSelf::End,
            taffy::style::AlignSelf::FlexStart => JsAlignSelf::FlexStart,
            taffy::style::AlignSelf::FlexEnd => JsAlignSelf::FlexEnd,
            taffy::style::AlignSelf::Center => JsAlignSelf::Center,
            taffy::style::AlignSelf::Baseline => JsAlignSelf::Baseline,
            taffy::style::AlignSelf::Stretch => JsAlignSelf::Stretch,
        }
    }
}

// =============================================================================
// Align Content
// =============================================================================

/// Multi-line content alignment enumeration
///
/// Controls the distribution of space between and around content items along the cross axis
/// in a multi-line flex container. This corresponds to the CSS `align-content` property.
///
/// **Note**: This property only has effect when `flex-wrap` is set to `Wrap` or `WrapReverse`.
///
/// @example
/// ```typescript
/// import { AlignContent, FlexWrap } from 'taffy-js';
///
/// style.flexWrap = FlexWrap.Wrap;
/// style.alignContent = AlignContent.SpaceBetween;  // Distribute lines evenly
/// ```
#[wasm_bindgen(js_name = AlignContent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum JsAlignContent {
    /// Lines packed toward the start of the cross axis
    Start = 0,
    /// Lines packed toward the end of the cross axis
    End = 1,
    /// Lines packed toward the start of the flex container
    FlexStart = 2,
    /// Lines packed toward the end of the flex container
    FlexEnd = 3,
    /// Lines centered within the container
    Center = 4,
    /// Lines stretched to fill the container
    Stretch = 5,
    /// Lines evenly distributed with first/last at edges
    SpaceBetween = 6,
    /// Lines evenly distributed with equal space around each
    SpaceAround = 7,
    /// Lines evenly distributed with equal space between each
    SpaceEvenly = 8,
}

impl From<JsAlignContent> for taffy::style::AlignContent {
    fn from(val: JsAlignContent) -> Self {
        match val {
            JsAlignContent::Start => taffy::style::AlignContent::Start,
            JsAlignContent::End => taffy::style::AlignContent::End,
            JsAlignContent::FlexStart => taffy::style::AlignContent::FlexStart,
            JsAlignContent::FlexEnd => taffy::style::AlignContent::FlexEnd,
            JsAlignContent::Center => taffy::style::AlignContent::Center,
            JsAlignContent::Stretch => taffy::style::AlignContent::Stretch,
            JsAlignContent::SpaceBetween => taffy::style::AlignContent::SpaceBetween,
            JsAlignContent::SpaceAround => taffy::style::AlignContent::SpaceAround,
            JsAlignContent::SpaceEvenly => taffy::style::AlignContent::SpaceEvenly,
        }
    }
}

impl From<taffy::style::AlignContent> for JsAlignContent {
    fn from(val: taffy::style::AlignContent) -> Self {
        match val {
            taffy::style::AlignContent::Start => JsAlignContent::Start,
            taffy::style::AlignContent::End => JsAlignContent::End,
            taffy::style::AlignContent::FlexStart => JsAlignContent::FlexStart,
            taffy::style::AlignContent::FlexEnd => JsAlignContent::FlexEnd,
            taffy::style::AlignContent::Center => JsAlignContent::Center,
            taffy::style::AlignContent::Stretch => JsAlignContent::Stretch,
            taffy::style::AlignContent::SpaceBetween => JsAlignContent::SpaceBetween,
            taffy::style::AlignContent::SpaceAround => JsAlignContent::SpaceAround,
            taffy::style::AlignContent::SpaceEvenly => JsAlignContent::SpaceEvenly,
        }
    }
}

// =============================================================================
// Justify Content
// =============================================================================

/// Main axis alignment enumeration
///
/// Defines how flex items are aligned and spaced along the main axis.
/// This corresponds to the CSS `justify-content` property.
///
/// @example
/// ```typescript
/// import { JustifyContent } from 'taffy-js';
///
/// style.justifyContent = JustifyContent.Center;        // Center items
/// style.justifyContent = JustifyContent.SpaceBetween;  // Distribute evenly
/// ```
#[wasm_bindgen(js_name = JustifyContent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum JsJustifyContent {
    /// Items packed toward the start of the main axis
    Start = 0,
    /// Items packed toward the end of the main axis
    End = 1,
    /// Items packed toward the start of the flex container
    FlexStart = 2,
    /// Items packed toward the end of the flex container
    FlexEnd = 3,
    /// Items centered along the main axis
    Center = 4,
    /// Items stretched along the main axis
    Stretch = 5,
    /// Items evenly distributed with first/last at edges
    SpaceBetween = 6,
    /// Items evenly distributed with equal space around each
    SpaceAround = 7,
    /// Items evenly distributed with equal space between each
    SpaceEvenly = 8,
}

impl From<JsJustifyContent> for taffy::style::JustifyContent {
    fn from(val: JsJustifyContent) -> Self {
        match val {
            JsJustifyContent::Start => taffy::style::JustifyContent::Start,
            JsJustifyContent::End => taffy::style::JustifyContent::End,
            JsJustifyContent::FlexStart => taffy::style::JustifyContent::FlexStart,
            JsJustifyContent::FlexEnd => taffy::style::JustifyContent::FlexEnd,
            JsJustifyContent::Center => taffy::style::JustifyContent::Center,
            JsJustifyContent::Stretch => taffy::style::JustifyContent::Stretch,
            JsJustifyContent::SpaceBetween => taffy::style::JustifyContent::SpaceBetween,
            JsJustifyContent::SpaceAround => taffy::style::JustifyContent::SpaceAround,
            JsJustifyContent::SpaceEvenly => taffy::style::JustifyContent::SpaceEvenly,
        }
    }
}

impl From<taffy::style::JustifyContent> for JsJustifyContent {
    fn from(val: taffy::style::JustifyContent) -> Self {
        match val {
            taffy::style::JustifyContent::Start => JsJustifyContent::Start,
            taffy::style::JustifyContent::End => JsJustifyContent::End,
            taffy::style::JustifyContent::FlexStart => JsJustifyContent::FlexStart,
            taffy::style::JustifyContent::FlexEnd => JsJustifyContent::FlexEnd,
            taffy::style::JustifyContent::Center => JsJustifyContent::Center,
            taffy::style::JustifyContent::Stretch => JsJustifyContent::Stretch,
            taffy::style::JustifyContent::SpaceBetween => JsJustifyContent::SpaceBetween,
            taffy::style::JustifyContent::SpaceAround => JsJustifyContent::SpaceAround,
            taffy::style::JustifyContent::SpaceEvenly => JsJustifyContent::SpaceEvenly,
        }
    }
}

// =============================================================================
// Overflow
// =============================================================================

/// Overflow handling enumeration
///
/// Defines how content that exceeds the container boundaries is handled.
/// This corresponds to the CSS `overflow` property.
///
/// @example
/// ```typescript
/// import { Overflow } from 'taffy-js';
///
/// style.overflow = { x: Overflow.Hidden, y: Overflow.Scroll };
/// ```
#[wasm_bindgen(js_name = Overflow)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum JsOverflow {
    /// Content is not clipped and may render outside the container
    Visible = 0,
    /// Content is clipped at the container boundary, but unlike Hidden, this forbids all scrolling
    Clip = 1,
    /// Content is clipped at the container boundary
    Hidden = 2,
    /// Always display scrollbars for scrollable content
    Scroll = 3,
}

impl From<JsOverflow> for taffy::style::Overflow {
    fn from(val: JsOverflow) -> Self {
        match val {
            JsOverflow::Visible => taffy::style::Overflow::Visible,
            JsOverflow::Hidden => taffy::style::Overflow::Hidden,
            JsOverflow::Scroll => taffy::style::Overflow::Scroll,
            JsOverflow::Clip => taffy::style::Overflow::Clip,
        }
    }
}

impl From<taffy::style::Overflow> for JsOverflow {
    fn from(val: taffy::style::Overflow) -> Self {
        match val {
            taffy::style::Overflow::Visible => JsOverflow::Visible,
            taffy::style::Overflow::Hidden => JsOverflow::Hidden,
            taffy::style::Overflow::Scroll => JsOverflow::Scroll,
            taffy::style::Overflow::Clip => JsOverflow::Clip,
        }
    }
}

// =============================================================================
// Box Sizing
// =============================================================================

/// Box sizing enumeration
///
/// Controls how the total width and height of an element is calculated.
/// This corresponds to the CSS `box-sizing` property.
///
/// @example
/// ```typescript
/// import { BoxSizing } from 'taffy-js';
///
/// style.boxSizing = BoxSizing.BorderBox;   // Size includes padding and border
/// style.boxSizing = BoxSizing.ContentBox;  // Size is content only
/// ```
#[wasm_bindgen(js_name = BoxSizing)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum JsBoxSizing {
    /// The width and height properties include padding and border
    BorderBox = 0,
    /// The width and height properties include only the content
    ContentBox = 1,
}

impl From<JsBoxSizing> for taffy::style::BoxSizing {
    fn from(val: JsBoxSizing) -> Self {
        match val {
            JsBoxSizing::BorderBox => taffy::style::BoxSizing::BorderBox,
            JsBoxSizing::ContentBox => taffy::style::BoxSizing::ContentBox,
        }
    }
}

impl From<taffy::style::BoxSizing> for JsBoxSizing {
    fn from(val: taffy::style::BoxSizing) -> Self {
        match val {
            taffy::style::BoxSizing::BorderBox => JsBoxSizing::BorderBox,
            taffy::style::BoxSizing::ContentBox => JsBoxSizing::ContentBox,
        }
    }
}
