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

test("styling_margin example 1", async () => {
  const tree = new TaffyTree();

  const childStyle = new Style();
  childStyle.size = { width: "100%", height: "100%" };
  const innerNode = tree.newLeaf(childStyle);

  const boxStyle = new Style();
  boxStyle.size = { width: 100, height: 100 };
  boxStyle.margin = { left: 20, top: 20, right: 20, bottom: 20 };
  const boxNode = tree.newWithChildren(boxStyle, [innerNode]);

  const rootStyle = new Style();
  rootStyle.size = { width: 200, height: 200 };
  rootStyle.display = Display.Flex;

  const root = tree.newWithChildren(rootStyle, [boxNode]);

  tree.computeLayout(root, { width: 200, height: 200 });

  return <TaffyTreePreview tree={tree} root={root} />;
});
