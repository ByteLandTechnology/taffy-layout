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

test("styling_flex-wrap example 1", async () => {
  const tree = new TaffyTree();

  const style = new Style({
    size: { width: 60, height: 40 },
    margin: { bottom: 5, right: 5 },
  });

  // Create many children to force wrapping
  const children = Array.from({ length: 8 }).map(() => tree.newLeaf(style));

  const rootStyle = new Style({
    display: Display.Flex,
    flexDirection: FlexDirection.Row,
    // CHANGE THIS: NoWrap, Wrap, WrapReverse
    flexWrap: FlexWrap.Wrap,
    size: { width: 200, height: 200 },
    padding: { left: 10, right: 10, top: 10, bottom: 10 },
  });

  const root = tree.newWithChildren(rootStyle, children);

  tree.computeLayout(root, { width: 200, height: 200 });

  return <TaffyTreePreview tree={tree} root={root} />;
});
