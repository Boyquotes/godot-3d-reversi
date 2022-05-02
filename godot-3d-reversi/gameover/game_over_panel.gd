extends Control

signal show_newgame_panel

func _ready():
	self.rect_scale = Vector2(0, 0)
	self.hide()

func _on_NewGame_pressed():
	$Fold.play("fold")
	emit_signal("show_newgame_panel")
	get_tree().paused = false

func _on_Close_pressed():
	$Fold.play("fold")
	get_tree().paused = false

func delay_show(black_result: int, white_result: int):
	# 0.8 sec
	$DelayTimer.wait_time = 0.8
	$DelayTimer.start()
	
	$HBox/NewGame.grab_focus()
	
	if black_result > white_result:
		$Result.text = "BlackWon"
	elif white_result > black_result:
		$Result.text = "WhiteWon"
	else:
		$Result.text = "Draw"

func _on_Fold_animation_finished(anim_name):
	if anim_name == "fold":
		self.hide()

func _on_Fold_animation_started(anim_name):
	if anim_name == "unfold":
		self.show()

func _on_DelayTimer_timeout():
	$Fold.play("unfold")
