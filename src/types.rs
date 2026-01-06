//! # Data Transfer Objects and TypeScript Types Module
//!
//! This module defines the Data Transfer Objects (DTOs) used for serializing data between
//! Rust and JavaScript, as well as the TypeScript type declarations included in the
//! generated `.d.ts` file.
//!
//! ## Overview
//!
//! Because wasm-bindgen has limited support for complex generic types, this module provides
//! DTOs that mirror Taffy's internal types with simpler structures that can be efficiently
//! serialized using `serde-wasm-bindgen`.
//!
//! ## DTOs Provided
//!
//! | DTO | Taffy Equivalent | Description |
//! |-----|------------------|-------------|
//! | [`DimensionDto`] | `Dimension` | Length, percentage, or auto |
//! | [`LengthPercentageDto`] | `LengthPercentage` | Length or percentage |
//! | [`LengthPercentageAutoDto`] | `LengthPercentageAuto` | Length, percentage, or auto |
//! | [`SizeDto<T>`] | `Size<T>` | Width and height pair |
//! | [`RectDto<T>`] | `Rect<T>` | Left, right, top, bottom quad |
//! | [`AvailableSizeDto`] | `Size<AvailableSpace>` | Layout constraints |
//! | [`AvailableSpaceDto`] | `AvailableSpace` | Single dimension constraint |
//!
//! ## TypeScript Declarations
//!
//! The `typescript_custom_section` in this module adds type definitions that appear
//! in the generated `taffy_js.d.ts` file, providing accurate types for:
//!
//! - `AvailableSpace`, `Size<T>`, `Rect<T>`, `Point<T>`
//! - `Dimension`, `LengthPercentage`, `LengthPercentageAuto`
//! - `MeasureFunction` callback signature
//! - Detailed grid layout info types

use serde::{Deserialize, Serialize};
use taffy::geometry::{Rect, Size};
use taffy::style::{AvailableSpace, CompactLength, Dimension, LengthPercentage, LengthPercentageAuto};
use wasm_bindgen::prelude::*;

// =============================================================================
// External Type Declarations for TypeScript
// =============================================================================

#[wasm_bindgen]
extern "C" {
    /// Available space argument type for layout computation
    ///
    /// Used when calling `computeLayout()` or `computeLayoutWithMeasure()`.
    ///
    /// # TypeScript
    ///
    /// ```typescript
    /// type AvailableSpace = { Definite: number } | "MinContent" | "MaxContent";
    /// interface Size<T> { width: T; height: T; }
    /// ```
    #[wasm_bindgen(typescript_type = "Size<AvailableSpace>")]
    pub type JsAvailableSizeArg;

    /// Measure function callback type
    ///
    /// Used with `computeLayoutWithMeasure()` for custom content measurement.
    #[wasm_bindgen(typescript_type = "MeasureFunction")]
    pub type JsMeasureFunctionArg;

    /// Overflow point type (x and y overflow settings)
    #[wasm_bindgen(typescript_type = "Point<Overflow>")]
    pub type JsPointOverflow;

    /// Single dimension type (Length, Percent, or Auto)
    #[wasm_bindgen(typescript_type = "Dimension")]
    pub type JsDimension;

    /// Size with dimension type (width and height)
    #[wasm_bindgen(typescript_type = "Size<Dimension>")]
    pub type JsSizeDimension;

    /// Rectangle with auto-supporting length/percentage values
    #[wasm_bindgen(typescript_type = "Rect<LengthPercentageAuto>")]
    pub type JsRectLengthPercentageAuto;

    /// Rectangle with length/percentage values (no auto)
    #[wasm_bindgen(typescript_type = "Rect<LengthPercentage>")]
    pub type JsRectLengthPercentage;

    /// Size with length/percentage values
    #[wasm_bindgen(typescript_type = "Size<LengthPercentage>")]
    pub type JsSizeLengthPercentage;

    // =========================================================================
    // Optional Enum Types (for consistent getter/setter signatures)
    // =========================================================================

    /// Optional AlignItems type for setters
    #[wasm_bindgen(typescript_type = "AlignItems | undefined")]
    pub type OneOptAlignItems;

    /// Optional AlignSelf type for setters
    #[wasm_bindgen(typescript_type = "AlignSelf | undefined")]
    pub type OneOptAlignSelf;

    /// Optional AlignContent type for setters
    #[wasm_bindgen(typescript_type = "AlignContent | undefined")]
    pub type OneOptAlignContent;

    /// Optional JustifyContent type for setters
    #[wasm_bindgen(typescript_type = "JustifyContent | undefined")]
    pub type OneOptJustifyContent;

    /// Optional number type for setters
    #[wasm_bindgen(typescript_type = "number | undefined")]
    pub type OnOptNumber;
}

