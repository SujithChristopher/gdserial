extends Control
## GdSerial v0.3.0 Example - Thread-Safe Serial Communication
## All serial operations are synchronized via Arc<Mutex<>>

var serial = GdSerial.new()
var lines_received: Array = []
var last_port: String = "COM3"

func _ready() -> void:
	print("\n=== GdSerial v0.3.0 Example ===")
	print("✓ Thread-safe with Arc<Mutex<>> interior mutability")
	print("✓ All serial methods are safe from any thread")
	print("✓ Lock overhead: ~10-20ns (negligible)\n")

	# Configure serial port
	serial.set_port(last_port)
	serial.set_baud_rate(9600)
	serial.set_timeout(100)

	# Try to open port
	if serial.open():
		print("✓ Serial port %s opened successfully" % last_port)
		update_status("Connected to %s" % last_port, Color.GREEN)
	else:
		print("✗ Failed to open port %s" % last_port)
		print("  Available ports:")
		var ports = serial.list_ports()
		for i in range(ports.size()):
			var port = ports[i]
			print("    - %s (%s) - %s" % [port["port_name"], port["port_type"], port["device_name"]])
		update_status("Failed to open port. Check console.", Color.RED)

	# Connect button signals
	var open_btn = find_child("OpenButton")
	var close_btn = find_child("CloseButton")
	var read_btn = find_child("ReadButton")
	var send_btn = find_child("SendButton")
	var clear_btn = find_child("ClearButton")

	if open_btn:
		open_btn.pressed.connect(_on_open_pressed)
	if close_btn:
		close_btn.pressed.connect(_on_close_pressed)
	if read_btn:
		read_btn.pressed.connect(_on_read_pressed)
	if send_btn:
		send_btn.pressed.connect(_on_send_pressed)
	if clear_btn:
		clear_btn.pressed.connect(_on_clear_pressed)

func _process(delta: float) -> void:
	# Update UI - all from main thread
	var bytes_label = find_child("BytesLabel")
	var status_label = find_child("StatusLabel")

	if serial.is_open():
		if bytes_label:
			bytes_label.text = "Bytes available: %d" % serial.bytes_available()
		if status_label and status_label.text.contains("Disconnected"):
			update_status("Connected to %s" % last_port, Color.GREEN)

		# Continuously read available data (main thread is safe!)
		if serial.bytes_available() > 0:
			var line = serial.readline()
			if line != "":
				lines_received.append("[%s] %s" % [Time.get_ticks_msec(), line])
				print("✓ Read: %s" % line)
				update_display()
	else:
		if bytes_label:
			bytes_label.text = "Port closed"
		if status_label and not status_label.text.contains("Disconnected"):
			update_status("Disconnected", Color.RED)

func _on_open_pressed() -> void:
	print("\n[Button] Open pressed")
	var port_input = find_child("PortInput")

	if port_input and port_input.text != "":
		last_port = port_input.text
		serial.set_port(last_port)

	if not serial.is_open():
		if serial.open():
			print("✓ Port %s opened" % last_port)
			update_status("Connected to %s" % last_port, Color.GREEN)
		else:
			print("✗ Failed to open port %s" % last_port)
			update_status("Failed to open port", Color.RED)
	else:
		print("! Port already open")

func _on_close_pressed() -> void:
	print("\n[Button] Close pressed")
	if serial.is_open():
		serial.close()
		print("✓ Port closed")
		update_status("Disconnected", Color.RED)
	else:
		print("! Port not open")

func _on_read_pressed() -> void:
	print("\n[Button] Read pressed")
	if not serial.is_open():
		print("✗ Port not open")
		return

	if serial.bytes_available() > 0:
		var line = serial.readline()
		lines_received.append("[%s] %s" % [Time.get_ticks_msec(), line])
		print("✓ Read: %s" % line)
		update_display()
	else:
		print("! No data available")

func _on_send_pressed() -> void:
	print("\n[Button] Send pressed")
	var input = find_child("SendInput")

	if not input or input.text == "":
		print("! No input text")
		return

	if not serial.is_open():
		print("✗ Port not open")
		return

	var text = input.text
	if serial.write_string(text):
		print("✓ Sent: %s" % text)
		lines_received.append("[%s] SENT: %s" % [Time.get_ticks_msec(), text])
		input.text = ""
		update_display()
	else:
		print("✗ Failed to send")

func _on_clear_pressed() -> void:
	print("\n[Button] Clear pressed")
	lines_received.clear()
	update_display()

func update_display() -> void:
	var data_label = find_child("DataLabel")
	if data_label:
		# Show last 20 lines
		var recent = lines_received.slice(-20)
		data_label.text = "\n".join(recent)

func update_status(message: String, color: Color) -> void:
	var status_label = find_child("StatusLabel")
	if status_label:
		status_label.text = message
		status_label.add_theme_color_override("font_color", color)
