extends TextureButton

onready var _fullscreen_img := preload("res://picture/fullscreen.png")
onready var _fullscreen_hover_img := preload("res://picture/fullscreen_hover.png")
onready var _fullscreen_disable_img := preload("res://picture/fullscreen_disable.png")
onready var _fullscreen_disable_hover_img := preload("res://picture/fullscreen_disable_hover.png")

var _fullscreen := false

func _ready():
	var os_name := OS.get_name().to_lower()
	if os_name == "android" or os_name == "ios":
		self.queue_free()

func _input(event):
	if event.is_action_pressed("toggle_fullscreen"):
		_pressed()

func _pressed():
	_fullscreen = not _fullscreen
	OS.window_fullscreen = _fullscreen
	if _fullscreen:
		self.texture_normal = _fullscreen_disable_img
		self.texture_hover = _fullscreen_disable_hover_img
	else:
		self.texture_normal = _fullscreen_img
		self.texture_hover = _fullscreen_hover_img