// =============================================================================
// TypeScript Custom Type Declarations
// =============================================================================

/// Additional TypeScript type declarations appended to the generated `.d.ts` file
///
/// These types provide accurate TypeScript definitions for complex types that
/// wasm-bindgen cannot automatically generate.
#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
/**
 * Available space constraint for layout computation.
 *
 * Specifies how much space is available for a node during layout calculation.
 * This is passed to `computeLayout()` to define the container constraints.
 *
 * @remarks
 * - Use `{ Definite: number }` when you have a fixed container size
 * - Use `"MinContent"` to shrink-wrap to the minimum content size
 * - Use `"MaxContent"` to expand to fit all content without wrapping
 *
 *
 * <details>
 * <summary><strong>TypeScript Example</strong></summary>
 *
 * ```typescript
 * import init, { TaffyTree, Style, type AvailableSpace, type Size } from 'taffy-js';
 *
 * await init();
 * const tree = new TaffyTree();
 * const root: bigint = tree.newLeaf(new Style());
 *
 * // Fixed size container with type annotation
 * const fixedSpace: Size<AvailableSpace> = {
 *   width: { Definite: 800 },
 *   height: { Definite: 600 }
 * };
 * tree.computeLayout(root, fixedSpace);
 *
 * // Flexible width, fixed height
 * const flexibleSpace: Size<AvailableSpace> = {
 *   width: "MaxContent",
 *   height: { Definite: 400 }
 * };
 * tree.computeLayout(root, flexibleSpace);
 * ```
 *
 * </details>
 */
export type AvailableSpace = { Definite: number } | "MinContent" | "MaxContent";

/**
 * Generic size type with width and height.
 *
 * A two-dimensional container for width and height values. The type parameter `T`
 * determines what kind of values are stored.
 *
 * @typeParam T - The type of each dimension (e.g., `number`, `Dimension`, `AvailableSpace`)
 *
 * @property width - The horizontal dimension value
 * @property height - The vertical dimension value
 *
 *
 * <details>
 * <summary><strong>TypeScript Example</strong></summary>
 *
 * ```typescript
 * import type { Size, Dimension, AvailableSpace } from 'taffy-js';
 *
 * // Size with explicit type parameters
 * const pixelSize: Size<number> = { width: 200, height: 100 };
 *
 * const dimensionSize: Size<Dimension> = {
 *   width: { Length: 200 },
 *   height: { Percent: 50 }
 * };
 *
 * const availableSize: Size<AvailableSpace> = {
 *   width: { Definite: 800 },
 *   height: "MaxContent"
 * };
 * ```
 *
 * </details>
 */
export interface Size<T> {
  /** The horizontal dimension value */
  width: T;
  /** The vertical dimension value */
  height: T;
}

/**
 * Custom measure function for leaf nodes with text or other dynamic content.
 *
 * This callback is invoked during layout computation for leaf nodes that need
 * custom sizing based on their content (e.g., text nodes that need text measurement).
 *
 * @param knownDimensions - Dimensions already determined by constraints. Each dimension
 *                          is `number` if known, or `null` if needs to be measured.
 * @param availableSpace - The available space constraints for the node. Can be definite
 *                         pixels, MinContent, or MaxContent.
 * @param node - The node ID (`bigint`) of the node being measured
 * @param context - User-provided context attached to the node via `newLeafWithContext()`
 * @param style - The node's current Style configuration
 * @returns The measured size of the content in pixels
 *
 *
 * <details>
 * <summary><strong>TypeScript Example</strong></summary>
 *
 * ```typescript
 * import init, { TaffyTree, Style, type MeasureFunction, type Size } from 'taffy-js';
 *
 * interface TextContext {
 *   text: string;
 *   fontSize: number;
 * }
 *
 * await init();
 * const tree = new TaffyTree();
 *
 * const style = new Style();
 * const context: TextContext = { text: "Hello, World!", fontSize: 16 };
 * const textNode: bigint = tree.newLeafWithContext(style, context);
 *
 * // Typed measure function
 * const measureText: MeasureFunction = (
 *   knownDimensions,
 *   availableSpace,
 *   node,
 *   context,
 *   style
 * ): Size<number> => {
 *   const ctx = context as TextContext | undefined;
 *   if (!ctx?.text) return { width: 0, height: 0 };
 *
 *   const width = knownDimensions.width ?? measureTextWidth(ctx.text, ctx.fontSize);
 *   const height = knownDimensions.height ?? ctx.fontSize * 1.2;
 *
 *   return { width, height };
 * };
 *
 * tree.computeLayoutWithMeasure(
 *   textNode,
 *   { width: { Definite: 200 }, height: "MaxContent" },
 *   measureText
 * );
 * ```
 *
 * </details>
 */
