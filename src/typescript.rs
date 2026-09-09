//! # TypeScript Custom Type Declarations Module
//!
//! This module contains additional TypeScript type declarations that are appended
//! to the generated `.d.ts` file via `wasm_bindgen(typescript_custom_section)`.
//!
//! ## Overview
//!
//! These types provide accurate TypeScript definitions for complex types that
//! wasm-bindgen cannot automatically generate, including:
//!
//! - `AvailableSpace`, `Size<T>`, `Rect<T>`, `Point<T>`
//! - `Dimension`, `LengthPercentage`, `LengthPercentageAuto`
//! - `MeasureFunction` callback signature
//! - Detailed grid layout info types
//! - `GridPlacement` and `Line<T>` for grid positioning

use wasm_bindgen::prelude::*;

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
 * - Use `number` to offer a definite amount of space
 * - Use `"min-content"` for an intrinsic minimum-content constraint
 * - Use `"max-content"` for an intrinsic maximum-content constraint
 * Available space does not override explicit sizes, min/max constraints, or
 * Flexbox wrapping rules. It is not a guaranteed final size.
 *
 * @example
 * ```typescript
 * import init, { TaffyTree, Style, type AvailableSpace, type Size } from 'taffy-layout';
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
 *   width: "max-content",
 *   height: 400
 * };
 * tree.computeLayout(root, flexibleSpace);
 * ```
 */
