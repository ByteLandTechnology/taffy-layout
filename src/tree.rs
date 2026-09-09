//! # Layout Tree Management Module
//!
//! This module provides the [`JsTaffyTree`] struct, which is the main entry point for
//! creating and managing layout trees. It wraps the native Taffy layout engine and
//! exposes a JavaScript-friendly API.
//!
//! ## Overview
//!
//! The `TaffyTree` class manages a collection of nodes organized in a tree structure.
//! Each node has:
//! - A unique ID (represented as `bigint` in JavaScript)
//! - A style configuration that defines its layout properties
//! - Optional context data (any JavaScript value)
//! - Zero or more child nodes
//!
//! ## Workflow
//!
//! 1. Create a `TaffyTree` instance
//! 2. Create nodes using `newLeaf()` or `newWithChildren()`
//! 3. Build the tree structure using `addChild()`, `setChildren()`, etc.
//! 4. Compute layout using `computeLayout()` or `computeLayoutWithMeasure()`
//! 5. Retrieve computed layouts using `getLayout()`
//!
//!
//! @example
//! ```typescript
//! import init, {
//!   TaffyTree,
//!   Style,
//!   Display,
//!   FlexDirection,
//!   type Size,
//!   type AvailableSpace,
//!   type Dimension
//! } from 'taffy-layout';
//!
//! await init();
//!
//! const tree = new TaffyTree();
//!
//! // Container style with explicit types
//! const containerStyle = new Style();
//! containerStyle.display = Display.Flex;
//! containerStyle.flexDirection = FlexDirection.Column;
//!
//! const containerSize: Size<Dimension> = {
//!   width: 300,
//!   height: 200
//! };
//! containerStyle.size = containerSize;
//!
//! // Child style
//! const childStyle = new Style();
//! childStyle.flexGrow = 1;
//!
//! // Create nodes with type annotations
//! const child1: bigint = tree.newLeaf(childStyle);
//! const child2: bigint = tree.newLeaf(childStyle);
//! const container: bigint = tree.newWithChildren(
//!   containerStyle,
//!   [child1, child2]
//! );
//!
//! // Compute layout with typed available space
//! const availableSpace: Size<AvailableSpace> = {
//!   width: 300,
//!   height: 200
//! };
//! tree.computeLayout(container, availableSpace);
//!
//! // Get typed layout
//! const layout = tree.getLayout(child1);
//! console.log(`Size: ${layout.width}x${layout.height}`);
//! ```
//!
//! ## Node IDs
//!
//! Node IDs are represented as `bigint` in JavaScript (u64 in Rust). They are stable
//! for the lifetime of the node and can be stored/compared as needed. Pass only IDs
//! of live nodes from the same tree. Removing a node or clearing the tree invalidates
//! the affected IDs; invalid IDs may cause a WebAssembly trap.
//!
//! ## Error Handling
//!
//! Errors returned by the layout engine are exposed as `TaffyError` exceptions.
//! Node IDs must satisfy the lifetime requirement above; invalid IDs are not
//! guaranteed to produce a `TaffyError`.
//!
//! @example
//! ```typescript
//! try {
//!   const tree = new TaffyTree();
//!   const style = new Style();
//!   const nodeId = tree.newLeaf(style);
//!   console.log('Created node:', nodeId);
//! } catch (e) {
//!   if (e instanceof TaffyError) {
//!     console.error('Error:', e.message);
//!   }
//! }
//! ```

use crate::error::{JsTaffyError, map_bool_result, map_node_result, map_void_result, to_js_error};
use crate::layout::JsLayout;
use crate::style::{GridTemplateAreaCounts, JsStyle};
use crate::types::{AvailableSizeDto, JsAvailableSizeArg, JsBigIntArray, JsMeasureFunctionArg};
#[cfg(feature = "detailed_layout_info")]
use crate::{
    DetailedGridInfoDto, DetailedGridItemsInfoDto, DetailedGridTracksInfoDto, JsDetailedLayoutInfo,
};

use js_sys::{Array, BigInt};
use std::collections::HashMap;
use taffy::TaffyError as NativeTaffyError;
use taffy::TaffyTree;
use taffy::prelude::*;
use taffy::style::{self as TaffyStyle};
#[cfg(feature = "detailed_layout_info")]
use taffy::tree::DetailedLayoutInfo;
use taffy::tree::{LayoutInput, LayoutOutput};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

#[cfg(feature = "detailed_layout_info")]
fn grid_tracks_info_dto(tracks: &taffy::DetailedGridTracksInfo) -> DetailedGridTracksInfoDto {
    // Upstream positions retain logical track order, including in RTL grids.
    // Distances between physical intervals also include content alignment space.
    let mut gutters = vec![0.0];
    gutters.extend(
        tracks
            .positions
            .windows(2)
            .map(|pair| (pair[1].start - pair[0].end).max(pair[0].start - pair[1].end)),
    );
    if !tracks.positions.is_empty() {
        gutters.push(0.0);
    }

    DetailedGridTracksInfoDto {
        negative_implicit_tracks: tracks.negative_implicit_tracks,
        explicit_tracks: tracks.explicit_tracks,
        positive_implicit_tracks: tracks.positive_implicit_tracks,
        gutters,
        sizes: tracks
            .positions
            .iter()
            .map(|track| track.end - track.start)
            .collect(),
        positions: tracks.positions.clone(),
    }
}

// =============================================================================
// TaffyTree Struct
// =============================================================================

