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

    // =========================================================================
    // Grid Types
    // =========================================================================

    /// Line grid placement type (start and end placements for grid-row/grid-column)
    #[wasm_bindgen(typescript_type = "Line<GridPlacement>")]
    pub type JsLineGridPlacement;

    /// Grid template columns/rows type (uses Taffy's native serde format)
    #[wasm_bindgen(typescript_type = "GridTemplateComponent[]")]
    pub type JsGridTemplateComponents;

    /// Grid template areas type
    #[wasm_bindgen(typescript_type = "GridTemplateArea[]")]
    pub type JsGridTemplateAreas;

    /// Grid line names type
    #[wasm_bindgen(typescript_type = "string[][]")]
    pub type JsGridLineNames;

    /// Non-repeated grid tracks (for auto-columns/rows)
    #[wasm_bindgen(typescript_type = "TrackSizingFunction[]")]
    pub type JsTrackSizingFunctions;

    /// Array of BigInt values
    #[wasm_bindgen(typescript_type = "bigint[]")]
    pub type JsBigIntArray;
}

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
// Point DTO
// =============================================================================

/// Data Transfer Object for point coordinates
///
/// A generic container for x and y values. Used for positions and other 2D coordinates.
///
/// @typeParam T - The type of each coordinate (e.g., `f32`)
///
/// @example
/// ```json
/// { "x": 10, "y": 20 }
/// ```
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PointDto<T> {
    /// The x coordinate
    pub x: T,
    /// The y coordinate
    pub y: T,
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
            AvailableSpaceDto::MinContent => serializer.serialize_str("min-content"),
            AvailableSpaceDto::MaxContent => serializer.serialize_str("max-content"),
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
                formatter.write_str("a number, 'min-content', or 'max-content'")
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
                    "min-content" => Ok(AvailableSpaceDto::MinContent),
                    "max-content" => Ok(AvailableSpaceDto::MaxContent),
                    _ => Err(E::unknown_variant(value, &["min-content", "max-content"])),
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
#[serde(rename_all = "camelCase")]
pub struct DetailedGridTracksInfoDto {
    pub negative_implicit_tracks: u16,
    pub explicit_tracks: u16,
    pub positive_implicit_tracks: u16,
    pub gutters: Vec<f32>,
    pub sizes: Vec<f32>,
}

/// DTO for grid item placement
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DetailedGridItemsInfoDto {
    pub row_start: u16,
    pub row_end: u16,
    pub column_start: u16,
    pub column_end: u16,
}

// =============================================================================
// Grid Placement DTOs
// =============================================================================

/// Data Transfer Object for grid placement values
///
/// Represents how an item is placed on a grid track (row or column).
/// Follows CSS `grid-row-start` / `grid-column-start` specification.
///
/// @example
/// ```json
/// "auto"
/// 2
/// { "span": 3 }
/// ```
#[derive(Debug, Clone)]
pub enum GridPlacementDto {
    Auto,
    Line(i16),
    Span(u16),
    NamedLine(i16, String), // Changed from String to (i16, String)
    NamedSpan(u16, String),
}

impl Serialize for GridPlacementDto {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        match self {
            GridPlacementDto::Auto => serializer.serialize_str("auto"),
            GridPlacementDto::Line(l) => serializer.serialize_i16(*l),
            GridPlacementDto::Span(s) => {
                let mut state = serializer.serialize_struct("Span", 1)?;
                state.serialize_field("span", s)?;
                state.end()
            }
            GridPlacementDto::NamedLine(l, s) => {
                // Output { line: n, ident: s }
                let mut state = serializer.serialize_struct("NamedLine", 2)?;
                state.serialize_field("line", l)?; // Use "line" as requested
                state.serialize_field("ident", s)?;
                state.end()
            }
            GridPlacementDto::NamedSpan(n, s) => {
                let mut state = serializer.serialize_struct("NamedSpan", 2)?;
                state.serialize_field("span", n)?;
                state.serialize_field("ident", s)?;
                state.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for GridPlacementDto {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GridPlacementVisitor;

        impl<'de> Visitor<'de> for GridPlacementVisitor {
            type Value = GridPlacementDto;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("'auto', line number, line object, or span object")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value == "auto" {
                    Ok(GridPlacementDto::Auto)
                } else {
                    // Previous "header" string support removed?
                    // User requested specific type. But backward compat?
                    // "auto" | number | { line: ..., ident: ... } ...
                    // User declaration REMOVED string.
                    // So "header" is strictly invalid.
                    // But I'll allow it as convenient shorthand if valid?
                    // No, user specifically rewrote stricter type.
                    Err(E::custom("String input only supported for 'auto'"))
                }
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
                Ok(GridPlacementDto::Line(value as i16))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
                Ok(GridPlacementDto::Line(value as i16))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> {
                Ok(GridPlacementDto::Line(value as i16))
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut span: Option<u16> = None;
                let mut ident: Option<String> = None;
                let mut line: Option<i16> = None; // For named line

                while let Some(key) = map.next_key::<String>()? {
                    if key == "span" {
                        span = Some(map.next_value()?);
                    } else if key == "ident" {
                        ident = Some(map.next_value()?);
                    } else if key == "line" {
                        line = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<serde::de::IgnoredAny>()?;
                    }
                }

                match (span, line, ident) {
                    (Some(s), None, Some(i)) => Ok(GridPlacementDto::NamedSpan(s, i)),
                    (Some(s), None, None) => Ok(GridPlacementDto::Span(s)),
                    (None, Some(l), Some(i)) => Ok(GridPlacementDto::NamedLine(l, i)),
                    (None, None, Some(i)) => Ok(GridPlacementDto::NamedSpan(1, i)), // { ident: "header" } -> span 1 header? Or named line default?
                    // User says { line: number, ident: string }.
                    // If line is missing, maybe default 0? But { ident: string } usually means "1st line with name".
                    // However, type says line is MANDATORY.
                    // I'll assume NamedSpan(1, i) if line & span missing.
                    // Or error?
                    // Let's stick to explicit match.
                    _ => Err(de::Error::custom("Invalid object for GridPlacement")),
                }
            }
        }

        deserializer.deserialize_any(GridPlacementVisitor)
    }
}

use taffy::geometry::Line;
use taffy::style::GridPlacement;

impl From<GridPlacement> for GridPlacementDto {
    fn from(val: GridPlacement) -> Self {
        match val {
            GridPlacement::Auto => GridPlacementDto::Auto,
            GridPlacement::Line(line) => GridPlacementDto::Line(line.as_i16()),
            GridPlacement::Span(span) => GridPlacementDto::Span(span),
            GridPlacement::NamedLine(name, idx) => {
                GridPlacementDto::NamedLine(idx, name.to_string())
            }
            GridPlacement::NamedSpan(name, span) => {
                GridPlacementDto::NamedSpan(span, name.to_string())
            }
        }
    }
}

impl From<GridPlacementDto> for GridPlacement {
    fn from(val: GridPlacementDto) -> Self {
        use taffy::style_helpers::TaffyGridLine;
        use taffy::style_helpers::TaffyGridSpan;

        match val {
            GridPlacementDto::Auto => GridPlacement::Auto,
            GridPlacementDto::Line(idx) => GridPlacement::from_line_index(idx),
            GridPlacementDto::Span(span) => GridPlacement::from_span(span),
            GridPlacementDto::NamedLine(idx, s) => GridPlacement::NamedLine(s.into(), idx), // NamedLine variant
            GridPlacementDto::NamedSpan(n, s) => GridPlacement::NamedSpan(s.into(), n),
        }
    }
}

/// Data Transfer Object for grid line placement (start and end)
///
/// Represents CSS grid-row or grid-column shorthand.
///
/// @example
/// ```json
/// { "start": 1, "end": 3 }
/// { "start": "auto", "end": { "span": 2 } }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LineGridPlacementDto {
    /// Start placement
    pub start: GridPlacementDto,
    /// End placement
    pub end: GridPlacementDto,
}

impl From<Line<GridPlacement>> for LineGridPlacementDto {
    fn from(val: Line<GridPlacement>) -> Self {
        LineGridPlacementDto {
            start: val.start.into(),
            end: val.end.into(),
        }
    }
}

impl From<LineGridPlacementDto> for Line<GridPlacement> {
    fn from(val: LineGridPlacementDto) -> Self {
        Line {
            start: val.start.into(),
            end: val.end.into(),
        }
    }
}

// =============================================================================
// Grid Track Sizing DTOs
// =============================================================================

#[derive(Debug, Clone)]
pub enum MinTrackSizingFunctionDto {
    Length(f32),
    Percent(f32),
    Auto,
    MinContent,
    MaxContent,
}

impl Serialize for MinTrackSizingFunctionDto {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Length(v) => serializer.serialize_f32(*v),
            Self::Percent(v) => serializer.serialize_str(&format!("{}%", v)),
            Self::Auto => serializer.serialize_str("auto"),
            Self::MinContent => serializer.serialize_str("min-content"),
            Self::MaxContent => serializer.serialize_str("max-content"),
        }
    }
}

