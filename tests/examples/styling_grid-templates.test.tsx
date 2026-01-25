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

test("styling_grid-templates example 1", async () => {
  const tree = new TaffyTree();

  const rootStyle = new Style({
    display: Display.Grid,
    size: { width: 260, height: 140 },
    gridTemplateColumns: [
      { min: 60, max: 60 },
      { min: 0, max: "1fr" },
      { min: 60, max: 60 },
    ],
    gridTemplateRows: [{ min: 0, max: "1fr" }],
    padding: { left: 10, right: 10, top: 10, bottom: 10 },
    gap: { width: 10, height: 10 },
  });

  const childStyle = new Style({
    alignSelf: AlignSelf.Center,
    justifySelf: AlignSelf.Center,
  });
  const child1 = tree.newLeaf(childStyle);
  const child2 = tree.newLeaf(childStyle);
  const child3 = tree.newLeaf(childStyle);
  const child4 = tree.newLeaf(childStyle);
  const child5 = tree.newLeaf(childStyle);
  const child6 = tree.newLeaf(childStyle);

  const root = tree.newWithChildren(rootStyle, [
    child1,
    child2,
    child3,
    child4,
    child5,
    child6,
  ]);

  tree.computeLayout(root, {
    width: 260,
    height: 140,
  });

  console.log(`Columns: 3`);

  return <TaffyTreePreview tree={tree} root={root} />;
});