/// The main layout tree class for creating nodes, computing layouts, and managing a tree of styled elements.
///
/// TaffyTree is the entry point for the Taffy layout engine. It manages
/// a tree of nodes and computes their layouts using Flexbox, Grid, and block algorithms.
///
/// Node IDs passed to this instance must identify live nodes created by the same
/// tree. Removed IDs and IDs invalidated by `clear()` must not be reused.
/// Invalid IDs may cause a WebAssembly trap instead of a `TaffyError`.
///
#[wasm_bindgen(js_name = TaffyTree)]
pub struct JsTaffyTree {
    /// The underlying Taffy tree with JsValue context type
    tree: TaffyTree<JsValue>,
    /// Taffy stores effective counts, so preserve explicit counts separately
    /// for style copies returned by getStyle() and measure callbacks.
    explicit_grid_template_area_counts: HashMap<u64, GridTemplateAreaCounts>,
}

impl Default for JsTaffyTree {
    fn default() -> Self {
        Self::new()
    }
}

impl JsTaffyTree {
    fn remember_grid_template_area_counts(&mut self, node: u64, style: &JsStyle) {
        let counts = style.explicit_grid_template_area_counts;
        if counts == GridTemplateAreaCounts::default() {
            self.explicit_grid_template_area_counts.remove(&node);
        } else {
            self.explicit_grid_template_area_counts.insert(node, counts);
        }
    }
}

#[wasm_bindgen(js_class = "TaffyTree")]
impl JsTaffyTree {
    // =========================================================================
    // Constructors
    // =========================================================================

    /// Creates a new empty TaffyTree
    ///
    /// The tree starts with no nodes. Use `newLeaf()` or `newWithChildren()`
    /// to add nodes.
    ///
    /// @example
    /// ```typescript
    /// const tree: TaffyTree = new TaffyTree();
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsTaffyTree {
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();
        JsTaffyTree {
            tree: TaffyTree::new(),
            explicit_grid_template_area_counts: HashMap::new(),
        }
    }

    /// Creates a new TaffyTree with pre-allocated capacity
    ///
    /// Use this when you know approximately how many nodes will be in the tree.
    /// This can improve performance by reducing memory reallocations.
    ///
    /// @param capacity - The number of nodes to pre-allocate space for
    ///
    /// @example
    /// ```typescript
    /// const tree: TaffyTree = TaffyTree.withCapacity(1000);
    /// ```
    #[wasm_bindgen(js_name = withCapacity)]
    pub fn with_capacity(capacity: usize) -> JsTaffyTree {
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();
        JsTaffyTree {
            tree: TaffyTree::with_capacity(capacity),
            explicit_grid_template_area_counts: HashMap::new(),
        }
    }

    // =========================================================================
    // Configuration
    // =========================================================================

    /// Enables rounding of layout values to whole pixels
    ///
    /// When enabled (default), cumulative box edges are rounded to whole pixels.
    /// Widths and heights are differences between those rounded edges, so equal
    /// fractional sizes can round differently. Margins and detailed grid track
    /// measurements can still contain fractions.
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// tree.enableRounding();
    /// ```
    #[wasm_bindgen(js_name = enableRounding)]
    pub fn enable_rounding(&mut self) {
        self.tree.enable_rounding();
    }

    /// Disables rounding of layout values
    ///
    /// When disabled, computed layout values retain their fractional precision.
    /// Use this when you need sub-pixel accuracy or when performing custom
    /// rounding.
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const node = tree.newLeaf(new Style());
    /// tree.disableRounding();
    /// const layout = tree.getLayout(node);
    /// console.log(layout.x);
    /// ```
    #[wasm_bindgen(js_name = disableRounding)]
    pub fn disable_rounding(&mut self) {
        self.tree.disable_rounding();
    }

    // =========================================================================
    // Node Creation
    // =========================================================================

    /// Creates a new leaf node with the given style
    ///
    /// A leaf node has no children. Use this for elements that contain
    /// content (like text) rather than other elements.
    ///
    /// @param style - The style configuration for the node
    /// @returns - The node ID (`bigint`)
    /// @throws `TaffyError` if the node cannot be created
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const style = new Style();
    /// style.size = { width: 100, height: 50 };
    /// const nodeId: bigint = tree.newLeaf(style);
    /// ```
    #[wasm_bindgen(js_name = newLeaf)]
    pub fn new_leaf(&mut self, style: &JsStyle) -> Result<u64, JsValue> {
        let node = map_node_result(self.tree.new_leaf(style.inner.clone()))?;
        self.remember_grid_template_area_counts(node, style);
        Ok(node)
    }

    /// Creates a new leaf node with an attached context value
    ///
    /// The context can be any JavaScript value and is passed to the measure
    /// function during layout computation. This is useful for storing
    /// references to text content or other dynamic data.
    ///
    /// @param style - The style configuration for the node
    /// @param context - Any JavaScript value to attach to the node
    /// @returns - The node ID (`bigint`)
    /// @throws `TaffyError` if the node cannot be created
    ///
    /// @example
    /// ```typescript
    /// interface TextContext { text: string; isBold: boolean; }
    ///
    /// const tree = new TaffyTree();
    /// const style = new Style();
    /// const context: TextContext = { text: "Hello, World!", isBold: true };
    /// const nodeId: bigint = tree.newLeafWithContext(style, context);
    /// ```
    #[wasm_bindgen(js_name = newLeafWithContext)]
    pub fn new_leaf_with_context(
        &mut self,
        style: &JsStyle,
        context: JsValue,
    ) -> Result<u64, JsValue> {
        let node = map_node_result(
            self.tree
                .new_leaf_with_context(style.inner.clone(), context),
        )?;
        self.remember_grid_template_area_counts(node, style);
        Ok(node)
    }

