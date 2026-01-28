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

test("styling_justify-content example 1", async () => {
  const tree = new TaffyTree();

  const style = new Style({
    size: { width: 50, height: 50 },
  });

  const child1 = tree.newLeaf(style);
  const child2 = tree.newLeaf(style);
  const child3 = tree.newLeaf(style);

  const rootStyle = new Style({
    display: Display.Flex,
    flexDirection: FlexDirection.Row,
    // CHANGE THIS to test different values
    justifyContent: JustifyContent.SpaceBetween,
    size: { width: 300, height: 80 },
    padding: { left: 10, right: 10, top: 10, bottom: 10 },
  });

  const root = tree.newWithChildren(rootStyle, [child1, child2, child3]);

  tree.computeLayout(root, { width: 300, height: 80 });

  return <TaffyTreePreview tree={tree} root={root} />;
});
