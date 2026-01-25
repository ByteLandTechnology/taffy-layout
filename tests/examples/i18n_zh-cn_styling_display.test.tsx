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

test("i18n_zh-CN_styling_display example 1", async () => {
  // Grid 演示
  const gridTree = new TaffyTree();
  const gridStyle = new Style();
  gridStyle.size = { width: 40, height: 40 };
  const gridChild1 = gridTree.newLeaf(gridStyle);
  const gridChild2 = gridTree.newLeaf(gridStyle);
  const gridChild3 = gridTree.newLeaf(gridStyle);
  const gridChild4 = gridTree.newLeaf(gridStyle);

  const gridRootStyle = new Style();
  gridRootStyle.display = Display.Grid;
  gridRootStyle.gridTemplateColumns = [
    { min: 0, max: "1fr" },
    { min: 0, max: "1fr" },
  ];
  gridRootStyle.gridTemplateRows = [
    { min: 0, max: "1fr" },
    { min: 0, max: "1fr" },
  ];
  gridRootStyle.gap = { width: 8, height: 8 };
  gridRootStyle.padding = { left: 10, right: 10, top: 10, bottom: 10 };
  gridRootStyle.size = { width: 140, height: 120 };

  const gridRoot = gridTree.newWithChildren(gridRootStyle, [
    gridChild1,
    gridChild2,
    gridChild3,
    gridChild4,
  ]);

  gridTree.computeLayout(gridRoot, {
    width: 140,
    height: 120,
  });

  console.log(`Flex mode: Flex, Grid columns: 2`);

  // Flex 演示设置
  const flexTree = new TaffyTree();
  const flexStyle = new Style();
  flexStyle.size = { width: 40, height: 40 };
  const flexChild1 = flexTree.newLeaf(flexStyle);
  const flexChild2 = flexTree.newLeaf(flexStyle);

  const flexRoot = flexTree.newWithChildren(
    new Style({
      display: Display.Flex,
      gap: { width: 8, height: 8 },
      padding: { left: 10, right: 10, top: 10, bottom: 10 },
      size: { width: 140, height: 120 },
    }),
    [flexChild1, flexChild2],
  );
  flexTree.computeLayout(flexRoot, { width: 140, height: 120 });

  return (
    <div style={{ display: "flex", gap: 16, flexWrap: "wrap" }}>
      <TaffyTreePreview tree={flexTree} root={flexRoot} />
      <TaffyTreePreview tree={gridTree} root={gridRoot} />
    </div>
  );
});