impl<'de> Deserialize<'de> for MinTrackSizingFunctionDto {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MinTrackVisitor;
        impl<'de> Visitor<'de> for MinTrackVisitor {
            type Value = MinTrackSizingFunctionDto;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("number, percentage string, or keyword")
            }
            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MinTrackSizingFunctionDto::Length(v as f32))
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MinTrackSizingFunctionDto::Length(v as f32))
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MinTrackSizingFunctionDto::Length(v as f32))
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match v {
                    "auto" => Ok(MinTrackSizingFunctionDto::Auto),
                    "min-content" => Ok(MinTrackSizingFunctionDto::MinContent),
                    "max-content" => Ok(MinTrackSizingFunctionDto::MaxContent),
                    s if s.ends_with('%') => {
                        let num = s[..s.len() - 1].parse::<f32>().map_err(E::custom)?;
                        Ok(MinTrackSizingFunctionDto::Percent(num))
                    }
                    _ => Err(E::custom(format!("Unknown MinTrackSizingFunction: {}", v))),
                }
            }
        }
        deserializer.deserialize_any(MinTrackVisitor)
    }
}

#[derive(Debug, Clone)]
pub enum MaxTrackSizingFunctionDto {
    Length(f32),
    Percent(f32),
    Fraction(f32),
    FitContent(f32),
    FitContentPercent(f32),
    Auto,
    MinContent,
    MaxContent,
}

