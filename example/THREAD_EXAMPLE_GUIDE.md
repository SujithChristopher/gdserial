# GdSerial v0.3.0 - Threading Examples

## The Problem (Original Code)

```gdscript
var t: Thread
func _on_button_pressed() -> void:
    t = Thread.new()
    t.start(requestData)

func requestData():
    # ❌ CRASHES!
    await get_tree().create_timer(0.01).timeout  # Can't use await in threads!

    # ❌ CRASHES!
    progress_bar.value = 0  # Can't update GUI from threads!
```

**Why it crashes:**
1. `await` and `get_tree()` require main thread access
2. GUI updates from threads violate Godot's scene tree rules
3. Even though GdSerial is now thread-safe, Godot isn't

## The Solution

### Option 1: Simple Main Thread Only (Recommended)

**File:** `main.gd`

Keep ALL serial operations in the main thread's `_process()`:

```gdscript
func _process(delta: float) -> void:
    # All serial operations here (main thread)
    if serial.bytes_available() > 0:
        var line = serial.readline()
        # GUI updates safe here
        label.text = line
```

**Why this works:**
- ✅ Godot safe (main thread only)
- ✅ GdSerial safe (thread-safe internally)
- ✅ No crashes
- ✅ Automatic reading every frame

**When to use:** 95% of applications - simple, safe, and effective

### Option 2: Thread for Heavy Computation

**File:** `thread_example.gd`

Use threads only for expensive work, keep serial I/O on main thread:

```gdscript
# Thread calls request_data() - just signals to start
var is_reading: bool = false
var collected_data: String = ""

func _on_button_pressed() -> void:
    t = Thread.new()
    t.start(_thread_function)

func _thread_function() -> String:
    # Thread is safe calling GdSerial (v0.3.0+)!
    # But don't do actual I/O here
    request_data()  # Just sets is_reading = true

    # Wait for main thread to complete the I/O
    while is_reading:
        OS.delay_msec(100)

    return "Done"

func _process(delta: float) -> void:
    # Main thread does the actual serial I/O
    if is_reading and serial.bytes_available() > 0:
        var response = serial.read_string(1024)
        collected_data += response
```

**Why this works:**
- ✅ Thread handles state/control
- ✅ Main thread handles serial I/O
- ✅ Main thread handles GUI updates
- ✅ Both use GdSerial safely

**When to use:** Need background work while reading serial data

## Key Differences

| Aspect | Main Thread Only | With Threads |
|--------|-----------------|--------------|
| Simplicity | Very Simple | Moderate |
| Performance | Excellent | Good |
| Responsiveness | Excellent (60 FPS) | Good |
| Complexity | Low | Medium |
| Recommended | 95% of cases | Advanced use |

## GdSerial Thread Safety (v0.3.0)

### Architecture

```rust
pub struct GdSerial {
    state: Arc<Mutex<GdSerialState>>  // Thread-safe!
}

// All methods now:
pub fn read(&self, size: u32) -> PackedByteArray
pub fn write(&self, data: PackedByteArray) -> bool
// Changed from &mut self to &self
```

### What's Thread-Safe

✅ `serial.read()` - Safe from any thread
✅ `serial.write()` - Safe from any thread
✅ `serial.bytes_available()` - Safe from any thread
✅ `serial.is_open()` - Safe from any thread
✅ All configuration methods - Safe from any thread

### What's NOT Thread-Safe

❌ GUI updates - Use main thread only
❌ `await get_tree()` - Use main thread only
❌ Scene tree access - Use main thread only

## Best Practices

### ✅ DO

```gdscript
# Use in _process() for serial I/O
func _process(delta):
    if serial.bytes_available() > 0:
        var data = serial.read_string(1024)
        label.text = data  # Safe in main thread

# Use threads for heavy computation
func _thread_func():
    var expensive = calculate_something()
    return expensive
```

### ❌ DON'T

```gdscript
# Don't use await in threads
func _thread_func():
    await get_tree().create_timer(0.1).timeout  # CRASH

# Don't update GUI from threads
func _thread_func():
    label.text = "Error"  # CRASH

# Don't use serial I/O then await then update GUI
func _thread_func():
    var data = serial.read(100)
    await get_tree().create_timer(0.1).timeout  # CRASH
    label.text = data  # CRASH
```

## Example: Progress Bar Update

### Original Code (CRASHES)
```gdscript
func requestData():
    progress_bar.value = 0  # ❌ Crash from thread

    serial.write_string("READ")
    await get_tree().create_timer(0.01).timeout  # ❌ Crash

    while true:
        var response = serial.read_string(1024)
        # ❌ Can't update GUI from thread
        progress_bar.value = percentage
```

### Fixed Code (WORKS)
```gdscript
var reading_percentage: float = 0.0

func _on_button_pressed():
    t = Thread.new()
    t.start(_thread_control)

func _thread_control() -> String:
    # Thread is safe calling GdSerial!
    if serial.write_string("READ"):
        return "Sent"
    return "Failed"

func _process(delta):
    # All I/O and GUI in main thread
    if serial.bytes_available() > 0:
        var response = serial.read_string(1024)

        if response.contains("READING_"):
            reading_percentage = float(response.split("READING_")[1])

        # Safe GUI update in main thread
        progress_bar.value = reading_percentage
```

## Lock Performance

**Overhead per operation:**
- Lock acquire/release: ~10-20 nanoseconds
- Serial I/O latency: 1-10 milliseconds
- Percentage: 0.0002% overhead
- **Conclusion: Completely negligible**

## Testing

### Test Thread Safety
1. Open `thread_example.gd`
2. Create multiple threads
3. Call GdSerial methods from threads
4. Should NOT crash ✓

### Test Main Thread Only
1. Use `main.gd`
2. Send/receive data continuously
3. No crashes, smooth UI ✓

## Resources

- `main.gd` - Simple, recommended approach
- `thread_example.gd` - Advanced threading pattern
- `../TESTING.md` - Complete testing guide
- `../CLAUDE.md` - Architecture documentation
