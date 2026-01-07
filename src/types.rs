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

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use taffy::geometry::{Rect, Size};
use taffy::style::{
    AvailableSpace, CompactLength, Dimension, LengthPercentage, LengthPercentageAuto,
};
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
    /// @example
    /// ```typescript
    /// type AvailableSpace = number | "minContent" | "maxContent";
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
    pub type JsOptionAlignItems;

    /// Optional AlignSelf type for setters
    #[wasm_bindgen(typescript_type = "AlignSelf | undefined")]
    pub type JsOptionAlignSelf;

    /// Optional AlignContent type for setters
    #[wasm_bindgen(typescript_type = "AlignContent | undefined")]
    pub type JsOptionAlignContent;

    /// Optional JustifyContent type for setters
    #[wasm_bindgen(typescript_type = "JustifyContent | undefined")]
    pub type JsOptionJustifyContent;

    /// Optional number type for setters
    #[wasm_bindgen(typescript_type = "number | undefined")]
    pub type JsOptionNumber;
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
 * - Use `number` when you have a fixed container size
 * - Use `"minContent"` to shrink-wrap to the minimum content size
 * - Use `"maxContent"` to expand to fit all content without wrapping
 *
 * @example
 * ```typescript
 * import init, { TaffyTree, Style, type AvailableSpace, type Size } from 'taffy-js';
 *
 * await init();
 * const tree = new TaffyTree();
 * const root: bigint = tree.newLeaf(new Style());
 *
 * // Fixed size container with type annotation
 * const fixedSpace: Size<AvailableSpace> = {
 *   width: 800,
 *   height: 600
 * };
 * tree.computeLayout(root, fixedSpace);
 *
 * // Flexible width, fixed height
 * const flexibleSpace: Size<AvailableSpace> = {
 *   width: "maxContent",
 *   height: 400
 * };
 * tree.computeLayout(root, flexibleSpace);
 * ```
 */
