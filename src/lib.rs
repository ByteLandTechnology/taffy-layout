//! Taffy WebAssembly bindings
//!
//! This library provides WebAssembly bindings for the Taffy layout library,
//! allowing it to be used in JavaScript/TypeScript environments.
//!
//! ## Logging
//!
//! Debug logging is available when the `debug` feature is enabled. Logs are
//! output to the browser console via `web_sys::console`. This is useful for
//! debugging layout issues in development.

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use taffy::{
    style as taffy_style,
    geometry::Size as TaffySize,
    tree::TaffyTree,
    prelude::*,
};

// ============================================================================
// Logging macros
// ============================================================================

/// Debug logging macro that outputs to the browser console.
/// Only active when the `debug` feature is enabled.
#[cfg(feature = "debug")]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        web_sys::console::debug_1(&format!($($arg)*).into());
    };
}

#[cfg(not(feature = "debug"))]
macro_rules! log_debug {
    ($($arg:tt)*) => {};
}

/// Info logging macro that outputs to the browser console.
#[cfg(feature = "debug")]
macro_rules! log_info {
    ($($arg:tt)*) => {
        web_sys::console::info_1(&format!($($arg)*).into());
    };
}

#[cfg(not(feature = "debug"))]
macro_rules! log_info {
    ($($arg:tt)*) => {};
}

/// Warning logging macro that outputs to the browser console.
/// Always active for important warnings.
macro_rules! log_warn {
    ($($arg:tt)*) => {
        web_sys::console::warn_1(&format!($($arg)*).into());
    };
}

/// Error logging macro that outputs to the browser console.
/// Always active for error reporting.
macro_rules! log_error {
    ($($arg:tt)*) => {
        web_sys::console::error_1(&format!($($arg)*).into());
    };
}


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// ============================================================================
// Type definitions
// ============================================================================

#[derive(Clone, Default)]
pub struct NodeContext {
    pub measure_func: Option<js_sys::Function>,
}

impl std::fmt::Debug for NodeContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeContext")
         .field("measure_func", &self.measure_func.is_some())
         .finish()
    }
}

/// Represents a length value in CSS.

/// Represents a length value in CSS.
///
/// This struct corresponds to a dimension value that can be specified in pixels, percentage, or auto.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Dimension {
    /// The numeric value of the dimension.
    /// - For `Pixels`, this is the number of pixels.
    /// - For `Percent`, this is the percentage value (0.0 to 100.0, or sometimes 0.0 to 1.0 depending on context, handled by internal logic).
    /// - For `Auto`, this value is typically ignored.
    pub value: f32,
    /// The unit of the dimension.
    pub unit: DimensionUnit,
}

/// The unit of a dimension.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum DimensionUnit {
    /// The dimension is specified in logical pixels.
    Pixels,
    /// The dimension is specified as a percentage of the parent's size.
    Percent,
    /// The dimension is determined automatically based on content or context.
    Auto,
}

/// Represents a point in 2D space.
///
/// Typically used for coordinates like absolute positioning offsets.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Point {
    /// The x-coordinate (horizontal).
    pub x: f32,
    /// The y-coordinate (vertical).
    pub y: f32,
}

/// Represents a size in 2D space.
///
/// Used for width and height dimensions.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Size {
    /// The width dimension.
    pub width: f32,
    /// The height dimension.
    pub height: f32,
}

impl From<TaffySize<f32>> for Size {
    fn from(size: TaffySize<f32>) -> Self {
        Size { width: size.width, height: size.height }
    }
}

/// Represents a rectangle defined by its edges.
///
/// Used for padding, margin, properties.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Rect {
    /// The left edge value.
    pub left: f32,
    /// The right edge value.
    pub right: f32,
    /// The top edge value.
    pub top: f32,
    /// The bottom edge value.
    pub bottom: f32,
}

/// Represents the computed layout of a node.
///
/// This struct contains the final position and size of a node after layout computation.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Layout {
    /// The absolute x-coordinate of the node relative to its parent.
    pub x: f32,
    /// The absolute y-coordinate of the node relative to its parent.
    pub y: f32,
    /// The computed width of the node.
    pub width: f32,
    /// The computed height of the node.
    pub height: f32,
}

impl From<taffy::Layout> for Layout {
    fn from(layout: taffy::Layout) -> Self {
        Layout {
            x: layout.location.x,
            y: layout.location.y,
            width: layout.size.width,
            height: layout.size.height,
        }
    }
}

/// The display style of a node.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Display {
    /// The node is hidden and does not take up space.
    None,
    /// The node behaves as a flex container.
    Flex,
    /// The node behaves as a grid container.
    Grid,
    /// The node behaves as a block element.
    Block,
}

/// Text alignment within a node (mostly ignored in flex/grid layout but preserved for compatibility).
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum TextAlign {
    Auto,
    Left,
    Right,
    Center,
    Justify,
}

/// The direction of the main axis for a flex container.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum FlexDirection {
    /// Items are placed horizontally from left to right.
    Row,
    /// Items are placed vertically from top to bottom.
    Column,
    /// Items are placed horizontally from right to left.
    RowReverse,
    /// Items are placed vertically from bottom to top.
    ColumnReverse,
}

/// How items are distributed along the main axis.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum JustifyContent {
    /// Items are packed toward the start of the layout direction.
    Start,
    /// Items are packed toward the end of the layout direction.
    End,
    /// Items are packed toward the start of the flex-direction.
    FlexStart,
    /// Items are packed toward the end of the flex-direction.
    FlexEnd,
    /// Items are centered along the line.
    Center,
    /// Items are evenly distributed; the first item is at the start, the last at the end.
    SpaceBetween,
    /// Items are evenly distributed with equal space around them.
    SpaceAround,
    /// Items are evenly distributed with equal space between them using the same gap at ends.
    SpaceEvenly,
}

/// How items are aligned along the cross axis.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum AlignItems {
    /// Items are aligned at the start of the cross axis.
    Start,
    /// Items are aligned at the end of the cross axis.
    End,
    /// Items are aligned at the start of the flex-direction cross axis.
    FlexStart,
    /// Items are aligned at the end of the flex-direction cross axis.
    FlexEnd,
    /// Items are aligned at the center of the cross axis.
    Center,
    /// Items are aligned based on their baselines.
    Baseline,
    /// Items are stretched to fill the container along the cross axis.
    Stretch,
}

/// How a single item is aligned along the cross axis, overriding `AlignItems`.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum AlignSelf {
    /// Auto
    Auto,
    /// items are aligned at the start of the cross axis.
    Start,
    /// Items are aligned at the end of the cross axis.
    End,
    /// Items are aligned at the start of the flex-direction cross axis.
    FlexStart,
    /// Items are aligned at the end of the flex-direction cross axis.
    FlexEnd,
    /// Items are aligned at the center of the cross axis.
    Center,
    /// Items are aligned based on their baselines.
    Baseline,
    /// Items are stretched to fill the container along the cross axis.
    Stretch,
}

/// How lines of content are aligned along the cross axis when there is extra space.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum AlignContent {
    /// Lines are packed toward the start of the cross axis.
    Start,
    /// Lines are packed toward the end of the cross axis.
    End,
    /// Lines are packed toward the start of the flex-direction cross axis.
    FlexStart,
    /// Lines are packed toward the end of the flex-direction cross axis.
    FlexEnd,
    /// Lines are packed toward the center of the cross axis.
    Center,
    /// Lines are evenly distributed; the first line is at the start, the last at the end.
    SpaceBetween,
    /// Lines are evenly distributed with equal space around them.
    SpaceAround,
    /// Lines are evenly distributed with equal space between them.
    SpaceEvenly,
    /// Lines are stretched to take up the remaining space.
    Stretch,
}

/// Whether flex items are forced into a single line or can wrap onto multiple lines.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum FlexWrap {
    /// Items are forced into a single line.
    NoWrap,
    /// Items wrap onto multiple lines.
    Wrap,
    /// Items wrap onto multiple lines in reverse order.
    WrapReverse,
}

