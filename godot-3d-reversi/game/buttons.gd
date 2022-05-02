extends HBoxContainer

func _on_SettingBtn_mouse_entered():
	$AnimationPlayer.play("setting_rotate")

func _on_SettingBtn_mouse_exited():
	$AnimationPlayer.play_backwards("setting_rotate")

func _on_Regret_mouse_entered():
	$AnimationPlayer.play("regret_rotate")

func _on_Regret_mouse_exited():
	$AnimationPlayer.play_backwards("regret_rotate")