export type AvailableSpace = number | "minContent" | "maxContent";

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
 * @example
 * ```typescript
 * import type { Size, Dimension, AvailableSpace } from 'taffy-js';
 *
 * // Size with explicit type parameters
 * const pixelSize: Size<number> = { width: 200, height: 100 };
 *
 * const dimensionSize: Size<Dimension> = {
 *   width: 200,
 *   height: "50%"
 * };
 *
 * const availableSize: Size<AvailableSpace> = {
 *   width: 800,
 *   height: "maxContent"
 * };
 * ```
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
 *                          is `number` if known, or `undefined` if needs to be measured.
 * @param availableSpace - The available space constraints for the node. Can be definite
 *                         pixels, "minContent", or "maxContent".
 * @param node - The node ID (`bigint`) of the node being measured
 * @param context - User-provided context attached to the node via `newLeafWithContext()`
 * @param style - The node's current Style configuration
 *
 * @returns - The measured size of the content in pixels
 *
 * @example
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
 *   { width: 200, height: "maxContent" },
 *   measureText
 * );
 * ```
 */
export type MeasureFunction = (
  knownDimensions: Size<number | undefined>,
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
 * - `number`: Fixed size in pixels
 * - `"{number}%"`: Percentage of parent's size (0-100)
 * - `"auto"`: Size determined by content or layout algorithm
 *
 * @example
 * ```typescript
 * import { Style, type Dimension, type Size } from 'taffy-js';
 *
 * const style = new Style();
 *
 * // With explicit type annotations
 * const fixedSize: Size<Dimension> = {
 *   width: 200,
 *   height: 100
 * };
 *
 * const percentSize: Size<Dimension> = {
 *   width: "50%",
 *   height: "100%"
 * };
 *
 * const autoSize: Size<Dimension> = {
 *   width: "auto",
 *   height: "auto"
 * };
 *
 * style.size = fixedSize;
 * ```
 */
export type Dimension = number | `${number}%` | "auto";

/**
 * Length or percentage value (no auto support).
 *
 * Used for properties that require explicit values, such as `padding`, `border`, and `gap`.
 *
 * @remarks
 * - `number`: Fixed size in pixels
 * - `"{number}%"`: Percentage of parent's size (0-100)
 *
 * @example
 * ```typescript
 * import { Style, type LengthPercentage, type Rect, type Size } from 'taffy-js';
 *
 * const style = new Style();
 *
 * const padding: Rect<LengthPercentage> = {
 *   left: 10,
 *   right: 10,
 *   top: 5,
 *   bottom: 5
 * };
 *
 * const gap: Size<LengthPercentage> = {
 *   width: "5%",
 *   height: "5%"
 * };
 *
 * style.padding = padding;
 * style.gap = gap;
 * ```
 */
export type LengthPercentage = number | `${number}%`;

/**
 * Length, percentage, or auto value.
 *
 * Used for properties that support auto values, such as `margin` and `inset`.
 *
 * @remarks
 * - `number`: Fixed size in pixels
 * - `"{number}%"`: Percentage of parent's size (0-100)
 * - `"auto"`: Automatic value (behavior depends on property)
 *
 * @example
 * ```typescript
 * import { Style, type LengthPercentageAuto, type Rect } from 'taffy-js';
 *
 * const style = new Style();
 *
 * // Auto margins for horizontal centering
 * const centerMargin: Rect<LengthPercentageAuto> = {
 *   left: "auto",
 *   right: "auto",
 *   top: 0,
 *   bottom: 0
 * };
 *
 * style.margin = centerMargin;
 * ```
 */
export type LengthPercentageAuto = number | `${number}%` | "auto";

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
 * @example
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
 * @example
 * ```typescript
 * import { Style, type Rect, type LengthPercentage, type LengthPercentageAuto } from 'taffy-js';
 *
 * const style = new Style();
 *
 * // Typed padding
 * const padding: Rect<LengthPercentage> = {
 *   left: 10,
 *   right: 10,
 *   top: 10,
 *   bottom: 10
 * };
 *
 * // Typed margin with auto
 * const margin: Rect<LengthPercentageAuto> = {
 *   left: "auto",
 *   right: "auto",
 *   top: 10,
 *   bottom: 30
 * };
 *
 * style.padding = padding;
 * style.margin = margin;
 * ```
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
 * @example
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
 */
export type DetailedLayoutInfo = DetailedGridInfo | null;

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
/// @remarks
/// This enum represents a dimension value that can be:
/// - A fixed length in pixels
/// - A percentage of the parent's size
/// - Auto (size determined by content or layout algorithm)
///
/// @example
/// ```json
/// 100.0
/// "50%"
/// "auto"
/// ```
/// @notes
/// This DTO converts bidirectionally with [`taffy::style::Dimension`].
#[derive(Debug, Clone)]
pub enum DimensionDto {
    /// Fixed length in pixels
    Length(f32),
    /// Percentage of parent dimension (0-100)
    Percent(f32),
    /// Automatic sizing
    Auto,
}

impl Serialize for DimensionDto {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            DimensionDto::Length(l) => serializer.serialize_f32(*l),
            DimensionDto::Percent(p) => {
                let s = format!("{}%", p);
                serializer.serialize_str(&s)
            }
            DimensionDto::Auto => serializer.serialize_str("auto"),
        }
    }
}

impl<'de> Deserialize<'de> for DimensionDto {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DimensionVisitor;

        impl<'de> Visitor<'de> for DimensionVisitor {
            type Value = DimensionDto;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a number, a percentage string ending in '%', or 'auto'")
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
                Ok(DimensionDto::Length(value as f32))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
                Ok(DimensionDto::Length(value as f32))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> {
                Ok(DimensionDto::Length(value as f32))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value == "auto" {
                    Ok(DimensionDto::Auto)
                } else if value.ends_with('%') {
                    // Try parsing the number part
                    let num_str = &value[..value.len() - 1];
                    match num_str.parse::<f32>() {
                        Ok(p) => Ok(DimensionDto::Percent(p)),
                        Err(_) => Err(E::custom("Invalid percentage value")),
                    }
                } else {
                    Err(E::custom("Expected 'auto' or a string ending with '%'"))
                }
            }
        }

