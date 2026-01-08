import { test } from "vitest";
import init, {
  TaffyTree,
  Style,
  // Add all other exports that might be needed
  Display,
  FlexDirection,
  AlignItems,
  AlignContent,
  JustifyContent,
  Position,
  FlexWrap,
  BoxSizing,
  GridAutoFlow,
  Overflow,
  AlignSelf,
  TextAlign,
  Dimension,
  AvailableSpace,
  Size,
  GridPlacement,
  Rect,
  LengthPercentage,
  LengthPercentageAuto,
  DetailedLayoutInfo,
  DetailedGridInfo,
  DetailedGridTracksInfo,
  DetailedGridItemsInfo,
  TrackSizingFunction,
  Point,
  TaffyError,
  Layout,
  MeasureFunction,
} from "taffy-layout";

// Global init for the suite
await init();

test("enums example 1", async () => {
  const style = new Style();
  style.display = Display.Flex; // Enable flexbox layout
  style.display = Display.Grid; // Enable grid layout
  style.display = Display.None; // Hide element from layout
});

test("enums example 2", async () => {
  const style = new Style();
  style.position = Position.Relative; // Normal document flow
  style.position = Position.Absolute; // Removed from flow, uses inset values
});

test("enums example 3", async () => {
  const style = new Style();
  style.flexDirection = FlexDirection.Row; // Horizontal, left to right
  style.flexDirection = FlexDirection.Column; // Vertical, top to bottom
});

test("enums example 4", async () => {
  const style = new Style();
  style.flexWrap = FlexWrap.NoWrap; // All items on single line
  style.flexWrap = FlexWrap.Wrap; // Items wrap to new lines
});

test("enums example 5", async () => {
  const style = new Style();
  style.alignItems = AlignItems.Center; // Center items on cross axis
  style.alignItems = AlignItems.Stretch; // Stretch items to fill container
});

test("enums example 6", async () => {
  const style = new Style();
  style.alignSelf = AlignSelf.Auto; // Use parent's align-items
  style.alignSelf = AlignSelf.Center; // Override to center this item
});

test("enums example 7", async () => {
  const style = new Style();
  style.flexWrap = FlexWrap.Wrap;
  style.alignContent = AlignContent.SpaceBetween; // Distribute lines evenly
});

test("enums example 8", async () => {
  const style = new Style();
  style.justifyContent = JustifyContent.Center; // Center items
  style.justifyContent = JustifyContent.SpaceBetween; // Distribute evenly
});

test("enums example 9", async () => {
  const style = new Style();
  style.overflow = { x: Overflow.Hidden, y: Overflow.Scroll };
});

test("enums example 10", async () => {
  const style = new Style();
  style.boxSizing = BoxSizing.BorderBox; // Size includes padding and border
  style.boxSizing = BoxSizing.ContentBox; // Size is content only
});

test("enums example 11", async () => {
  const style = new Style();
  style.textAlign = TextAlign.LegacyCenter; // Center block children
});

test("enums example 12", async () => {
  const style = new Style();
  style.gridAutoFlow = GridAutoFlow.Row; // Fill rows first
  style.gridAutoFlow = GridAutoFlow.Column; // Fill columns first
  style.gridAutoFlow = GridAutoFlow.RowDense; // Fill rows, pack densely
});
