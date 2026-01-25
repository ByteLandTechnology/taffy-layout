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

test("i18n_zh-CN_cookbook_media-card example 1", async () => {
  const tree = new TaffyTree();

  const cardStyle = new Style({
    display: Display.Flex,
    flexDirection: FlexDirection.Row,
    gap: { width: 12, height: 0 },
    size: { width: 420, height: 120 },
    padding: { left: 12, right: 12, top: 12, bottom: 12 },
  });

  const image = tree.newLeaf(
    new Style({
      size: { width: 80, height: 80 },
    }),
  );

  const textContainer = tree.newWithChildren(
    new Style({
      display: Display.Flex,
      flexDirection: FlexDirection.Column,
      flexGrow: 1,
      gap: { width: 0, height: 6 },
    }),
    [
      tree.newLeaf(new Style({ size: { width: "100%", height: 20 } })), // 标题占位符
      tree.newLeaf(new Style({ size: { width: "60%", height: 16 } })), // 副标题占位符
    ],
  );

  const root = tree.newWithChildren(cardStyle, [image, textContainer]);

  tree.computeLayout(root, { width: 420, height: 120 });

  return <TaffyTreePreview tree={tree} root={root} />;
});
