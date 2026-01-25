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

test("i18n_ja-JP_getting-started_quick-start example 1", async () => {
  // 1. ライブラリを初期化
  const tree = new TaffyTree();

  // 2. スタイルを作成
  const containerStyle = new Style({
    display: Display.Flex,
    flexDirection: FlexDirection.Column,
    alignItems: AlignItems.Center,
    size: { width: 300, height: 200 },
    padding: { left: 10, right: 10, top: 10, bottom: 10 },
  });

  const childStyle = new Style({
    flexGrow: 1,
    size: { width: "100%", height: "auto" },
  });

  // 3. ノードツリーを作成
  //    （newWithChildren を使用する場合、リーフノードを先に作成する必要があります）
  const child1 = tree.newLeaf(childStyle);
  const child2 = tree.newLeaf(childStyle);
  const container = tree.newWithChildren(containerStyle, [child1, child2]);

  // 4. レイアウトを計算
  //    ルートノードと利用可能なスペースを渡します
  tree.computeLayout(container, { width: 300, height: 200 });

  // 5. 計算結果を読み取り
  const containerLayout = tree.getLayout(container);
  const child1Layout = tree.getLayout(child1);

  console.log(`Container: ${containerLayout.width}x${containerLayout.height}`);
  // 出力: Container: 300x200

  // 6. デバッグ：ツリー構造全体を出力
  console.log(tree.printTree(container));
});
