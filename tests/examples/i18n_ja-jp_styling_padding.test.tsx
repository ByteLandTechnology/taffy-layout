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

test("i18n_ja-JP_styling_padding example 1", async () => {
  const tree = new TaffyTree();

  const childStyle = new Style();
  childStyle.flexGrow = 1;
  const innerNode = tree.newLeaf(childStyle);

  const boxStyle = new Style();
  boxStyle.size = { width: 150, height: 100 };
  boxStyle.display = Display.Flex;
  boxStyle.padding = { left: 20, top: 20, right: 20, bottom: 20 };
  const boxNode = tree.newWithChildren(boxStyle, [innerNode]);

  const rootStyle = new Style();
  rootStyle.size = { width: 200, height: 200 };

  const root = tree.newWithChildren(rootStyle, [boxNode]);

  tree.computeLayout(root, { width: 200, height: 200 });

  return <TaffyTreePreview tree={tree} root={root} />;
});
