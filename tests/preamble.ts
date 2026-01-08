import {
  TaffyTree,
  Style,
  // Types needed for property creation if any
  Display,
  FlexDirection,
  AlignItems,
  JustifyContent,
  Position,
} from "taffy-layout";

// Helper mocks
const measureText = (text: string, width: any) => ({ width: 0, height: 0 });
const measureTextWidth = (text: string) => 0;
const measureTextHeight = (text: string, width: any) => 0;

export function makeTestEnv() {
  const tree = new TaffyTree();
  const style = new Style();
  const rootId = tree.newLeaf(style);

  // Create commonly used nodes
  const childId = tree.newLeaf(style);
  const child1 = childId;
  const child2 = tree.newLeaf(style);
  const child3 = tree.newLeaf(style);

  // Populate tree for removal/traversal examples (best effort default state)
  try {
    tree.addChild(rootId, child1);
    tree.addChild(rootId, child2);
    tree.addChild(rootId, child3);
  } catch (e) {}

  return {
    // Classes aliased for compatibility
    TaffyTree,
    Style,

    // Functions
    init: () => Promise.resolve(), // Already initialized globally
    loadTaffy: () => Promise.resolve(),
    measureText,
    measureTextWidth,
    measureTextHeight,

    // Variables
    tree,
    style,
    rootId,
    root: rootId,
    rootNode: rootId,
    childId,
    child1,
    child2,
    child3,
    parentId: rootId,
    nodeId: rootId,
    node: rootId,
    parent: rootId,
    child: childId,
    gridNode: rootId,

    // Indices
    index: 0,
    start_index: 0,
    startIndex: 0,
    end_index: 1,
    endIndex: 1,

    // New nodes
    new_child: childId,
    newChild: childId,
    newChildId: childId,
    id1: child1,
    id2: child2,

    // Data
    availableSpace: { width: 100, height: 100 },
    container: rootId,

    // Styles
    textStyle: new Style(),
    childStyle: new Style(),
    containerStyle: new Style(),
    itemStyle: new Style(),
    rowStyle: new Style(),
    gridStyle: new Style(),
    absoluteStyle: new Style(),
    percentStyle: new Style(),
    imgStyle: new Style(),
  };
}
