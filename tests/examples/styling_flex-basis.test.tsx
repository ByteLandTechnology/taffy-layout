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

test("styling_flex-basis example 1", async () => {
  const tree = new TaffyTree();

  const style1 = new Style();
  style1.flexBasis = 100;
  style1.size = { width: "auto", height: "100%" };
  const child1 = tree.newLeaf(style1);

  const style2 = new Style();
  style2.flexBasis = 200;
  style2.size = { width: "auto", height: "100%" };
  const child2 = tree.newLeaf(style2);

  const rootStyle = new Style();
  rootStyle.display = Display.Flex;
  rootStyle.flexDirection = FlexDirection.Row;
  rootStyle.size = { width: 350, height: 60 };
  rootStyle.gap = { width: 8, height: 0 };
  rootStyle.padding = { left: 6, right: 6, top: 6, bottom: 6 };

  const root = tree.newWithChildren(rootStyle, [child1, child2]);

  tree.computeLayout(root, {
    width: 350,
    height: 60,
  });

  console.log(`Child 1: flexBasis 100, Child 2: flexBasis 200`);

  return <TaffyTreePreview tree={tree} root={root} />;
});
