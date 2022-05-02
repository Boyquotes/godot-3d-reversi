extends Node

func _ready():
	$Panel.rect_position = Vector2(0, -375)
	
func do_hide():
	$Panel/Animation.play_backwards("slide")
	get_tree().paused = false

func do_show():
	$Panel/Animation.play("slide")
	get_tree().paused = true
	$Panel/VBox/Continue.grab_focus()

func _on_main_menu_pressed():
	get_tree().paused = false
	var err := get_tree().change_scene("res://main_scene/main.tscn")
	assert(err == OK)

func _on_new_game_pressed():
	do_hide()
	get_node("../NewGame").do_show()

func _on_Setting_pressed():
	get_node("../Settings").do_show()
