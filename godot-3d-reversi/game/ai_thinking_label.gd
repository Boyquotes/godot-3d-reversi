extends Label

const TIME = 0.25
const texts := ["AIThinking1", "AIThinking2", "AIThinking3"]

var counter := 0
var current_index := 0

func _ready():
	self.hide()

func _process(delta):
	counter += 1
	if counter % int(TIME / delta) == 0:
		current_index += 1
		self.text = texts[current_index % 3]
