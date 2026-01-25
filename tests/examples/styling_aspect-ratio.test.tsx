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

test("styling_aspect-ratio example 1", async () => {
  const tree = new TaffyTree();

  const container = new Style({
    display: Display.Flex,
    size: { width: 300, height: 300 },
    alignItems: AlignItems.Center,
    justifyContent: JustifyContent.Center,
  });

  const imagePlaceholder = tree.newLeaf(
    new Style({
      // Fix width, let height be determined by ratio
      size: { width: 100, height: "auto" },
      aspectRatio: 16 / 9, // Wide
    }),
  );

  const root = tree.newWithChildren(container, [imagePlaceholder]);

  tree.computeLayout(root, { width: 300, height: 300 });

  return <TaffyTreePreview tree={tree} root={root} />;
});
