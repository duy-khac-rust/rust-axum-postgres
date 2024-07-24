/*


pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    //  -- ...
    ChatReqHasNoMessages {
        model_into: ModelInfo
    }
    LastChatMessageIsNoUser {
        model_info: ModelInfo,
        actual_role: ChaRole,    
    }

    // -- ...
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> core::result::Result<(), core::> {
        write!(fmt, "{self:?}")
    }   
}

impl std::error::Error for Error {}

1. constructorsNot needed for all types. When target T is immutable (Arc<inner>)
    - impl Default          - CharRequest::default()
    - new(...)              - CharRequest::new(vec![ChatMessage::user("Why sky red?")])
    - from_...(...)         - ChatRequest::from_system("Answer in one line.")
    - custom_name(...)      - ChatMessage::user("Why sky red?")
    - impl From<...>        - impl From<String> for ModelName {...}
2. Setters (chainable)
    - with_..(self, ...)    - chat_req.with_system("Answer in one line.")
    - append_..(self, ...)  - chat_req.append_message(ChatMessage::user("Why sky red?"))
    - insert_..(self, k , v)- client_builder.insert_adapter_config(adapter_kind, adapter_config)
3. Build                    Not needed for all types. When target T is immutable (Arc<inner>)
    - ..Build               - ClientBuilder
    - build()               - T of Result<T> //if validation
    - ..setter..patterns..  - client_builder.with_reqwest(reqwest) 
    - T::builder()          - Client::builder() 
    
    */