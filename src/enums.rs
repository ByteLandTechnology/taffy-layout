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
/// import { Style, Display } from 'taffy-layout';
///
/// const style = new Style();
/// style.display = Display.Flex;  // Enable flexbox layout
/// style.display = Display.Grid;  // Enable grid layout
/// style.display = Display.None;  // Hide element from layout
/// ```
#[wasm_bindgen(js_name = Display)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum JsDisplay {
    /// Block layout for the element's children, including flow, margins, and floats
    Block = 0,
    /// Flexbox layout for one-dimensional item arrangement
    Flex = 1,
    /// CSS Grid layout for two-dimensional item arrangement
    Grid = 2,
    /// Element is removed from layout calculation entirely
    None = 3,
    /// Block layout that always establishes a new block formatting context
    FlowRoot = 4,
}

impl From<JsDisplay> for taffy::style::Display {
    fn from(val: JsDisplay) -> Self {
        match val {
            JsDisplay::Block => taffy::style::Display::Block,
            JsDisplay::Flex => taffy::style::Display::Flex,
            JsDisplay::Grid => taffy::style::Display::Grid,
            JsDisplay::None => taffy::style::Display::None,
            JsDisplay::FlowRoot => taffy::style::Display::FlowRoot,
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
            taffy::style::Display::FlowRoot => JsDisplay::FlowRoot,
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
/// import { Style, Position } from 'taffy-layout';
///
/// const style = new Style();
/// style.position = Position.Relative;  // Normal document flow
/// style.position = Position.Absolute;  // Removed from flow, uses inset values
/// ```
#[wasm_bindgen(js_name = Position)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum JsPosition {
    /// Element participates in normal document flow
    Relative = 0,
    /// Element is removed from flow and positioned within its parent layout container
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
// Writing Direction
// =============================================================================

/// Writing direction used for logical layout and bidirectional positioning.
#[wasm_bindgen(js_name = Direction)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum JsDirection {
    /// Left-to-right writing direction
    Ltr = 0,
    /// Right-to-left writing direction
    Rtl = 1,
}

impl From<JsDirection> for taffy::style::Direction {
    fn from(val: JsDirection) -> Self {
        match val {
            JsDirection::Ltr => taffy::style::Direction::Ltr,
            JsDirection::Rtl => taffy::style::Direction::Rtl,
        }
    }
}

impl From<taffy::style::Direction> for JsDirection {
    fn from(val: taffy::style::Direction) -> Self {
        match val {
            taffy::style::Direction::Ltr => JsDirection::Ltr,
            taffy::style::Direction::Rtl => JsDirection::Rtl,
        }
    }
}

// =============================================================================
// Float and Clear
// =============================================================================

/// Controls whether a box floats to the left or right in block layout.
#[wasm_bindgen(js_name = Float)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum JsFloat {
    /// Float to the physical left side of the containing block
    Left = 0,
    /// Float to the physical right side of the containing block
    Right = 1,
    /// Do not float
    None = 2,
}

impl From<JsFloat> for taffy::style::Float {
    fn from(val: JsFloat) -> Self {
        match val {
            JsFloat::Left => taffy::style::Float::Left,
            JsFloat::Right => taffy::style::Float::Right,
            JsFloat::None => taffy::style::Float::None,
        }
    }
}

impl From<taffy::style::Float> for JsFloat {
    fn from(val: taffy::style::Float) -> Self {
        match val {
            taffy::style::Float::Left => JsFloat::Left,
            taffy::style::Float::Right => JsFloat::Right,
            taffy::style::Float::None => JsFloat::None,
        }
    }
}

/// Controls which preceding floats a box must clear in block layout.
#[wasm_bindgen(js_name = Clear)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum JsClear {
    /// Clear preceding left floats
    Left = 0,
    /// Clear preceding right floats
    Right = 1,
    /// Clear preceding floats on both sides
    Both = 2,
    /// Do not clear preceding floats
    None = 3,
}

impl From<JsClear> for taffy::style::Clear {
    fn from(val: JsClear) -> Self {
        match val {
            JsClear::Left => taffy::style::Clear::Left,
            JsClear::Right => taffy::style::Clear::Right,
            JsClear::Both => taffy::style::Clear::Both,
            JsClear::None => taffy::style::Clear::None,
        }
    }
}

