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

test("i18n_ja-JP_styling_inset example 1", async () => {
  const tree = new TaffyTree();

  const containerStyle = new Style({
    size: { width: 200, height: 100 },
    display: Display.Flex,
  });

  // 右下隅に固定された絶対配置アイテム
  const absoluteItem = tree.newLeaf(
    new Style({
      position: Position.Absolute,
      size: { width: 50, height: 50 },

      // 右下隅に固定
      inset: { bottom: 10, right: 10, top: "auto", left: "auto" },
    }),
  );

  const root = tree.newWithChildren(containerStyle, [absoluteItem]);

  tree.computeLayout(root, { width: 200, height: 100 });

  return <TaffyTreePreview tree={tree} root={root} />;
});
