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

test("styling_size example 1", async () => {
  const tree = new TaffyTree();

  const style = new Style();
  style.size = { width: 200, height: 100 };
  style.minSize = { width: 120, height: 60 };
  style.maxSize = { width: 300, height: 160 };

  const child = tree.newLeaf(style);

  const rootStyle = new Style();
  rootStyle.display = Display.Flex;
  rootStyle.flexDirection = FlexDirection.Row;
  rootStyle.size = { width: 240, height: 140 };
  rootStyle.padding = { left: 10, right: 10, top: 10, bottom: 10 };

  const root = tree.newWithChildren(rootStyle, [child]);

  tree.computeLayout(root, {
    width: 240,
    height: 140,
  });

  console.log(tree.printTree(root));

  return <TaffyTreePreview tree={tree} root={root} />;
});

test("styling_size example 2", async () => {
  const tree = new TaffyTree();

  const containerStyle = new Style({
    // Fixed size
    size: { width: 200, height: 200 },
    display: Display.Flex,
    flexDirection: FlexDirection.Column,
    alignItems: AlignItems.Center,
    justifyContent: JustifyContent.Center,
    gap: { width: 0, height: 10 },
  });

  const percentageChild = tree.newLeaf(
    new Style({
      // 80% of parent width, fixed 30px height
      size: { width: "80%", height: 30 },
    }),
  );

  const minMaxChild = tree.newLeaf(
    new Style({
      // Wants to be 10px, but forced to at least 50px
      size: { width: 10, height: 30 },
      minSize: { width: 50, height: "auto" },
    }),
  );

  const root = tree.newWithChildren(containerStyle, [
    percentageChild,
    minMaxChild,
  ]);

  tree.computeLayout(root, { width: 200, height: 200 });

  return <TaffyTreePreview tree={tree} root={root} />;
});