        deserializer.deserialize_any(DimensionVisitor)
    }
}

impl From<DimensionDto> for Dimension {
    fn from(v: DimensionDto) -> Self {
        match v {
            DimensionDto::Length(f) => Dimension::length(f),
            DimensionDto::Percent(f) => Dimension::percent(f / 100.0),
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
                CompactLength::PERCENT_TAG => DimensionDto::Percent(d.value() * 100.0),
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
/// @example
/// ```json
/// 10.0
/// "25%"
/// ```
#[derive(Debug, Clone)]
pub enum LengthPercentageDto {
    /// Fixed length in pixels
    Length(f32),
    /// Percentage of parent dimension (0-100)
    Percent(f32),
}

impl Serialize for LengthPercentageDto {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            LengthPercentageDto::Length(l) => serializer.serialize_f32(*l),
            LengthPercentageDto::Percent(p) => {
                let s = format!("{}%", p);
                serializer.serialize_str(&s)
            }
        }
    }
}

impl<'de> Deserialize<'de> for LengthPercentageDto {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LengthPercentageVisitor;

        impl<'de> Visitor<'de> for LengthPercentageVisitor {
            type Value = LengthPercentageDto;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a number or a percentage string ending in '%'")
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
                Ok(LengthPercentageDto::Length(value as f32))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
                Ok(LengthPercentageDto::Length(value as f32))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> {
                Ok(LengthPercentageDto::Length(value as f32))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value.ends_with('%') {
                    // Try parsing the number part
                    let num_str = &value[..value.len() - 1];
                    match num_str.parse::<f32>() {
                        Ok(p) => Ok(LengthPercentageDto::Percent(p)),
                        Err(_) => Err(E::custom("Invalid percentage value")),
                    }
                } else {
                    Err(E::custom("Expected a string ending with '%'"))
                }
            }
        }

        deserializer.deserialize_any(LengthPercentageVisitor)
    }
}

impl From<LengthPercentageDto> for LengthPercentage {
    fn from(v: LengthPercentageDto) -> Self {
        match v {
            LengthPercentageDto::Length(f) => LengthPercentage::length(f),
            LengthPercentageDto::Percent(f) => LengthPercentage::percent(f / 100.0),
        }
    }
}

impl From<LengthPercentage> for LengthPercentageDto {
    fn from(val: LengthPercentage) -> Self {
        let inner = val.into_raw();
        match inner.tag() {
            CompactLength::LENGTH_TAG => LengthPercentageDto::Length(inner.value()),
            CompactLength::PERCENT_TAG => LengthPercentageDto::Percent(inner.value() * 100.0),
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
/// @example
/// ```json
/// 10.0
/// "25%"
/// "auto"
/// ```
#[derive(Debug, Clone)]
pub enum LengthPercentageAutoDto {
    /// Fixed length in pixels
    Length(f32),
    /// Percentage of parent dimension (0-100)
    Percent(f32),
    /// Automatic value (e.g., auto margins for centering)
    Auto,
}

impl Serialize for LengthPercentageAutoDto {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            LengthPercentageAutoDto::Length(l) => serializer.serialize_f32(*l),
            LengthPercentageAutoDto::Percent(p) => {
                let s = format!("{}%", p);
                serializer.serialize_str(&s)
            }
            LengthPercentageAutoDto::Auto => serializer.serialize_str("auto"),
        }
    }
}

impl<'de> Deserialize<'de> for LengthPercentageAutoDto {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LengthPercentageAutoVisitor;