    /// Creates a new node with the given children
    ///
    /// Use this to create container nodes that have child elements.
    /// The children must already exist in the tree.
    ///
    /// @param style - The style configuration for the node
    /// @param children - Array of child node IDs (as bigint[])
    ///
    /// @returns - The node ID (`bigint`)
    ///
    /// @throws `TaffyError` if the node cannot be created
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const containerStyle = new Style();
    /// containerStyle.display = Display.Flex;
    ///
    /// const child1: bigint = tree.newLeaf(new Style());
    /// const child2: bigint = tree.newLeaf(new Style());
    ///
    /// const container: bigint = tree.newWithChildren(
    ///   containerStyle,
    ///   [child1, child2]
    /// );
    /// ```
    #[wasm_bindgen(js_name = newWithChildren)]
    pub fn new_with_children(
        &mut self,
        style: &JsStyle,
        children: JsBigIntArray,
    ) -> Result<u64, JsValue> {
        let children: Vec<u64> = serde_wasm_bindgen::from_value(children.into())?;
        let children_ids: Vec<NodeId> = children.iter().map(|&id| NodeId::from(id)).collect();
        let node = map_node_result(
            self.tree
                .new_with_children(style.inner.clone(), &children_ids),
        )?;
        self.remember_grid_template_area_counts(node, style);
        Ok(node)
    }

    // =========================================================================
    // Tree Operations
    // =========================================================================

    /// Removes all nodes from the tree
    ///
    /// This clears the entire tree, removing all nodes and their relationships.
    /// Use this to reset the tree for reuse.
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// tree.clear();
    /// console.log(tree.totalNodeCount());
    /// ```
    #[wasm_bindgen(js_name = clear)]
    pub fn clear(&mut self) {
        self.tree.clear();
        self.explicit_grid_template_area_counts.clear();
    }

    /// Removes a node from the tree
    ///
    /// Only the specified node is deleted. It is detached from its parent, and
    /// its direct children become parentless. Descendant nodes remain in the tree.
    ///
    /// @param node - The node ID to remove
    ///
    /// @returns - The removed node ID (`bigint`)
    ///
    /// @remarks The node ID must identify a live node in this tree.
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const nodeId = tree.newLeaf(new Style());
    /// const removedId: bigint = tree.remove(nodeId);
    /// // nodeId is no longer valid and must not be passed to this tree again.
    /// ```
    #[wasm_bindgen(js_name = remove)]
    pub fn remove(&mut self, node: u64) -> Result<u64, JsValue> {
        let removed = map_node_result(self.tree.remove(NodeId::from(node)))?;
        self.explicit_grid_template_area_counts.remove(&removed);
        Ok(removed)
    }

    // =========================================================================
    // Node Context
    // =========================================================================

    /// Sets a context value for a node
    ///
    /// The context can be any JavaScript value and is passed to the measure
    /// function during layout computation.
    ///
    /// @param node - The node ID
    /// @param context - Any JavaScript value to attach
    ///
    /// @remarks The node ID must identify a live node in this tree.
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const nodeId = tree.newLeaf(new Style());
    /// interface Context { text: string };
    /// tree.setNodeContext(nodeId, { text: "Updated text" } as Context);
    /// ```
    #[wasm_bindgen(js_name = setNodeContext)]
    pub fn set_node_context(&mut self, node: u64, context: JsValue) -> Result<(), JsValue> {
        map_void_result(
            self.tree
                .set_node_context(NodeId::from(node), Some(context)),
        )
    }

    /// Gets the context value for a node
    ///
    /// @param node - The node ID
    ///
    /// @returns - The attached context value, or `undefined` if none is set
    ///
    /// Object contexts retain their JavaScript identity. Mutating their fields
    /// does not mark the node dirty; call `markDirty()` before recomputing.
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const nodeId = tree.newLeaf(new Style());
    /// interface Context { text: string };
    /// const context = tree.getNodeContext(nodeId) as Context | undefined;
    /// if (context) {
    ///   console.log(context.text);
    /// }
    /// ```
    #[wasm_bindgen(js_name = getNodeContext)]
    pub fn get_node_context(&self, node: u64) -> Result<JsValue, JsValue> {
        match self.tree.get_node_context(NodeId::from(node)) {
            Some(ctx) => Ok(ctx.clone()),
            None => Ok(JsValue::UNDEFINED),
        }
    }

    /// Gets a mutable reference to the context value for a node
    ///
    /// In JavaScript, this behaves the same as `getNodeContext()` since
    /// JavaScript objects are always passed by reference.
    /// Field mutations require an explicit `markDirty()` before recomputing.
    ///
    /// @param node - The node ID
    ///
    /// @returns - The attached context value, or `undefined` if none is set
    #[wasm_bindgen(js_name = getNodeContextMut)]
    pub fn get_node_context_mut(&mut self, node: u64) -> Result<JsValue, JsValue> {
        match self.tree.get_node_context_mut(NodeId::from(node)) {
            Some(ctx) => Ok(ctx.clone()),
            None => Ok(JsValue::UNDEFINED),
        }
    }

