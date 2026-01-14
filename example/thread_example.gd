extends Control

signal update_val_bar(new_val: float)

@onready var progress_bar: ProgressBar = $"../ProgressBar"
@onready var label: Label = $Label

var serial: GdSerial
var t: Thread = null
var is_reading: bool = false
var reading_complete: bool = false
var collected_data: String = ""
var last_percentage: float = 0.0

func _ready() -> void:
	serial = GdSerial.new()
	print("Available ports:")
	var ports = serial.list_ports()
	update_val_bar.connect(updateBar)

	for i in range(ports.size()):
		var port_info = ports[i]
		print("- ", port_info["port_name"], " (", port_info["port_type"], ")")
		if port_info["port_name"].contains("ACM"):
			serial.set_port(port_info["port_name"])
			serial.set_baud_rate(115200)
			serial.set_timeout(1000)
			if serial.open():
				print("Port opened successfully!")
				break

func updateBar(new_val):
	progress_bar.value = new_val

func _process(delta: float) -> void:
	# All serial operations happen here in main thread
	if is_reading and serial.is_open():
		if serial.bytes_available() > 0:
			var response = serial.read_string(1024)
			print("Response: ", response)

			# Handle progress updates
			if response.contains("READING_"):
				var parts = response.split("READING_")
				if parts.size() > 1:
					last_percentage = float(parts[1])
					update_val_bar.emit(last_percentage)

			# Handle final data
			if response.contains("DATA_"):
				collected_data += response

				# Keep reading until we get END marker
				if not collected_data.contains("END"):
					# Will continue reading in next frame
					pass
				else:
					# Data complete!
					is_reading = false
					reading_complete = true

					# Process payload
					var payload: String = collected_data.trim_prefix("DATA_")
					payload = payload.rsplit("END")[0]
					payload = payload.strip_edges()  # Remove whitespace
					print("Final payload: ", payload)
					label.text = payload

func request_data() -> void:
	"""Request data from device (safe to call from any thread)"""
	print("\n[REQUEST DATA] Starting data request...")

	# Reset state
	is_reading = true
	reading_complete = false
	collected_data = ""
	last_percentage = 0.0

	# Update UI
	progress_bar.value = 0
	label.text = "Reading..."

	# Send command (thread-safe in v0.3.0!)
	if serial.is_open():
		serial.write_string("READ")
		print("[REQUEST DATA] Command sent, waiting for response...")
	else:
		print("[REQUEST DATA] Port not open!")
		is_reading = false

func _on_button_pressed() -> void:
	print("\n=== Button Pressed ===")

	if is_reading:
		print("! Already reading, ignoring request")
		return

	if t == null or not t.is_alive():
		# Create a simple thread that just calls request_data
		t = Thread.new()
		t.start(_thread_function)
	else:
		print("! Thread already running")

## Simple thread function - just calls the main function
func _thread_function() -> String:
	print("[THREAD] Started")

	# This is NOW SAFE to call from a thread (v0.3.0+)!
	request_data()

	print("[THREAD] Waiting for completion...")
	# Wait for reading to complete (from main thread)
	var timeout = 0
	while is_reading and timeout < 300:  # 30 second timeout
		OS.delay_msec(10)
		timeout += 1

	if reading_complete:
		print("[THREAD] ✓ Reading completed successfully")
		return "Success"
	else:
		print("[THREAD] ✗ Reading timeout or failed")
		is_reading = false
		return "Timeout"
