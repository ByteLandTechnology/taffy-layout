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
  tree.computeLayout(rootNode, { width: 800, height: 600 });

  // Get layout for a specific node
  const layout = tree.getLayout(rootNode);

  // Access layout properties
  console.log(`Position: (${layout.x}, ${layout.y})`);
  console.log(`Size: ${layout.width} x ${layout.height}`);
  console.log(`Padding: top=${layout.paddingTop}, left=${layout.paddingLeft}`);

  tree.free();
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

test("layout example 3", async () => {
  const tree = new TaffyTree();
  const root = tree.newLeaf(new Style());
  tree.computeLayout(root, { width: 100, height: 100 });
  const layout = tree.getLayout(root);
  const pos = layout.position;
  console.log(`Position: (${pos.x}, ${pos.y})`);
  tree.free();
});

test("layout example 4", async () => {
  const tree = new TaffyTree();
  const root = tree.newLeaf(new Style());
  tree.computeLayout(root, { width: 100, height: 100 });
  const layout = tree.getLayout(root);
  const size = layout.size;
  console.log(`Size: ${size.width} x ${size.height}`);
  tree.free();
});

test("layout example 5", async () => {
  const tree = new TaffyTree();
  const root = tree.newLeaf(new Style());
  tree.computeLayout(root, { width: 100, height: 100 });
  const layout = tree.getLayout(root);
  const contentSize = layout.contentSize;
  console.log(`Content: ${contentSize.width} x ${contentSize.height}`);
  tree.free();
});

test("layout example 6", async () => {
  const tree = new TaffyTree();
  const root = tree.newLeaf(new Style());
  tree.computeLayout(root, { width: 100, height: 100 });
  const layout = tree.getLayout(root);
  const scrollbarSize = layout.scrollbarSize;
  console.log(`Scrollbar: ${scrollbarSize.width} x ${scrollbarSize.height}`);
  tree.free();
});

test("layout example 7", async () => {
  const tree = new TaffyTree();
  const root = tree.newLeaf(new Style());
  tree.computeLayout(root, { width: 100, height: 100 });
  const layout = tree.getLayout(root);
  const border = layout.border;
  console.log(
    `Border: ${border.left} ${border.right} ${border.top} ${border.bottom}`,
  );
  tree.free();
});

test("layout example 8", async () => {
  const tree = new TaffyTree();
  const root = tree.newLeaf(new Style());
  tree.computeLayout(root, { width: 100, height: 100 });
  const layout = tree.getLayout(root);
  const padding = layout.padding;
  console.log(
    `Padding: ${padding.left} ${padding.right} ${padding.top} ${padding.bottom}`,
  );
  tree.free();
});

test("layout example 9", async () => {
  const tree = new TaffyTree();
  const root = tree.newLeaf(new Style());
  tree.computeLayout(root, { width: 100, height: 100 });
  const layout = tree.getLayout(root);
  const margin = layout.margin;
  console.log(
    `Margin: ${margin.left} ${margin.right} ${margin.top} ${margin.bottom}`,
  );
  tree.free();
});

test("layout example 10", async () => {
  const tree = new TaffyTree();
  const root = tree.newLeaf(new Style());
  tree.computeLayout(root, { width: 800, height: 600 });
  const layout = tree.getLayout(root);

  // Read single property
  const width = layout.get("width");

  // Read compound property
  const pos = layout.get("position");

  // Read multiple properties with destructuring
  const [position, size] = layout.get("position", "size");

  tree.free();
});