/// Positioning strategy for a node.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Position {
    /// Relative to its normal position in the flow.
    Static, // Note: Taffy treats Static mostly same as Relative for layout purposes
    /// Relative to its normal position in the flow.
    Relative,
    /// Removed from the flow and positioned relative to its containing block.
    Absolute,
}

/// Grid auto-placement algorithm controls how auto-placed items get flowed into the grid.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum GridAutoFlow {
    /// Items are placed by filling each row in turn, adding new rows as necessary.
    Row,
    /// Items are placed by filling each column in turn, adding new columns as necessary.
    Column,
    /// Items are placed by filling each row, attempting to fill holes earlier in the grid.
    RowDense,
    /// Items are placed by filling each column, attempting to fill holes earlier in the grid.
    ColumnDense,
}

/// Start and End are used for logical positioning (e.g. paddingStart).
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Edge {
    Left,
    Right,
    Top,
    Bottom,
    Start,
    End,
    Horizontal,
    Vertical,
    All,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Gutter {
    Column,
    Row,
    All,
}

/// Style values for a node.
///
/// This struct aggregates all layout styles that can be applied to a node.
/// Optional fields allow partial updates or default values.
#[wasm_bindgen]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Style {
    /// The display mode of the node (e.g. Flex, Grid, None).
    pub display: Option<Display>,
    /// The positioning strategy (e.g. Relative, Absolute).
    pub position: Option<Position>,

    // Size
    /// The width of the node.
    pub width: Option<Dimension>,
    /// The height of the node.
    pub height: Option<Dimension>,
    /// The minimum width of the node.
    pub min_width: Option<Dimension>,
    /// The minimum height of the node.
    pub min_height: Option<Dimension>,
    /// The maximum width of the node.
    pub max_width: Option<Dimension>,
    /// The maximum height of the node.
    pub max_height: Option<Dimension>,

    // Position
    /// The offset from the left edge (used with Position::Absolute/Relative).
    pub left: Option<Dimension>,
    /// The offset from the right edge.
    pub right: Option<Dimension>,
    /// The offset from the top edge.
    pub top: Option<Dimension>,
    /// The offset from the bottom edge.
    pub bottom: Option<Dimension>,

    // Margin
    /// The margin on the left side.
    pub margin_left: Option<Dimension>,
    /// The margin on the right side.
    pub margin_right: Option<Dimension>,
    /// The margin on the top side.
    pub margin_top: Option<Dimension>,
    /// The margin on the bottom side.
    pub margin_bottom: Option<Dimension>,

    // Padding
    /// The padding on the left side.
    pub padding_left: Option<Dimension>,
    /// The padding on the right side.
    pub padding_right: Option<Dimension>,
    /// The padding on the top side.
    pub padding_top: Option<Dimension>,
    /// The padding on the bottom side.
    pub padding_bottom: Option<Dimension>,

    // Flexbox
    /// The flex direction (e.g. Row, Column).
    pub flex_direction: Option<FlexDirection>,
    /// Whether flex items should wrap.
    pub flex_wrap: Option<FlexWrap>,
    /// How much the item will grow relative to the rest of the flexible items.
    pub flex_grow: Option<f32>,
    /// How much the item will shrink relative to the rest of the flexible items.
    pub flex_shrink: Option<f32>,
    /// The initial main size of a flex item.
    pub flex_basis: Option<Dimension>,
    /// How items are distributed along the main axis.
    pub justify_content: Option<JustifyContent>,
    /// How items are aligned along the cross axis.
    pub align_items: Option<AlignItems>,
    /// How a single item is aligned along the cross axis.
    pub align_self: Option<AlignSelf>,
    /// How lines of content are aligned along the cross axis.
    pub align_content: Option<AlignContent>,
    /// The gap between rows (flex/grid).
    pub row_gap: Option<Dimension>,
    /// The gap between columns (flex/grid).
    pub column_gap: Option<Dimension>,

    // Grid
    /// Row track definitions for grid layout.
    #[wasm_bindgen(skip)]
    pub grid_template_rows: Option<Vec<TrackDefinition>>,
    /// Column track definitions for grid layout.
    #[wasm_bindgen(skip)]
    pub grid_template_columns: Option<Vec<TrackDefinition>>,
    /// Auto-generated row track definitions.
    #[wasm_bindgen(skip)]
    pub grid_auto_rows: Option<Vec<TrackDefinition>>,
    /// Auto-generated column track definitions.
    #[wasm_bindgen(skip)]
    pub grid_auto_columns: Option<Vec<TrackDefinition>>,
    /// Algorithm for auto-placing items in the grid.
    pub grid_auto_flow: Option<GridAutoFlow>,
    /// Placement of the item in grid rows.
    #[wasm_bindgen(skip)]
    pub grid_row: Option<Line>,
    /// Placement of the item in grid columns.
    #[wasm_bindgen(skip)]
    pub grid_column: Option<Line>,

    // Block
    /// Text alignment.
    pub text_align: Option<TextAlign>,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            display: Some(Display::Flex),
            text_align: None,
            position: Some(Position::Relative),
            width: None,
            height: None,
            min_width: None,
            min_height: None,
            max_width: None,
            max_height: None,
            left: None,
            right: None,
            top: None,
            bottom: None,
            margin_left: None,
            margin_right: None,
            margin_top: None,
            margin_bottom: None,
            padding_left: None,
            padding_right: None,
            padding_top: None,
            padding_bottom: None,
            flex_direction: None,
            flex_wrap: None,
            flex_grow: None,
            flex_shrink: None,
            flex_basis: None,
            justify_content: None,
            align_items: None,
            align_self: None,
            align_content: None,
            row_gap: None,
            column_gap: None,
            grid_template_rows: None,
            grid_template_columns: None,
            grid_auto_rows: None,
            grid_auto_columns: None,
            grid_auto_flow: None,
            grid_row: None,
            grid_column: None,
        }
    }
}

/// Definition of a single grid track (row or column).
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TrackDefinition {
    /// The numeric value of the track size.
    pub value: f32,
    /// The unit of the track size.
    pub unit: TrackUnit,
}

/// The unit for a grid track definition.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum TrackUnit {
    /// The track size is specified in logical pixels.
    Pixels,
    /// The track size is specified as a percentage of the container.
    Percent,
    /// The track size is a fraction of the remaining free space (fr unit).
    Fraction, // 'fr' unit
    /// The track size is determined automatically.
    Auto,
    /// The track size is the minimum size needed to fit the content.
    MinContent,
    /// The track size is the maximum size needed to fit the content.
    MaxContent,
}

/// Represents a grid line placement (start and end).
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Line {
    /// The start line index (1-based).
    pub start: Option<i16>,
    /// The end line index (1-based).
    pub end: Option<i16>,
}

/// The available space for layout computation.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct AvailableSpace {
    /// The available width (None means undefined/max-content).
    pub width: Option<f32>,
    /// The available height (None means undefined/max-content).
    pub height: Option<f32>,
}

// ============================================================================
// Conversion helpers
// ============================================================================

