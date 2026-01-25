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

test("i18n_zh-CN_styling_grid-auto-flow example 1", async () => {
  const tree = new TaffyTree();
  const rootStyle = new Style({ display: Display.Grid });
  const child1 = tree.newLeaf(new Style());
  const child2 = tree.newLeaf(new Style());
  const child3 = tree.newLeaf(new Style());
  const child4 = tree.newLeaf(new Style());

  rootStyle.gridTemplateColumns = [
    { min: 60, max: 60 },
    { min: 60, max: 60 },
  ];
  rootStyle.gridTemplateRows = [
    { min: 40, max: 40 },
    { min: 40, max: 40 },
  ];
  rootStyle.gap = { width: 8, height: 8 };
  rootStyle.size = { width: 160, height: 100 };
  rootStyle.padding = { left: 8, right: 8, top: 8, bottom: 8 };

  const root = tree.newWithChildren(rootStyle, [
    child1,
    child2,
    child3,
    child4,
  ]);

  tree.computeLayout(root, {
    width: 160,
    height: 100,
  });

  console.log(`Auto flow: Row`);

  return <TaffyTreePreview tree={tree} root={root} />;
});
