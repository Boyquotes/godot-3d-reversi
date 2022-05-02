extends Control

var loader: ResourceInteractiveLoader

func _ready():
	if OS.get_name() == "HTML5":
		JavaScript.eval("console.log('done');")

func load_game():
	$VBox/ChBtn.hide()
	$VBox/EnBtn.hide()
	loader = ResourceLoader.load_interactive("res://game/game.tscn")
	assert(loader)
	$ColorRect/Fade.play("fade_in")

func _on_Fade_animation_finished(_anim_name):
	var err := loader.wait()
	assert(err == ERR_FILE_EOF)
	err = get_tree().change_scene_to(loader.get_resource())
	assert(err == OK)

func _on_EnBtn_pressed():
	TranslationServer.set_locale("en")
	load_game()

func _on_ChBtn_pressed():
	TranslationServer.set_locale("zh-TW")
	load_game()
