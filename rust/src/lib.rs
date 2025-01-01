use godot::prelude::*;
use godot::init::ExtensionLibrary;
use inklings::{Client, provider::OpenAIProvider, provider::AnthropicProvider, types::{Message, Role}};

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=Node)]
struct RustGreeter {
    base: Base<Node>,
    openai_key: Option<String>,
    anthropic_key: Option<String>
}

#[godot_api]
impl INode for RustGreeter {
    fn init(base: Base<Node>) -> Self {
        Self { 
            base,
            openai_key: None,
            anthropic_key: None
        }
    }
}

#[godot_api]
impl RustGreeter {
    #[func]
    fn get_greeting(&self) -> GString {
        "Hello from Rust!".into()
    }

    #[func]
    fn set_openai_key(&mut self, key: GString) {
        self.openai_key = Some(key.to_string());
    }

    #[func]
    fn set_anthropic_key(&mut self, key: GString) {
        self.anthropic_key = Some(key.to_string());
    }

    #[func]
    fn get_response(&self, prompt: GString) -> Dictionary {
        let openai_resp = match &self.openai_key {
            Some(key) => {
                let runtime = tokio::runtime::Runtime::new().unwrap();
                let result = runtime.block_on(async {
                    let provider = OpenAIProvider::new(key.clone(), None);
                    let client = Client::new(Box::new(provider));
                    let messages = vec![
                        Message {
                            role: Role::User,
                            content: prompt.to_string(),
                        },
                    ];

                    match client.chat(messages).await {
                        Ok(res) => res,
                        Err(err) => format!("Request error: {}", err),
                    }
                });
                result
            }
            None => "Please set OpenAI API key first".to_string()
        };

        let anthropic_resp = match &self.anthropic_key {
            Some(key) => {
                let runtime = tokio::runtime::Runtime::new().unwrap();
                let result = runtime.block_on(async {
                    let provider = AnthropicProvider::new(key.clone(), None);
                    let client = Client::new(Box::new(provider));
                    let messages = vec![
                        Message {
                            role: Role::User,
                            content: prompt.to_string(),
                        },
                    ];

                    match client.chat(messages).await {
                        Ok(res) => res,
                        Err(err) => format!("Request error: {}", err),
                    }
                });
                result
            }
            None => "Please set Anthropic API key first".to_string()
        };

        let mut result_dict = Dictionary::new();
        result_dict.insert("openai", openai_resp);
        result_dict.insert("anthropic", anthropic_resp);
        result_dict
    }
}
