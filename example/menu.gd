extends Control
## Menu to choose between examples

func _ready() -> void:
	var simple_btn = find_child("SimpleButton")
	var thread_btn = find_child("ThreadButton")

	if simple_btn:
		simple_btn.pressed.connect(_on_simple_pressed)
	if thread_btn:
		thread_btn.pressed.connect(_on_thread_pressed)

func _on_simple_pressed() -> void:
	print("Loading simple example...")
	get_tree().change_scene_to_file("res://main.tscn")

func _on_thread_pressed() -> void:
	print("Loading thread example...")
	get_tree().change_scene_to_file("res://thread_example.tscn")
