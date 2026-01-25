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

test("i18n_zh-CN_styling_grid example 1", async () => {
  const tree = new TaffyTree();

  const rootStyle = new Style({
    display: Display.Grid,
    size: { width: 200, height: 200 },
    // 定义 2 列，每列 50% 宽度
    gridTemplateColumns: [
      { min: "50%", max: "50%" },
      { min: "50%", max: "50%" },
    ],
    // 定义 2 行：第一行固定 50px，第二行占剩余空间
    gridTemplateRows: [
      { min: 50, max: 50 },
      { min: 0, max: "1fr" },
    ],
    gap: { width: 5, height: 5 },
  });

  const itemStyle = new Style({
    alignContent: AlignContent.Center,
    justifyContent: JustifyContent.Center,
  });

  const child1 = tree.newLeaf(itemStyle); // 0,0
  const child2 = tree.newLeaf(itemStyle); // 0,1
  const child3 = tree.newLeaf(itemStyle); // 1,0
  const child4 = tree.newLeaf(itemStyle); // 1,1

  const root = tree.newWithChildren(rootStyle, [
    child1,
    child2,
    child3,
    child4,
  ]);

  tree.computeLayout(root, { width: 200, height: 200 });

  return <TaffyTreePreview tree={tree} root={root} />;
});
