use serde::{Serialize, Serializer};

#[derive(Clone, Copy, Debug)]
pub enum Token<'a> {
    Bearer(&'a str),
    Bot(&'a str),
}
impl<'a> ToString for Token<'a> {
    fn to_string(&self) -> String {
        match self {
            Token::Bearer(token) => format!("Bearer {}", token),
            Token::Bot(token) => format!("Bot {}", token),
        }
    }
}
impl<'a> Serialize for Token<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
