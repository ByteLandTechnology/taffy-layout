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

test("i18n_ja-JP_styling_align-content example 1", async () => {
  const tree = new TaffyTree();

  const itemStyle = new Style({
    size: { width: 80, height: 30 },
    margin: { bottom: 5 },
  });

  // 折り返しを強制するために十分な数の子要素を作成
  const children = Array.from({ length: 5 }).map(() => tree.newLeaf(itemStyle));

  const rootStyle = new Style({
    display: Display.Flex,
    flexDirection: FlexDirection.Row,
    flexWrap: FlexWrap.Wrap, // alignContent を動作させるために必須
    size: { width: 200, height: 200 }, // 余分な垂直スペースが必要

    // ここを変更して様々な値をテスト
    alignContent: AlignContent.Center,
  });

  const root = tree.newWithChildren(rootStyle, children);

  tree.computeLayout(root, { width: 200, height: 200 });

  return <TaffyTreePreview tree={tree} root={root} />;
});