        impl<'de> Visitor<'de> for LengthPercentageAutoVisitor {
            type Value = LengthPercentageAutoDto;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a number, a percentage string ending in '%', or 'auto'")
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
                Ok(LengthPercentageAutoDto::Length(value as f32))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
                Ok(LengthPercentageAutoDto::Length(value as f32))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> {
                Ok(LengthPercentageAutoDto::Length(value as f32))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value == "auto" {
                    Ok(LengthPercentageAutoDto::Auto)
                } else if value.ends_with('%') {
                    // Try parsing the number part
                    let num_str = &value[..value.len() - 1];
                    match num_str.parse::<f32>() {
                        Ok(p) => Ok(LengthPercentageAutoDto::Percent(p)),
                        Err(_) => Err(E::custom("Invalid percentage value")),
                    }
                } else {
                    Err(E::custom("Expected 'auto' or a string ending with '%'"))
                }
            }
        }

        deserializer.deserialize_any(LengthPercentageAutoVisitor)
    }
}

impl From<LengthPercentageAutoDto> for LengthPercentageAuto {
    fn from(v: LengthPercentageAutoDto) -> Self {
        match v {
            LengthPercentageAutoDto::Length(f) => LengthPercentageAuto::length(f),
            LengthPercentageAutoDto::Percent(f) => LengthPercentageAuto::percent(f / 100.0),
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
                CompactLength::PERCENT_TAG => {
                    LengthPercentageAutoDto::Percent(inner.value() * 100.0)
                }
                _ => LengthPercentageAutoDto::Auto,
            }
        }
    }
}

// =============================================================================
// PointOverflow DTO
// =============================================================================

/// Data Transfer Object for overflow values (x and y)
///
/// @example
/// ```json
/// { "x": 2, "y": 3 }
/// ```
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PointOverflowDto {
    /// The x-axis value (Overflow enum discriminant)
    pub x: u8,
    /// The y-axis value (Overflow enum discriminant)
    pub y: u8,
}

impl From<PointOverflowDto> for taffy::geometry::Point<taffy::style::Overflow> {
    fn from(v: PointOverflowDto) -> Self {
        use crate::enums::JsOverflow;

        // NOTE: Guard expressions ensure match arms stay in sync with JsOverflow discriminants.
        // If JsOverflow values change, these will fail to compile.
        const VISIBLE: u8 = JsOverflow::Visible as u8;
        const CLIP: u8 = JsOverflow::Clip as u8;
        const HIDDEN: u8 = JsOverflow::Hidden as u8;
        const SCROLL: u8 = JsOverflow::Scroll as u8;

        taffy::geometry::Point {
            x: match v.x {
                VISIBLE => taffy::style::Overflow::Visible,
                CLIP => taffy::style::Overflow::Clip,
                HIDDEN => taffy::style::Overflow::Hidden,
                SCROLL => taffy::style::Overflow::Scroll,
                _ => taffy::style::Overflow::Visible,
            },
            y: match v.y {
                VISIBLE => taffy::style::Overflow::Visible,
                CLIP => taffy::style::Overflow::Clip,
                HIDDEN => taffy::style::Overflow::Hidden,
                SCROLL => taffy::style::Overflow::Scroll,
                _ => taffy::style::Overflow::Visible,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overflow_discriminants() {
        use crate::enums::JsOverflow;

        // Ensure JsOverflow discriminants are as expected
        assert_eq!(JsOverflow::Visible as u8, 0);
        assert_eq!(JsOverflow::Clip as u8, 1);
        assert_eq!(JsOverflow::Hidden as u8, 2);
        assert_eq!(JsOverflow::Scroll as u8, 3);

        // Ensure roundtrip conversion works
        let dto = PointOverflowDto { x: 2, y: 3 };
        let point: taffy::geometry::Point<taffy::style::Overflow> = dto.into();
        assert_eq!(point.x, taffy::style::Overflow::Hidden);
        assert_eq!(point.y, taffy::style::Overflow::Scroll);
    }
}

// =============================================================================
// Size DTO
// =============================================================================

/// Data Transfer Object for two-dimensional sizes
///
/// A generic container for width and height values.
///
/// @typeParam T - The type of each dimension (e.g., `DimensionDto`, `LengthPercentageDto`)
///
/// @example
/// ```json
/// { "width": 100, "height": 50 }
/// { "width": "50%", "height": "auto" }
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
/// @typeParam T - The type of each side (e.g., `LengthPercentageDto`, `LengthPercentageAutoDto`)
///
/// @example
/// ```json
/// { "left": 10, "right": 10, "top": 5, "bottom": 5 }
/// { "left": "5%", "right": "5%", "top": "auto", "bottom": "auto" }
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
/// @example
/// ```json
/// { "width": 800, "height": 600 }
/// { "width": "maxContent", "height": 400 }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AvailableSizeDto {
    /// Horizontal space constraint
    pub width: AvailableSpaceDto,
    /// Vertical space constraint
    pub height: AvailableSpaceDto,
}

/// Single dimension available space constraint
///
/// @example
/// ```json
/// "maxContent"
/// 800
/// ```
#[derive(Debug, Clone)]
pub enum AvailableSpaceDto {
    /// A specific size in pixels
    Definite(f32),
    /// Minimize to fit content (may cause wrapping)
    MinContent,
    /// Maximize to fit content without wrapping
    MaxContent,
}

impl Serialize for AvailableSpaceDto {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            AvailableSpaceDto::Definite(val) => serializer.serialize_f32(*val),
            AvailableSpaceDto::MinContent => serializer.serialize_str("minContent"),
            AvailableSpaceDto::MaxContent => serializer.serialize_str("maxContent"),
        }
    }
}