export type AvailableSpace = number | "min-content" | "max-content";

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
 * import type { Size, Dimension, AvailableSpace } from 'taffy-layout';
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
 *   height: "max-content"
 * };
 * ```
 */
export type Size<T> = {
  /** The horizontal dimension value */
  width: T;
  /** The vertical dimension value */
  height: T;
};

/**
 * Custom measure function for leaf nodes with text or other dynamic content.
 *
 * This callback is invoked during layout computation for leaf nodes that need
 * custom sizing based on their content (e.g., text nodes that need text measurement).
 *
 * @param knownDimensions - Dimensions already determined by constraints. Each dimension
 *                          is `number` if known, or `undefined` if needs to be measured.
 * @param availableSpace - The available space constraints for the node. Can be definite
 *                         pixels, "min-content", or "max-content".
 * @param node - The node ID (`bigint`) of the node being measured
 * @param context - Value attached via `newLeafWithContext()` or `setNodeContext()`,
 *                  or `undefined` when the node has no attached context
 * @param style - An owned copy of the node's current Style; call `free()` when finished
 *
 * @returns - The measured size of the content in pixels
 *
 * @remarks
 * Padding, borders, size constraints, and aspect ratios are applied by the layout
 * engine around this content measurement. Available space is adjusted for the
 * content box. Measurements may be cached, so the callback need not run for every
 * node on every layout pass. Call `markDirty()` after changing measured content
 * without changing its style or context.
 * A context is optional; nodes created with `newLeaf()` can also be measured.
 * Mutating an attached context object or changing the measurement function does
 * not invalidate cached measurements; mark the affected nodes dirty first.
 * The callback must be synchronous. Thrown exceptions and invalid return values
 * are currently converted to a zero content measurement by the binding. Record
 * failures and handle them outside `computeLayoutWithMeasure()` if needed.
 *
 * @example
 * ```typescript
 * import init, { TaffyTree, Style, type MeasureFunction, type Size } from 'taffy-layout';
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
 * // Helper function to measure text width
 * const measureTextWidth = (text: string, fontSize: number) => text.length * fontSize * 0.6;
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
 *   style.free(); // This measurement does not need to read the style copy.
 *   if (!ctx?.text) {
 *     return { width: knownDimensions.width ?? 0, height: knownDimensions.height ?? 0 };
 *   }
 *
 *   const width = knownDimensions.width ?? measureTextWidth(ctx.text, ctx.fontSize);
 *   const height = knownDimensions.height ?? ctx.fontSize * 1.2;
 *
 *   return { width, height };
 * };
 *
 * tree.computeLayoutWithMeasure(
 *   textNode,
 *   { width: 200, height: "max-content" },
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
 * - `"{number}%"`: Percentage resolved against the property's containing size; `"100%"` is the full reference size, and values above 100 are allowed
 * - `"auto"`: Size determined by content or layout algorithm
 *
 * @example
 * ```typescript
 * import { Style, type Dimension, type Size } from 'taffy-layout';
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
 * - `"{number}%"`: Percentage whose reference size depends on the property; `"100%"` is the full reference size, and values above 100 are allowed
 *
 * @example
 * ```typescript
 * import { Style, type LengthPercentage, type Rect, type Size } from 'taffy-layout';
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
 * - `"{number}%"`: Percentage whose reference size depends on the property; margin percentages use the containing width, while inset percentages use the corresponding axis
 * - `"auto"`: Automatic value (behavior depends on property)
 *
 * @example
 * ```typescript
 * import { Style, type LengthPercentageAuto, type Rect } from 'taffy-layout';
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
 * import { Style, Overflow, type Point } from 'taffy-layout';
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
export type Point<T> = {
  /** The horizontal (x-axis) value */
  x: T;
  /** The vertical (y-axis) value */
  y: T;
};

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
 * import { Style, type Rect, type LengthPercentage, type LengthPercentageAuto } from 'taffy-layout';
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
export type Rect<T> = {
  /** The left side value */
  left: T;
  /** The right side value */
  right: T;
  /** The top side value */
  top: T;
  /** The bottom side value */
  bottom: T;
};

/**
 * Detailed layout information (for grid layouts).
 *
 * Returned by `detailedLayoutInfo()` after computing a grid container's layout.
 * Contains `rows`, `columns`, and `items` directly, or is `null` if no grid details
 * have been stored. Childless grids use leaf layout and do not generate details.
 * Previously stored details can remain after changing display mode or removing
 * children. Read them after laying out a current grid container with children;
 * a non-null result alone does not establish that the details are current.
 *
 * @remarks
 * This is only available when the `detailed_layout_info` feature is enabled.
 *
 * @example
 * ```typescript
 * import { TaffyTree, Style, Display, type DetailedLayoutInfo } from 'taffy-layout';
 *
 * const tree = new TaffyTree();
 * const style = new Style();
 * style.display = Display.Grid;
 * const child = tree.newLeaf(new Style());
 * const gridNode = tree.newWithChildren(style, [child]);
 * tree.computeLayout(gridNode, { width: 100, height: 100 });
 *
 * const info: DetailedLayoutInfo = tree.detailedLayoutInfo(gridNode);
 *
 * if (info !== null) {
 *   console.log('Rows:', info.rows.sizes);
 *   console.log('Columns:', info.columns.sizes);
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
export type DetailedGridInfo = {
  /** Information about the grid's row tracks */
  rows: DetailedGridTracksInfo;
  /** Information about the grid's column tracks */
  columns: DetailedGridTracksInfo;
  /** Placement information for each grid item */
  items: DetailedGridItemsInfo[];
};

/**
 * Information about grid tracks (rows or columns).
 *
 * Tracks use logical order: left-to-right for LTR columns, right-to-left for
 * RTL columns, and top-to-bottom for rows.
 *
 * @property negativeImplicitTracks - Number of implicit tracks before explicit tracks
 * @property explicitTracks - Number of explicitly defined tracks
 * @property positiveImplicitTracks - Number of implicit tracks after explicit tracks
 * @property gutters - Physical spacing between tracks, including content alignment
 * @property sizes - Array of track sizes (in pixels)
 * @property positions - Physical start/end coordinates of each track
 */
export type DetailedGridTracksInfo = {
  /** Number of implicit tracks before explicit tracks (for negative line numbers) */
  negativeImplicitTracks: number;
  /** Number of explicit tracks, including tracks established by gridTemplateAreas and its counts */
  explicitTracks: number;
  /** Number of implicit tracks created after explicit tracks */
  positiveImplicitTracks: number;
  /**
   * Physical spacing between adjacent tracks in pixels, including space added
   * by content alignment. Contains sizes.length + 1 entries with zero at both
   * ends (or [0] for no tracks).
   */
  gutters: number[];
  /** Computed sizes of each track in pixels */
  sizes: number[];
  /**
   * Physical start/end coordinates relative to the container's border box,
   * stored in logical track order. Each start is the physical left/top edge;
   * each end is the physical right/bottom edge.
   */
  positions: Line<number>[];
};

/**
 * Information about a grid item's placement.
 *
 * Specifies which grid lines the item spans on both axes.
 * Line numbers are 1-indexed from the first generated line, including leading
 * implicit tracks. They can differ from the explicit grid line numbers used in styles.
 *
 * @property rowStart - Starting row line number (1-indexed)
 * @property rowEnd - Ending row line number (exclusive)
 * @property columnStart - Starting column line number (1-indexed)
 * @property columnEnd - Ending column line number (exclusive)
 */
export type DetailedGridItemsInfo = {
  /** Starting row line (1-indexed) */
  rowStart: number;
  /** Ending row line (exclusive) */
  rowEnd: number;
  /** Starting column line (1-indexed) */
  columnStart: number;
  /** Ending column line (exclusive) */
  columnEnd: number;
};

/**
 * Grid placement type for positioning grid items.
 *
 * Specifies how an item is placed on a grid track (row or column).
 * Follows CSS `grid-row-start` / `grid-column-start` specification.
 *
 * @remarks
 * - `"auto"`: Auto-placement using the grid's flow algorithm
 * - `number`: Place at a specific line index (1-indexed, can be negative)
 * - `{ span: number }`: Span a specified number of tracks
 *
 * @example
 * ```typescript
 * import type { GridPlacement, Line } from 'taffy-layout';
 *
 * // Line index (CSS: grid-row-start: 2)
 * const lineIndex: GridPlacement = 2;
 *
 * // Auto placement (CSS: grid-row-start: auto)
 * const auto: GridPlacement = "auto";
 *
 * // Span (CSS: grid-row-start: span 3)
 * const span: GridPlacement = { span: 3 };
 *
 * // Named line (CSS: grid-row-start: header 2)
 * const named: GridPlacement = { line: 2, ident: "header" };
 *
 * // Named span (CSS: grid-row-start: span 2 header)
 * const namedSpan: GridPlacement = { span: 2, ident: "header" };
 * ```
 */
export type GridPlacement = "auto" | number | {line: number; ident: string} | {span: number; ident?: string};

/**
 * Line type representing start and end positions.
 *
 * A container for start and end values, used for CSS grid-row and grid-column
 * shorthand properties.
 *
 * @typeParam T - The type of start and end values
 *
 * @property start - The starting line/position
 * @property end - The ending line/position
 *
 * @example
 * ```typescript
 * import { Style, Display, type Line, type GridPlacement } from 'taffy-layout';
 *
 * const style = new Style();
 * style.display = Display.Grid;
 *
 * // CSS: grid-row: 1 / 3
 * style.gridRow = { start: 1, end: 3 };
 *
 * // CSS: grid-column: 1 / span 2
 * style.gridColumn = { start: 1, end: { span: 2 } };
 *
 * // CSS: grid-row: auto / auto
 * style.gridRow = { start: "auto", end: "auto" };
 * ```
 */
export type Line<T> = {
  /** The starting position (CSS: *-start) */
  start: T;
  /** The ending position (CSS: *-end) */
  end: T;
}
/**
 * Grid track repetition parameter.
 *
 * Defines how many times a track pattern should repeat.
 *
 * @remarks
 * - `number`: Exact number of repetitions (e.g. `repeat(3, ...)`).
 * - `"auto-fill"`: Fills the container with as many tracks as possible.
 * - `"auto-fit"`: Fills the container, collapsing empty tracks.
 */
export type RepetitionCount = number | "auto-fill" | "auto-fit";

/**
 * Minimum track sizing function.
 *
 * Defines the minimum size of a grid track.
 */
export type MinTrackSizingFunction = number | `${number}%` | "auto" | "min-content" | "max-content";

/**
 * Maximum track sizing function.
 *
 * Defines the maximum size of a grid track.
 * The supported `"fit-content"` token uses a zero-pixel fit-content limit.
 * Parameterized strings such as `"fit-content(100px)"` are not supported.
 */
export type MaxTrackSizingFunction = number | `${number}%` | `${number}fr` | "auto" | "min-content" | "max-content" | "fit-content";

/**
 * Track sizing function (min/max pair).
 *
 * Defines the size range for a single grid track.
 */
export type TrackSizingFunction = {min: MinTrackSizingFunction; max: MaxTrackSizingFunction};

/**
 * Grid track repetition definition.
 */
export type GridTemplateRepetition = {
  count: RepetitionCount;
  tracks: TrackSizingFunction[];
  lineNames?: string[][];
};

/**
 * Grid track sizing definition.
 *
 * Can be a single track sizing function or a repetition of tracks.
 */
export type GridTemplateComponent = TrackSizingFunction | GridTemplateRepetition;

/**
 * Named grid area definition.
 * 
 * Defines a named area within the grid and its boundaries. Line numbers start
 * at 1 and end lines are exclusive.
 */
export type GridTemplateArea = {
  /** The name of the grid area */
  name: string;
  /** Start row line */
  rowStart: number;
  /** End row line */
  rowEnd: number;
  /** Start column line */
  columnStart: number;
  /** End column line */
  columnEnd: number;
};

/**
 * Valid property keys for Style.get() method.
 *
 * Supports both object properties and individual flat properties.
 *
 * @example
 * ```typescript
 * const style = new Style();
 * // Top-level properties
 * style.get("display", "flexGrow");
 *
 * // Individual flat properties
 * style.get("width", "marginLeft", "paddingTop");
 * 
 * // Object properties
 * style.get("size", "margin");
 * ```
 */
export type StyleProperty =
  // Layout Mode
  | "display" | "position" | "direction" | "float" | "clear" | "boxSizing"
  // Overflow
  | "overflow" | "overflowX" | "overflowY"
  // Flexbox
  | "flexDirection" | "flexWrap" | "flexGrow" | "flexShrink" | "flexBasis"
  // Alignment
  | "alignItems" | "alignSelf" | "alignContent"
  | "justifyContent" | "justifyItems" | "justifySelf"
  // Sizing
  | "aspectRatio"
  | "size" | "width" | "height"
  | "minSize" | "minWidth" | "minHeight"
  | "maxSize" | "maxWidth" | "maxHeight"
  // Spacing
  | "margin" | "marginLeft" | "marginRight" | "marginTop" | "marginBottom"
  | "padding" | "paddingLeft" | "paddingRight" | "paddingTop" | "paddingBottom"
  | "border" | "borderLeft" | "borderRight" | "borderTop" | "borderBottom"
  | "inset" | "left" | "right" | "top" | "bottom"
  | "gap" | "columnGap" | "rowGap"
  // Block layout
  | "itemIsTable" | "itemIsReplaced" | "scrollbarWidth" | "textAlign"
  // Grid layout
  | "gridAutoFlow"
  | "gridRow" | "gridRowStart" | "gridRowEnd"
  | "gridColumn" | "gridColumnStart" | "gridColumnEnd"
  | "gridTemplateRows" | "gridTemplateColumns"
  | "gridAutoRows" | "gridAutoColumns"
  | "gridTemplateAreas" | "gridTemplateAreaRowCount" | "gridTemplateAreaColumnCount"
  | "gridTemplateRowNames" | "gridTemplateColumnNames";

/**
 * Valid property keys for Layout.get() method.
 *
 * Supports both object properties and individual flat properties.
 *
 * @example
 * ```typescript
 * import { TaffyTree, Style } from "taffy-layout";
 *
 * const tree = new TaffyTree();
 * const root = tree.newLeaf(new Style());
 * tree.computeLayout(root, { width: 100, height: 100 });
 * const layout = tree.getLayout(root);
 * // Object properties
 * layout.get("position", "size");
 *
 * // Individual flat properties
 * layout.get("width", "height", "marginLeft");
 *
 * // Mixed
 * layout.get("position", "width", "paddingTop");
 *
 * tree.free();
 * ```
 */
export type LayoutProperty =
  // Rendering order
  | "order"
  // Position
  | "position" | "x" | "y"
  // Size
  | "size" | "width" | "height"
  // Content size
  | "contentSize" | "contentWidth" | "contentHeight"
  // Scrollbar size
  | "scrollbarSize" | "scrollbarWidth" | "scrollbarHeight"
  // Border
  | "border" | "borderLeft" | "borderRight" | "borderTop" | "borderBottom"
  // Padding
  | "padding" | "paddingLeft" | "paddingRight" | "paddingTop" | "paddingBottom"
  // Margin
  | "margin" | "marginLeft" | "marginRight" | "marginTop" | "marginBottom";

/**
 * Type-safe property values for Layout.get().
 *
 * Maps property paths to their expected value types.
 */
export type LayoutPropertyValues = {
  [K in LayoutProperty]:
    K extends "order" ? number :
    K extends "position" ? Point<number> :
    K extends "x" | "y" ? number :
    K extends "size" ? Size<number> :
    K extends "width" | "height" ? number :
    K extends "contentSize" ? Size<number> :
    K extends "contentWidth" | "contentHeight" ? number :
    K extends "scrollbarSize" ? Size<number> :
    K extends "scrollbarWidth" | "scrollbarHeight" ? number :
    K extends "border" ? Rect<number> :
    K extends "borderLeft" | "borderRight" | "borderTop" | "borderBottom" ? number :
    K extends "padding" ? Rect<number> :
    K extends "paddingLeft" | "paddingRight" | "paddingTop" | "paddingBottom" ? number :
    K extends "margin" ? Rect<number> :
    K extends "marginLeft" | "marginRight" | "marginTop" | "marginBottom" ? number :
    unknown;
};

/**
 * Type-safe property values for batch setting.
 *
 * Maps property paths to their expected value types.
 */
export type StylePropertyValues = {
  [K in StyleProperty]?: 
    K extends "display" ? Display :
    K extends "position" ? Position :
    K extends "direction" ? Direction :
    K extends "float" ? Float :
    K extends "clear" ? Clear :
    K extends "boxSizing" ? BoxSizing :
    K extends "overflow" ? Point<Overflow> :
    K extends "overflowX" | "overflowY" ? Overflow :
    K extends "flexDirection" ? FlexDirection :
    K extends "flexWrap" ? FlexWrap :
    K extends "flexGrow" | "flexShrink" | "scrollbarWidth" ? number :
    K extends "flexBasis" ? Dimension :
    K extends "alignItems" | "justifyItems" ? AlignItems | undefined :
    K extends "alignSelf" | "justifySelf" ? AlignSelf | undefined :
    K extends "alignContent" ? AlignContent | undefined :
    K extends "justifyContent" ? JustifyContent | undefined :
    K extends "aspectRatio" ? number | undefined :
    K extends "size" | "minSize" | "maxSize" ? Size<Dimension> :
    K extends "width" | "height" | "minWidth" | "minHeight" | "maxWidth" | "maxHeight" ? Dimension :
    K extends "margin" | "inset" ? Rect<LengthPercentageAuto> :
    K extends "marginLeft" | "marginRight" | "marginTop" | "marginBottom" | "left" | "right" | "top" | "bottom" ? LengthPercentageAuto :
    K extends "padding" | "border" ? Rect<LengthPercentage> :
    K extends "paddingLeft" | "paddingRight" | "paddingTop" | "paddingBottom" | "borderLeft" | "borderRight" | "borderTop" | "borderBottom" ? LengthPercentage :
    K extends "gap" ? Size<LengthPercentage> :
    K extends "columnGap" | "rowGap" ? LengthPercentage :
    K extends "itemIsTable" | "itemIsReplaced" ? boolean :
    K extends "textAlign" ? TextAlign :
    K extends "gridAutoFlow" ? GridAutoFlow :
    K extends "gridRow" | "gridColumn" ? Line<GridPlacement> :
    K extends "gridRowStart" | "gridRowEnd" | "gridColumnStart" | "gridColumnEnd" ? GridPlacement :
    K extends "gridTemplateRows" | "gridTemplateColumns" ? GridTemplateComponent[] :
    K extends "gridAutoRows" | "gridAutoColumns" ? TrackSizingFunction[] :
    K extends "gridTemplateAreas" ? GridTemplateArea[] :
    K extends "gridTemplateAreaRowCount" | "gridTemplateAreaColumnCount" ? number :
    K extends "gridTemplateRowNames" | "gridTemplateColumnNames" ? string[][] :
    unknown;
};

// Module augmentation for stronger typing on Style class methods
declare module "./taffy_wasm" {
  interface Style {
    /**
     * Reads multiple style properties in a single WASM call.
     * Supports both object properties and individual flat properties.
     *
     * @returns A single value for one key, values in key order for multiple keys,
     * or `undefined` when no keys are supplied
     *
     * @throws Error if any property key is unknown.
     *
     * @remarks
     * - Multiple literal keys produce a typed tuple for destructuring
     * - A dynamic array of keys produces an array of the corresponding value types
     * - `get("alignSelf")` and `get("justifySelf")` return `undefined` when unset
     *   or assigned `AlignSelf.Auto`; direct property reads return `AlignSelf.Auto`
     *
     * @example
     * ```typescript
     * const style = new Style();
     * style.display = Display.Flex;
     *
     * // Single property - returns exact type (includes undefined for optional properties)
     * const display = style.get("display"); // Display | undefined
     *
     * // Individual flat property - returns exact type
     * const width = style.get("width"); // Dimension | undefined
     *
     * // Optional properties return undefined when not set
     * const alignItems = style.get("alignItems"); // AlignItems | undefined
     *
     * // Two properties - returns tuple for destructuring
     * const [d, w] = style.get("display", "width"); // [Display | undefined, Dimension | undefined]
     *
     * // Three properties - returns tuple for destructuring
     * const [d2, w2, f] = style.get("display", "width", "flexGrow");
     *
     * // Four literal keys also return a typed tuple
     * const values = style.get("display", "width", "flexGrow", "flexShrink");
     * // values: [Display | undefined, Dimension | undefined, number | undefined, number | undefined]
     * ```
     */
    get(): undefined;
    get<K extends StyleProperty>(...keys: [K]): StylePropertyValues[K];
    get<K1 extends StyleProperty, K2 extends StyleProperty>(
      ...keys: [K1, K2]
    ): [StylePropertyValues[K1], StylePropertyValues[K2]];
    get<K1 extends StyleProperty, K2 extends StyleProperty, K3 extends StyleProperty>(
      ...keys: [K1, K2, K3]
    ): [StylePropertyValues[K1], StylePropertyValues[K2], StylePropertyValues[K3]];
    get<Keys extends StyleProperty[]>(...keys: Keys): StylePropertyArrayValues<Keys>;

    /**
     * Sets multiple style properties in a single WASM call.
     * Supports both object properties and individual flat properties.
     *
     * @param props - Object mapping property keys to their values
     *
     * @remarks
     * Only accepts valid property keys with their corresponding value types.
     *
     * @throws Error if any property key is unknown.
     *
     * @example
     * ```typescript
     * const style = new Style();
     * style.set({
     *   display: Display.Flex,
     *   width: 200,
     *   marginLeft: 10,
     *   marginRight: "auto"
     * });
     * ```
     */
    set(props: StylePropertyValues): void;
  }

  interface Layout {
    /**
     * Reads multiple layout properties in a single WASM call.
     * Supports both object properties and individual flat properties.
     *
     * @returns A single value for one key, values in key order for multiple keys,
     * or `undefined` when no keys are supplied
     *
     * @throws Error if any property key is unknown.
     *
     * @remarks
     * - Single property: returns exact value type
     * - Multiple literal keys: returns a typed tuple for destructuring
     * - A dynamic array of keys: returns an array of the corresponding value types
     *
     * @example
     * ```typescript
     * import { TaffyTree, Style } from "taffy-layout";
     *
     * const tree = new TaffyTree();
     * const root = tree.newLeaf(new Style());
     * tree.computeLayout(root, { width: 100, height: 100 });
     * const layout = tree.getLayout(root);
     *
     * // Single property - returns exact type
     * const width = layout.get("width"); // number
     *
     * // Two properties - returns tuple for destructuring
     * const [pos, size] = layout.get("position", "size");
     * // pos: Point<number>, size: Size<number>
     *
     * // Three properties - returns tuple for destructuring
     * const [x, y, w] = layout.get("x", "y", "width");
     *
     * // Four literal keys also return a typed tuple
     * const values = layout.get("x", "y", "width", "height");
     * // values type is: [number, number, number, number]
     *
     * tree.free();
     * ```
     */
    get(): undefined;
    get<K extends LayoutProperty>(...keys: [K]): LayoutPropertyValues[K];
    get<K1 extends LayoutProperty, K2 extends LayoutProperty>(
      ...keys: [K1, K2]
    ): [LayoutPropertyValues[K1], LayoutPropertyValues[K2]];
    get<K1 extends LayoutProperty, K2 extends LayoutProperty, K3 extends LayoutProperty>(
      ...keys: [K1, K2, K3]
    ): [LayoutPropertyValues[K1], LayoutPropertyValues[K2], LayoutPropertyValues[K3]];
    get<Keys extends LayoutProperty[]>(...keys: Keys): LayoutPropertyArrayValues<Keys>;
  }
}

/**
 * Maps property keys to value types, preserving tuple or array structure.
 */
type StylePropertyArrayValues<Keys extends StyleProperty[]> = {
  [K in keyof Keys]: Keys[K] extends StyleProperty ? StylePropertyValues[Keys[K]] : unknown;
};

/**
 * Helper type to convert an array of layout property keys to an array of their value types.
 */
type LayoutPropertyArrayValues<Keys extends LayoutProperty[]> = {
  [K in keyof Keys]: Keys[K] extends LayoutProperty ? LayoutPropertyValues[Keys[K]] : unknown;
};
"#;
