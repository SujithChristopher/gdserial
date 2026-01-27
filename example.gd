extends Node

# --- CONFIGURATION CONSTANTS ---
const PARITY_NONE = 0
const PARITY_ODD = 1
const PARITY_EVEN = 2

const STOP_BITS_ONE = 1
const STOP_BITS_TWO = 2

const FLOW_CONTROL_NONE = 0
const FLOW_CONTROL_SOFTWARE = 1
const FLOW_CONTROL_HARDWARE = 2

# --- BASIC USAGE (Single port, synchronous) ---
var serial: GdSerial

# --- ADVANCED USAGE (Multi-port, Asynchronous signals) ---
var manager: GdSerialManager

func _ready() -> void:
	# 1. SIMPLE USAGE EXAMPLE
	run_simple_example()
	
	# 2. ADVANCED USAGE EXAMPLE (Async)
	run_manager_example()

## Basic usage showing how to list, open and read a single port
func run_simple_example() -> void:
	print("--- Running Simple Example ---")
	serial = GdSerial.new()
	
	# List all available COM ports with detailed info
	print("Available COM ports:")
	var ports: Dictionary = serial.list_ports()
	for i: int in ports:
		var port_info: Dictionary = ports[i]
		print("Port: ", port_info["port_name"], " (", port_info["device_name"], ") - Type: ", port_info["port_type"])
	
	# Configure serial port settings
	serial.set_port("COM3") # Change this to your actual port
	serial.set_baud_rate(9600)
	serial.set_data_bits(8)
	serial.set_parity(PARITY_NONE)
	serial.set_stop_bits(STOP_BITS_ONE)
	serial.set_flow_control(FLOW_CONTROL_NONE)
	serial.set_timeout(1000) # 1 second timeout
	
	# Open the port
	if serial.open():
		print("Serial port opened successfully!")
		
		# Write string data
		serial.write_string("Hello Arduino!")
		
		# Write line with newline
		serial.writeline("AT+VERSION?")
		
		# Wait for response (Polling approach)
		await get_tree().create_timer(0.2).timeout
		if serial.bytes_available() > 0:
			var response := serial.read_string(100)
			print("Received: ", response)
		
		# Close the port
		serial.close()
	else:
		print("Failed to open serial port (Simple)")

## Advanced usage using GdSerialManager for non-blocking multi-port access
func run_manager_example() -> void:
	print("\n--- Running Manager Example (Async) ---")
	manager = GdSerialManager.new()
	
	# Connect signals for async events
	manager.data_received.connect(_on_serial_data)
	manager.port_disconnected.connect(_on_serial_disconnect)
	
	# Open a port in async mode
	# This starts a dedicated reader thread automatically
	if manager.open_port("COM3", 9600, 1000):
		print("Port COM3 opened using Manager")
		
		# Write data to a specific port
		var data := "Test Manager".to_utf8_buffer()
		manager.write_port("COM3", data)
	else:
		print("Failed to open COM3 with Manager")

# For the Manager to work, you must call poll_events() periodically
# (e.g., in _process) to trigger signals and receive data
func _process(_delta: float) -> void:
	if manager:
		# poll_events() returns an Array of Dictionaries with event details
		# AND emits the data_received / port_disconnected signals
		var events: Array = manager.poll_events()
		for event: Dictionary in events:
			if event.has("disconnected"):
				print("Event Polling: Port ", event["port"], " disconnected!")

# Signal callback for new data
func _on_serial_data(port: String, data: PackedByteArray) -> void:
	var text := data.get_string_from_utf8()
	print("Async Data from ", port, ": ", text)

# Signal callback for disconnection
func _on_serial_disconnect(port: String) -> void:
	print("Critical: Port ", port, " was disconnected!")