fn convert_style(style: &Style) -> taffy_style::Style {
    let mut taffy_style = taffy_style::Style::default();

    // Display
    if let Some(display) = style.display {
        taffy_style.display = match display {
            Display::None => taffy_style::Display::None,
            Display::Flex => taffy_style::Display::Flex,
            Display::Grid => taffy_style::Display::Grid,
            Display::Block => taffy_style::Display::Block,
        };
    }

    // Position
    if let Some(position) = style.position {
        taffy_style.position = match position {
            Position::Static => taffy_style::Position::Relative,
            Position::Relative => taffy_style::Position::Relative,
            Position::Absolute => taffy_style::Position::Absolute,
        };
    }

    // Size
    fn to_dimension(dim: &Dimension) -> taffy_style::Dimension {
        match dim.unit {
            DimensionUnit::Pixels => taffy_style::Dimension::length(dim.value),
            DimensionUnit::Percent => taffy_style::Dimension::percent(dim.value / 100.0),
            DimensionUnit::Auto => taffy_style::Dimension::auto(),
        }
    }

    fn to_length_percentage_auto(dim: &Dimension) -> taffy_style::LengthPercentageAuto {
        match dim.unit {
            DimensionUnit::Pixels => taffy_style::LengthPercentageAuto::length(dim.value),
            DimensionUnit::Percent => taffy_style::LengthPercentageAuto::percent(dim.value / 100.0),
            DimensionUnit::Auto => taffy_style::LengthPercentageAuto::auto(),
        }
    }

    fn to_length_percentage(dim: &Dimension) -> taffy_style::LengthPercentage {
        match dim.unit {
            DimensionUnit::Pixels => taffy_style::LengthPercentage::length(dim.value),
            DimensionUnit::Percent => taffy_style::LengthPercentage::percent(dim.value / 100.0),
            DimensionUnit::Auto => taffy_style::LengthPercentage::length(0.0),
        }
    }

    if let Some(ref w) = style.width { taffy_style.size.width = to_dimension(w); }
    if let Some(ref h) = style.height { taffy_style.size.height = to_dimension(h); }
    if let Some(ref w) = style.min_width { taffy_style.min_size.width = to_dimension(w); }
    if let Some(ref h) = style.min_height { taffy_style.min_size.height = to_dimension(h); }
    if let Some(ref w) = style.max_width { taffy_style.max_size.width = to_dimension(w); }
    if let Some(ref h) = style.max_height { taffy_style.max_size.height = to_dimension(h); }

    // Position (inset)
    if let Some(ref l) = style.left { taffy_style.inset.left = to_length_percentage_auto(l); }
    if let Some(ref r) = style.right { taffy_style.inset.right = to_length_percentage_auto(r); }
    if let Some(ref t) = style.top { taffy_style.inset.top = to_length_percentage_auto(t); }
    if let Some(ref b) = style.bottom { taffy_style.inset.bottom = to_length_percentage_auto(b); }

    // Margin
    if let Some(ref m) = style.margin_left { taffy_style.margin.left = to_length_percentage_auto(m); }
    if let Some(ref m) = style.margin_right { taffy_style.margin.right = to_length_percentage_auto(m); }
    if let Some(ref m) = style.margin_top { taffy_style.margin.top = to_length_percentage_auto(m); }
    if let Some(ref m) = style.margin_bottom { taffy_style.margin.bottom = to_length_percentage_auto(m); }

    // Padding
    if let Some(ref p) = style.padding_left { taffy_style.padding.left = to_length_percentage(p); }
    if let Some(ref p) = style.padding_right { taffy_style.padding.right = to_length_percentage(p); }
    if let Some(ref p) = style.padding_top { taffy_style.padding.top = to_length_percentage(p); }
    if let Some(ref p) = style.padding_bottom { taffy_style.padding.bottom = to_length_percentage(p); }

    // Flexbox
    if let Some(dir) = style.flex_direction {
        taffy_style.flex_direction = match dir {
            FlexDirection::Row => taffy_style::FlexDirection::Row,
            FlexDirection::Column => taffy_style::FlexDirection::Column,
            FlexDirection::RowReverse => taffy_style::FlexDirection::RowReverse,
            FlexDirection::ColumnReverse => taffy_style::FlexDirection::ColumnReverse,
        };
    }

    if let Some(wrap) = style.flex_wrap {
        taffy_style.flex_wrap = match wrap {
            FlexWrap::NoWrap => taffy_style::FlexWrap::NoWrap,
            FlexWrap::Wrap => taffy_style::FlexWrap::Wrap,
            FlexWrap::WrapReverse => taffy_style::FlexWrap::WrapReverse,
        };
    }

    if let Some(g) = style.flex_grow { taffy_style.flex_grow = g; }
    if let Some(s) = style.flex_shrink { taffy_style.flex_shrink = s; }
    if let Some(ref b) = style.flex_basis { taffy_style.flex_basis = to_dimension(b); }

    if let Some(jc) = style.justify_content {
        taffy_style.justify_content = Some(match jc {
            JustifyContent::Start => taffy_style::JustifyContent::Start,
            JustifyContent::End => taffy_style::JustifyContent::End,
            JustifyContent::FlexStart => taffy_style::JustifyContent::FlexStart,
            JustifyContent::FlexEnd => taffy_style::JustifyContent::FlexEnd,
            JustifyContent::Center => taffy_style::JustifyContent::Center,
            JustifyContent::SpaceBetween => taffy_style::JustifyContent::SpaceBetween,
            JustifyContent::SpaceAround => taffy_style::JustifyContent::SpaceAround,
            JustifyContent::SpaceEvenly => taffy_style::JustifyContent::SpaceEvenly,
        });
    }

    if let Some(ai) = style.align_items {
        taffy_style.align_items = Some(match ai {
            AlignItems::Start => taffy_style::AlignItems::Start,
            AlignItems::End => taffy_style::AlignItems::End,
            AlignItems::FlexStart => taffy_style::AlignItems::FlexStart,
            AlignItems::FlexEnd => taffy_style::AlignItems::FlexEnd,
            AlignItems::Center => taffy_style::AlignItems::Center,
            AlignItems::Baseline => taffy_style::AlignItems::Baseline,
            AlignItems::Stretch => taffy_style::AlignItems::Stretch,
        });
    }

    if let Some(align_self) = style.align_self {
        taffy_style.align_self = match align_self {
            AlignSelf::Auto => None,
            AlignSelf::Start => Some(taffy::style::AlignItems::Start),
            AlignSelf::End => Some(taffy::style::AlignItems::End),
            AlignSelf::FlexStart => Some(taffy::style::AlignItems::FlexStart),
            AlignSelf::FlexEnd => Some(taffy::style::AlignItems::FlexEnd),
            AlignSelf::Center => Some(taffy::style::AlignItems::Center),
            AlignSelf::Baseline => Some(taffy::style::AlignItems::Baseline),
            AlignSelf::Stretch => Some(taffy::style::AlignItems::Stretch),
        };
    }

    if let Some(ac) = style.align_content {
        taffy_style.align_content = Some(match ac {
            AlignContent::Start => taffy_style::AlignContent::Start,
            AlignContent::End => taffy_style::AlignContent::End,
            AlignContent::FlexStart => taffy_style::AlignContent::FlexStart,
            AlignContent::FlexEnd => taffy_style::AlignContent::FlexEnd,
            AlignContent::Center => taffy_style::AlignContent::Center,
            AlignContent::SpaceBetween => taffy_style::AlignContent::SpaceBetween,
            AlignContent::SpaceAround => taffy_style::AlignContent::SpaceAround,
            AlignContent::SpaceEvenly => taffy_style::AlignContent::SpaceEvenly,
            AlignContent::Stretch => taffy_style::AlignContent::Stretch,
        });
    }

    if let Some(ref g) = style.row_gap { taffy_style.gap.width = to_length_percentage(g); }
    if let Some(ref g) = style.column_gap { taffy_style.gap.height = to_length_percentage(g); }

    // Grid
    fn to_track_sizing_function(track: &TrackDefinition) -> taffy_style::TrackSizingFunction {

        use taffy::geometry::MinMax;
        use taffy::style_helpers::{length, percent, fr, auto, min_content, max_content};

        match track.unit {
            TrackUnit::Pixels => MinMax {
                min: length(track.value),
                max: length(track.value),
            },
            TrackUnit::Percent => MinMax {
                min: percent(track.value / 100.0),
                max: percent(track.value / 100.0),
            },
            TrackUnit::Fraction => MinMax {
                min: auto(),
                max: fr(track.value),
            },
            TrackUnit::Auto => MinMax { min: auto(), max: auto() },
            TrackUnit::MinContent => MinMax { min: min_content(), max: min_content() },
            TrackUnit::MaxContent => MinMax { min: max_content(), max: max_content() },
        }
    }

    // Helper to convert TrackDefinition to NonRepeatedTrackSizingFunction if possible, 
    // or we construct GridTemplateComponent which supports Repeat.
    
    fn to_grid_template_blocks(tracks: &[TrackDefinition]) -> Vec<taffy_style::GridTemplateComponent<<taffy_style::Style as taffy_style::CoreStyle>::CustomIdent>> {
        tracks.iter().map(|t| {
             let sizing = to_track_sizing_function(t);
             taffy_style::GridTemplateComponent::Single(sizing)
        }).collect()
    }
    
    fn to_grid_auto_blocks(tracks: &[TrackDefinition]) -> Vec<taffy_style::TrackSizingFunction> {
        tracks.iter().map(|t| {
            to_track_sizing_function(t)
        }).collect()
    }

    if let Some(ref tracks) = style.grid_template_rows {
        taffy_style.grid_template_rows = to_grid_template_blocks(tracks);
    }
    if let Some(ref tracks) = style.grid_template_columns {
        taffy_style.grid_template_columns = to_grid_template_blocks(tracks);
    }
    if let Some(ref tracks) = style.grid_auto_rows {
        taffy_style.grid_auto_rows = to_grid_auto_blocks(tracks);
    }
    if let Some(ref tracks) = style.grid_auto_columns {
        taffy_style.grid_auto_columns = to_grid_auto_blocks(tracks);
    }

    if let Some(flow) = style.grid_auto_flow {
        taffy_style.grid_auto_flow = match flow {
            GridAutoFlow::Row => taffy_style::GridAutoFlow::Row,
            GridAutoFlow::Column => taffy_style::GridAutoFlow::Column,
            GridAutoFlow::RowDense => taffy_style::GridAutoFlow::RowDense,
            GridAutoFlow::ColumnDense => taffy_style::GridAutoFlow::ColumnDense,
        };
    }

    if let Some(ref line) = style.grid_row {
        taffy_style.grid_row = taffy::geometry::Line {
            start: line.start.map_or(taffy_style::GridPlacement::Auto, |v| taffy_style::GridPlacement::from_line_index(v)),
            end: line.end.map_or(taffy_style::GridPlacement::Auto, |v| taffy_style::GridPlacement::from_line_index(v)),
        };
    }
    if let Some(ref line) = style.grid_column {
        taffy_style.grid_column = taffy::geometry::Line {
            start: line.start.map_or(taffy_style::GridPlacement::Auto, |v| taffy_style::GridPlacement::from_line_index(v)),
            end: line.end.map_or(taffy_style::GridPlacement::Auto, |v| taffy_style::GridPlacement::from_line_index(v)),
        };
    }

    if let Some(align) = style.text_align {
        taffy_style.text_align = match align {
            TextAlign::Auto => taffy_style::TextAlign::Auto,
            TextAlign::Left => taffy_style::TextAlign::LegacyLeft,
            TextAlign::Right => taffy_style::TextAlign::LegacyRight,
            TextAlign::Center => taffy_style::TextAlign::LegacyCenter,
            TextAlign::Justify => taffy_style::TextAlign::Auto,
        };
    }

    taffy_style
}

