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

test("i18n_ja-JP_styling_grid example 1", async () => {
  const tree = new TaffyTree();

  const rootStyle = new Style({
    display: Display.Grid,
    size: { width: 200, height: 200 },
    // 2つの列を定義（残りのスペースを等分する 1fr）
    gridTemplateColumns: [
      { min: 0, max: "1fr" },
      { min: 0, max: "1fr" },
    ],
    // 2つの行を定義：1行目は固定50px、2行目は残りスペース
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
