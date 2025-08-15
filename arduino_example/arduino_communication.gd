extends Node

# Simple Arduino Communication Example for GdSerial
# This script demonstrates basic communication with the arduino_example.ino sketch

var serial: GdSerial
var port_name: String = "COM10"  # Change this to match your Arduino port

func _ready():
	serial = GdSerial.new()
	print("Arduino Communication Example")
	print("Available ports: ", serial.list_ports())
	
	# Configure serial port settings
	serial.set_port(port_name)
	serial.set_baud_rate(9600)
	serial.set_timeout(1000)
	
	# Try to connect
	if serial.open():
		print("Connected to Arduino on ", port_name)
		
		# Wait for Arduino to initialize
		await get_tree().create_timer(2.0).timeout
		
		# Test basic commands
		test_communication()
	else:
		print("Failed to connect to Arduino on ", port_name)

var last_check_time: float = 0.0

func _process(_delta):
	
	# Read any incoming data
	if serial.is_open():
		# Check connection health periodically
		if not serial.check_connection():
			print("Arduino disconnected!")
			return
			
		var available = serial.bytes_available()
		if available > 0:
			var data = serial.readline().strip_edges()
			if data.length() > 0:
				print("Arduino: ", data)

func test_communication():
	print("\n=== Testing Arduino Communication ===")
	
	# Test ping
	print("Testing ping...")
	send_command("ping")
	await get_tree().create_timer(0.5).timeout
	
	# Test hello
	print("Testing hello...")
	send_command("hello")
	await get_tree().create_timer(0.5).timeout
	
	# Test echo
	print("Testing echo...")
	send_command("echo Hello from Godot!")
	await get_tree().create_timer(0.5).timeout
	
	# Test counter
	print("Testing counter (3 times)...")
	for i in range(3):
		send_command("count")
		await get_tree().create_timer(0.5).timeout
	
	# Test time
	print("Testing time...")
	send_command("time")
	await get_tree().create_timer(0.5).timeout
	
	print("=== Communication test complete ===")

func send_command(command: String):
	if serial.is_open():
		if serial.println(command):
			print("Sent: ", command)
		else:
			print("Failed to send: ", command)
	else:
		print("Serial port not open")

func _exit_tree():
	if serial.is_open():
		serial.close()
		print("Serial port closed")