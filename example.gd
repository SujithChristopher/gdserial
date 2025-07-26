# Example GDScript code showing how to use GdSerial
# This demonstrates the PySerial-like functionality for Godot

extends Node

var serial: GdSerial

func _ready():
	# Create a new GdSerial instance
	serial = GdSerial.new()
	
	# List all available COM ports with device names
	print("Available COM ports:")
	var ports = serial.list_ports()
	for i in range(ports.size()):
		var port_info = ports[i]
		print("Port: ", port_info["port_name"], " (", port_info["device_name"], ") - Type: ", port_info["port_type"])
	
	# Example: Get device name for a specific port
	if ports.size() > 0:
		var first_port = ports[0]["port_name"]
		var device_name = serial.get_port_device_name(first_port)
		print("Device name for ", first_port, ": ", device_name)
		print("Note: Device names use USB product descriptor when available")
		print("      Falls back to manufacturer + device type for known VIDs")
	
	# Configure serial port settings
	serial.set_port("COM3")  # Change this to your actual port
	serial.set_baud_rate(9600)
	serial.set_timeout(1000)  # 1 second timeout
	
	# Open the port
	if serial.open():
		print("Serial port opened successfully!")
		
		# Example: Write string data
		serial.write_string("Hello Arduino!")
		
		# Example: Write line with newline
		serial.writeline("AT+VERSION?")
		
		# Example: Read available data
		await get_tree().create_timer(0.1).timeout  # Wait for response
		if serial.bytes_available() > 0:
			var response = serial.read_string(100)
			print("Received: ", response)
		
		# Example: Read line by line
		var line = serial.readline()
		if line != "":
			print("Line received: ", line)
		
		# Close the port when done
		serial.close()
	else:
		print("Failed to open serial port")

# Example function for continuous monitoring
func monitor_serial():
	if not serial.is_open():
		return
	
	while serial.bytes_available() > 0:
		var data = serial.readline()
		if data != "":
			print("Monitor: ", data)
			# Process your data here