fn to_available_space(space: &AvailableSpace) -> TaffySize<taffy_style::AvailableSpace> {
    TaffySize {
        width: match space.width {
            Some(w) => taffy_style::AvailableSpace::Definite(w),
            None => taffy_style::AvailableSpace::MaxContent,
        },
        height: match space.height {
            Some(h) => taffy_style::AvailableSpace::Definite(h),
            None => taffy_style::AvailableSpace::MaxContent,
        },
    }
}

// ============================================================================
// Taffy wrapper
// ============================================================================

use std::collections::HashMap;

thread_local! {
    static TAFFY: RefCell<TaffyTree<NodeContext>> = RefCell::new(TaffyTree::new());
    static NEXT_NODE_ID: RefCell<u32> = RefCell::new(0);
    /// Maps node ID to its allocation epoch (a unique identifier for each allocation)
    static NODE_EPOCHS: RefCell<HashMap<u64, u64>> = RefCell::new(HashMap::new());
    /// Counter for generating unique allocation epochs
    static EPOCH_COUNTER: RefCell<u64> = RefCell::new(0);
}

/// Get the next unique epoch
fn next_epoch() -> u64 {
    EPOCH_COUNTER.with(|counter| {
        let mut c = counter.borrow_mut();
        *c += 1;
        *c
    })
}

/// Get the current epoch for a node ID (None if not valid)
fn get_node_epoch(id: u64) -> Option<u64> {
    NODE_EPOCHS.with(|epochs| epochs.borrow().get(&id).copied())
}

/// Register a new node with a fresh epoch, returns the epoch
fn register_node_with_epoch(id: u64) -> u64 {
    let epoch = next_epoch();
    NODE_EPOCHS.with(|epochs| epochs.borrow_mut().insert(id, epoch));
    epoch
}

/// Unregister a node, but only if the epoch matches
fn unregister_node_if_epoch_matches(id: u64, expected_epoch: u64) -> bool {
    NODE_EPOCHS.with(|epochs| {
        let mut map = epochs.borrow_mut();
        if let Some(&current_epoch) = map.get(&id) {
            if current_epoch == expected_epoch {
                map.remove(&id);
                return true;
            }
        }
        false
    })
}

/// Check if a node is valid with the given epoch
fn is_node_valid_with_epoch(id: u64, epoch: u64) -> bool {
    get_node_epoch(id) == Some(epoch)
}

/// Clear all node epochs
fn clear_node_epochs() {
    NODE_EPOCHS.with(|epochs| epochs.borrow_mut().clear());
}

/// Creates a new leaf node with the specified style.
///
/// # Arguments
///
/// * `style` - The style object to apply to the new node.
///
/// # Returns
///
/// The ID of the created node as a `u32`.
///
/// # Errors
///
/// Returns a `JsValue` error if the style cannot be deserialized or if node creation fails.
#[wasm_bindgen]
pub fn new_leaf(style: JsValue) -> Result<u64, JsValue> {
    log_debug!("[taffy] new_leaf: creating new leaf node");
    TAFFY.with(|taffy| {
        let style: Style = serde_wasm_bindgen::from_value(style).map_err(|e| {
            log_error!("[taffy] new_leaf: failed to deserialize style: {:?}", e);
            e
        })?;
        let taffy_style = convert_style(&style);

        let mut taffy_ref = taffy.borrow_mut();
        let node_id = taffy_ref.new_leaf(taffy_style)
            .map_err(|e| {
                log_error!("[taffy] new_leaf: failed to create leaf: {}", e);
                JsValue::from_str(&format!("Failed to create leaf: {}", e))
            })?;

        let id: u64 = node_id.into();
        log_debug!("[taffy] new_leaf: created node with id={}", id);
        Ok(id)
    })
}

/// Creates a new node with children and the specified style.
///
/// # Arguments
///
/// * `style` - The style object to apply to the new node.
/// * `children` - An array of child node IDs (`u32`) to attach to this node.
///
/// # Returns
///
/// The ID of the created node as a `u32`.
///
/// # Errors
///
/// Returns a `JsValue` error if the style cannot be deserialized or if node creation fails.
#[wasm_bindgen]
pub fn new_with_children(style: JsValue, children: &[u64]) -> Result<u64, JsValue> {
    log_debug!("[taffy] new_with_children: creating node with {} children", children.len());
    TAFFY.with(|taffy| {
        let style: Style = serde_wasm_bindgen::from_value(style).map_err(|e| {
            log_error!("[taffy] new_with_children: failed to deserialize style: {:?}", e);
            e
        })?;
        let taffy_style = convert_style(&style);

        let child_ids: Vec<taffy::NodeId> = children.iter()
            .map(|id| taffy::NodeId::from(*id))
            .collect();

        let mut taffy_ref = taffy.borrow_mut();
        let node_id = taffy_ref.new_with_children(taffy_style, &child_ids)
            .map_err(|e| {
                log_error!("[taffy] new_with_children: failed to create node: {}", e);
                JsValue::from_str(&format!("Failed to create node: {}", e))
            })?;

        let id: u64 = node_id.into();
        log_debug!("[taffy] new_with_children: created node id={} with children={:?}", id, children);
        Ok(id)
    })
}

