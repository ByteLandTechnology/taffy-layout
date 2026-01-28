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

test("styling_gap example 1", async () => {
  const tree = new TaffyTree();

  const itemStyle = new Style({ size: { width: 60, height: 40 } });

  const rootStyle = new Style({
    display: Display.Flex,
    flexDirection: FlexDirection.Row,
    flexWrap: FlexWrap.Wrap,
    size: { width: 200, height: 100 },

    // Gap adds space strictly BETWEEN items, not on the outside edges
    gap: { width: 10, height: 10 },
  });

  const children = Array.from({ length: 6 }).map(() => tree.newLeaf(itemStyle));

  const root = tree.newWithChildren(rootStyle, children);

  tree.computeLayout(root, { width: 200, height: 100 });

  return <TaffyTreePreview tree={tree} root={root} />;
});
