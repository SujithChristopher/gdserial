# Issue #12 Investigation: Thread Safety in GdSerial

## Problem Summary
User reports application crashes when trying to use GdSerial in a background thread. The crash occurs when attempting to offload serial port operations to prevent blocking the main thread.

## Root Causes Identified

### 1. Mutable Access Requirements
- GdSerial methods use `&mut self` which requires exclusive mutable access
- This is incompatible with concurrent thread access
- Rust prevents multiple threads from holding mutable references to the same object

### 2. Non-Thread-Safe Port State
- `port: Option<Box<dyn SerialPort>>` is wrapped in the struct without synchronization
- `SerialPort` trait is not guaranteed to be `Send + Sync`
- Multiple threads cannot safely access the same port instance

### 3. Godot Scene Tree Restrictions
- Godot only allows main thread modifications to scene tree nodes
- User code attempts to modify `progress_bar.value` and `label.text` from worker thread
- This violates Godot's threading model and causes crashes

### 4. No Thread-Safe Communication Mechanisms
- GdSerial doesn't use `call_thread_safe()` or `set_thread_safe()` methods
- These are required for communicating with Godot nodes from background threads

## Godot Threading Architecture
- Only main thread can access scene tree and node properties
- Background threads can call `call_thread_safe()` to queue operations for main thread
- Godot uses work queues to safely marshal operations back to main thread

## Current Code Issues
In `src/lib.rs`:
- All methods take `&mut self` (lines 191-476)
- No synchronization primitives (Mutex, Arc, etc.)
- Methods directly modify internal state without thread safety
- No queue-based communication mechanism

## Solution Approach Needed
1. Wrap internal state in `Arc<Mutex<>>` for thread-safe access
2. Refactor methods to accept `&self` instead of `&mut self` where possible
3. Use interior mutability pattern (Mutex) for shared mutable state
4. Provide documentation on proper thread usage with Godot
5. Add examples showing correct threading pattern with GUI updates

## Key Constraint
- Must maintain backward compatibility with existing API where possible
- Godot scene tree updates must happen on main thread (user responsibility)
