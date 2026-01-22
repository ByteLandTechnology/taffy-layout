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

test("readme example 1", async () => {
  // Initialize WebAssembly module

  // Create a layout tree
  const tree = new TaffyTree();

  // Create container style
  const containerStyle = new Style();
  containerStyle.display = Display.Flex;
  containerStyle.flexDirection = FlexDirection.Column;
  containerStyle.alignItems = AlignItems.Center;

  // You can set size as an object
  containerStyle.size = { width: 300, height: 200 };

  // Or use individual width/height properties
  containerStyle.width = 300;
  containerStyle.height = 200;

  // Set padding as an object
  containerStyle.padding = { left: 10, right: 10, top: 10, bottom: 10 };

  // Or use individual padding properties
  containerStyle.paddingLeft = 10;
  containerStyle.paddingRight = 10;
  containerStyle.paddingTop = 10;
  containerStyle.paddingBottom = 10;

  // Create child styles
  const childStyle = new Style();
  childStyle.flexGrow = 1;
  childStyle.width = "100%";
  childStyle.height = "auto";

  // Create nodes
  const child1 = tree.newLeaf(childStyle);
  const child2 = tree.newLeaf(childStyle);
  const container = tree.newWithChildren(
    containerStyle,
    BigUint64Array.from([child1, child2]),
  );

  // Compute layout
  tree.computeLayout(container, { width: 300, height: 200 });

  // Read computed layouts
  const containerLayout = tree.getLayout(container);
  const child1Layout = tree.getLayout(child1);
  const child2Layout = tree.getLayout(child2);

  console.log(`Container: ${containerLayout.width}x${containerLayout.height}`);
  console.log(
    `Child 1: ${child1Layout.width}x${child1Layout.height} at (${child1Layout.x}, ${child1Layout.y})`,
  );
  console.log(
    `Child 2: ${child2Layout.width}x${child2Layout.height} at (${child2Layout.x}, ${child2Layout.y})`,
  );
});

test("readme example 2", async () => {
  const tree = new TaffyTree();
  const textStyle = new Style();
  const rootNode = tree.newLeaf(new Style());
  const measureTextWidth = (text: string) => text.length * 8;
  const measureTextHeight = (text: string, width: number) => 20;

  const textNode = tree.newLeafWithContext(textStyle, {
    text: "Hello, World!",
  });

  tree.computeLayoutWithMeasure(
    rootNode,
    { width: 800, height: "max-content" },
    (known, available, node, context, style) => {
      if (context?.text) {
        // Your text measurement logic here
        const width = measureTextWidth(context.text);
        const height = measureTextHeight(
          context.text,
          available.width as number,
        );
        return { width, height };
      }
      return { width: 0, height: 0 };
    },
  );
});

test("readme example 3", async () => {
  try {
    const tree = new TaffyTree();
    const style = new Style();
    const nodeId = tree.newLeaf(style);
    console.log("Created node:", nodeId);
  } catch (e) {
    if (e instanceof TaffyError) {
      console.error("Error:", e.message);
    }
  }
});

test("readme example 4", async () => {
  const rowStyle = new Style();
  rowStyle.display = Display.Flex;
  rowStyle.flexDirection = FlexDirection.Row;
  rowStyle.justifyContent = JustifyContent.SpaceBetween;
  rowStyle.gap = { width: 10, height: 0 };
});

test("readme example 5", async () => {
  const gridStyle = new Style();
  gridStyle.display = Display.Grid;
  gridStyle.gridAutoFlow = GridAutoFlow.Row;
  gridStyle.gap = { width: 10, height: 10 };

  // Grid item placement
  const itemStyle = new Style();
  itemStyle.gridRow = { start: 1, end: 3 }; // Spans 2 rows
  itemStyle.gridColumn = { start: 1, end: { span: 2 } }; // Spans 2 columns
});

test("readme example 6", async () => {
  const gridStyle = new Style();
  gridStyle.display = Display.Grid;
  gridStyle.gridTemplateAreas = [
    { name: "header", rowStart: 1, rowEnd: 2, columnStart: 1, columnEnd: 4 },
    { name: "sidebar", rowStart: 2, rowEnd: 4, columnStart: 1, columnEnd: 2 },
    { name: "main", rowStart: 2, rowEnd: 4, columnStart: 2, columnEnd: 4 },
    { name: "footer", rowStart: 4, rowEnd: 5, columnStart: 1, columnEnd: 4 },
  ];

  // Named grid lines
  gridStyle.gridTemplateRowNames = [
    ["header-start"],
    ["header-end", "content-start"],
    ["content-end", "footer-start"],
    ["footer-end"],
  ];
});

test("readme example 7", async () => {
  const absoluteStyle = new Style();
  absoluteStyle.position = Position.Absolute;
  absoluteStyle.inset = { left: 10, top: 10, right: "auto", bottom: "auto" };
  absoluteStyle.size = { width: 100, height: 50 };
});

test("readme example 8", async () => {
  const percentStyle = new Style();
  percentStyle.size = {
    width: "50%", // 50% of parent
    height: "100%", // 100% of parent
  };
});

test("readme example 9", async () => {
  const imgStyle = new Style();
  imgStyle.itemIsReplaced = true;
  imgStyle.aspectRatio = 16 / 9; // 16:9 aspect ratio
  imgStyle.size = { width: "100%", height: "auto" };
});
