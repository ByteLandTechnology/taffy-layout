import React from "react";
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

// Mock TaffyTreePreview component
const TaffyTreePreview = (_props: any) => null;

test("getting-started_configuration example 1", async () => {
  // Initialize with capacity for 1,000 nodes
  const tree = TaffyTree.withCapacity(1000);
  console.log(`Initial Node Capacity: ${tree.totalNodeCount()}`); // 0 actual nodes

  const style = new Style({
    display: Display.Flex,
    size: { width: 200, height: 40 },
    alignItems: AlignItems.Center,
    justifyContent: JustifyContent.Center,
  });

  const root = tree.newLeaf(style);
  tree.computeLayout(root, { width: 200, height: 40 });

  return <TaffyTreePreview tree={tree} root={root} />;
});

test("getting-started_configuration example 2", async () => {
  const tree = new TaffyTree();

  // Create two items that would sum to 101px
  // 50.5 + 50.5 = 101
  const style = new Style({
    size: { width: 50.5, height: 50 },
    display: Display.Flex,
    justifyContent: JustifyContent.Center,
    alignItems: AlignItems.Center,
  });
  const child1 = tree.newLeaf(style);
  const child2 = tree.newLeaf(style);

  const rootStyle = new Style({
    display: Display.Flex,
    flexDirection: FlexDirection.Row,
    size: { width: 150, height: 60 },
    alignItems: AlignItems.Center,
  });
  const root = tree.newWithChildren(rootStyle, [child1, child2]);

  // 1. Default (Rounding Enabled)
  tree.computeLayout(root, { width: 150, height: 60 });
  let layout1 = tree.getLayout(child1);
  // layout1.width might be rounded to 51 or 50 depending on algorithm

  // 2. Disable Rounding
  tree.disableRounding();
  tree.computeLayout(root, { width: 150, height: 60 });
  layout1 = tree.getLayout(child1);
  // layout1.width will be exactly 50.5

  console.log(`Precise Width: ${layout1.width}`);

  return <TaffyTreePreview tree={tree} root={root} />;
});

test("getting-started_configuration example 3", async () => {
  const tree = new TaffyTree();

  // ... use tree ...

  // Option 1: Reuse the tree (Recommended)
  // Clears all nodes but keeps memory allocated
  tree.clear();

  // Option 2: Free completely
  tree.free();
});
