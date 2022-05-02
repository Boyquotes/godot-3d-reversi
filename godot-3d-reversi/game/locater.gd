extends Spatial

const LOCATER_SPEED = 18

onready var global := get_node("/root/Global")

func _process(delta):
	if global.animation_speed != INF:
		self.translation = lerp(self.translation, global.locater_target, LOCATER_SPEED * delta * global.animation_speed)
	else:
		self.translation = global.locater_target