impl<'de> Deserialize<'de> for AvailableSpaceDto {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AvailableSpaceVisitor;

        impl<'de> Visitor<'de> for AvailableSpaceVisitor {
            type Value = AvailableSpaceDto;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a number, 'minContent', or 'maxContent'")
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
                Ok(AvailableSpaceDto::Definite(value as f32))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
                Ok(AvailableSpaceDto::Definite(value as f32))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> {
                Ok(AvailableSpaceDto::Definite(value as f32))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    "minContent" => Ok(AvailableSpaceDto::MinContent),
                    "maxContent" => Ok(AvailableSpaceDto::MaxContent),
                    _ => Err(E::unknown_variant(value, &["minContent", "maxContent"])),
                }
            }
        }

        deserializer.deserialize_any(AvailableSpaceVisitor)
    }
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

impl From<AvailableSpace> for AvailableSpaceDto {
    fn from(s: AvailableSpace) -> Self {
        match s {
            AvailableSpace::Definite(v) => AvailableSpaceDto::Definite(v),
            AvailableSpace::MinContent => AvailableSpaceDto::MinContent,
            AvailableSpace::MaxContent => AvailableSpaceDto::MaxContent,
        }
    }
}

// =============================================================================
// Detailed Layout Info DTOs
// =============================================================================

/// DTO for detailed grid layout info
#[derive(Serialize)]
pub struct DetailedGridInfoDto {
    pub rows: DetailedGridTracksInfoDto,
    pub columns: DetailedGridTracksInfoDto,
    pub items: Vec<DetailedGridItemsInfoDto>,
}

/// DTO for grid track info (rows or columns)
#[derive(Serialize)]
pub struct DetailedGridTracksInfoDto {
    pub negative_implicit_tracks: u16,
    pub explicit_tracks: u16,
    pub positive_implicit_tracks: u16,
    pub gutters: Vec<f32>,
    pub sizes: Vec<f32>,
}

/// DTO for grid item placement
#[derive(Serialize)]
pub struct DetailedGridItemsInfoDto {
    pub row_start: u16,
    pub row_end: u16,
    pub column_start: u16,
    pub column_end: u16,
}
