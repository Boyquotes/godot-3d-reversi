extends Spatial

enum Stone {NONE = 0, BLACK = 1, WHITE = 2, BORDER = -1}

var fabric_black: Resource = preload("res://material/fabric_black.material")
var fabric_white: Resource = preload("res://material/fabric_white.material")

var metal_black: Resource = preload("res://material/metal_black.material")
var metal_white: Resource = preload("res://material/metal_white.material")

var self_color = Stone.WHITE

func _ready():
	assert(fabric_black and fabric_white)
	assert(metal_white and metal_black)

func reverse(s: int) -> int:
	assert(s == Stone.BLACK or s == Stone.WHITE)
	match s:
		Stone.BLACK:
			return Stone.WHITE
		Stone.WHITE:
			return Stone.BLACK
		_:
			return Stone.NONE

func set_texture(texture: String):
	if texture == "fabric":
		$Mesh.set_surface_material(0, fabric_black)
		$Mesh.set_surface_material(1, fabric_white)
	else:
		$Mesh.set_surface_material(0, metal_black)
		$Mesh.set_surface_material(1, metal_white)

func set_animation_speed(speed: float):
	$Mesh/Animation.playback_speed = speed

func play_animation_flip():
	# delay for a short time, then flip
	$FlipDelayTimer.start()

func _on_FlipDelayTimer_timeout():
	if self_color == Stone.WHITE:
		$Mesh/Animation.play("flip")
	else:
		$Mesh/Animation.play_backwards("flip")
	self_color = reverse(self_color)

func play_anmiation_put():
	$Mesh/Animation.play("put")

func play_animation_leave():
	$Mesh/Animation.play("leave")

func set_to_color(color: int):
	self_color = color
	if color == Stone.BLACK:
		$Mesh.rotation = Vector3(PI, 0, 0)
	else:
		$Mesh.rotation = Vector3(0, 0, 0)

func _on_Animation_finished(anim_name):
	if anim_name == "leave":
		self.queue_free()
