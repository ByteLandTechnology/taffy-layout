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

test("i18n_zh-CN_core-concepts_index example 1", async () => {
  const tree = new TaffyTree();

  const style = new Style();
  style.flexGrow = 1;

  const child1 = tree.newLeaf(style);
  const child2 = tree.newLeaf(style);

  const rootStyle = new Style();
  rootStyle.display = Display.Flex;
  rootStyle.flexDirection = FlexDirection.Row;
  rootStyle.size = { width: 400, height: 200 };

  const root = tree.newWithChildren(rootStyle, [child1, child2]);

  tree.computeLayout(root, {
    width: 400,
    height: 200,
  });

  console.log(tree.printTree(root));

  return <TaffyTreePreview tree={tree} root={root} />;
});
