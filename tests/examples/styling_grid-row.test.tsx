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

test("styling_grid-row example 1", async () => {
  const tree = new TaffyTree();

  const childStyle = new Style();
  // Occupy rows from grid line 1 to 3
  childStyle.gridRow = { start: 1, end: { span: 2 } };
  childStyle.size = { width: "auto", height: "auto" };
  const child = tree.newLeaf(childStyle);

  const rootStyle = new Style();
  rootStyle.display = Display.Grid;
  rootStyle.gridTemplateColumns = [{ min: 60, max: 60 }];
  rootStyle.gridTemplateRows = [
    { min: 40, max: 40 },
    { min: 40, max: 40 },
    { min: 40, max: 40 },
  ];
  rootStyle.gap = { width: 8, height: 8 };
  rootStyle.size = { width: 100, height: 200 };
  rootStyle.padding = { left: 8, right: 8, top: 8, bottom: 8 };

  const root = tree.newWithChildren(rootStyle, [child]);

  tree.computeLayout(root, {
    width: 100,
    height: 200,
  });

  console.log(`Row span: 2`);

  return <TaffyTreePreview tree={tree} root={root} />;
});