export type MeasureFunction = (
  knownDimensions: Size<number | null>,
  availableSpace: Size<AvailableSpace>,
  node: bigint,
  context: any,
  style: Style,
) => Size<number>;

/**
 * Dimension type supporting length, percentage, or auto values.
 *
 * Used for sizing properties like `width`, `height`, `flexBasis`, etc.
 *
 * @remarks
 * - `{ Length: number }`: Fixed size in pixels
 * - `{ Percent: number }`: Percentage of parent's size (0-100)
 * - `"Auto"`: Size determined by content or layout algorithm
 *
 *
 * <details>
 * <summary><strong>TypeScript Example</strong></summary>
 *
 * ```typescript
 * import { Style, type Dimension, type Size } from 'taffy-js';
 *
 * const style = new Style();
 *
 * // With explicit type annotations
 * const fixedSize: Size<Dimension> = {
 *   width: { Length: 200 },
 *   height: { Length: 100 }
 * };
 *
 * const percentSize: Size<Dimension> = {
 *   width: { Percent: 50 },
 *   height: { Percent: 100 }
 * };
 *
 * const autoSize: Size<Dimension> = {
 *   width: "Auto",
 *   height: "Auto"
 * };
 *
 * style.size = fixedSize;
 * ```
 *
 * </details>
 */
export type Dimension = { Length: number } | { Percent: number } | "Auto";

/**
 * Length or percentage value (no auto support).
 *
 * Used for properties that require explicit values, such as `padding`, `border`, and `gap`.
 *
 * @remarks
 * - `{ Length: number }`: Fixed size in pixels
 * - `{ Percent: number }`: Percentage of parent's size (0-100)
 *
 *
 * <details>
 * <summary><strong>TypeScript Example</strong></summary>
 *
 * ```typescript
 * import { Style, type LengthPercentage, type Rect, type Size } from 'taffy-js';
 *
 * const style = new Style();
 *
 * const padding: Rect<LengthPercentage> = {
 *   left: { Length: 10 },
 *   right: { Length: 10 },
 *   top: { Length: 5 },
 *   bottom: { Length: 5 }
 * };
 *
 * const gap: Size<LengthPercentage> = {
 *   width: { Percent: 5 },
 *   height: { Percent: 5 }
 * };
 *
 * style.padding = padding;
 * style.gap = gap;
 * ```
 *
 * </details>
 */
export type LengthPercentage = { Length: number } | { Percent: number };

/**
 * Length, percentage, or auto value.
 *
 * Used for properties that support auto values, such as `margin` and `inset`.
 *
 * @remarks
 * - `{ Length: number }`: Fixed size in pixels
 * - `{ Percent: number }`: Percentage of parent's size (0-100)
 * - `"Auto"`: Automatic value (behavior depends on property)
 *
 *
 * <details>
 * <summary><strong>TypeScript Example</strong></summary>
 *
 * ```typescript
 * import { Style, type LengthPercentageAuto, type Rect } from 'taffy-js';
 *
 * const style = new Style();
 *
 * // Auto margins for horizontal centering
 * const centerMargin: Rect<LengthPercentageAuto> = {
 *   left: "Auto",
 *   right: "Auto",
 *   top: { Length: 0 },
 *   bottom: { Length: 0 }
 * };
 *
 * style.margin = centerMargin;
 * ```
 *
 * </details>
 */
export type LengthPercentageAuto = { Length: number } | { Percent: number } | "Auto";