/// Adds a child node to a parent node.
///
/// # Arguments
///
/// * `parent` - The ID of the parent node.
/// * `child` - The ID of the child node to add.
///
/// # Errors
///
/// Returns a `JsValue` error if the operation fails (e.g., recursive hierarchy).
#[wasm_bindgen]
pub fn add_child(parent: u64, child: u64) -> Result<(), JsValue> {
    log_debug!("[taffy] add_child: parent={}, child={}", parent, child);
    TAFFY.with(|taffy| {
        let mut taffy_ref = taffy.borrow_mut();
        let parent_id = taffy::NodeId::from(parent);
        let child_id = taffy::NodeId::from(child);

        taffy_ref.add_child(parent_id, child_id)
            .map_err(|e| {
                log_error!("[taffy] add_child: failed to add child {} to parent {}: {}", child, parent, e);
                JsValue::from_str(&format!("Failed to add child: {}", e))
            })?;
        log_debug!("[taffy] add_child: successfully added child {} to parent {}", child, parent);
        Ok(())
    })
}

/// Removes a child node from a parent node.
///
/// # Arguments
///
/// * `parent` - The ID of the parent node.
/// * `child` - The ID of the child node to remove.
///
/// # Errors
///
/// Returns a `JsValue` error if the child is not found in the parent.
#[wasm_bindgen]
pub fn remove_child(parent: u64, child: u64) -> Result<(), JsValue> {
    log_debug!("[taffy] remove_child: parent={}, child={}", parent, child);
    TAFFY.with(|taffy| {
        let mut taffy_ref = taffy.borrow_mut();
        let parent_id = taffy::NodeId::from(parent);
        let child_id = taffy::NodeId::from(child);

        taffy_ref.remove_child(parent_id, child_id)
            .map_err(|e| {
                log_error!("[taffy] remove_child: failed to remove child {} from parent {}: {}", child, parent, e);
                JsValue::from_str(&format!("Failed to remove child: {}", e))
            })?;
        log_debug!("[taffy] remove_child: successfully removed child {} from parent {}", child, parent);
        Ok(())
    })
}

/// Sets the children of a node, replacing any existing children.
///
/// # Arguments
///
/// * `parent` - The ID of the parent node.
/// * `children` - An array of child node IDs to set.
///
/// # Errors
///
/// Returns a `JsValue` error if the operation fails.
#[wasm_bindgen]
pub fn set_children(parent: u64, children: &[u64]) -> Result<(), JsValue> {
    log_debug!("[taffy] set_children: parent={}, children={:?}", parent, children);
    TAFFY.with(|taffy| {
        let mut taffy_ref = taffy.borrow_mut();
        let parent_id = taffy::NodeId::from(parent);

        let child_ids: Vec<taffy::NodeId> = children.iter()
            .map(|id| taffy::NodeId::from(*id))
            .collect();

        taffy_ref.set_children(parent_id, &child_ids)
            .map_err(|e| {
                log_error!("[taffy] set_children: failed to set children for parent {}: {}", parent, e);
                JsValue::from_str(&format!("Failed to set children: {}", e))
            })?;
        log_debug!("[taffy] set_children: successfully set {} children for parent {}", children.len(), parent);
        Ok(())
    })
}

/// Removes a node from the tree and frees its resources.
///
/// # Arguments
///
/// * `node` - The ID of the node to remove.
///
/// # Errors
///
/// Returns a `JsValue` error if the node does not exist or cannot be removed.
#[wasm_bindgen]
pub fn remove_node(node: u64) -> Result<(), JsValue> {
    log_debug!("[taffy] remove_node: node={}", node);
    TAFFY.with(|taffy| {
        let mut taffy_ref = taffy.borrow_mut();
        let node_id = taffy::NodeId::from(node);

        taffy_ref.remove(node_id)
            .map_err(|e| {
                log_error!("[taffy] remove_node: failed to remove node {}: {}", node, e);
                JsValue::from_str(&format!("Failed to remove node: {}", e))
            })?;
        log_debug!("[taffy] remove_node: successfully removed node {}", node);
        Ok(())
    })
}

/// Retrieves the list of children IDs for a given node.
///
/// # Arguments
///
/// * `parent` - The ID of the parent node.
///
/// # Returns
///
/// A boxed array of child node IDs (`Box<[u32]>`).
///
/// # Errors
///
/// Returns a `JsValue` error if the node does not exist.
#[wasm_bindgen]
pub fn get_children(parent: u64) -> Result<Box<[u64]>, JsValue> {
    TAFFY.with(|taffy| {
        let taffy_ref = taffy.borrow();
        let parent_id = taffy::NodeId::from(parent);

        let children = taffy_ref.children(parent_id)
            .map_err(|e| JsValue::from_str(&format!("Failed to get children: {}", e)))?;

        Ok(children.iter().map(|id: &taffy::NodeId| {
            let val: u64 = (*id).into();
            val
        }).collect())
    })
}

/// Retrieves the parent ID of a given node.
///
/// # Arguments
///
/// * `node` - The ID of the node to query.
///
/// # Returns
///
/// An `Option<u32>` containing the parent ID if it exists, or `None` if the node is a root or orphan.
///
/// # Errors
///
/// Returns a `JsValue` error if internal tree access fails.
#[wasm_bindgen]
pub fn get_parent(node: u64) -> Result<Option<u64>, JsValue> {
    TAFFY.with(|taffy| {
        let taffy_ref = taffy.borrow();
        let node_id = taffy::NodeId::from(node);

        Ok(taffy_ref.parent(node_id).map(|id| {
            let val: u64 = id.into();
            val
        }))
    })
}

/// Updates the style of an existing node.
///
/// # Arguments
///
/// * `node` - The ID of the node to update.
/// * `style` - The new style object to apply.
///
/// # Errors
///
/// Returns a `JsValue` error if the style cannot be deserialized or if the node does not exist.
#[wasm_bindgen]
pub fn set_style(node: u64, style: JsValue) -> Result<(), JsValue> {
    log_debug!("[taffy] set_style: node={}", node);
    TAFFY.with(|taffy| {
        let style: Style = serde_wasm_bindgen::from_value(style).map_err(|e| {
            log_error!("[taffy] set_style: failed to deserialize style for node {}: {:?}", node, e);
            e
        })?;
        let taffy_style = convert_style(&style);

        let mut taffy_ref = taffy.borrow_mut();
        let node_id = taffy::NodeId::from(node);

        taffy_ref.set_style(node_id, taffy_style)
            .map_err(|e| {
                log_error!("[taffy] set_style: failed to set style for node {}: {}", node, e);
                JsValue::from_str(&format!("Failed to set style: {}", e))
            })?;
        log_debug!("[taffy] set_style: successfully set style for node {}", node);
        Ok(())
    })
}

