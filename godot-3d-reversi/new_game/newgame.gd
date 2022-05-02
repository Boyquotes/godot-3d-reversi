extends Node

const strength := ["weak", "medium", "strong"]

signal game_start(player1_role, player2_role, first_color)

func _ready():
	$Panel.rect_position = Vector2(0, -525)
	# at first time, hide buttons that can close panel
	$Panel/Close.hide()
	$Panel/HBox/Cancel.hide()
	
func do_hide():
	$Panel/AnimationPlayer.play_backwards("slide")
	get_tree().paused = false
	$Timer.start()

func do_show():
	$Panel/AnimationPlayer.play("slide")
	get_tree().paused = true

func _on_new_game_start():
	do_hide()
	var first := "black"
	var player1 := "human"
	if get_node("Panel/Container/P1/Role/Computer").pressed:
		player1 = strength[get_node("Panel/Container/P1/Level/Computer").value as int]
	var player2 := "human"
	if get_node("Panel/Container/P2/Role/Computer").pressed:
		player2 = strength[get_node("Panel/Container/P2/Level/Computer").value as int]
	emit_signal("game_start", player1, player2, first)

func _on_camera_on_position():
	do_show()

func _on_Timer_timeout():
	$Panel/Close.show()
	$Panel/HBox/Cancel.show()