/**
 * Point with x and y coordinates/values.
 *
 * Used for properties that have separate horizontal (x) and vertical (y) values,
 * such as `overflow`.
 *
 * @typeParam T - The type of each coordinate
 *
 * @property x - The horizontal value
 * @property y - The vertical value
 *
 *
 * <details>
 * <summary><strong>TypeScript Example</strong></summary>
 *
 * ```typescript
 * import { Style, Overflow, type Point } from 'taffy-js';
 *
 * const style = new Style();
 *
 * const overflow: Point<typeof Overflow[keyof typeof Overflow]> = {
 *   x: Overflow.Hidden,
 *   y: Overflow.Scroll
 * };
 *
 * style.overflow = overflow;
 * ```
 *
 * </details>
 */
export interface Point<T> {
  /** The horizontal (x-axis) value */
  x: T;
  /** The vertical (y-axis) value */
  y: T;
}

/**
 * Rectangle with left, right, top, and bottom values.
 *
 * Used for box model properties like `margin`, `padding`, `border`, and `inset`.
 *
 * @typeParam T - The type of each side value
 *
 * @property left - The left side value
 * @property right - The right side value
 * @property top - The top side value
 * @property bottom - The bottom side value
 *
 *
 * <details>
 * <summary><strong>TypeScript Example</strong></summary>
 *
 * ```typescript
 * import { Style, type Rect, type LengthPercentage, type LengthPercentageAuto } from 'taffy-js';
 *
 * const style = new Style();
 *
 * // Typed padding
 * const padding: Rect<LengthPercentage> = {
 *   left: { Length: 10 },
 *   right: { Length: 10 },
 *   top: { Length: 10 },
 *   bottom: { Length: 10 }
 * };
 *
 * // Typed margin with auto
 * const margin: Rect<LengthPercentageAuto> = {
 *   left: "Auto",
 *   right: "Auto",
 *   top: { Length: 10 },
 *   bottom: { Length: 30 }
 * };
 *
 * style.padding = padding;
 * style.margin = margin;
 * ```
 *
 * </details>
 */
export interface Rect<T> {
  /** The left side value */
  left: T;
  /** The right side value */
  right: T;
  /** The top side value */
  top: T;
  /** The bottom side value */
  bottom: T;
}

/**
 * Detailed layout information (for grid layouts).
 *
 * Returned by `detailedLayoutInfo()` for nodes using CSS Grid layout.
 * Contains detailed information about grid tracks and item placement.
 *
 * @remarks
 * This is only available when the `detailed_layout_info` feature is enabled.
 *
 *
 * <details>
 * <summary><strong>TypeScript Example</strong></summary>
 *
 * ```typescript
 * import type { DetailedLayoutInfo, DetailedGridInfo } from 'taffy-js';
 *
 * const info: DetailedLayoutInfo = tree.detailedLayoutInfo(gridNode);
 *
 * if (info !== "None" && typeof info === 'object' && 'Grid' in info) {
 *   const grid: DetailedGridInfo = info.Grid;
 *   console.log('Rows:', grid.rows.sizes);
 *   console.log('Columns:', grid.columns.sizes);
 * }
 * ```
 *
 * </details>
 */
export type DetailedLayoutInfo = { Grid: DetailedGridInfo } | "None";

/**
 * Detailed information about a grid layout.
 *
 * Contains information about grid rows, columns, and item placement.
 *
 * @property rows - Information about row tracks
 * @property columns - Information about column tracks
 * @property items - Array of item placement information
 */
export interface DetailedGridInfo {
  /** Information about the grid's row tracks */
  rows: DetailedGridTracksInfo;
  /** Information about the grid's column tracks */
  columns: DetailedGridTracksInfo;
  /** Placement information for each grid item */
  items: DetailedGridItemsInfo[];
}

/**
 * Information about grid tracks (rows or columns).
 *
 * Provides detailed sizing and gutter information for a set of grid tracks.
 *
 * @property negative_implicit_tracks - Number of implicit tracks before explicit tracks
 * @property explicit_tracks - Number of explicitly defined tracks
 * @property positive_implicit_tracks - Number of implicit tracks after explicit tracks
 * @property gutters - Array of gutter sizes between tracks (in pixels)
 * @property sizes - Array of track sizes (in pixels)
 */
export interface DetailedGridTracksInfo {
  /** Number of implicit tracks before explicit tracks (for negative line numbers) */
  negative_implicit_tracks: number;
  /** Number of tracks explicitly defined in grid-template-rows/columns */
  explicit_tracks: number;
  /** Number of implicit tracks created after explicit tracks */
  positive_implicit_tracks: number;
  /** Gap sizes between tracks in pixels */
  gutters: number[];
  /** Computed sizes of each track in pixels */
  sizes: number[];
}

