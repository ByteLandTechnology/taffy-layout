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

test("styling_margin-padding-border example 1", async () => {
  const tree = new TaffyTree();

  // Inner content
  const contentNode = tree.newLeaf(
    new Style({
      flexGrow: 1, // Fill available space
    }),
  );

  // Container demonstrating box model
  const boxNode = tree.newWithChildren(
    new Style({
      size: { width: 200, height: 120 },
      display: Display.Flex,

      // 1. Margin (Outside)
      margin: { left: 20, top: 20 },

      // 2. Border (Boundary) - Logical width only
      border: { left: 5, right: 5, top: 5, bottom: 5 },

      // 3. Padding (Inside)
      padding: { left: 15, right: 15, top: 15, bottom: 15 },
    }),
    [contentNode],
  );

  // Root to hold the example
  const root = tree.newWithChildren(
    new Style({
      size: { width: 300, height: 200 },
    }),
    [boxNode],
  );

  tree.computeLayout(root, { width: 300, height: 200 });

  // Visualize the hierarchy
  console.log(tree.printTree(root));

  return <TaffyTreePreview tree={tree} root={root} />;
});
