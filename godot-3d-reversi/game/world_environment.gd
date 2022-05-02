extends WorldEnvironment

func _ready():
	var os_name := OS.get_name().to_lower()
	if os_name == "html5" or os_name == "android" or os_name == "ios":
		_on_Setting_quality_change(0)

func _on_Setting_quality_change(level):
	match level:
		2:
			self.environment.ssao_enabled = true
			self.environment.ss_reflections_enabled = true
			self.environment.tonemap_mode = Environment.TONE_MAPPER_REINHARDT
			self.environment.tonemap_white = 10
			self.environment.glow_enabled = true
		1:
			self.environment.ssao_enabled = false
			self.environment.ss_reflections_enabled = false
			self.environment.tonemap_mode = Environment.TONE_MAPPER_FILMIC
			self.environment.tonemap_white = 6
			self.environment.glow_enabled = true
		_:
			self.environment.ssao_enabled = false
			self.environment.ss_reflections_enabled = false
			self.environment.tonemap_mode = Environment.TONE_MAPPER_LINEAR
			self.environment.tonemap_white = 6
			self.environment.glow_enabled = true