    /// Gets context values for multiple nodes at once
    ///
    /// This is more efficient than calling `getNodeContext()` multiple times
    /// when you need to access contexts for many nodes.
    ///
    /// @param children - Array of node IDs
    ///
    /// @returns - Array of context values (undefined for nodes without context)
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const id1 = tree.newLeaf(new Style());
    /// const id2 = tree.newLeaf(new Style());
    /// const nodes = [id1, id2];
    /// const contexts = tree.getDisjointNodeContextMut(nodes);
    /// ```
    #[wasm_bindgen(js_name = getDisjointNodeContextMut)]
    pub fn get_disjoint_node_context_mut(
        &mut self,
        children: JsBigIntArray,
    ) -> Result<Box<[JsValue]>, JsValue> {
        let children: Vec<u64> = serde_wasm_bindgen::from_value(children.into())?;
        let mut results = Vec::with_capacity(children.len());
        for id in children.iter() {
            match self.tree.get_node_context_mut(NodeId::from(*id)) {
                Some(ctx) => results.push(ctx.clone()),
                None => results.push(JsValue::UNDEFINED),
            }
        }
        Ok(results.into_boxed_slice())
    }

    // =========================================================================
    // Child Management
    // =========================================================================

    /// Appends a child node to a parent
    ///
    /// The child is added as the last child of the parent.
    ///
    /// @param parent - The parent node ID
    /// @param child - The child node ID to add
    ///
    /// @remarks Both node IDs must identify live nodes in this tree.
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const parentId = tree.newLeaf(new Style());
    /// const childId = tree.newLeaf(new Style());
    /// tree.addChild(parentId, childId);
    /// ```
    #[wasm_bindgen(js_name = addChild)]
    pub fn add_child(&mut self, parent: u64, child: u64) -> Result<(), JsValue> {
        map_void_result(
            self.tree
                .add_child(NodeId::from(parent), NodeId::from(child)),
        )
    }

    /// Inserts a child at a specific index
    ///
    /// @param parent - The parent node ID
    /// @param index - The position to insert at (0-based)
    /// @param child - The child node ID to insert
    ///
    /// @throws `TaffyError` if the index is out of bounds
    /// @remarks Both node IDs must identify live nodes in this tree.
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const parentId = tree.newLeaf(new Style());
    /// const childId = tree.newLeaf(new Style());
    /// tree.insertChildAtIndex(parentId, 0, childId);
    /// ```
    #[wasm_bindgen(js_name = insertChildAtIndex)]
    pub fn insert_child_at_index(
        &mut self,
        parent: u64,
        index: usize,
        child: u64,
    ) -> Result<(), JsValue> {
        map_void_result(self.tree.insert_child_at_index(
            NodeId::from(parent),
            index,
            NodeId::from(child),
        ))
    }

    /// Replaces all children of a node
    ///
    /// Existing child relationships are replaced with the new array. Detached
    /// child nodes remain in the tree.
    ///
    /// @param parent - The parent node ID
    /// @param children - Array of new child node IDs
    ///
    /// @remarks The parent ID must identify a live node in this tree.
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const parentId = tree.newLeaf(new Style());
    /// const child1 = tree.newLeaf(new Style());
    /// const child2 = tree.newLeaf(new Style());
    /// const child3 = tree.newLeaf(new Style());
    /// const children = [child1, child2, child3];
    /// tree.setChildren(parentId, children);
    /// ```
    #[wasm_bindgen(js_name = setChildren)]
    pub fn set_children(&mut self, parent: u64, children: JsBigIntArray) -> Result<(), JsValue> {
        let children: Vec<u64> = serde_wasm_bindgen::from_value(children.into())?;
        let children_ids: Vec<NodeId> = children.iter().map(|&id| NodeId::from(id)).collect();
        map_void_result(self.tree.set_children(NodeId::from(parent), &children_ids))
    }

    /// Removes a specific child from a parent
    ///
    /// The child must currently belong to this parent. It remains in the tree
    /// after its parent relationship is removed.
    ///
    /// @param parent - The parent node ID
    /// @param child - The child node ID to remove
    ///
    /// @returns - The removed child ID (`bigint`)
    ///
    /// @remarks Both node IDs must identify live nodes in this tree.
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const parentId = tree.newLeaf(new Style());
    /// const childId = tree.newLeaf(new Style());
    /// tree.addChild(parentId, childId);
    /// tree.removeChild(parentId, childId);
    /// ```
    #[wasm_bindgen(js_name = removeChild)]
    pub fn remove_child(&mut self, parent: u64, child: u64) -> Result<u64, JsValue> {
        map_node_result(
            self.tree
                .remove_child(NodeId::from(parent), NodeId::from(child)),
        )
    }

    /// Removes a child at a specific index
    ///
    /// @param parent - The parent node ID
    /// @param index - The index of the child to remove (0-based)
    ///
    /// @returns - The removed child ID (`bigint`)
    ///
    /// @throws `TaffyError` if the index is out of bounds
    /// @remarks Node IDs must identify live nodes in this tree.
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const parentId = tree.newLeaf(new Style());
    /// const childId = tree.newLeaf(new Style());
    /// tree.addChild(parentId, childId);
    /// const removedId: bigint = tree.removeChildAtIndex(parentId, 0);
    /// ```
    #[wasm_bindgen(js_name = removeChildAtIndex)]
    pub fn remove_child_at_index(&mut self, parent: u64, index: usize) -> Result<u64, JsValue> {
        map_node_result(self.tree.remove_child_at_index(NodeId::from(parent), index))
    }

