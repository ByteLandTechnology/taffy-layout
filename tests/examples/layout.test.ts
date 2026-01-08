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

test("layout example 1", async () => {
  // Compute layout first
  const tree = new TaffyTree();
  const rootNode = tree.newLeaf(new Style());
  const nodeId = rootNode;
  tree.computeLayout(rootNode, { width: 800, height: 600 });

  // Get layout for a specific node
  const layout = tree.getLayout(nodeId);

  // Access layout properties
  console.log(`Position: (${layout.x}, ${layout.y})`);
  console.log(`Size: ${layout.width} x ${layout.height}`);
  console.log(`Padding: top=${layout.paddingTop}, left=${layout.paddingLeft}`);
});

test("layout example 2", async () => {
  const tree = new TaffyTree();
  const rootId = tree.newLeaf(new Style());
  const node = rootId;

  tree.computeLayout(rootId, { width: 800, height: 600 });
  const layout = tree.getLayout(node);

  console.log("Position:", layout.x, layout.y);
  console.log("Size:", layout.width, layout.height);
  console.log("Content:", layout.contentWidth, layout.contentHeight);
  console.log(
    "Padding:",
    layout.paddingTop,
    layout.paddingRight,
    layout.paddingBottom,
    layout.paddingLeft,
  );
  console.log(
    "Border:",
    layout.borderTop,
    layout.borderRight,
    layout.borderBottom,
    layout.borderLeft,
  );
  console.log(
    "Margin:",
    layout.marginTop,
    layout.marginRight,
    layout.marginBottom,
    layout.marginLeft,
  );
  console.log("Scrollbar:", layout.scrollbarWidth, layout.scrollbarHeight);
  console.log("Order:", layout.order);
});
