//! # Utility Functions Module
//!
//! This module provides utility functions used throughout the Taffy Layout crate.
//!
//! ## Functions
//!
//! - [`serialize`]: Converts Rust types to `JsValue` using serde-wasm-bindgen
//! - [`log`]: Outputs debug messages to the browser console
//!
//! ## Usage
//!
//! These utilities are primarily used internally by other modules but can be
//! helpful for debugging custom implementations.

use serde::Serialize;
use wasm_bindgen::prelude::*;

// =============================================================================
// Serialization
// =============================================================================

/// Serializes a Rust value to a JavaScript value
///
/// This function uses `serde-wasm-bindgen` to convert any serializable Rust type
/// to its JavaScript representation. It's used internally to convert complex types
/// like `Size`, `Rect`, and dimension values to JavaScript objects.
///
/// @typeParam T - Any type that implements `serde::Serialize`
///
/// @param val - A reference to the value to serialize
///
/// @returns - A `JsValue` representing the serialized data, or `null` if serialization fails
///
/// @example
/// ```rust,ignore
/// let size = SizeDto { width: 100.0, height: 50.0 };
/// let js_value = serialize(&size);
/// // js_value is now { width: 100, height: 50 } in JavaScript
/// ```
///
/// @example
/// ```typescript
/// const size = { width: 100, height: 50 };
/// const js_value = serialize(size);
/// // js_value is now { width: 100, height: 50 } in JavaScript
/// ```
///
/// @performance
/// This function allocates a new JavaScript object on each call. For hot paths,
/// consider caching the result or using more direct conversion methods.
pub fn serialize<T: Serialize + ?Sized>(val: &T) -> JsValue {
    serde_wasm_bindgen::to_value(val).unwrap_or(JsValue::NULL)
}

// =============================================================================
// Console Logging
// =============================================================================

#[wasm_bindgen]
extern "C" {
    /// Logs a message to the browser console
    ///
    /// This function is a direct binding to JavaScript's `console.log()`. It's useful
    /// for debugging during development but should be removed or gated behind a
    /// feature flag in production builds.
    ///
    /// @param s - The message to log
    ///
    /// @example
    /// ```rust,ignore
    /// log("Debug: processing node 123");
    /// log(&format!("Error: {}", error_message));
    /// ```
    ///
    /// @example
    /// ```typescript
    /// console.log(s);
    /// ```
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}
