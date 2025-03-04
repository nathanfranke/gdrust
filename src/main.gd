extends Node


@onready var my_node: MyNode = %MyNode
@onready var label: Label = %Label


var ref_counted := MyRefCounted.example_associated("hello")
var virtual := ExampleVirtual.new()


func _ready() -> void:
	print("ref_counted: %s" % ref_counted.example_var) # hello
	print("virtual: %s" % virtual._example_virtual()) # 5


func _process(_delta: float) -> void:
	MySingleton.example_concrete(my_node, label)
