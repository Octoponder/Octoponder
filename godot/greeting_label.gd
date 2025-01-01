extends Label

func _ready():
	pass

func _on_GetRandomTextButton_pressed():
	var greeter = $"../RustGreeter"
	greeter.set_openai_key($"../PasswordInput".text)
	greeter.set_anthropic_key($"../AnthropicInput".text)
	var user_message = $"../MessageInput".text
	var result = greeter.get_response(user_message if user_message else "tell me about yourself")
	text = "OpenAI:\n" + str(result["openai"]) + "\n\nAnthropic:\n" + str(result["anthropic"])
