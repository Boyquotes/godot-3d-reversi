extends Node

onready var viewport := get_viewport()

func _ready():
	$Panel.rect_position = Vector2(140, -625)
	
func do_hide():
	$Panel/AnimationPlayer.play_backwards("slide")

func do_show():
	$Panel/AnimationPlayer.play("slide")

func _on_en_pressed():
	TranslationServer.set_locale("en")

func _on_zh_pressed():
	TranslationServer.set_locale("zh-TW")

func _on_fabric_pressed():
	pass # Replace with function body.

func _on_metal_pressed():
	pass # Replace with function body.

func _on_animation_fast_pressed():
	pass # Replace with function body.

func _on_animation_norm_pressed():
	pass # Replace with function body.

func _on_animation_disabled_pressed():
	pass # Replace with function body.

func _on_hint_on_pressed():
	pass # Replace with function body.

func _on_hint_off_pressed():
	pass # Replace with function body.

func _on_shadow_on_pressed():
	pass # Replace with function body.

func _on_shadow_off_pressed():
	pass # Replace with function body.

func _on_aa_high_pressed():
	viewport.msaa = Viewport.MSAA_16X
	viewport.fxaa = false

func _on_aa_medium_pressed():
	viewport.msaa = Viewport.MSAA_4X
	viewport.fxaa = false

func _on_aa_low_pressed():
	viewport.msaa = Viewport.MSAA_DISABLED
	viewport.fxaa = true
	#viewport.sharpen_intensity = 0.5

func _on_aa_disabled_pressed():
	viewport.msaa = Viewport.MSAA_DISABLED
	viewport.fxaa = false

func _on_vsync_on_pressed():
	OS.set_use_vsync(true)

func _on_vsync_off_pressed():
	OS.set_use_vsync(false)

func _on_framerate_120_pressed():
	Engine.target_fps = 120

func _on_framerate_60_pressed():
	Engine.target_fps = 60

func _on_framerate_30_pressed():
	Engine.target_fps = 30

func _on_framerate_default_pressed():
	Engine.target_fps = 0
