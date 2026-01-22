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

test("style example 1", async () => {
  const style = new Style();

  // Configure with type-safe enums
  style.display = Display.Flex;
  style.flexDirection = FlexDirection.Column;
  style.alignItems = AlignItems.Center;

  // Set dimensions with explicit types
  const size: Size<Dimension> = {
    width: 200,
    height: 100,
  };
  style.size = size;

  const padding: Rect<LengthPercentage> = {
    left: 10,
    right: 10,
    top: 5,
    bottom: 5,
  };
  style.padding = padding;

  const tree = new TaffyTree();
  const nodeId: bigint = tree.newLeaf(style);
});

test("style example 2", async () => {
  const style = new Style({
    display: Display.Flex,
    flexDirection: FlexDirection.Column,
    width: 200,
    marginLeft: 10,
  });
});

test("style example 3", async () => {
  const style = new Style();

  // Set multiple properties at once
  style.set({
    display: Display.Flex,
    flexDirection: FlexDirection.Column,
    width: 200,
    marginLeft: 10,
  });

  // Get a single property
  const d = style.get("display");

  // Get multiple properties with destructuring
  const [display, flexDirection, width] = style.get(
    "display",
    "flexDirection",
    "width",
  );
});

test("style example 4", async () => {
  // Create with defaults
  const style = new Style();
  console.log(style.display); // Display.Block

  // Create with initial properties
  const style2 = new Style({
    display: Display.Flex,
    flexDirection: FlexDirection.Column,
    width: 200,
    marginLeft: 10,
  });
});

test("style example 5", async () => {
  const style = new Style();
  style.display = Display.Flex;
});

test("style example 6", async () => {
  const style = new Style();
  style.position = Position.Absolute;
  style.inset = { left: 10, top: 10, right: "auto", bottom: "auto" };
});

test("style example 7", async () => {
  const style = new Style();
  style.flexDirection = FlexDirection.Column;
});

test("style example 8", async () => {
  const style = new Style();
  style.flexWrap = FlexWrap.Wrap;
});

test("style example 9", async () => {
  const style = new Style();
  style.flexGrow = 2;
});

test("style example 10", async () => {
  const style = new Style();
  style.flexShrink = 2;
});

test("style example 11", async () => {
  const style = new Style();
  style.alignItems = AlignItems.Center;
});

test("style example 12", async () => {
  const style = new Style();
  style.alignSelf = AlignSelf.Stretch;
});

test("style example 13", async () => {
  const style = new Style();
  style.alignContent = AlignContent.SpaceBetween;
});

test("style example 14", async () => {
  const style = new Style();
  style.justifyContent = JustifyContent.Center;
});

test("style example 15", async () => {
  const style = new Style();
  style.aspectRatio = 16 / 9;
});

test("style example 16", async () => {
  const style = new Style();
  style.overflow = { x: Overflow.Hidden, y: Overflow.Scroll };
});

test("style example 17", async () => {
  const style = new Style();
  style.overflowX = Overflow.Hidden;
});

test("style example 18", async () => {
  const style = new Style();
  style.overflowY = Overflow.Scroll;
});

test("style example 19", async () => {
  const style = new Style();
  style.boxSizing = BoxSizing.ContentBox;
});

test("style example 20", async () => {
  const style = new Style();
  style.flexBasis = 100;
});

test("style example 21", async () => {
  const style = new Style();
  style.size = { width: 200, height: "100%" };
});

test("style example 22", async () => {
  const style = new Style();
  style.minSize = { width: 100, height: "auto" };
});

test("style example 23", async () => {
  const style = new Style();
  style.maxSize = { width: "auto", height: 500 };
});

test("style example 24", async () => {
  const style = new Style();
  style.width = 200;
  style.width = "50%";
  style.width = "auto";
});

test("style example 25", async () => {
  const style = new Style();
  style.height = 100;
  style.height = "75%";
  style.height = "auto";
});

test("style example 26", async () => {
  const style = new Style();
  style.minWidth = 100;
  style.minWidth = "auto";
});

test("style example 27", async () => {
  const style = new Style();
  style.minHeight = 50;
  style.minHeight = "auto";
});

test("style example 28", async () => {
  const style = new Style();
  style.maxWidth = 500;
  style.maxWidth = "auto";
});

test("style example 29", async () => {
  const style = new Style();
  style.maxHeight = 300;
  style.maxHeight = "auto";
});

test("style example 30", async () => {
  const style = new Style();
  style.margin = { left: 10, right: 10, top: 5, bottom: 5 };
});

test("style example 31", async () => {
  const style = new Style();
  style.marginLeft = 10;
  style.marginLeft = "5%";
  style.marginLeft = "auto";
});

test("style example 32", async () => {
  const style = new Style();
  style.marginRight = 10;
  style.marginRight = "5%";
  style.marginRight = "auto";
});

test("style example 33", async () => {
  const style = new Style();
  style.marginTop = 5;
  style.marginTop = "10%";
  style.marginTop = "auto";
});

test("style example 34", async () => {
  const style = new Style();
  style.marginBottom = 5;
  style.marginBottom = "10%";
  style.marginBottom = "auto";
});

test("style example 35", async () => {
  const style = new Style();
  style.padding = { left: 20, right: 20, top: 10, bottom: 10 };
});

test("style example 36", async () => {
  const style = new Style();
  style.paddingLeft = 20;
  style.paddingLeft = "10%";
});

test("style example 37", async () => {
  const style = new Style();
  style.paddingRight = 20;
  style.paddingRight = "10%";
});

test("style example 38", async () => {
  const style = new Style();
  style.paddingTop = 10;
  style.paddingTop = "5%";
});

test("style example 39", async () => {
  const style = new Style();
  style.paddingBottom = 10;
  style.paddingBottom = "5%";
});

