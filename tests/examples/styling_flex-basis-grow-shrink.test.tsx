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

test("styling_flex-basis-grow-shrink example 1", async () => {
  const tree = new TaffyTree();

  const fixedStyle = new Style();
  fixedStyle.size = { width: 120, height: "100%" };
  const child1 = tree.newLeaf(fixedStyle);

  const growStyle = new Style();
  growStyle.flexGrow = 1;
  growStyle.size = { width: "auto", height: "100%" };
  const child2 = tree.newLeaf(growStyle);

  const rootStyle = new Style();
  rootStyle.display = Display.Flex;
  rootStyle.flexDirection = FlexDirection.Row;
  rootStyle.size = { width: 240, height: 60 };
  rootStyle.gap = { width: 8, height: 0 };
  rootStyle.padding = { left: 6, right: 6, top: 6, bottom: 6 };

  const root = tree.newWithChildren(rootStyle, [child1, child2]);

  tree.computeLayout(root, {
    width: 240,
    height: 60,
  });

  console.log(`Fixed width: 120, grow: 1`);

  return <TaffyTreePreview tree={tree} root={root} />;
});
