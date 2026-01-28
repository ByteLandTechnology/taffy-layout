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

test("core-concepts_objects-style example 1", async () => {
  const style = new Style({
    display: Display.Flex,
    flexDirection: FlexDirection.Row,
    size: { width: 100, height: 100 },
    padding: { left: 10, right: 10, top: 10, bottom: 10 },
  });
});
