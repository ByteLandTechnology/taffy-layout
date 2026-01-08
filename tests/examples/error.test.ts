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

test("error example 1", async () => {
  try {
    const tree = new TaffyTree();
    const node = tree.newLeaf(new Style());
    tree.remove(node);
  } catch (e) {
    if (e instanceof TaffyError) {
      console.error(e.message);
    }
  }
});

test("error example 2", async () => {
  const tree = new TaffyTree();
  const style = new Style();
  const nodeId = tree.newLeaf(style); // Returns bigint or throws TaffyError
});

test("error example 3", async () => {
  const tree = new TaffyTree();
  const nodeId = tree.newLeaf(new Style());
  const style = new Style();
  tree.setStyle(nodeId, style); // Returns void or throws TaffyError
});
