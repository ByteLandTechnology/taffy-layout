import React from "react";
import { test } from "vitest";
import init, {
  TaffyTree,
  Style,
  // Add all other exports that might be needed
  Display,
  Direction,
  Float,
  Clear,
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
  MinTrackSizingFunction,
  MaxTrackSizingFunction,
  GridTemplateArea,
  GridTemplateComponent,
  GridTemplateRepetition,
  RepetitionCount,
  StyleProperty,
  StylePropertyValues,
  LayoutProperty,
  Line,
  Point,
  TaffyError,
  Layout,
  MeasureFunction,
} from "taffy-layout";

// Global init for the suite
await init();

// Mock TaffyTreePreview component
const TaffyTreePreview = (_props: any) => null;

test("i18n_ja-JP_styling_flex-shrink example 1", async () => {
  const tree = new TaffyTree();

  const noShrinkStyle = new Style();
  noShrinkStyle.flexShrink = 0;
  noShrinkStyle.size = { width: 200, height: "100%" };
  const child1 = tree.newLeaf(noShrinkStyle);

  const shrinkStyle = new Style();
  shrinkStyle.flexShrink = 1;
  shrinkStyle.size = { width: 200, height: "100%" };
  const child2 = tree.newLeaf(shrinkStyle);

  const rootStyle = new Style();
  rootStyle.display = Display.Flex;
  rootStyle.flexDirection = FlexDirection.Row;
  rootStyle.size = { width: 300, height: 60 }; // 親は子の合計幅より小さい
  rootStyle.gap = { width: 8, height: 0 };
  rootStyle.padding = { left: 6, right: 6, top: 6, bottom: 6 };

  const root = tree.newWithChildren(rootStyle, [child1, child2]);

  tree.computeLayout(root, {
    width: 300,
    height: 60,
  });

  const firstLayout = tree.getLayout(child1);
  const secondLayout = tree.getLayout(child2);
  console.log(`子 1: ${firstLayout.width}px, 子 2: ${secondLayout.width}px`);
  // 子 1: 200px, 子 2: 80px
  firstLayout.free();
  secondLayout.free();

  return <TaffyTreePreview tree={tree} root={root} />;
});
