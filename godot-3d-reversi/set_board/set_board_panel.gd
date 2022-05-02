extends Control

signal assign_board(board, current_color)

func _ready():
	self.hide()
	self.rect_scale = Vector2(0, 0)
	for _i in range(63):
		$Grid.add_child($Grid/Button.duplicate())
	$Grid.get_child(0).grab_focus()

func _on_OK_pressed():
	var board := ""
	for btn in $Grid.get_children():
		board += btn.color()
	emit_signal("assign_board", board, $HBox/Button.color())
	$Fold.play("fold")
	get_tree().paused = false

func _on_Cancel_pressed():
	$Fold.play("fold")
	get_tree().paused = false

func _on_Fold_animation_finished(anim_name):
	if anim_name == "fold":
		self.hide()

func _on_Fold_animation_started(anim_name):
	if anim_name == "unfold":
		self.show()

func _on_SetBoardBtn_pressed():
	$Fold.play("unfold")
	get_tree().paused = true

func _on_debug_toggled(button_pressed):
	if button_pressed:
		get_node("../DebugLabel").show()
	else:
		get_node("../DebugLabel").hide()