test("style example 40", async () => {
  const style = new Style();
  style.border = { left: 1, right: 1, top: 1, bottom: 1 };
});

test("style example 41", async () => {
  const style = new Style();
  style.borderLeft = 1;
  style.borderLeft = "2%";
});

test("style example 42", async () => {
  const style = new Style();
  style.borderRight = 1;
  style.borderRight = "2%";
});

test("style example 43", async () => {
  const style = new Style();
  style.borderTop = 1;
  style.borderTop = "2%";
});

test("style example 44", async () => {
  const style = new Style();
  style.borderBottom = 1;
  style.borderBottom = "2%";
});

test("style example 45", async () => {
  const style = new Style();
  style.gap = { width: 10, height: 10 };
});

test("style example 46", async () => {
  const style = new Style();
  style.columnGap = 10;
  style.columnGap = "5%";
});

test("style example 47", async () => {
  const style = new Style();
  style.rowGap = 10;
  style.rowGap = "5%";
});

test("style example 48", async () => {
  const style = new Style();
  style.position = Position.Absolute;
  style.inset = { left: 0, top: 0, right: "auto", bottom: "auto" };
});

test("style example 49", async () => {
  const style = new Style();
  style.position = Position.Absolute;
  style.left = 0;
  style.left = "10%";
  style.left = "auto";
});

test("style example 50", async () => {
  const style = new Style();
  style.position = Position.Absolute;
  style.right = 0;
  style.right = "10%";
  style.right = "auto";
});

test("style example 51", async () => {
  const style = new Style();
  style.position = Position.Absolute;
  style.top = 0;
  style.top = "10%";
  style.top = "auto";
});

test("style example 52", async () => {
  const style = new Style();
  style.position = Position.Absolute;
  style.bottom = 0;
  style.bottom = "10%";
  style.bottom = "auto";
});

test("style example 53", async () => {
  const style = new Style();
  style.overflow = { x: Overflow.Scroll, y: Overflow.Scroll };
  style.scrollbarWidth = 15;
});

test("style example 54", async () => {
  const style = new Style();
  style.textAlign = TextAlign.LegacyCenter;
});

test("style example 55", async () => {
  const style = new Style();
  style.display = Display.Grid;
  style.justifyItems = AlignItems.Center;
});

test("style example 56", async () => {
  const style = new Style();
  style.justifySelf = AlignSelf.End;
});

test("style example 57", async () => {
  const style = new Style();
  style.display = Display.Grid;
  style.gridAutoFlow = GridAutoFlow.Column;
});

test("style example 58", async () => {
  const style = new Style();
  style.display = Display.Grid;
  // CSS: grid-row: 1 / 3
  style.gridRow = { start: 1, end: 3 };
  // CSS: grid-row: 2 / span 2
  style.gridRow = { start: 2, end: { span: 2 } };
});

test("style example 59", async () => {
  const style = new Style();
  style.display = Display.Grid;
  // CSS: grid-column: 1 / 4
  style.gridColumn = { start: 1, end: 4 };
  // CSS: grid-column: auto / span 3
  style.gridColumn = { start: "auto", end: { span: 3 } };
});

test("style example 60", async () => {
  const style = new Style();
  style.display = Display.Grid;
  style.gridRowStart = 1;
  style.gridRowStart = "auto";
  style.gridRowStart = { span: 2 };
});

test("style example 61", async () => {
  const style = new Style();
  style.display = Display.Grid;
  style.gridRowEnd = 3;
  style.gridRowEnd = "auto";
  style.gridRowEnd = { span: 2 };
});

test("style example 62", async () => {
  const style = new Style();
  style.display = Display.Grid;
  style.gridColumnStart = 1;
  style.gridColumnStart = "auto";
  style.gridColumnStart = { span: 2 };
});

test("style example 63", async () => {
  const style = new Style();
  style.display = Display.Grid;
  style.gridColumnEnd = 4;
  style.gridColumnEnd = "auto";
  style.gridColumnEnd = { span: 3 };
});

test("style example 64", async () => {
  const style = new Style();
  style.display = Display.Grid;
  style.gridTemplateColumns = [
    { min: 200, max: 200 },
    { min: "auto", max: "1fr" },
    { min: "auto", max: "1fr" },
  ];
});

test("style example 65", async () => {
  const style = new Style();
  style.display = Display.Grid;
  style.gridAutoRows = [{ min: "auto", max: "auto" }];
});

test("style example 66", async () => {
  const style = new Style();
  style.display = Display.Grid;
  style.gridTemplateAreas = [
    { name: "header", rowStart: 1, rowEnd: 2, columnStart: 1, columnEnd: 4 },
    { name: "main", rowStart: 2, rowEnd: 4, columnStart: 2, columnEnd: 4 },
  ];
});

test("style example 67", async () => {
  const style = new Style();
  style.gridTemplateRowNames = [
    ["header-start"],
    ["header-end", "main-start"],
    ["main-end"],
  ];
});

test("style example 68", async () => {
  const style = new Style();
  style.gridTemplateColumnNames = [
    ["sidebar-start"],
    ["sidebar-end", "main-start"],
    ["main-end"],
  ];
});

test("style example 69", async () => {
  const style = new Style();
  style.display = Display.Flex;
  style.size = { width: 100, height: "50%" };

  // Read single property
  const d = style.get("display");

  // Read nested property
  const w = style.get("width");

  // Read multiple properties with destructuring
  const [display, width, margin] = style.get("display", "width", "marginLeft");
});

test("style example 70", async () => {
  const style = new Style();

  // Set multiple properties at once
  style.set({
    display: Display.Flex,
    flexDirection: FlexDirection.Column,
    width: 200,
    height: "50%",
    marginLeft: 10,
    marginRight: "auto",
  });
});
