extends Label

func _on_Timer_timeout():
	self.text = "FPS %d" % Performance.get_monitor(Performance.TIME_FPS)