    /// Replaces a child at a specific index
    ///
    /// @param parent - The parent node ID
    /// @param index - The index of the child to replace (0-based)
    /// @param newChild - The new child node ID
    ///
    /// @returns - The replaced (old) child ID (`bigint`)
    ///
    /// @throws `TaffyError` if the index is out of bounds
    /// @remarks Node IDs must identify live nodes in this tree.
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const parentId = tree.newLeaf(new Style());
    /// const oldChild = tree.newLeaf(new Style());
    /// const newChildId = tree.newLeaf(new Style());
    /// tree.addChild(parentId, oldChild);
    /// const child = tree.newLeaf(new Style()); // filler child at index 0 if needed, but index 1 implies 2 children
    /// tree.insertChildAtIndex(parentId, 0, child);
    ///
    /// const oldChildId: bigint = tree.replaceChildAtIndex(parentId, 1, newChildId);
    /// ```
    #[wasm_bindgen(js_name = replaceChildAtIndex)]
    pub fn replace_child_at_index(
        &mut self,
        parent: u64,
        index: usize,
        #[wasm_bindgen(js_name = "newChild")] new_child: u64,
    ) -> Result<u64, JsValue> {
        map_node_result(self.tree.replace_child_at_index(
            NodeId::from(parent),
            index,
            NodeId::from(new_child),
        ))
    }

    /// Gets the child at a specific index
    ///
    /// @param parent - The parent node ID
    /// @param index - The index of the child (0-based)
    ///
    /// @returns - The child node ID (`bigint`)
    ///
    /// @throws `TaffyError` if the index is out of bounds
    /// @remarks Node IDs must identify live nodes in this tree.
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const parentId = tree.newLeaf(new Style());
    /// const childId = tree.newLeaf(new Style());
    /// tree.addChild(parentId, childId);
    /// const firstChild: bigint = tree.getChildAtIndex(parentId, 0);
    /// ```
    #[wasm_bindgen(js_name = getChildAtIndex)]
    pub fn get_child_at_index(&self, parent: u64, index: usize) -> Result<u64, JsValue> {
        map_node_result(self.tree.child_at_index(NodeId::from(parent), index))
    }

    /// Removes a range of children
    ///
    /// Detaches children from `startIndex` (inclusive) to `endIndex` (exclusive).
    /// The detached nodes remain in the tree.
    ///
    /// @param parent - The parent node ID
    /// @param startIndex - Start of range (inclusive)
    /// @param endIndex - End of range (exclusive)
    ///
    /// @remarks The parent ID must identify a live node in this tree. Range
    /// endpoints must be integers satisfying `0 <= startIndex <= endIndex <= childCount(parent)`.
    /// An invalid range may cause a WebAssembly trap instead of a `TaffyError`.
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const parentId = tree.newLeaf(new Style());
    /// const child0 = tree.newLeaf(new Style());
    /// const child1 = tree.newLeaf(new Style());
    /// const child2 = tree.newLeaf(new Style());
    /// const child3 = tree.newLeaf(new Style());
    /// tree.setChildren(parentId, [child0, child1, child2, child3]);
    ///
    /// tree.removeChildrenRange(parentId, 1, 3); // Removes child1 and child2
    /// ```
    #[wasm_bindgen(js_name = removeChildrenRange)]
    pub fn remove_children_range(
        &mut self,
        parent: u64,
        #[wasm_bindgen(js_name = "startIndex")] start_index: usize,
        #[wasm_bindgen(js_name = "endIndex")] end_index: usize,
    ) -> Result<(), JsValue> {
        map_void_result(
            self.tree
                .remove_children_range(NodeId::from(parent), start_index..end_index),
        )
    }

    /// Gets the total number of nodes in the tree
    ///
    /// @returns - The total count of all nodes
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const count: number = tree.totalNodeCount();
    /// ```
    #[wasm_bindgen(js_name = totalNodeCount)]
    pub fn total_node_count(&self) -> usize {
        self.tree.total_node_count()
    }

    /// Gets the number of children of a node
    ///
    /// @param parent - The parent node ID
    ///
    /// @returns - The number of direct children
    ///
    /// @remarks The node ID must identify a live node in this tree.
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const parentId = tree.newLeaf(new Style());
    /// const count: number = tree.childCount(parentId);
    /// ```
    #[wasm_bindgen(js_name = childCount)]
    pub fn child_count(&self, parent: u64) -> usize {
        self.tree.child_count(NodeId::from(parent))
    }

    /// Gets the parent of a node
    ///
    /// @param child - The child node ID
    ///
    /// @returns - The parent node ID, or `undefined` if the node has no parent
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const parentId = tree.newLeaf(new Style());
    /// const childId = tree.newLeaf(new Style());
    /// tree.addChild(parentId, childId);
    /// const parent: bigint | undefined = tree.parent(childId);
    /// ```
    #[wasm_bindgen(js_name = parent)]
    pub fn parent(&self, child: u64) -> Option<u64> {
        self.tree.parent(NodeId::from(child)).map(u64::from)
    }

    /// Gets all children of a node
    ///
    /// @param parent - The parent node ID
    ///
    /// @returns - Array of child node IDs
    ///
    /// @remarks The parent ID must identify a live node in this tree.
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const parentId = tree.newLeaf(new Style());
    /// const children = tree.children(parentId);
    /// ```
    #[wasm_bindgen(js_name = children)]
    pub fn children(&self, parent: u64) -> Result<JsBigIntArray, JsValue> {
        self.tree
            .children(NodeId::from(parent))
            .map(|c| {
                let array = Array::new();
                for &id in c.iter() {
                    array.push(&BigInt::from(u64::from(id)));
                }
                array.unchecked_into()
            })
            .map_err(to_js_error)
    }

    // =========================================================================
    // Style Management
    // =========================================================================

