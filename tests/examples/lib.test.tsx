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

test("lib example 1", async () => {
  const tree = new TaffyTree();
  const style = new Style();
  style.display = Display.Flex;
  style.flexDirection = FlexDirection.Column;

  const root = tree.newLeaf(style);
  tree.computeLayout(root, { width: "max-content", height: "max-content" });

  const layout = tree.getLayout(root);
  console.log(`Width: ${layout.width}, Height: ${layout.height}`);
});
