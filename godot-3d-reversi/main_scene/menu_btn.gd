extends Button

var main_scene: Resource = null

func _ready():
	if OS.get_name() == "HTML5":
		JavaScript.eval("console.log('done');")

func _pressed():
	get_node("/root/Menu/VBox/EnBtn").hide()
	get_node("/root/Menu/VBox/ChBtn").hide()
	
	if self.text == "English":
		TranslationServer.set_locale("en")
	else:
		TranslationServer.set_locale("zh-TW")
	$Fade.play("fade_in")
	
func load_main():
	main_scene = load("res://game/game.tscn")
	main_scene.instance()

func _on_AnimationPlayer_animation_finished(_anim_name):
	load_main()
	var err = get_tree().change_scene_to(main_scene)
	assert(err == OK)
