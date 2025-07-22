@tool
extends EditorPlugin

func _enter_tree():
	print("GdSerial plugin activated")

func _exit_tree():
	print("GdSerial plugin deactivated")