    /// Sets the style for an existing node
    ///
    /// This replaces the node's current style with the provided one.
    /// The node will be marked as dirty and require re-layout.
    ///
    /// @param node - The node ID
    /// @param style - The new style configuration
    ///
    /// @remarks The node ID must identify a live node in this tree.
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const nodeId = tree.newLeaf(new Style());
    /// const newStyle = new Style();
    /// newStyle.flexGrow = 2;
    /// tree.setStyle(nodeId, newStyle);
    /// ```
    #[wasm_bindgen(js_name = setStyle)]
    pub fn set_style(&mut self, node: u64, style: &JsStyle) -> Result<(), JsValue> {
        map_void_result(self.tree.set_style(NodeId::from(node), style.inner.clone()))?;
        self.remember_grid_template_area_counts(node, style);
        Ok(())
    }

    /// Gets the style for a node
    ///
    /// @param node - The node ID
    ///
    /// @returns - An owned copy of the node's `Style`; call `free()` when finished
    ///
    /// Changes to this copy affect the tree only after calling `setStyle()`.
    ///
    /// @remarks The node ID must identify a live node in this tree.
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const nodeId = tree.newLeaf(new Style());
    /// const style: Style = tree.getStyle(nodeId);
    /// console.log('Flex grow:', style.flexGrow);
    /// ```
    #[wasm_bindgen(js_name = getStyle)]
    pub fn style(&self, node: u64) -> Result<JsStyle, JsValue> {
        match self.tree.style(NodeId::from(node)) {
            Ok(s) => Ok(JsStyle {
                inner: s.clone(),
                explicit_grid_template_area_counts: self
                    .explicit_grid_template_area_counts
                    .get(&node)
                    .copied()
                    .unwrap_or_default(),
            }),
            Err(e) => Err(JsValue::from(JsTaffyError::from(e))),
        }
    }

    // =========================================================================
    // Layout Results
    // =========================================================================

    /// Gets the computed layout for a node
    ///
    /// Call this after `computeLayout()` to retrieve the computed position
    /// and size for a node.
    ///
    /// @param node - The node ID
    ///
    /// @returns - An owned snapshot of the computed `Layout`; call `free()` when finished
    ///
    /// Recomputing the tree does not update an existing Layout snapshot.
    ///
    /// @remarks The node ID must identify a live node in this tree.
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const style = new Style();
    /// style.size = { width: 100, height: 100 };
    /// const rootId = tree.newLeaf(style);
    /// const nodeId = rootId;
    ///
    /// tree.computeLayout(rootId, { width: 800, height: 600 });
    /// const layout: Layout = tree.getLayout(nodeId);
    /// console.log(`Position: (${layout.x}, ${layout.y}), Size: ${layout.width}x${layout.height}`);
    /// ```
    #[wasm_bindgen(js_name = getLayout)]
    pub fn layout(&self, node: u64) -> Result<JsLayout, JsValue> {
        match self.tree.layout(NodeId::from(node)) {
            Ok(l) => Ok(JsLayout::from(l)),
            Err(e) => Err(JsValue::from(JsTaffyError::from(e))),
        }
    }

    /// Gets the unrounded (fractional) layout for a node
    ///
    /// Returns the raw computed values before any rounding is applied.
    /// Useful when you need sub-pixel precision.
    ///
    /// @param node - The node ID
    ///
    /// @returns - An owned snapshot of the unrounded `Layout`; call `free()` when finished
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const nodeId = tree.newLeaf(new Style());
    /// const layout: Layout = tree.unroundedLayout(nodeId);
    /// console.log(`Exact width: ${layout.width}`);
    /// ```
    #[wasm_bindgen(js_name = unroundedLayout)]
    pub fn unrounded_layout(&self, node: u64) -> JsLayout {
        JsLayout::from(self.tree.unrounded_layout(NodeId::from(node)))
    }

    /// Gets detailed layout information for grid layouts
    ///
    /// @note
    /// This method is only available when the `detailed_layout_info`
    /// feature is enabled.
    ///
    /// @param node - The node ID
    ///
    /// @returns - An object containing the last stored rows, columns, and items,
    /// or `null` if no grid details have been stored. A childless grid uses leaf
    /// layout and does not produce grid details. Previously stored details can
    /// remain after changing display mode or removing children, so read this
    /// after computing a current grid container with children.
    ///
    /// @remarks The node ID must identify a live node in this tree.
    #[cfg(feature = "detailed_layout_info")]
    #[wasm_bindgen(js_name = detailedLayoutInfo)]
    pub fn detailed_layout_info(&self, node: u64) -> Result<JsDetailedLayoutInfo, JsValue> {
        match self.tree.detailed_layout_info(NodeId::from(node)) {
            DetailedLayoutInfo::Grid(info) => {
                let dto = DetailedGridInfoDto {
                    rows: grid_tracks_info_dto(&info.rows),
                    columns: grid_tracks_info_dto(&info.columns),
                    items: info
                        .items
                        .iter()
                        .map(|item| DetailedGridItemsInfoDto {
                            row_start: item.row_start,
                            row_end: item.row_end,
                            column_start: item.column_start,
                            column_end: item.column_end,
                        })
                        .collect(),
                };
                Ok(serde_wasm_bindgen::to_value(&dto)
                    .unwrap_or(JsValue::NULL)
                    .unchecked_into())
            }
            DetailedLayoutInfo::None => Ok(JsValue::NULL.unchecked_into()),
        }
    }

    // =========================================================================
    // Dirty Tracking
    // =========================================================================