/// Computes the layout for a tree starting from the specified root node.
///
/// # Arguments
///
/// * `root` - The ID of the root node of the tree to lay out.
/// * `available_space` - The available space constraints for the layout.
///
/// # Errors
///
/// Returns a `JsValue` error if the layout computation fails.
#[wasm_bindgen]
pub fn compute_layout(root: u64, available_space: JsValue) -> Result<(), JsValue> {
    log_debug!("[taffy] compute_layout: root={}", root);
    TAFFY.with(|taffy| {
        let taffy_space = if available_space.is_undefined() || available_space.is_null() {
            log_debug!("[taffy] compute_layout: using MaxContent for available space");
            taffy::Size {
                width: taffy::style::AvailableSpace::MaxContent,
                height: taffy::style::AvailableSpace::MaxContent,
            }
        } else {
            let space: AvailableSpace = serde_wasm_bindgen::from_value(available_space).map_err(|e| {
                log_error!("[taffy] compute_layout: failed to deserialize available_space: {:?}", e);
                e
            })?;
            log_debug!("[taffy] compute_layout: available_space=({:?}, {:?})", space.width, space.height);
            to_available_space(&space)
        };

        let mut taffy_ref = taffy.try_borrow_mut()
            .map_err(|_| {
                log_error!("[taffy] compute_layout: failed to borrow TAFFY mutably - possible reentrant call");
                JsValue::from_str("Failed to borrow TAFFY mutably (compute_layout)")
            })?;
        let root_id = taffy::NodeId::from(root);

        taffy_ref.compute_layout_with_measure(
            root_id, 
            taffy_space,
            |_known_dimensions, available_space, _node_id, node_context, _style| -> TaffySize<f32> {
                if let Some(ctx) = node_context {
                    if let Some(func) = &ctx.measure_func {
                         let width = match available_space.width {
                             taffy::style::AvailableSpace::Definite(w) => w,
                             taffy::style::AvailableSpace::MinContent => 0.0,
                             taffy::style::AvailableSpace::MaxContent => f32::MAX,
                         };

                         let this = JsValue::NULL;
                         let width_val = JsValue::from_f64(width as f64);
                         
                         // We need to catch panics/errors from JS
                         match func.call1(&this, &width_val) {
                             Ok(res) => {
                                 if let Ok(size) = serde_wasm_bindgen::from_value::<Size>(res) {
                                      return TaffySize { width: size.width, height: size.height };
                                 }
                             },
                             Err(e) => {
                                 log_warn!("[taffy] compute_layout: measure function call failed: {:?}", e);
                             },
                         }
                    }
                }
                 TaffySize::ZERO
            }
        )
            .map_err(|e| {
                log_error!("[taffy] compute_layout: layout computation failed: {}", e);
                JsValue::from_str(&format!("Failed to compute layout: {}", e))
            })?;
        log_info!("[taffy] compute_layout: successfully computed layout for root={}", root);
        Ok(())
    })
}

/// Retrieves the computed layout information for a specific node.
///
/// # Arguments
///
/// * `node` - The ID of the node to query.
///
/// # Returns
///
/// A `Layout` object containing the x, y, width, and height of the node.
///
/// # Errors
///
/// Returns a `JsValue` error if the node does not exist or layout information is unavailable.
#[wasm_bindgen]
pub fn get_layout(node: u64) -> Result<JsValue, JsValue> {
    TAFFY.with(|taffy| {
        let taffy_ref = taffy.borrow();
        let node_id = taffy::NodeId::from(node);

        let layout = taffy_ref.layout(node_id)
            .map_err(|e| JsValue::from_str(&format!("Failed to get layout: {}", e)))?;

        let result = Layout::from(*layout);
        serde_wasm_bindgen::to_value(&result)
            .map_err(|e| JsValue::from_str(&format!("Failed to serialize layout: {}", e)))
    })
}

/// Marks a node and its ancestors as dirty, requiring a layout re-computation.
///
/// # Arguments
///
/// * `node` - The ID of the node to mark dirty.
///
/// # Errors
///
/// Returns a `JsValue` error if the node does not exist.
#[wasm_bindgen]
pub fn mark_dirty(node: u64) -> Result<(), JsValue> {
    TAFFY.with(|taffy| {
        let mut taffy_ref = taffy.borrow_mut();
        let node_id = taffy::NodeId::from(node);

        taffy_ref.mark_dirty(node_id)
            .map_err(|e| JsValue::from_str(&format!("Failed to mark dirty: {}", e)))?;
        Ok(())
    })
}

/// Clear all nodes from the layout tree.
///
/// This removes all nodes and resets the tree to an empty state.
/// Any existing node IDs become invalid after this call.
#[wasm_bindgen]
pub fn clear() -> Result<(), JsValue> {
    log_info!("[taffy] clear: clearing all nodes");
    clear_node_epochs();
    TAFFY.with(|taffy| {
        #[allow(unused_variables)]
        let count = taffy.borrow().total_node_count();
        taffy.borrow_mut().clear();
        log_debug!("[taffy] clear: removed {} nodes", count);
        Ok(())
    })
}

/// Get the total number of nodes currently in the layout tree.
///
/// # Returns
///
/// The total count of all nodes in the tree.
#[wasm_bindgen]
pub fn node_count() -> u32 {
    let count = TAFFY.with(|taffy| {
        taffy.borrow().total_node_count() as u32
    });
    log_debug!("[taffy] node_count: {}", count);
    count
}

/// Helper function to create a dimension
#[wasm_bindgen]
pub fn dimension(value: f32, unit: DimensionUnit) -> Dimension {
    Dimension { value, unit }
}

/// Helper function to create a pixel dimension
#[wasm_bindgen]
pub fn px(value: f32) -> Dimension {
    Dimension { value, unit: DimensionUnit::Pixels }
}

/// Helper function to create a percent dimension
#[wasm_bindgen]
pub fn percent(value: f32) -> Dimension {
    Dimension { value, unit: DimensionUnit::Percent }
}

/// Auto dimension constant
#[wasm_bindgen]
pub fn auto() -> Dimension {
    Dimension { value: 0.0, unit: DimensionUnit::Auto }
}

// ============================================================================
// Utility functions
// ============================================================================

/// Initialize the WASM module.
///
/// This function is automatically called when the WASM module is loaded.
/// It sets up the console error panic hook for better error messages in development.
#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    
    log_info!("[taffy] Taffy WASM module initialized");
}

// ============================================================================
// OO Wrapper (Yoga-like API)
// ============================================================================

#[wasm_bindgen]
pub struct TaffyNode {
    pub id: u64,
    // Internal epoch for validating node ownership - not exposed to JS
    epoch: u64,
}

impl Drop for TaffyNode {
    fn drop(&mut self) {
        // Best effort cleanup. We must not panic in Drop.
        // Check if our epoch matches the current epoch for this node ID.
        // If not, the node was already removed and recreated, so we shouldn't remove it.
        if !is_node_valid_with_epoch(self.id, self.epoch) {
            return;
        }
        
        TAFFY.with(|taffy| {
            if let Ok(mut taffy_ref) = taffy.try_borrow_mut() {
                let node_id = taffy::NodeId::from(self.id);
                // Ignore errors - node may have already been removed
                let _ = taffy_ref.remove(node_id);
                // Only unregister if our epoch still matches
                unregister_node_if_epoch_matches(self.id, self.epoch);
            }
            // If we can't borrow, we just leak the node. This is better than panicking.
        });
    }
}

#[wasm_bindgen]
impl TaffyNode {
    #[wasm_bindgen(constructor)]
    pub fn new(style: JsValue) -> Result<TaffyNode, JsValue> {
        let id = new_leaf(style)?;
        // Register this node with a unique epoch
        let epoch = register_node_with_epoch(id);
        Ok(TaffyNode { id, epoch })
    }

    pub fn free(self) -> Result<(), JsValue> {
        // Only remove if our epoch is still valid
        if is_node_valid_with_epoch(self.id, self.epoch) {
            unregister_node_if_epoch_matches(self.id, self.epoch);
            let result = TAFFY.with(|taffy| {
                let mut taffy_ref = taffy.borrow_mut();
                let node_id = taffy::NodeId::from(self.id);
                taffy_ref.remove(node_id)
                    .map_err(|e| JsValue::from_str(&format!("Failed to remove node: {}", e)))
            });
            // Prevent Drop from running after we've already removed the node
            std::mem::forget(self);
            result?;
        } else {
            std::mem::forget(self);
        }
        Ok(())
    }

    pub fn set_style(&self, style: JsValue) -> Result<(), JsValue> {
        set_style(self.id, style)
    }
    
    pub fn style(&self) -> Result<JsValue, JsValue> {
        // Taffy doesn't easily expose getting the style back in the same format yet without keeping a copy.
        // For now, this might not be supported or we could implement reading from Taffy.
        // Taffy's style() returns taffy::Style, we would need to reverse convert it to our Style struct.
        Err(JsValue::from_str("getting style is not yet supported"))
    }
    
    #[wasm_bindgen(js_name = computeLayout)]
    pub fn compute_layout(&self, available_space: JsValue) -> Result<(), JsValue> {
        compute_layout(self.id, available_space)
    }
    
    /* dispose removed in favor of Drop */

