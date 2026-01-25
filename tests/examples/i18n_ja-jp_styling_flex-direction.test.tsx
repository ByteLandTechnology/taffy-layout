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

test("i18n_ja-JP_styling_flex-direction example 1", async () => {
  const tree = new TaffyTree();

  const style = new Style({
    size: { width: 40, height: 40 },
    margin: { bottom: 5, right: 5 },
  });

  const child1 = tree.newLeaf(style);
  const child2 = tree.newLeaf(style);
  const child3 = tree.newLeaf(style);

  const rootStyle = new Style({
    display: Display.Flex,
    // ここを変更: Row, Column, RowReverse, ColumnReverse
    flexDirection: FlexDirection.Row,
    size: { width: 250, height: 150 },
    padding: { left: 10, right: 10, top: 10, bottom: 10 },
  });

  const root = tree.newWithChildren(rootStyle, [child1, child2, child3]);

  tree.computeLayout(root, { width: 250, height: 150 });

  return <TaffyTreePreview tree={tree} root={root} />;
});