/**
 * Information about a grid item's placement.
 *
 * Specifies which grid lines the item spans on both axes.
 * Line numbers are 1-indexed, with 1 being the first line.
 *
 * @property row_start - Starting row line number (1-indexed)
 * @property row_end - Ending row line number (exclusive)
 * @property column_start - Starting column line number (1-indexed)
 * @property column_end - Ending column line number (exclusive)
 */
export interface DetailedGridItemsInfo {
  /** Starting row line (1-indexed) */
  row_start: number;
  /** Ending row line (exclusive) */
  row_end: number;
  /** Starting column line (1-indexed) */
  column_start: number;
  /** Ending column line (exclusive) */
  column_end: number;
}
"#;

// =============================================================================
// Dimension DTO
// =============================================================================

/// Data Transfer Object for CSS dimension values
///
/// This enum represents a dimension value that can be:
/// - A fixed length in pixels
/// - A percentage of the parent's size
/// - Auto (size determined by content or layout algorithm)
///
/// # JSON Representation
///
/// ```json
/// { "Length": 100 }
/// { "Percent": 50 }
/// "Auto"
/// ```
///
/// # Conversion
///
/// This DTO converts bidirectionally with [`taffy::style::Dimension`].
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum DimensionDto {
    /// Fixed length in pixels
    Length(f32),
    /// Percentage of parent dimension (0-100)
    Percent(f32),
    /// Automatic sizing
    Auto,
}

impl From<DimensionDto> for Dimension {
    fn from(v: DimensionDto) -> Self {
        match v {
            DimensionDto::Length(f) => Dimension::length(f),
            DimensionDto::Percent(f) => Dimension::percent(f),
            DimensionDto::Auto => Dimension::auto(),
        }
    }
}

impl From<Dimension> for DimensionDto {
    fn from(d: Dimension) -> Self {
        if d.is_auto() {
            DimensionDto::Auto
        } else {
            match d.into_raw().tag() {
                CompactLength::LENGTH_TAG => DimensionDto::Length(d.value()),
                CompactLength::PERCENT_TAG => DimensionDto::Percent(d.value()),
                _ => DimensionDto::Auto,
            }
        }
    }
}

// =============================================================================
// LengthPercentage DTO
// =============================================================================

/// Data Transfer Object for length or percentage values
///
/// Similar to [`DimensionDto`] but does not support "Auto".
/// Used for properties like padding and border that require explicit values.
///
/// # JSON Representation
///
/// ```json
/// { "Length": 10 }
/// { "Percent": 25 }
/// ```
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LengthPercentageDto {
    /// Fixed length in pixels
    Length(f32),
    /// Percentage of parent dimension (0-100)
    Percent(f32),
}

impl From<LengthPercentageDto> for LengthPercentage {
    fn from(v: LengthPercentageDto) -> Self {
        match v {
            LengthPercentageDto::Length(f) => LengthPercentage::length(f),
            LengthPercentageDto::Percent(f) => LengthPercentage::percent(f),
        }
    }
}

impl From<LengthPercentage> for LengthPercentageDto {
    fn from(val: LengthPercentage) -> Self {
        let inner = val.into_raw();
        match inner.tag() {
            CompactLength::LENGTH_TAG => LengthPercentageDto::Length(inner.value()),
            CompactLength::PERCENT_TAG => LengthPercentageDto::Percent(inner.value()),
            _ => LengthPercentageDto::Length(0.0),
        }
    }
}

// =============================================================================
// LengthPercentageAuto DTO
// =============================================================================

/// Data Transfer Object for length, percentage, or auto values
///
/// Used for properties like margin and inset that support "Auto".
///
/// # JSON Representation
///
/// ```json
/// { "Length": 10 }
/// { "Percent": 25 }
/// "Auto"
/// ```
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum LengthPercentageAutoDto {
    /// Fixed length in pixels
    Length(f32),
    /// Percentage of parent dimension (0-100)
    Percent(f32),
    /// Automatic value (e.g., auto margins for centering)
    Auto,
}

impl From<LengthPercentageAutoDto> for LengthPercentageAuto {
    fn from(v: LengthPercentageAutoDto) -> Self {
        match v {
            LengthPercentageAutoDto::Length(f) => LengthPercentageAuto::length(f),
            LengthPercentageAutoDto::Percent(f) => LengthPercentageAuto::percent(f),
            LengthPercentageAutoDto::Auto => LengthPercentageAuto::auto(),
        }
    }
}

