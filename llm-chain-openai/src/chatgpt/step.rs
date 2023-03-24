use super::prompt::ChatPromptTemplate;
use async_openai::types::CreateChatCompletionRequest;
use llm_chain::{traits, Parameters};

/// The `Model` enum represents the available ChatGPT models that you can use through the OpenAI API. These models have different capabilities and performance characteristics, allowing you to choose the one that best suits your needs.
///
/// Currently, the available models are:
/// - `ChatGPT3_5Turbo`: A high-performance and versatile model that offers a great balance of speed, quality, and affordability.
/// - `Other(String)`: A variant that allows you to specify a custom model name as a string, in case new models are introduced or you have access to specialized models.
///
/// # Example
///
/// ```
/// use llm_chain_openai::chatgpt::Model;
///
/// let turbo_model = Model::ChatGPT3_5Turbo;
/// let custom_model = Model::Other("your_custom_model_name".to_string());
/// ```
pub enum Model {
    ChatGPT3_5Turbo,
    Other(String),
}

impl ToString for Model {
    fn to_string(&self) -> String {
        match &self {
            Self::ChatGPT3_5Turbo => "gpt-3.5-turbo".to_string(),
            Self::Other(model) => model.to_string(),
        }
    }
}

/// The `Step` struct represents an individual step within a chain for ChatGPT models. It is responsible for configuring the input parameters for the model and providing the prompt.
///
/// By creating a `Step`, you can customize the model and prompt used for a particular stage within an `llm-chain` workflow. This allows for granular control over the text generation process, enabling you to create sophisticated multi-step chains.
///
/// # Example
///
/// ```
/// use llm_chain_openai::chatgpt::{Step, Model, ChatPromptTemplate};
/// let model = Model::ChatGPT3_5Turbo;
/// let prompt = ChatPromptTemplate::system_and_user("You are an assistant that speaks like Shakespeare.", "tell me a joke");
///
/// let chat_gpt_step = Step::new(model, prompt);
/// ```
pub struct Step {
    model: Model,
    prompt: ChatPromptTemplate,
}
impl Step {
    pub fn new<P: Into<ChatPromptTemplate>>(model: Model, prompt: P) -> Step {
        let prompt = prompt.into();
        Step { model, prompt }
    }
}

impl traits::Step for Step {
    type Output = CreateChatCompletionRequest;
    fn format(&self, parameters: &Parameters) -> Self::Output {
        CreateChatCompletionRequest {
            model: self.model.to_string(),
            messages: self.prompt.format(parameters),
            temperature: None,
            top_p: None,
            n: Some(1),
            stream: None,
            stop: None,
            max_tokens: None, // We should consider something here
            presence_penalty: None,
            frequency_penalty: None,
            logit_bias: None,
            user: None,
        }
    }
}