    /// Marks a node as dirty (requiring re-layout)
    ///
    /// Use this when a node's content has changed but its style hasn't.
    /// For example, when text content changes and needs remeasuring.
    ///
    /// @param node - The node ID to mark dirty
    ///
    /// @remarks The node ID must identify a live node in this tree.
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const content = { text: "Original text" };
    /// const style = new Style();
    /// const nodeId = tree.newLeafWithContext(style, content);
    /// style.free();
    /// const availableSpace = { width: 100, height: 100 };
    /// const measureText: MeasureFunction = (known, _available, _node, context, measuredStyle) => {
    ///   measuredStyle.free();
    ///   // Approximate single-line text using an 8-pixel character width.
    ///   return {
    ///     width: known.width ?? (context?.text?.length ?? 0) * 8,
    ///     height: known.height ?? 16
    ///   };
    /// };
    /// tree.computeLayoutWithMeasure(nodeId, availableSpace, measureText);
    ///
    /// // Mutating the attached object does not automatically invalidate measurement.
    /// content.text = "Updated, longer text";
    /// tree.markDirty(nodeId);
    /// tree.computeLayoutWithMeasure(nodeId, availableSpace, measureText);
    /// tree.free();
    /// ```
    #[wasm_bindgen(js_name = markDirty)]
    pub fn mark_dirty(&mut self, node: u64) -> Result<(), JsValue> {
        map_void_result(self.tree.mark_dirty(NodeId::from(node)))
    }

    /// Checks if a node is dirty (needs re-layout)
    ///
    /// A node is dirty if its style or content has changed since the last
    /// layout computation.
    ///
    /// @param node - The node ID to check
    ///
    /// @returns - true if dirty, false otherwise
    ///
    /// @remarks The node ID must identify a live node in this tree.
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const rootId = tree.newLeaf(new Style());
    /// const nodeId = rootId;
    /// const availableSpace = { width: 100, height: 100 };
    ///
    /// if (tree.dirty(nodeId)) {
    ///   tree.computeLayout(rootId, availableSpace);
    /// }
    /// ```
    #[wasm_bindgen(js_name = dirty)]
    pub fn dirty(&self, node: u64) -> Result<bool, JsValue> {
        map_bool_result(self.tree.dirty(NodeId::from(node)))
    }

    // =========================================================================
    // Layout Computation
    // =========================================================================

    /// Updates the stored layout of the provided node and its children
    ///
    /// The measure function is called for leaf nodes (nodes without children) that
    /// require measurement according to the layout algorithm (Flexbox/Grid).
    /// For example, this is used for text nodes or other content that has intrinsic size.
    /// The callback returns content dimensions; the layout engine applies padding,
    /// borders, constraints, and aspect ratios. Cached measurements may skip calls.
    /// Mutating context fields or replacing the callback requires `markDirty()`
    /// on affected nodes. Callback exceptions or invalid return values currently
    /// produce a zero content measurement instead of propagating an exception.
    /// A context is optional; callbacks for nodes without one receive `undefined`.
    ///
    /// @param node - The root node ID to compute layout for
    /// @param availableSpace - The available space constraints
    /// @param measureFunc - A function that measures leaf node content
    ///
    /// @throws `TaffyError` if available space cannot be parsed
    /// @remarks The node ID must identify a live node in this tree.
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const textStyle = new Style();
    /// const textNode = tree.newLeafWithContext(textStyle, { text: "Hello, measured text" });
    /// textStyle.free();
    ///
    /// tree.computeLayoutWithMeasure(
    ///   textNode,
    ///   { width: 800, height: "max-content" },
    ///   (known, available, node, context, style) => {
    ///     style.free(); // This example only needs the attached text.
    ///     const text: string = context?.text ?? "";
    ///     // Approximate monospaced measurement; real text uses font metrics.
    ///     const naturalWidth = text.length * 8;
    ///     const minimumWidth = Math.max(0, ...text.split(/\s+/).map(word => word.length * 8));
    ///     const width = known.width ?? (
    ///       available.width === "min-content" ? minimumWidth :
    ///       available.width === "max-content" ? naturalWidth :
    ///       Math.min(naturalWidth, Math.max(0, available.width))
    ///     );
    ///     const lines = text.length === 0 ? 0 : Math.ceil(naturalWidth / Math.max(8, width));
    ///     return { width, height: known.height ?? lines * 16 };
    ///   }
    /// );
    /// const layout = tree.getLayout(textNode);
    /// console.log(layout.width, layout.height);
    /// layout.free();
    /// tree.free();
    /// ```
    #[wasm_bindgen(js_name = computeLayoutWithMeasure)]
    pub fn compute_layout_with_measure(
        &mut self,
        node: u64,
        #[wasm_bindgen(js_name = "availableSpace")] available_space: JsAvailableSizeArg,
        #[wasm_bindgen(js_name = "measureFunc")] measure_func: JsMeasureFunctionArg,
    ) -> Result<(), JsValue> {
        let js_value: JsValue = available_space.unchecked_into();
        let js_space = match serde_wasm_bindgen::from_value::<AvailableSizeDto>(js_value) {
            Ok(s) => s,
            Err(_) => {
                return Err(JsValue::from(JsTaffyError::from(
                    NativeTaffyError::InvalidInputNode(NodeId::from(node)),
                )));
            }
        };

        let space: Size<AvailableSpace> = js_space.into();
        let func: js_sys::Function = measure_func.unchecked_into();
        let explicit_grid_template_area_counts = &self.explicit_grid_template_area_counts;
        let measure = |inputs: LayoutInput,
                       node: NodeId,
                       context: Option<&mut JsValue>,
                       native_style: &TaffyStyle::Style|
         -> LayoutOutput {
            // The public callback measures content. Keep upstream's leaf sizing
            // around it so padding, borders, constraints and sizing modes apply.
            taffy::compute_leaf_layout(
                inputs,
                native_style,
                |_, _| 0.0,
                |known_dimensions, available_space| {
                    let this = JsValue::NULL;
                    let known_val =
                        serde_wasm_bindgen::to_value(&known_dimensions).unwrap_or(JsValue::NULL);
                    let available_dto = AvailableSizeDto {
                        width: available_space.width.into(),
                        height: available_space.height.into(),
                    };
                    let available_val =
                        serde_wasm_bindgen::to_value(&available_dto).unwrap_or(JsValue::NULL);
                    let ctx = context.cloned().unwrap_or(JsValue::UNDEFINED);
                    let style = JsStyle {
                        inner: native_style.clone(),
                        explicit_grid_template_area_counts: explicit_grid_template_area_counts
                            .get(&u64::from(node))
                            .copied()
                            .unwrap_or_default(),
                    };
                    let style_val = JsValue::from(style);
                    let node_val = JsValue::from(u64::from(node));
                    let args = js_sys::Array::new();
                    args.push(&known_val);
                    args.push(&available_val);
                    args.push(&node_val);
                    args.push(&ctx);
                    args.push(&style_val);
                    let result_val = func.apply(&this, &args).unwrap_or(JsValue::UNDEFINED);
                    serde_wasm_bindgen::from_value(result_val).unwrap_or(Size::ZERO)
                },
            )
        };
        map_void_result(
            self.tree
                .compute_layout_with_measure(NodeId::from(node), space, measure),
        )
    }