impl From<LengthPercentageAuto> for LengthPercentageAutoDto {
    fn from(val: LengthPercentageAuto) -> Self {
        let inner = val.into_raw();
        if inner.is_auto() {
            LengthPercentageAutoDto::Auto
        } else {
            match inner.tag() {
                CompactLength::LENGTH_TAG => LengthPercentageAutoDto::Length(inner.value()),
                CompactLength::PERCENT_TAG => LengthPercentageAutoDto::Percent(inner.value()),
                _ => LengthPercentageAutoDto::Auto,
            }
        }
    }
}

// =============================================================================
// Size DTO
// =============================================================================

/// Data Transfer Object for two-dimensional sizes
///
/// A generic container for width and height values.
///
/// # Type Parameters
///
/// - `T`: The type of each dimension (e.g., `DimensionDto`, `LengthPercentageDto`)
///
/// # JSON Representation
///
/// ```json
/// { "width": { "Length": 100 }, "height": { "Length": 50 } }
/// ```
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SizeDto<T> {
    /// The width component
    pub width: T,
    /// The height component
    pub height: T,
}

impl<T, U> From<SizeDto<T>> for Size<U>
where
    T: Into<U>,
    U: Copy,
{
    fn from(v: SizeDto<T>) -> Self {
        Size {
            width: v.width.into(),
            height: v.height.into(),
        }
    }
}

// =============================================================================
// Rect DTO
// =============================================================================

/// Data Transfer Object for four-sided rectangles
///
/// A generic container for left, right, top, and bottom values.
/// Used for margin, padding, border, and inset properties.
///
/// # Type Parameters
///
/// - `T`: The type of each side (e.g., `LengthPercentageDto`, `LengthPercentageAutoDto`)
///
/// # JSON Representation
///
/// ```json
/// { "left": { "Length": 10 }, "right": { "Length": 10 }, "top": { "Length": 5 }, "bottom": { "Length": 5 } }
/// ```
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RectDto<T> {
    /// Left side value
    pub left: T,
    /// Right side value
    pub right: T,
    /// Top side value
    pub top: T,
    /// Bottom side value
    pub bottom: T,
}

impl<T, U> From<RectDto<T>> for Rect<U>
where
    T: Into<U>,
    U: Copy,
{
    fn from(v: RectDto<T>) -> Self {
        Rect {
            left: v.left.into(),
            right: v.right.into(),
            top: v.top.into(),
            bottom: v.bottom.into(),
        }
    }
}

// =============================================================================
// Available Space DTOs
// =============================================================================

/// Data Transfer Object for available space constraints
///
/// Used when calling `computeLayout()` to specify how much space
/// is available for the layout.
///
/// # JSON Representation
///
/// ```json
/// { "width": { "Definite": 800 }, "height": { "Definite": 600 } }
/// { "width": "MaxContent", "height": { "Definite": 400 } }
/// ```
#[derive(Deserialize, Debug, Clone)]
pub struct AvailableSizeDto {
    /// Horizontal space constraint
    pub width: AvailableSpaceDto,
    /// Vertical space constraint
    pub height: AvailableSpaceDto,
}

/// Single dimension available space constraint
///
/// # Variants
///
/// - `Definite(f32)`: A specific size in pixels
/// - `MinContent`: Minimize size to fit content
/// - `MaxContent`: Maximize size without breaking content
#[derive(Deserialize, Debug, Clone)]
pub enum AvailableSpaceDto {
    /// A specific size in pixels
    Definite(f32),
    /// Minimize to fit content (may cause wrapping)
    MinContent,
    /// Maximize to fit content without wrapping
    MaxContent,
}

impl From<AvailableSizeDto> for Size<AvailableSpace> {
    fn from(s: AvailableSizeDto) -> Self {
        Size {
            width: s.width.into(),
            height: s.height.into(),
        }
    }
}

impl From<AvailableSpaceDto> for AvailableSpace {
    fn from(s: AvailableSpaceDto) -> Self {
        match s {
            AvailableSpaceDto::Definite(v) => AvailableSpace::Definite(v),
            AvailableSpaceDto::MinContent => AvailableSpace::MinContent,
            AvailableSpaceDto::MaxContent => AvailableSpace::MaxContent,
        }
    }
}
