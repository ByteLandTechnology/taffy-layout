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

test("i18n_ja-JP_advanced_error-handling example 1", async () => {
  const tree = new TaffyTree();
  const parentNode = tree.newLeaf(new Style());

  try {
    // 親は有効ですが子がないため、インデックス 0 は範囲外です
    tree.getChildAtIndex(parentNode, 0);
  } catch (e) {
    if (e instanceof TaffyError) {
      console.error(`Taffy Layout Error: ${e.message}`);
      e.free();
    } else {
      throw e;
    }
  } finally {
    tree.free();
  }
});

test("i18n_ja-JP_advanced_error-handling example 2", async () => {
  const tree = new TaffyTree();
  const parentNode = tree.newLeaf(new Style());
  const index = 0;

  const count = tree.childCount(parentNode);
  if (Number.isInteger(index) && index >= 0 && index < count) {
    const child = tree.getChildAtIndex(parentNode, index);
    // ... 子を安全に使用
  }
});