    #[wasm_bindgen(js_name = getLayout)]
    pub fn get_layout(&self) -> Result<Layout, JsValue> {
        TAFFY.with(|taffy| {
             let taffy_ref = taffy.borrow();
             let node_id = taffy::NodeId::from(self.id);
             let layout = taffy_ref.layout(node_id)
                 .map_err(|e| JsValue::from_str(&format!("Failed to get layout: {}", e)))?;
             Ok(Layout::from(*layout))
        })
    }
    
    #[wasm_bindgen(js_name = markDirty)]
    pub fn mark_dirty(&self) -> Result<(), JsValue> {
        TAFFY.with(|taffy| {
            let mut taffy_ref = taffy.borrow_mut();
            let node_id = taffy::NodeId::from(self.id);
            taffy_ref.mark_dirty(node_id)
                .map_err(|e| JsValue::from_str(&format!("Failed to mark dirty: {}", e)))?;
            Ok(())
        })
    }

    #[wasm_bindgen(js_name = addChild)]
    pub fn add_child(&self, child: &TaffyNode) -> Result<(), JsValue> {
        add_child(self.id, child.id)
    }

    #[wasm_bindgen(js_name = insertChild)]
    pub fn insert_child(&self, child: &TaffyNode, index: usize) -> Result<(), JsValue> {
         TAFFY.with(|taffy| {
            let mut taffy_ref = taffy.borrow_mut();
            let parent_id = taffy::NodeId::from(self.id);
            let child_id = taffy::NodeId::from(child.id);

            taffy_ref.insert_child_at_index(parent_id, index, child_id)
                .map_err(|e| JsValue::from_str(&format!("Failed to insert child: {}", e)))?;
            Ok(())
        })
    }
    
    #[wasm_bindgen(js_name = removeChild)]
    pub fn remove_child(&self, child: &TaffyNode) -> Result<(), JsValue> {
        remove_child(self.id, child.id)
    }
    
    #[wasm_bindgen(js_name = setChildren)]
    pub fn set_children(&self, children: &[u64]) -> Result<(), JsValue> {
        // Note: this takes raw IDs because handling array of TaffyNode objects from JS is tricky in wasm-bindgen
        set_children(self.id, children)
    }

    #[wasm_bindgen(js_name = setDisplay)]
    pub fn set_display(&self, display: Display) -> Result<(), JsValue> {
        self.update_style(|s| {
            s.display = match display {
                Display::None => taffy::style::Display::None,
                Display::Flex => taffy::style::Display::Flex,
                Display::Grid => taffy::style::Display::Grid,
                Display::Block => taffy::style::Display::Block,
            };
        })
    }

    #[wasm_bindgen(js_name = setPositionType)]
    pub fn set_position_type(&self, position: Position) -> Result<(), JsValue> {
         self.update_style(|s| {
            s.position = match position {
                Position::Static => taffy::style::Position::Relative, // Taffy treats Static as Relative usually
                Position::Relative => taffy::style::Position::Relative,
                Position::Absolute => taffy::style::Position::Absolute,
            };
        })
    }

    #[wasm_bindgen(js_name = setFlexDirection)]
    pub fn set_flex_direction(&self, direction: FlexDirection) -> Result<(), JsValue> {
        self.update_style(|s| {
            s.flex_direction = match direction {
                FlexDirection::Row => taffy::style::FlexDirection::Row,
                FlexDirection::Column => taffy::style::FlexDirection::Column,
                FlexDirection::RowReverse => taffy::style::FlexDirection::RowReverse,
                FlexDirection::ColumnReverse => taffy::style::FlexDirection::ColumnReverse,
            };
        })
    }

    #[wasm_bindgen(js_name = setFlexWrap)]
    pub fn set_flex_wrap(&self, wrap: FlexWrap) -> Result<(), JsValue> {
        self.update_style(|s| {
            s.flex_wrap = match wrap {
                FlexWrap::NoWrap => taffy::style::FlexWrap::NoWrap,
                FlexWrap::Wrap => taffy::style::FlexWrap::Wrap,
                FlexWrap::WrapReverse => taffy::style::FlexWrap::WrapReverse,
            };
        })
    }

    #[wasm_bindgen(js_name = setAlignItems)]
    pub fn set_align_items(&self, align: AlignItems) -> Result<(), JsValue> {
        self.update_style(|s| {
            s.align_items = Some(match align {
                AlignItems::Start => taffy::style::AlignItems::Start,
                AlignItems::End => taffy::style::AlignItems::End,
                AlignItems::FlexStart => taffy::style::AlignItems::FlexStart,
                AlignItems::FlexEnd => taffy::style::AlignItems::FlexEnd,
                AlignItems::Center => taffy::style::AlignItems::Center,
                AlignItems::Baseline => taffy::style::AlignItems::Baseline,
                AlignItems::Stretch => taffy::style::AlignItems::Stretch,
            });
        })
    }

    #[wasm_bindgen(js_name = setAlignSelf)]
    pub fn set_align_self(&self, align: AlignSelf) -> Result<(), JsValue> {
        self.update_style(|s| {
             s.align_self = match align {
                AlignSelf::Auto => None,
                AlignSelf::Start => Some(taffy::style::AlignItems::Start),
                AlignSelf::End => Some(taffy::style::AlignItems::End),
                AlignSelf::FlexStart => Some(taffy::style::AlignItems::FlexStart),
                AlignSelf::FlexEnd => Some(taffy::style::AlignItems::FlexEnd),
                AlignSelf::Center => Some(taffy::style::AlignItems::Center),
                AlignSelf::Baseline => Some(taffy::style::AlignItems::Baseline),
                AlignSelf::Stretch => Some(taffy::style::AlignItems::Stretch),
            };
        })
    }

    #[wasm_bindgen(js_name = setJustifyContent)]
    pub fn set_justify_content(&self, justify: JustifyContent) -> Result<(), JsValue> {
        self.update_style(|s| {
            s.justify_content = Some(match justify {
                JustifyContent::Start => taffy::style::JustifyContent::Start,
                JustifyContent::End => taffy::style::JustifyContent::End,
                JustifyContent::FlexStart => taffy::style::JustifyContent::FlexStart,
                JustifyContent::FlexEnd => taffy::style::JustifyContent::FlexEnd,
                JustifyContent::Center => taffy::style::JustifyContent::Center,
                JustifyContent::SpaceBetween => taffy::style::JustifyContent::SpaceBetween,
                JustifyContent::SpaceAround => taffy::style::JustifyContent::SpaceAround,
                JustifyContent::SpaceEvenly => taffy::style::JustifyContent::SpaceEvenly,
            });
        })
    }

    #[wasm_bindgen(js_name = setFlexGrow)]
    pub fn set_flex_grow(&self, value: f32) -> Result<(), JsValue> {
        self.update_style(|s| s.flex_grow = value)
    }

    #[wasm_bindgen(js_name = setFlexShrink)]
    pub fn set_flex_shrink(&self, value: f32) -> Result<(), JsValue> {
        self.update_style(|s| s.flex_shrink = value)
    }

    #[wasm_bindgen(js_name = setFlexBasis)]
    pub fn set_flex_basis(&self, value: f32) -> Result<(), JsValue> {
        self.update_style(|s| s.flex_basis = taffy::style::Dimension::length(value))
    }

    #[wasm_bindgen(js_name = setFlexBasisPercent)]
    pub fn set_flex_basis_percent(&self, value: f32) -> Result<(), JsValue> {
        self.update_style(|s| s.flex_basis = taffy::style::Dimension::percent(value / 100.0))
    }
    
    #[wasm_bindgen(js_name = setFlexBasisAuto)]
    pub fn set_flex_basis_auto(&self) -> Result<(), JsValue> {
        self.update_style(|s| s.flex_basis = taffy::style::Dimension::auto())
    }

