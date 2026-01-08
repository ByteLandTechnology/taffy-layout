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

test("typescript example 1", async () => {
  const tree = new TaffyTree();
  const root: bigint = tree.newLeaf(new Style());

  // Fixed size container with type annotation
  const fixedSpace: Size<AvailableSpace> = {
    width: 800,
    height: 600,
  };
  tree.computeLayout(root, fixedSpace);

  // Flexible width, fixed height
  const flexibleSpace: Size<AvailableSpace> = {
    width: "max-content",
    height: 400,
  };
  tree.computeLayout(root, flexibleSpace);
});

test("typescript example 2", async () => {
  // Size with explicit type parameters
  const pixelSize: Size<number> = { width: 200, height: 100 };

  const dimensionSize: Size<Dimension> = {
    width: 200,
    height: "50%",
  };

  const availableSize: Size<AvailableSpace> = {
    width: 800,
    height: "max-content",
  };
});

test("typescript example 3", async () => {
  interface TextContext {
    text: string;
    fontSize: number;
  }

  const tree = new TaffyTree();

  const style = new Style();
  const context: TextContext = { text: "Hello, World!", fontSize: 16 };
  const textNode: bigint = tree.newLeafWithContext(style, context);

  // Helper function to measure text width
  const measureTextWidth = (text: string, fontSize: number) =>
    text.length * fontSize * 0.6;

  // Typed measure function
  const measureText: MeasureFunction = (
    knownDimensions,
    availableSpace,
    node,
    context,
    style,
  ): Size<number> => {
    const ctx = context as TextContext | undefined;
    if (!ctx?.text) return { width: 0, height: 0 };

    const width =
      knownDimensions.width ?? measureTextWidth(ctx.text, ctx.fontSize);
    const height = knownDimensions.height ?? ctx.fontSize * 1.2;

    return { width, height };
  };

  tree.computeLayoutWithMeasure(
    textNode,
    { width: 200, height: "max-content" },
    measureText,
  );
});

test("typescript example 4", async () => {
  const style = new Style();

  // With explicit type annotations
  const fixedSize: Size<Dimension> = {
    width: 200,
    height: 100,
  };

  const percentSize: Size<Dimension> = {
    width: "50%",
    height: "100%",
  };

  const autoSize: Size<Dimension> = {
    width: "auto",
    height: "auto",
  };

  style.size = fixedSize;
});

test("typescript example 5", async () => {
  const style = new Style();

  const padding: Rect<LengthPercentage> = {
    left: 10,
    right: 10,
    top: 5,
    bottom: 5,
  };

  const gap: Size<LengthPercentage> = {
    width: "5%",
    height: "5%",
  };

  style.padding = padding;
  style.gap = gap;
});

test("typescript example 6", async () => {
  const style = new Style();

  // Auto margins for horizontal centering
  const centerMargin: Rect<LengthPercentageAuto> = {
    left: "auto",
    right: "auto",
    top: 0,
    bottom: 0,
  };

  style.margin = centerMargin;
});

test("typescript example 7", async () => {
  const style = new Style();

  const overflow: Point<(typeof Overflow)[keyof typeof Overflow]> = {
    x: Overflow.Hidden,
    y: Overflow.Scroll,
  };

  style.overflow = overflow;
});

test("typescript example 8", async () => {
  const style = new Style();

  // Typed padding
  const padding: Rect<LengthPercentage> = {
    left: 10,
    right: 10,
    top: 10,
    bottom: 10,
  };

  // Typed margin with auto
  const margin: Rect<LengthPercentageAuto> = {
    left: "auto",
    right: "auto",
    top: 10,
    bottom: 30,
  };

  style.padding = padding;
  style.margin = margin;
});

test("typescript example 9", async () => {
  const tree = new TaffyTree();
  const style = new Style();
  style.display = Display.Grid;
  const gridNode = tree.newLeaf(style);
  tree.computeLayout(gridNode, { width: 100, height: 100 });

  const info: DetailedLayoutInfo = tree.detailedLayoutInfo(gridNode);

  if (info && typeof info === "object" && "Grid" in info) {
    const grid = info.Grid as DetailedGridInfo;
    console.log("Rows:", grid.rows.sizes);
    console.log("Columns:", grid.columns.sizes);
  }
});

test("typescript example 10", async () => {
  // Line index (CSS: grid-row-start: 2)
  const lineIndex: GridPlacement = 2;

  // Auto placement (CSS: grid-row-start: auto)
  const auto: GridPlacement = "auto";

  // Span (CSS: grid-row-start: span 3)
  const span: GridPlacement = { span: 3 };

  // Named line (CSS: grid-row-start: header 2)
  const named: GridPlacement = { line: 2, ident: "header" };

  // Named span (CSS: grid-row-start: span 2 header)
  const namedSpan: GridPlacement = { span: 2, ident: "header" };
});

test("typescript example 11", async () => {
  const style = new Style();
  style.display = Display.Grid;

  // CSS: grid-row: 1 / 3
  style.gridRow = { start: 1, end: 3 };

  // CSS: grid-column: 1 / span 2
  style.gridColumn = { start: 1, end: { span: 2 } };

  // CSS: grid-row: auto / auto
  style.gridRow = { start: "auto", end: "auto" };
});