impl Serialize for MaxTrackSizingFunctionDto {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Length(v) => serializer.serialize_f32(*v),
            Self::Percent(v) => serializer.serialize_str(&format!("{}%", v)),
            Self::Fraction(v) => serializer.serialize_str(&format!("{}fr", v)),
            Self::FitContent(_) => serializer.serialize_str("fit-content"),
            Self::FitContentPercent(_) => serializer.serialize_str("fit-content"),
            Self::Auto => serializer.serialize_str("auto"),
            Self::MinContent => serializer.serialize_str("min-content"),
            Self::MaxContent => serializer.serialize_str("max-content"),
        }
    }
}

impl<'de> Deserialize<'de> for MaxTrackSizingFunctionDto {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MaxTrackVisitor;
        impl<'de> Visitor<'de> for MaxTrackVisitor {
            type Value = MaxTrackSizingFunctionDto;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("number, %, fr, or keyword")
            }
            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MaxTrackSizingFunctionDto::Length(v as f32))
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MaxTrackSizingFunctionDto::Length(v as f32))
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MaxTrackSizingFunctionDto::Length(v as f32))
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match v {
                    "auto" => Ok(MaxTrackSizingFunctionDto::Auto),
                    "min-content" => Ok(MaxTrackSizingFunctionDto::MinContent),
                    "max-content" => Ok(MaxTrackSizingFunctionDto::MaxContent),
                    "fit-content" => Ok(MaxTrackSizingFunctionDto::FitContent(0.0)),
                    s if s.ends_with("fr") => {
                        let num = s[..s.len() - 2].parse::<f32>().map_err(E::custom)?;
                        Ok(MaxTrackSizingFunctionDto::Fraction(num))
                    }
                    s if s.ends_with('%') => {
                        let num = s[..s.len() - 1].parse::<f32>().map_err(E::custom)?;
                        Ok(MaxTrackSizingFunctionDto::Percent(num))
                    }
                    _ => Err(E::custom(format!("Unknown MaxTrackSizingFunction: {}", v))),
                }
            }
        }
        deserializer.deserialize_any(MaxTrackVisitor)
    }
}

#[derive(Debug, Clone)]
pub enum RepetitionCountDto {
    Count(u16),
    AutoFill,
    AutoFit,
}

impl Serialize for RepetitionCountDto {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Count(v) => serializer.serialize_u16(*v),
            Self::AutoFill => serializer.serialize_str("auto-fill"),
            Self::AutoFit => serializer.serialize_str("auto-fit"),
        }
    }
}