    /// Computes the layout for a subtree
    ///
    /// This is the main layout computation method. Call this on the root node
    /// to compute layouts for all nodes in the tree.
    ///
    /// @param node - The root node ID to compute layout for
    /// @param availableSpace - The available space constraints
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const rootId = tree.newLeaf(new Style());
    ///
    /// // Fixed size container
    /// tree.computeLayout(rootId, { width: 800, height: 600 });
    ///
    /// // Flexible width, fixed height
    /// tree.computeLayout(rootId, { width: "max-content", height: 600 });
    ///
    /// // Minimum content size
    /// tree.computeLayout(rootId, { width: "min-content", height: "min-content" });
    /// ```
    ///
    /// @throws `TaffyError` if available space cannot be parsed
    /// @remarks The node ID must identify a live node in this tree.
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const rootId = tree.newLeaf(new Style());
    /// tree.computeLayout(rootId, { width: 800, height: 600 });
    /// ```
    #[wasm_bindgen(js_name = computeLayout)]
    pub fn compute_layout(
        &mut self,
        node: u64,
        #[wasm_bindgen(js_name = "availableSpace")] available_space: JsAvailableSizeArg,
    ) -> Result<(), JsValue> {
        let js_value: JsValue = available_space.unchecked_into();
        match serde_wasm_bindgen::from_value::<AvailableSizeDto>(js_value) {
            Ok(js_space) => {
                let space: Size<AvailableSpace> = js_space.into();
                map_void_result(self.tree.compute_layout(NodeId::from(node), space))
            }
            Err(_) => Err(JsValue::from(JsTaffyError::from(
                NativeTaffyError::InvalidInputNode(NodeId::from(node)),
            ))),
        }
    }

    // =========================================================================
    // Utilities
    // =========================================================================

    /// Returns a text representation of the tree structure for debugging
    ///
    /// Formats the subtree starting from the given node. Pass the returned
    /// string to `console.log()` to print it.
    ///
    /// @param node - The root node ID to print from
    ///
    /// @returns - A string representation of the tree structure
    ///
    /// @example
    /// ```typescript
    /// const tree = new TaffyTree();
    /// const rootId = tree.newLeaf(new Style());
    /// const output = tree.printTree(rootId);
    /// console.log(output);
    /// ```
    #[wasm_bindgen(js_name = printTree)]
    pub fn print_tree(&self, node: u64) -> String {
        let tree = &self.tree;
        let root_id = NodeId::from(node);

        fn print_node(
            tree: &TaffyTree<JsValue>,
            node_id: NodeId,
            has_sibling: bool,
            lines_string: String,
        ) -> String {
            let layout = tree.get_final_layout(node_id);
            let display = tree.get_debug_label(node_id);
            let num_children = tree.child_count(node_id);

            let fork_string = if has_sibling {
                "├── "
            } else {
                "└── "
            };

            let result = format!(
                "{lines}{fork} {display} [x: {x:<4} y: {y:<4} w: {w:<4} h: {h:<4} content_w: {cw:<4} content_h: {ch:<4} border: l:{bl} r:{br} t:{bt} b:{bb}, padding: l:{pl} r:{pr} t:{pt} b:{pb}] ({key:?})\n",
                lines = lines_string,
                fork = fork_string,
                display = display,
                x = layout.location.x,
                y = layout.location.y,
                w = layout.size.width,
                h = layout.size.height,
                cw = layout.scrollable_overflow_rect.right,
                ch = layout.scrollable_overflow_rect.bottom,
                bl = layout.border.left,
                br = layout.border.right,
                bt = layout.border.top,
                bb = layout.border.bottom,
                pl = layout.padding.left,
                pr = layout.padding.right,
                pt = layout.padding.top,
                pb = layout.padding.bottom,
                key = u64::from(node_id),
            );

            let bar = if has_sibling { "│   " } else { "    " };
            let new_string = lines_string + bar;

            let mut child_output = String::new();
            for (index, child) in tree.child_ids(node_id).enumerate() {
                let has_sibling = index < num_children - 1;
                child_output.push_str(&print_node(tree, child, has_sibling, new_string.clone()));
            }

            result + &child_output
        }

        print_node(tree, root_id, false, String::new())
    }
}
