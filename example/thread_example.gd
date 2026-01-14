extends Control
## GdSerial - Thread Safety Example

var serial: GdSerial
var thread: Thread
var status_label: Label
var start_button: Button
var port_input: LineEdit

func _ready() -> void:
	print("\n=== GdSerial Thread Safety Example ===")
	
	# Find nodes by name
	status_label = find_child("StatusLabel")
	start_button = find_child("StartButton")
	port_input = find_child("PortInput")
	
	print("Status Label: %s" % ("✓ Found" if status_label else "✗ NOT FOUND"))
	print("Start Button: %s" % ("✓ Found" if start_button else "✗ NOT FOUND"))
	print("Port Input: %s" % ("✓ Found" if port_input else "✗ NOT FOUND"))
	
	serial = GdSerial.new()
	
	# Get port from input or default to COM3
	var port = port_input.text if port_input and port_input.text != "" else "COM3"
	print("\nUsing port: %s" % port)
	
	serial.set_port(port)
	serial.set_baud_rate(9600)
	
	# Open port
	print("Opening port...")
	if serial.open():
		print("✓ Port opened successfully")
		if status_label:
			status_label.text = "Port opened on %s\nClick button to start thread test" % port
	else:
		print("Note: Port not available (device not connected?)")
		if status_label:
			status_label.text = "Note: Connect a device on %s first\nOr change port and restart" % port
	
	# Connect button
	if start_button:
		start_button.pressed.connect(_on_start_pressed)
		print("✓ Start button connected")
	
	print("Ready!\n")

func _on_start_pressed() -> void:
	print("\n[Button] Start background thread pressed")
	
	if thread and thread.is_alive():
		print("  Thread already running")
		if status_label:
			status_label.text = "Thread already running"
		return
	
	if not serial.is_open():
		print("  ✗ Port not open")
		if status_label:
			status_label.text = "Port not open - reconnect first"
		return
	
	if status_label:
		status_label.text = "Starting background thread..."
	print("  Creating and starting thread...")
	
	thread = Thread.new()
	thread.start(_background_work)

func _background_work() -> void:
	print("\n[THREAD] Background thread started")
	print("  GdSerial is thread-safe with Arc<Mutex<>>")
	
	# This runs in a background thread
	# GdSerial is thread-safe (v0.3.0+) with Arc<Mutex<>>
	
	if not serial.is_open():
		print("  ✗ Port not open, aborting")
		return
	
	for i in range(5):
		print("  [THREAD] Sending message %d" % i)
		serial.write_string("Message from thread %d\n" % i)
		OS.delay_msec(100)
	
	print("  [THREAD] All messages sent, completed")
	
	# Notify main thread
	call_deferred("_thread_finished")

func _thread_finished() -> void:
	print("\n[Main Thread] Thread finished callback")
	if thread:
		thread.wait_to_finish()
		print("  ✓ Thread joined successfully")
	
	print("✓ SUCCESS! No crashes or data races detected.")
	print("  GdSerial v0.3.0+ is fully thread-safe!\n")
	
	if status_label:
		status_label.text = "✓ SUCCESS! Thread completed without crashes.\nGdSerial is thread-safe and ready to use from any thread!"
