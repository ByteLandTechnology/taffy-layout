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

test("i18n_ja-JP_advanced_error-handling example 1", async () => {
  const tree = new TaffyTree();
  const someNodeId = tree.newLeaf(new Style());

  try {
    // 例：無効な可能性があるノードにアクセスしようとする
    const layout = tree.getLayout(someNodeId);
  } catch (e) {
    if (e instanceof TaffyError) {
      console.error(`Taffy Layout Error: ${e.message}`);
    } else {
      throw e;
    }
  }
});

test("i18n_ja-JP_advanced_error-handling example 2", async () => {
  const tree = new TaffyTree();
  const parentNode = tree.newLeaf(new Style());
  const index = 0;

  const count = tree.childCount(parentNode);
  if (index < count) {
    const child = tree.getChildAtIndex(parentNode, index);
    // ... 子を安全に使用
  }
});