    // Width
    #[wasm_bindgen(js_name = setWidth)]
    pub fn set_width(&self, value: f32) -> Result<(), JsValue> {
        self.update_style(|s| s.size.width = taffy::style::Dimension::length(value))
    }
    #[wasm_bindgen(js_name = setWidthPercent)]
    pub fn set_width_percent(&self, value: f32) -> Result<(), JsValue> {
         self.update_style(|s| s.size.width = taffy::style::Dimension::percent(value / 100.0))
    }
    #[wasm_bindgen(js_name = setWidthAuto)]
    pub fn set_width_auto(&self) -> Result<(), JsValue> {
         self.update_style(|s| s.size.width = taffy::style::Dimension::auto())
    }

    // Height
    #[wasm_bindgen(js_name = setHeight)]
    pub fn set_height(&self, value: f32) -> Result<(), JsValue> {
        self.update_style(|s| s.size.height = taffy::style::Dimension::length(value))
    }
    #[wasm_bindgen(js_name = setHeightPercent)]
    pub fn set_height_percent(&self, value: f32) -> Result<(), JsValue> {
         self.update_style(|s| s.size.height = taffy::style::Dimension::percent(value / 100.0))
    }
    #[wasm_bindgen(js_name = setHeightAuto)]
    pub fn set_height_auto(&self) -> Result<(), JsValue> {
         self.update_style(|s| s.size.height = taffy::style::Dimension::auto())
    }

    // Min Width
    #[wasm_bindgen(js_name = setMinWidth)]
    pub fn set_min_width(&self, value: f32) -> Result<(), JsValue> {
        self.update_style(|s| s.min_size.width = taffy::style::Dimension::length(value))
    }
    #[wasm_bindgen(js_name = setMinWidthPercent)]
    pub fn set_min_width_percent(&self, value: f32) -> Result<(), JsValue> {
         self.update_style(|s| s.min_size.width = taffy::style::Dimension::percent(value / 100.0))
    }

    // Min Height
    #[wasm_bindgen(js_name = setMinHeight)]
    pub fn set_min_height(&self, value: f32) -> Result<(), JsValue> {
        self.update_style(|s| s.min_size.height = taffy::style::Dimension::length(value))
    }
    #[wasm_bindgen(js_name = setMinHeightPercent)]
    pub fn set_min_height_percent(&self, value: f32) -> Result<(), JsValue> {
         self.update_style(|s| s.min_size.height = taffy::style::Dimension::percent(value / 100.0))
    }
    
    // Margin
    #[wasm_bindgen(js_name = setMargin)]
    pub fn set_margin(&self, edge: Edge, value: f32) -> Result<(), JsValue> {
        self.update_style(|s| {
            let val = taffy::style::LengthPercentageAuto::length(value);
            match edge {
                 Edge::Left => s.margin.left = val,
                 Edge::Right => s.margin.right = val,
                 Edge::Top => s.margin.top = val,
                 Edge::Bottom => s.margin.bottom = val,
                 Edge::Start => s.margin.left = val, 
                 Edge::End => s.margin.right = val,
                 Edge::Horizontal => { s.margin.left = val; s.margin.right = val; },
                 Edge::Vertical => { s.margin.top = val; s.margin.bottom = val; },
                 Edge::All => { s.margin.left = val; s.margin.right = val; s.margin.top = val; s.margin.bottom = val; },
            }
        })
    }
    
    #[wasm_bindgen(js_name = setMarginAuto)]
    pub fn set_margin_auto(&self, edge: Edge) -> Result<(), JsValue> {
         self.update_style(|s| {
            let val = taffy::style::LengthPercentageAuto::auto();
            match edge {
                 Edge::Left => s.margin.left = val,
                 Edge::Right => s.margin.right = val,
                 Edge::Top => s.margin.top = val,
                 Edge::Bottom => s.margin.bottom = val,
                 Edge::Start => s.margin.left = val,
                 Edge::End => s.margin.right = val,
                 Edge::Horizontal => { s.margin.left = val; s.margin.right = val; },
                 Edge::Vertical => { s.margin.top = val; s.margin.bottom = val; },
                 Edge::All => { s.margin = taffy::geometry::Rect { left: val, right: val, top: val, bottom: val }; },
            }
        })
    }
    
    // Padding
    #[wasm_bindgen(js_name = setPadding)]
    pub fn set_padding(&self, edge: Edge, value: f32) -> Result<(), JsValue> {
        self.update_style(|s| {
            let val = taffy::style::LengthPercentage::length(value);
             match edge {
                 Edge::Left => s.padding.left = val,
                 Edge::Right => s.padding.right = val,
                 Edge::Top => s.padding.top = val,
                 Edge::Bottom => s.padding.bottom = val,
                 Edge::Start => s.padding.left = val,
                 Edge::End => s.padding.right = val,
                 Edge::Horizontal => { s.padding.left = val; s.padding.right = val; },
                 Edge::Vertical => { s.padding.top = val; s.padding.bottom = val; },
                 Edge::All => { s.padding.left = val; s.padding.right = val; s.padding.top = val; s.padding.bottom = val; },
            }
        })
    }
    
    // Border
    #[wasm_bindgen(js_name = setBorder)]
    pub fn set_border(&self, edge: Edge, value: f32) -> Result<(), JsValue> {
        self.update_style(|s| {
            let val = taffy::style::LengthPercentage::length(value);
             match edge {
                 Edge::Left => s.border.left = val,
                 Edge::Right => s.border.right = val,
                 Edge::Top => s.border.top = val,
                 Edge::Bottom => s.border.bottom = val,
                 Edge::Start => s.border.left = val,
                 Edge::End => s.border.right = val,
                 Edge::Horizontal => { s.border.left = val; s.border.right = val; },
                 Edge::Vertical => { s.border.top = val; s.border.bottom = val; },
                 Edge::All => { s.border.left = val; s.border.right = val; s.border.top = val; s.border.bottom = val; },
            }
        })
    }
    
    // Gap
    #[wasm_bindgen(js_name = setGap)]
    pub fn set_gap(&self, gutter: Gutter, value: f32) -> Result<(), JsValue> {
        self.update_style(|s| {
            let val = taffy::style::LengthPercentage::length(value);
            match gutter {
                Gutter::Column => s.gap.width = val,
                Gutter::Row => s.gap.height = val,
                Gutter::All => { s.gap.width = val; s.gap.height = val; },
            }
        })
    }

    #[wasm_bindgen(js_name = setMeasureFunc)]
    pub fn set_measure_func(&self, js_func: js_sys::Function) -> Result<(), JsValue> {
        TAFFY.with(|taffy| {
            let mut taffy_ref = taffy.borrow_mut();
            let node_id = taffy::NodeId::from(self.id);
            
            // Set context with function
            // We use set_node_context which overwrites. If we had other data we'd need to be careful.
            // But since NodeContext only has measure_func, this is fine.
            let context = NodeContext { measure_func: Some(js_func) };
            
            // Taffy 0.9 way to set context. 
            // set_node_context returns Result in some versions, check compatibility.
            // Assuming it aligns with getting it.
            let _ = taffy_ref.set_node_context(node_id, Some(context));
            
            taffy_ref.mark_dirty(node_id)
                 .map_err(|e| JsValue::from_str(&format!("Failed to set measure func: {}", e)))?;
            Ok(())
        })
    }

    // Helper
    fn update_style<F>(&self, f: F) -> Result<(), JsValue> 
    where F: FnOnce(&mut taffy::style::Style) {
        // Check if node is valid with our epoch BEFORE trying to borrow TAFFY
        if !is_node_valid_with_epoch(self.id, self.epoch) {
            return Err(JsValue::from_str("Node does not exist (already removed or recreated)"));
        }
        
        TAFFY.with(|taffy| {
            let mut taffy_ref = taffy.try_borrow_mut()
                .map_err(|_| JsValue::from_str("Failed to borrow TAFFY mutably (update_style)"))?;
            let node_id = taffy::NodeId::from(self.id);
            
            // Node should be valid at this point since we checked above
            let mut style = taffy_ref.style(node_id)
                .map_err(|e| JsValue::from_str(&format!("Node not found: {}", e)))?.clone();
            
            f(&mut style);
            
            taffy_ref.set_style(node_id, style).map_err(|e| JsValue::from_str(&format!("Failed to set style: {}", e)))?;
            Ok(())
        })
    }
}