impl From<taffy::style::Clear> for JsClear {
    fn from(val: taffy::style::Clear) -> Self {
        match val {
            taffy::style::Clear::Left => JsClear::Left,
            taffy::style::Clear::Right => JsClear::Right,
            taffy::style::Clear::Both => JsClear::Both,
            taffy::style::Clear::None => JsClear::None,
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
/// import { Style, FlexDirection } from 'taffy-layout';
///
/// const style = new Style();
/// style.flexDirection = FlexDirection.Row;     // Horizontal, following direction (LTR by default)
/// style.flexDirection = FlexDirection.Column;  // Vertical, top to bottom
/// ```
#[wasm_bindgen(js_name = FlexDirection)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum JsFlexDirection {
    /// Main axis follows the container's horizontal writing direction (LTR or RTL)
    Row = 0,
    /// Main axis runs vertically from top to bottom
    Column = 1,
    /// Main axis runs opposite to the container's horizontal writing direction
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
/// import { Style, FlexWrap } from 'taffy-layout';
///
/// const style = new Style();
/// style.flexWrap = FlexWrap.NoWrap;  // All items on single line
/// style.flexWrap = FlexWrap.Wrap;    // Items wrap to new lines
/// ```
#[wasm_bindgen(js_name = FlexWrap)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum JsFlexWrap {
    /// All flex items are placed on a single line
    NoWrap = 0,
    /// Flex items wrap into lines along the cross axis, following the container's direction
    Wrap = 1,
    /// Flex items wrap into lines in the reverse cross-axis direction
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
/// import { Style, AlignItems } from 'taffy-layout';
///
/// const style = new Style();
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
    /// Items aligned to the start edge determined by the item's own direction
    SelfStart = 7,
    /// Items aligned to the end edge determined by the item's own direction
    SelfEnd = 8,
    /// Safe start alignment that avoids start-edge overflow
    SafeStart = 9,
    /// Safe end alignment that avoids start-edge overflow
    SafeEnd = 10,
    /// Safe flex-start alignment that avoids start-edge overflow
    SafeFlexStart = 11,
    /// Safe flex-end alignment that avoids start-edge overflow
    SafeFlexEnd = 12,
    /// Safe center alignment that avoids start-edge overflow
    SafeCenter = 13,
    /// Safe self-start alignment that avoids start-edge overflow
    SafeSelfStart = 14,
    /// Safe self-end alignment that avoids start-edge overflow
    SafeSelfEnd = 15,
}

impl From<JsAlignItems> for taffy::style::AlignItems {
    fn from(val: JsAlignItems) -> Self {
        match val {
            JsAlignItems::Start => taffy::style::AlignItems::START,
            JsAlignItems::End => taffy::style::AlignItems::END,
            JsAlignItems::FlexStart => taffy::style::AlignItems::FLEX_START,
            JsAlignItems::FlexEnd => taffy::style::AlignItems::FLEX_END,
            JsAlignItems::Center => taffy::style::AlignItems::CENTER,
            JsAlignItems::Baseline => taffy::style::AlignItems::BASELINE,
            JsAlignItems::Stretch => taffy::style::AlignItems::STRETCH,
            JsAlignItems::SelfStart => taffy::style::AlignItems::SELF_START,
            JsAlignItems::SelfEnd => taffy::style::AlignItems::SELF_END,
            JsAlignItems::SafeStart => taffy::style::AlignItems::SAFE_START,
            JsAlignItems::SafeEnd => taffy::style::AlignItems::SAFE_END,
            JsAlignItems::SafeFlexStart => taffy::style::AlignItems::SAFE_FLEX_START,
            JsAlignItems::SafeFlexEnd => taffy::style::AlignItems::SAFE_FLEX_END,
            JsAlignItems::SafeCenter => taffy::style::AlignItems::SAFE_CENTER,
            JsAlignItems::SafeSelfStart => taffy::style::AlignItems::SAFE_SELF_START,
            JsAlignItems::SafeSelfEnd => taffy::style::AlignItems::SAFE_SELF_END,
        }
    }
}

impl From<taffy::style::AlignItems> for JsAlignItems {
    fn from(val: taffy::style::AlignItems) -> Self {
        use taffy::style::{AlignItemsKeyword as Keyword, AlignmentSafety as Safety};

        match (val.keyword, val.safety) {
            (Keyword::Start, Safety::Unsafe) => JsAlignItems::Start,
            (Keyword::End, Safety::Unsafe) => JsAlignItems::End,
            (Keyword::FlexStart, Safety::Unsafe) => JsAlignItems::FlexStart,
            (Keyword::FlexEnd, Safety::Unsafe) => JsAlignItems::FlexEnd,
            (Keyword::SelfStart, Safety::Unsafe) => JsAlignItems::SelfStart,
            (Keyword::SelfEnd, Safety::Unsafe) => JsAlignItems::SelfEnd,
            (Keyword::Center, Safety::Unsafe) => JsAlignItems::Center,
            (Keyword::Start, Safety::Safe) => JsAlignItems::SafeStart,
            (Keyword::End, Safety::Safe) => JsAlignItems::SafeEnd,
            (Keyword::FlexStart, Safety::Safe) => JsAlignItems::SafeFlexStart,
            (Keyword::FlexEnd, Safety::Safe) => JsAlignItems::SafeFlexEnd,
            (Keyword::SelfStart, Safety::Safe) => JsAlignItems::SafeSelfStart,
            (Keyword::SelfEnd, Safety::Safe) => JsAlignItems::SafeSelfEnd,
            (Keyword::Center, Safety::Safe) => JsAlignItems::SafeCenter,
            (Keyword::Baseline, _) => JsAlignItems::Baseline,
            (Keyword::Stretch, _) => JsAlignItems::Stretch,
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
/// import { Style, AlignSelf } from 'taffy-layout';
///
/// const style = new Style();
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
    /// Item aligned to the start edge determined by its own direction
    SelfStart = 8,
    /// Item aligned to the end edge determined by its own direction
    SelfEnd = 9,
    /// Safe start alignment that avoids start-edge overflow
    SafeStart = 10,
    /// Safe end alignment that avoids start-edge overflow
    SafeEnd = 11,
    /// Safe flex-start alignment that avoids start-edge overflow
    SafeFlexStart = 12,
    /// Safe flex-end alignment that avoids start-edge overflow
    SafeFlexEnd = 13,
    /// Safe center alignment that avoids start-edge overflow
    SafeCenter = 14,
    /// Safe self-start alignment that avoids start-edge overflow
    SafeSelfStart = 15,
    /// Safe self-end alignment that avoids start-edge overflow
    SafeSelfEnd = 16,
}

impl From<JsAlignSelf> for taffy::style::AlignSelf {
    fn from(val: JsAlignSelf) -> Self {
        match val {
            JsAlignSelf::Auto => taffy::style::AlignSelf::STRETCH,
            JsAlignSelf::Start => taffy::style::AlignSelf::START,
            JsAlignSelf::End => taffy::style::AlignSelf::END,
            JsAlignSelf::FlexStart => taffy::style::AlignSelf::FLEX_START,
            JsAlignSelf::FlexEnd => taffy::style::AlignSelf::FLEX_END,
            JsAlignSelf::Center => taffy::style::AlignSelf::CENTER,
            JsAlignSelf::Baseline => taffy::style::AlignSelf::BASELINE,
            JsAlignSelf::Stretch => taffy::style::AlignSelf::STRETCH,
            JsAlignSelf::SelfStart => taffy::style::AlignSelf::SELF_START,
            JsAlignSelf::SelfEnd => taffy::style::AlignSelf::SELF_END,
            JsAlignSelf::SafeStart => taffy::style::AlignSelf::SAFE_START,
            JsAlignSelf::SafeEnd => taffy::style::AlignSelf::SAFE_END,
            JsAlignSelf::SafeFlexStart => taffy::style::AlignSelf::SAFE_FLEX_START,
            JsAlignSelf::SafeFlexEnd => taffy::style::AlignSelf::SAFE_FLEX_END,
            JsAlignSelf::SafeCenter => taffy::style::AlignSelf::SAFE_CENTER,
            JsAlignSelf::SafeSelfStart => taffy::style::AlignSelf::SAFE_SELF_START,
            JsAlignSelf::SafeSelfEnd => taffy::style::AlignSelf::SAFE_SELF_END,
        }
    }
}

impl From<taffy::style::AlignSelf> for JsAlignSelf {
    fn from(val: taffy::style::AlignSelf) -> Self {
        use taffy::style::{AlignItemsKeyword as Keyword, AlignmentSafety as Safety};

        match (val.keyword, val.safety) {
            (Keyword::Start, Safety::Unsafe) => JsAlignSelf::Start,
            (Keyword::End, Safety::Unsafe) => JsAlignSelf::End,
            (Keyword::FlexStart, Safety::Unsafe) => JsAlignSelf::FlexStart,
            (Keyword::FlexEnd, Safety::Unsafe) => JsAlignSelf::FlexEnd,
            (Keyword::SelfStart, Safety::Unsafe) => JsAlignSelf::SelfStart,
            (Keyword::SelfEnd, Safety::Unsafe) => JsAlignSelf::SelfEnd,
            (Keyword::Center, Safety::Unsafe) => JsAlignSelf::Center,
            (Keyword::Start, Safety::Safe) => JsAlignSelf::SafeStart,
            (Keyword::End, Safety::Safe) => JsAlignSelf::SafeEnd,
            (Keyword::FlexStart, Safety::Safe) => JsAlignSelf::SafeFlexStart,
            (Keyword::FlexEnd, Safety::Safe) => JsAlignSelf::SafeFlexEnd,
            (Keyword::SelfStart, Safety::Safe) => JsAlignSelf::SafeSelfStart,
            (Keyword::SelfEnd, Safety::Safe) => JsAlignSelf::SafeSelfEnd,
            (Keyword::Center, Safety::Safe) => JsAlignSelf::SafeCenter,
            (Keyword::Baseline, _) => JsAlignSelf::Baseline,
            (Keyword::Stretch, _) => JsAlignSelf::Stretch,
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
/// In Flexbox, this property distributes wrapped lines. In Grid it aligns row
/// tracks; in block layout it aligns the block content vertically.
///
/// @example
/// ```typescript
/// import { Style, AlignContent, FlexWrap } from 'taffy-layout';
///
/// const style = new Style();
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
    /// Safe start alignment that avoids start-edge overflow
    SafeStart = 9,
    /// Safe end alignment that avoids start-edge overflow
    SafeEnd = 10,
    /// Safe flex-start alignment that avoids start-edge overflow
    SafeFlexStart = 11,
    /// Safe flex-end alignment that avoids start-edge overflow
    SafeFlexEnd = 12,
    /// Safe center alignment that avoids start-edge overflow
    SafeCenter = 13,
}

impl From<JsAlignContent> for taffy::style::AlignContent {
    fn from(val: JsAlignContent) -> Self {
        match val {
            JsAlignContent::Start => taffy::style::AlignContent::START,
            JsAlignContent::End => taffy::style::AlignContent::END,
            JsAlignContent::FlexStart => taffy::style::AlignContent::FLEX_START,
            JsAlignContent::FlexEnd => taffy::style::AlignContent::FLEX_END,
            JsAlignContent::Center => taffy::style::AlignContent::CENTER,
            JsAlignContent::Stretch => taffy::style::AlignContent::STRETCH,
            JsAlignContent::SpaceBetween => taffy::style::AlignContent::SPACE_BETWEEN,
            JsAlignContent::SpaceAround => taffy::style::AlignContent::SPACE_AROUND,
            JsAlignContent::SpaceEvenly => taffy::style::AlignContent::SPACE_EVENLY,
            JsAlignContent::SafeStart => taffy::style::AlignContent::SAFE_START,
            JsAlignContent::SafeEnd => taffy::style::AlignContent::SAFE_END,
            JsAlignContent::SafeFlexStart => taffy::style::AlignContent::SAFE_FLEX_START,
            JsAlignContent::SafeFlexEnd => taffy::style::AlignContent::SAFE_FLEX_END,
            JsAlignContent::SafeCenter => taffy::style::AlignContent::SAFE_CENTER,
        }
    }
}

impl From<taffy::style::AlignContent> for JsAlignContent {
    fn from(val: taffy::style::AlignContent) -> Self {
        use taffy::style::{AlignContentKeyword as Keyword, AlignmentSafety as Safety};

        match (val.keyword, val.safety) {
            (Keyword::Start, Safety::Unsafe) => JsAlignContent::Start,
            (Keyword::End, Safety::Unsafe) => JsAlignContent::End,
            (Keyword::FlexStart, Safety::Unsafe) => JsAlignContent::FlexStart,
            (Keyword::FlexEnd, Safety::Unsafe) => JsAlignContent::FlexEnd,
            (Keyword::Center, Safety::Unsafe) => JsAlignContent::Center,
            (Keyword::Start, Safety::Safe) => JsAlignContent::SafeStart,
            (Keyword::End, Safety::Safe) => JsAlignContent::SafeEnd,
            (Keyword::FlexStart, Safety::Safe) => JsAlignContent::SafeFlexStart,
            (Keyword::FlexEnd, Safety::Safe) => JsAlignContent::SafeFlexEnd,
            (Keyword::Center, Safety::Safe) => JsAlignContent::SafeCenter,
            (Keyword::Stretch, _) => JsAlignContent::Stretch,
            (Keyword::SpaceBetween, _) => JsAlignContent::SpaceBetween,
            (Keyword::SpaceAround, _) => JsAlignContent::SpaceAround,
            (Keyword::SpaceEvenly, _) => JsAlignContent::SpaceEvenly,
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
/// import { Style, JustifyContent } from 'taffy-layout';
///
/// const style = new Style();
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
    /// Safe start alignment that avoids start-edge overflow
    SafeStart = 9,
    /// Safe end alignment that avoids start-edge overflow
    SafeEnd = 10,
    /// Safe flex-start alignment that avoids start-edge overflow
    SafeFlexStart = 11,
    /// Safe flex-end alignment that avoids start-edge overflow
    SafeFlexEnd = 12,
    /// Safe center alignment that avoids start-edge overflow
    SafeCenter = 13,
}

impl From<JsJustifyContent> for taffy::style::JustifyContent {
    fn from(val: JsJustifyContent) -> Self {
        match val {
            JsJustifyContent::Start => taffy::style::JustifyContent::START,
            JsJustifyContent::End => taffy::style::JustifyContent::END,
            JsJustifyContent::FlexStart => taffy::style::JustifyContent::FLEX_START,
            JsJustifyContent::FlexEnd => taffy::style::JustifyContent::FLEX_END,
            JsJustifyContent::Center => taffy::style::JustifyContent::CENTER,
            JsJustifyContent::Stretch => taffy::style::JustifyContent::STRETCH,
            JsJustifyContent::SpaceBetween => taffy::style::JustifyContent::SPACE_BETWEEN,
            JsJustifyContent::SpaceAround => taffy::style::JustifyContent::SPACE_AROUND,
            JsJustifyContent::SpaceEvenly => taffy::style::JustifyContent::SPACE_EVENLY,
            JsJustifyContent::SafeStart => taffy::style::JustifyContent::SAFE_START,
            JsJustifyContent::SafeEnd => taffy::style::JustifyContent::SAFE_END,
            JsJustifyContent::SafeFlexStart => taffy::style::JustifyContent::SAFE_FLEX_START,
            JsJustifyContent::SafeFlexEnd => taffy::style::JustifyContent::SAFE_FLEX_END,
            JsJustifyContent::SafeCenter => taffy::style::JustifyContent::SAFE_CENTER,
        }
    }
}

impl From<taffy::style::JustifyContent> for JsJustifyContent {
    fn from(val: taffy::style::JustifyContent) -> Self {
        use taffy::style::{AlignContentKeyword as Keyword, AlignmentSafety as Safety};

        match (val.keyword, val.safety) {
            (Keyword::Start, Safety::Unsafe) => JsJustifyContent::Start,
            (Keyword::End, Safety::Unsafe) => JsJustifyContent::End,
            (Keyword::FlexStart, Safety::Unsafe) => JsJustifyContent::FlexStart,
            (Keyword::FlexEnd, Safety::Unsafe) => JsJustifyContent::FlexEnd,
            (Keyword::Center, Safety::Unsafe) => JsJustifyContent::Center,
            (Keyword::Start, Safety::Safe) => JsJustifyContent::SafeStart,
            (Keyword::End, Safety::Safe) => JsJustifyContent::SafeEnd,
            (Keyword::FlexStart, Safety::Safe) => JsJustifyContent::SafeFlexStart,
            (Keyword::FlexEnd, Safety::Safe) => JsJustifyContent::SafeFlexEnd,
            (Keyword::Center, Safety::Safe) => JsJustifyContent::SafeCenter,
            (Keyword::Stretch, _) => JsJustifyContent::Stretch,
            (Keyword::SpaceBetween, _) => JsJustifyContent::SpaceBetween,
            (Keyword::SpaceAround, _) => JsJustifyContent::SpaceAround,
            (Keyword::SpaceEvenly, _) => JsJustifyContent::SpaceEvenly,
        }
    }
}

// =============================================================================
// Overflow
// =============================================================================

/// Overflow handling enumeration
///
/// Controls overflow sizing, automatic minimum sizes, and scrollbar space.
/// This corresponds to the CSS `overflow` property.
/// Taffy computes layout only; clipping, drawing, and scrolling are implemented
/// by the renderer consuming the layout.
///
/// @example
/// ```typescript
/// import { Style, Overflow } from 'taffy-layout';
///
/// const style = new Style();
/// style.overflow = { x: Overflow.Hidden, y: Overflow.Scroll };
/// ```
#[wasm_bindgen(js_name = Overflow)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum JsOverflow {
    /// Visible overflow semantics; retain content-based automatic minimum sizes
    Visible = 0,
    /// Clipped, non-scrollable overflow semantics; retain content-based automatic minimum sizes
    Clip = 1,
    /// Hidden overflow semantics; allow the automatic minimum size to shrink to zero
    Hidden = 2,
    /// Reserve scrollbar space using scrollbarWidth and allow zero automatic minimum sizes
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
/// import { Style, BoxSizing } from 'taffy-layout';
///
/// const style = new Style();
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

// =============================================================================
// Text Align
// =============================================================================

/// Text alignment enumeration (for block layout)
///
/// Used by block layout to implement the legacy behaviour of `<center>` and
/// `<div align="left | right | center">`.
///
/// @example
/// ```typescript
/// import { Style, TextAlign } from 'taffy-layout';
///
/// const style = new Style();
/// style.textAlign = TextAlign.LegacyCenter;  // Center block children
/// ```
#[wasm_bindgen(js_name = TextAlign)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum JsTextAlign {
    /// No special legacy text align behaviour
    Auto = 0,
    /// Corresponds to `-webkit-left` or `-moz-left` in browsers
    LegacyLeft = 1,
    /// Corresponds to `-webkit-right` or `-moz-right` in browsers
    LegacyRight = 2,
    /// Corresponds to `-webkit-center` or `-moz-center` in browsers
    LegacyCenter = 3,
}

impl From<JsTextAlign> for taffy::style::TextAlign {
    fn from(val: JsTextAlign) -> Self {
        match val {
            JsTextAlign::Auto => taffy::style::TextAlign::Auto,
            JsTextAlign::LegacyLeft => taffy::style::TextAlign::LegacyLeft,
            JsTextAlign::LegacyRight => taffy::style::TextAlign::LegacyRight,
            JsTextAlign::LegacyCenter => taffy::style::TextAlign::LegacyCenter,
        }
    }
}

impl From<taffy::style::TextAlign> for JsTextAlign {
    fn from(val: taffy::style::TextAlign) -> Self {
        match val {
            taffy::style::TextAlign::Auto => JsTextAlign::Auto,
            taffy::style::TextAlign::LegacyLeft => JsTextAlign::LegacyLeft,
            taffy::style::TextAlign::LegacyRight => JsTextAlign::LegacyRight,
            taffy::style::TextAlign::LegacyCenter => JsTextAlign::LegacyCenter,
        }
    }
}

// =============================================================================
// Grid Auto Flow
// =============================================================================

/// Grid auto flow enumeration
///
/// Controls whether grid items are placed row-wise or column-wise, and whether
/// the sparse or dense packing algorithm is used.
///
/// @example
/// ```typescript
/// import { Style, GridAutoFlow } from 'taffy-layout';
///
/// const style = new Style();
/// style.gridAutoFlow = GridAutoFlow.Row;       // Fill rows first
/// style.gridAutoFlow = GridAutoFlow.Column;    // Fill columns first
/// style.gridAutoFlow = GridAutoFlow.RowDense;  // Fill rows, pack densely
/// ```
#[wasm_bindgen(js_name = GridAutoFlow)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum JsGridAutoFlow {
    /// Items are placed by filling each row in turn, adding new rows as necessary
    Row = 0,
    /// Items are placed by filling each column in turn, adding new columns as necessary
    Column = 1,
    /// Combines `Row` with the dense packing algorithm
    RowDense = 2,
    /// Combines `Column` with the dense packing algorithm
    ColumnDense = 3,
}

impl From<JsGridAutoFlow> for taffy::style::GridAutoFlow {
    fn from(val: JsGridAutoFlow) -> Self {
        match val {
            JsGridAutoFlow::Row => taffy::style::GridAutoFlow::Row,
            JsGridAutoFlow::Column => taffy::style::GridAutoFlow::Column,
            JsGridAutoFlow::RowDense => taffy::style::GridAutoFlow::RowDense,
            JsGridAutoFlow::ColumnDense => taffy::style::GridAutoFlow::ColumnDense,
        }
    }
}

impl From<taffy::style::GridAutoFlow> for JsGridAutoFlow {
    fn from(val: taffy::style::GridAutoFlow) -> Self {
        match val {
            taffy::style::GridAutoFlow::Row => JsGridAutoFlow::Row,
            taffy::style::GridAutoFlow::Column => JsGridAutoFlow::Column,
            taffy::style::GridAutoFlow::RowDense => JsGridAutoFlow::RowDense,
            taffy::style::GridAutoFlow::ColumnDense => JsGridAutoFlow::ColumnDense,
        }
    }
}

pub(crate) trait CheckedJsEnum: Sized {
    fn from_repr(value: u8) -> Option<Self>;
}

macro_rules! impl_checked_js_enum {
    ($enum:ident { $($variant:ident),+ $(,)? }) => {
        impl CheckedJsEnum for $enum {
            fn from_repr(value: u8) -> Option<Self> {
                match value {
                    $(value if value == $enum::$variant as u8 => Some($enum::$variant),)+
                    _ => None,
                }
            }
        }
    };
}

impl_checked_js_enum!(JsDisplay {
    Block,
    Flex,
    Grid,
    None,
    FlowRoot
});
impl_checked_js_enum!(JsPosition { Relative, Absolute });
impl_checked_js_enum!(JsDirection { Ltr, Rtl });
impl_checked_js_enum!(JsFloat { Left, Right, None });
impl_checked_js_enum!(JsClear {
    Left,
    Right,
    Both,
    None
});
impl_checked_js_enum!(JsFlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse
});
impl_checked_js_enum!(JsFlexWrap {
    NoWrap,
    Wrap,
    WrapReverse
});
impl_checked_js_enum!(JsAlignItems {
    Start,
    End,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
    SelfStart,
    SelfEnd,
    SafeStart,
    SafeEnd,
    SafeFlexStart,
    SafeFlexEnd,
    SafeCenter,
    SafeSelfStart,
    SafeSelfEnd
});
impl_checked_js_enum!(JsAlignSelf {
    Auto,
    Start,
    End,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
    SelfStart,
    SelfEnd,
    SafeStart,
    SafeEnd,
    SafeFlexStart,
    SafeFlexEnd,
    SafeCenter,
    SafeSelfStart,
    SafeSelfEnd
});
impl_checked_js_enum!(JsAlignContent {
    Start,
    End,
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
    SafeStart,
    SafeEnd,
    SafeFlexStart,
    SafeFlexEnd,
    SafeCenter
});
impl_checked_js_enum!(JsJustifyContent {
    Start,
    End,
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
    SafeStart,
    SafeEnd,
    SafeFlexStart,
    SafeFlexEnd,
    SafeCenter
});
impl_checked_js_enum!(JsOverflow {
    Visible,
    Clip,
    Hidden,
    Scroll
});
impl_checked_js_enum!(JsBoxSizing {
    BorderBox,
    ContentBox
});
impl_checked_js_enum!(JsTextAlign {
    Auto,
    LegacyLeft,
    LegacyRight,
    LegacyCenter
});
impl_checked_js_enum!(JsGridAutoFlow {
    Row,
    Column,
    RowDense,
    ColumnDense
});
