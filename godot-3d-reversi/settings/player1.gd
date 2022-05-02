extends VBoxContainer

func _ready():
	$Level.hide()

func _on_Human_pressed():
	$Level.hide()

func _on_Computer_pressed():
	$Level.show()

func _on_Computer_value_changed(value):
	$Level/Label.text = "Lv%d" % (value + 1)