impl<'de> Deserialize<'de> for RepetitionCountDto {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RepetitionVisitor;
        impl<'de> Visitor<'de> for RepetitionVisitor {
            type Value = RepetitionCountDto;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("number or keyword")
            }
            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(RepetitionCountDto::Count(v as u16))
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(RepetitionCountDto::Count(v as u16))
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(RepetitionCountDto::Count(v as u16))
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match v {
                    "auto-fill" => Ok(RepetitionCountDto::AutoFill),
                    "auto-fit" => Ok(RepetitionCountDto::AutoFit),
                    _ => Err(E::custom(format!("Unknown RepetitionCount: {}", v))),
                }
            }
        }
        deserializer.deserialize_any(RepetitionVisitor)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrackSizingFunctionDto {
    pub min: MinTrackSizingFunctionDto,
    pub max: MaxTrackSizingFunctionDto,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GridTemplateAreaDto {
    pub name: String,
    pub row_start: u16,
    pub row_end: u16,
    pub column_start: u16,
    pub column_end: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum GridTemplateComponentDto {
    Single(TrackSizingFunctionDto),
    Repeat {
        #[serde(rename = "count")]
        count: RepetitionCountDto,
        tracks: Vec<TrackSizingFunctionDto>,
        #[serde(default, rename = "lineNames")]
        line_names: Vec<Vec<String>>,
    },
}

use taffy::style::{
    CheapCloneStr, GridTemplateArea, GridTemplateComponent, GridTemplateRepetition,
    MaxTrackSizingFunction, MinTrackSizingFunction, RepetitionCount, TrackSizingFunction,
};

impl<S: CheapCloneStr> From<GridTemplateArea<S>> for GridTemplateAreaDto
where
    String: From<S>,
{
    fn from(val: GridTemplateArea<S>) -> Self {
        GridTemplateAreaDto {
            name: val.name.into(),
            row_start: val.row_start,
            row_end: val.row_end,
            column_start: val.column_start,
            column_end: val.column_end,
        }
    }
}

impl<S: CheapCloneStr> From<GridTemplateAreaDto> for GridTemplateArea<S>
where
    S: From<String>,
{
    fn from(val: GridTemplateAreaDto) -> Self {
        GridTemplateArea {
            name: val.name.into(),
            row_start: val.row_start,
            row_end: val.row_end,
            column_start: val.column_start,
            column_end: val.column_end,
        }
    }
}

// Min conversions
impl From<MinTrackSizingFunction> for MinTrackSizingFunctionDto {
    fn from(val: MinTrackSizingFunction) -> Self {
        let raw = val.into_raw();
        match raw.tag() {
            CompactLength::LENGTH_TAG => MinTrackSizingFunctionDto::Length(raw.value()),
            CompactLength::PERCENT_TAG => MinTrackSizingFunctionDto::Percent(raw.value()),
            CompactLength::AUTO_TAG => MinTrackSizingFunctionDto::Auto,
            CompactLength::MIN_CONTENT_TAG => MinTrackSizingFunctionDto::MinContent,
            CompactLength::MAX_CONTENT_TAG => MinTrackSizingFunctionDto::MaxContent,
            _ => MinTrackSizingFunctionDto::Auto,
        }
    }
}

impl From<MinTrackSizingFunctionDto> for MinTrackSizingFunction {
    fn from(val: MinTrackSizingFunctionDto) -> Self {
        match val {
            MinTrackSizingFunctionDto::Length(v) => MinTrackSizingFunction::length(v),
            MinTrackSizingFunctionDto::Percent(v) => MinTrackSizingFunction::percent(v),
            MinTrackSizingFunctionDto::Auto => MinTrackSizingFunction::auto(),
            MinTrackSizingFunctionDto::MinContent => MinTrackSizingFunction::min_content(),
            MinTrackSizingFunctionDto::MaxContent => MinTrackSizingFunction::max_content(),
        }
    }
}

// Max conversions
impl From<MaxTrackSizingFunction> for MaxTrackSizingFunctionDto {
    fn from(val: MaxTrackSizingFunction) -> Self {
        let raw = val.into_raw();
        match raw.tag() {
            CompactLength::LENGTH_TAG => MaxTrackSizingFunctionDto::Length(raw.value()),
            CompactLength::PERCENT_TAG => MaxTrackSizingFunctionDto::Percent(raw.value()),
            CompactLength::FR_TAG => MaxTrackSizingFunctionDto::Fraction(raw.value()),
            CompactLength::FIT_CONTENT_PX_TAG => MaxTrackSizingFunctionDto::FitContent(raw.value()),
            CompactLength::FIT_CONTENT_PERCENT_TAG => {
                MaxTrackSizingFunctionDto::FitContentPercent(raw.value())
            }
            CompactLength::AUTO_TAG => MaxTrackSizingFunctionDto::Auto,
            CompactLength::MIN_CONTENT_TAG => MaxTrackSizingFunctionDto::MinContent,
            CompactLength::MAX_CONTENT_TAG => MaxTrackSizingFunctionDto::MaxContent,
            _ => MaxTrackSizingFunctionDto::Auto,
        }
    }
}

impl From<MaxTrackSizingFunctionDto> for MaxTrackSizingFunction {
    fn from(val: MaxTrackSizingFunctionDto) -> Self {
        match val {
            MaxTrackSizingFunctionDto::Length(v) => MaxTrackSizingFunction::length(v),
            MaxTrackSizingFunctionDto::Percent(v) => MaxTrackSizingFunction::percent(v),
            MaxTrackSizingFunctionDto::Fraction(v) => MaxTrackSizingFunction::fr(v),
            MaxTrackSizingFunctionDto::FitContent(v) => MaxTrackSizingFunction::fit_content_px(v),
            MaxTrackSizingFunctionDto::FitContentPercent(v) => {
                MaxTrackSizingFunction::fit_content_percent(v)
            }
            MaxTrackSizingFunctionDto::Auto => MaxTrackSizingFunction::auto(),
            MaxTrackSizingFunctionDto::MinContent => MaxTrackSizingFunction::min_content(),
            MaxTrackSizingFunctionDto::MaxContent => MaxTrackSizingFunction::max_content(),
        }
    }
}

// TrackSizingFunction conversions (Struct)
impl From<TrackSizingFunction> for TrackSizingFunctionDto {
    fn from(val: TrackSizingFunction) -> Self {
        TrackSizingFunctionDto {
            min: val.min.into(),
            max: val.max.into(),
        }
    }
}

impl From<TrackSizingFunctionDto> for TrackSizingFunction {
    fn from(val: TrackSizingFunctionDto) -> Self {
        TrackSizingFunction {
            min: val.min.into(),
            max: val.max.into(),
        }
    }
}

// Repetition conversions
// Repetition conversions
impl From<RepetitionCount> for RepetitionCountDto {
    fn from(val: RepetitionCount) -> Self {
        match val {
            RepetitionCount::Count(c) => RepetitionCountDto::Count(c),
            RepetitionCount::AutoFill => RepetitionCountDto::AutoFill,
            RepetitionCount::AutoFit => RepetitionCountDto::AutoFit,
        }
    }
}

impl From<RepetitionCountDto> for RepetitionCount {
    fn from(val: RepetitionCountDto) -> Self {
        match val {
            RepetitionCountDto::Count(c) => RepetitionCount::Count(c),
            RepetitionCountDto::AutoFill => RepetitionCount::AutoFill,
            RepetitionCountDto::AutoFit => RepetitionCount::AutoFit,
        }
    }
}

// GridTemplateComponentDto maps to GridTemplateComponent (Generic)
impl<S: CheapCloneStr> From<GridTemplateComponent<S>> for GridTemplateComponentDto
where
    String: From<S>,
{
    fn from(val: GridTemplateComponent<S>) -> Self {
        match val {
            GridTemplateComponent::Single(n) => GridTemplateComponentDto::Single(n.into()),
            GridTemplateComponent::Repeat(rep) => {
                let dtos: Vec<TrackSizingFunctionDto> =
                    rep.tracks.into_iter().map(|t| t.into()).collect();
                let line_names: Vec<Vec<String>> = rep
                    .line_names
                    .into_iter()
                    .map(|names| names.into_iter().map(|n| n.into()).collect())
                    .collect();
                GridTemplateComponentDto::Repeat {
                    count: rep.count.into(),
                    tracks: dtos,
                    line_names,
                }
            }
        }
    }
}

impl<S: CheapCloneStr> From<GridTemplateComponentDto> for GridTemplateComponent<S>
where
    S: From<String>,
{
    fn from(val: GridTemplateComponentDto) -> Self {
        match val {
            GridTemplateComponentDto::Single(n) => GridTemplateComponent::Single(n.into()),
            GridTemplateComponentDto::Repeat {
                count,
                tracks,
                line_names,
            } => {
                let layout_tracks: Vec<TrackSizingFunction> =
                    tracks.into_iter().map(|t| t.into()).collect();
                let layout_line_names: Vec<Vec<S>> = line_names
                    .into_iter()
                    .map(|names| names.into_iter().map(|n| n.into()).collect())
                    .collect();

                GridTemplateComponent::Repeat(GridTemplateRepetition {
                    count: count.into(),
                    tracks: layout_tracks,
                    line_names: layout_line_names,
                })
            }
        }
    }
}
