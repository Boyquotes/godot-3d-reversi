extends Node

onready var othello_ai := preload("res://game/othello_ai.gdns").new()
signal ai_think_completed(result)

# at least 800 ms
const MIN_TIME = 800

# if ai is running in thread
var ai_running := false

# record timestamp of ai start running
var start_time: int

var result: String

func ai_run(input, color, ai_level):
	othello_ai.run(input, color, ai_level)
	ai_running = true
	start_time = OS.get_ticks_msec()

func _emit_after_timeout():
	print(result)
	get_node("../../DebugLabel").text = result.substr(2)
	emit_signal("ai_think_completed", result)

func _process(_delta):
	if ai_running:
		var res = othello_ai.done()
		if len(res) > 1:
			ai_running = false
			result = res
			
			var spent_time := OS.get_ticks_msec() - start_time
			var sleep_time := (MIN_TIME - spent_time) as float / 1000.0
			if sleep_time < 0.1:
				sleep_time = 0.1
			
			$DelayTimer.wait_time = sleep_time
			$DelayTimer.start()

func _on_DelayTimer_timeout():
	emit_signal("ai_think_completed", result)
