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

test("i18n_ja-JP_cookbook_flex-row-cards example 1", async () => {
  const tree = new TaffyTree();

  const style = new Style({
    flexGrow: 1,
    size: { height: "100%", width: "auto" },
  });

  const children = Array.from({ length: 4 }).map(() => tree.newLeaf(style));

  const rootStyle = new Style({
    display: Display.Flex,
    flexDirection: FlexDirection.Row,
    gap: { width: 12, height: 0 },
    size: { width: 500, height: 120 },
    padding: { left: 12, right: 12, top: 12, bottom: 12 },
  });

  const root = tree.newWithChildren(rootStyle, children);

  tree.computeLayout(root, { width: 500, height: 120 });

  return <TaffyTreePreview tree={tree} root={root} />;
});
