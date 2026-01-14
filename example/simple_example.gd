extends Control
## GdSerial - Simple Example

var serial = GdSerial.new()
var port_input: LineEdit
var send_input: LineEdit
var output_label: Label
var open_button: Button
var close_button: Button
var send_button: Button
var read_button: Button

func _ready() -> void:
	print("\n=== GdSerial Simple Example ===")
	
	# Find all nodes by name
	port_input = find_child("PortInput")
	send_input = find_child("SendInput")
	output_label = find_child("Output")
	open_button = find_child("OpenButton")
	close_button = find_child("CloseButton")
	send_button = find_child("SendButton")
	read_button = find_child("ReadButton")
	
	print("Port Input: %s" % ("✓ Found" if port_input else "✗ NOT FOUND"))
	print("Send Input: %s" % ("✓ Found" if send_input else "✗ NOT FOUND"))
	print("Output Label: %s" % ("✓ Found" if output_label else "✗ NOT FOUND"))
	print("Open Button: %s" % ("✓ Found" if open_button else "✗ NOT FOUND"))
	print("Close Button: %s" % ("✓ Found" if close_button else "✗ NOT FOUND"))
	print("Send Button: %s" % ("✓ Found" if send_button else "✗ NOT FOUND"))
	print("Read Button: %s" % ("✓ Found" if read_button else "✗ NOT FOUND"))
	
	# Connect button signals
	if open_button:
		open_button.pressed.connect(_on_open_pressed)
		print("✓ Open button connected")
	if close_button:
		close_button.pressed.connect(_on_close_pressed)
		print("✓ Close button connected")
	if send_button:
		send_button.pressed.connect(_on_send_pressed)
		print("✓ Send button connected")
	if read_button:
		read_button.pressed.connect(_on_read_pressed)
		print("✓ Read button connected")
	
	print("Ready!\n")

func _process(delta: float) -> void:
	# Continuously read data if port is open
	if serial.is_open() and serial.bytes_available() > 0:
		var data = serial.readline()
		if data != "":
			print("  ↓ Received: %s" % data)
			if output_label:
				output_label.text = "Received: %s" % data

func _on_open_pressed() -> void:
	print("\n[Button] Open pressed")
	var port = port_input.text if port_input else "COM3"
	if port == "":
		port = "COM3"
	
	print("  Port: %s" % port)
	serial.set_port(port)
	serial.set_baud_rate(9600)
	
	if serial.open():
		print("  ✓ Port opened successfully")
		if output_label:
			output_label.text = "✓ Opened %s" % port
	else:
		print("  ✗ Failed to open port")
		if output_label:
			output_label.text = "✗ Failed to open %s" % port

func _on_close_pressed() -> void:
	print("\n[Button] Close pressed")
	serial.close()
	print("  ✓ Port closed")
	if output_label:
		output_label.text = "Port closed"

func _on_send_pressed() -> void:
	print("\n[Button] Send pressed")
	var text = send_input.text if send_input else ""
	if text == "":
		print("  No message entered")
		if output_label:
			output_label.text = "No message entered"
		return
	
	if not serial.is_open():
		print("  ✗ Port not open")
		if output_label:
			output_label.text = "Port not open"
		return
	
	print("  Sending: %s" % text)
	if serial.write_string(text + "\n"):
		print("  ✓ Message sent")
		if output_label:
			output_label.text = "Sent: %s" % text
		if send_input:
			send_input.text = ""
	else:
		print("  ✗ Failed to send")
		if output_label:
			output_label.text = "Failed to send"

func _on_read_pressed() -> void:
	print("\n[Button] Read pressed")
	if not serial.is_open():
		print("  ✗ Port not open")
		if output_label:
			output_label.text = "Port not open"
		return
	
	var data = serial.readline()
	if data != "":
		print("  ✓ Received: %s" % data)
		if output_label:
			output_label.text = "Received: %s" % data
	else:
		print("  No data available")
		if output_label:
			output_label.text = "No data available"
