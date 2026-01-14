extends Control
## Simple example: Thread-safe GdSerial calls from background thread
## v0.3.0: All GdSerial methods are safe from any thread!

var serial: GdSerial
var thread: Thread = null

func _ready() -> void:
	serial = GdSerial.new()

	# Find and open first available port
	var ports = serial.list_ports()
	print("Available ports: %d" % ports.size())

	for port_info in ports:
		print("  - %s" % port_info["port_name"])
		if serial.open():
			break
		serial.set_port(port_info["port_name"])

func _process(delta: float) -> void:
	# Main thread handles all GUI updates
	if serial.is_open() and serial.bytes_available() > 0:
		var data = serial.read_string(256)
		print("✓ Received: %s" % data)

func _on_button_pressed() -> void:
	print("\n=== Starting background thread ===")

	if thread and thread.is_alive():
		print("! Thread already running")
		return

	thread = Thread.new()
	thread.start(_background_work)

## Background thread - can call GdSerial safely (v0.3.0+)!
func _background_work() -> String:
	print("[THREAD] Started - calling GdSerial from background thread")

	if not serial.is_open():
		return "Port not open"

	# This is NOW SAFE! GdSerial is thread-safe with Arc<Mutex<>>
	for i in range(5):
		if serial.write_string("Test %d\n" % i):
			print("[THREAD] ✓ Sent: Test %d" % i)
		OS.delay_msec(200)

	print("[THREAD] ✓ Completed")
	return "Done"
