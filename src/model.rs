use std::fmt;

#[derive(Debug, Clone)]
pub struct Model {
    pub name: String,
    pub campany: Campany,
}

#[derive(Debug, Clone, Copy)]
pub enum Campany {
    OpenAI,
    Claude,
}

// Modelを表示するための実装
impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

// ModelをStringに変換するための実装
impl From<Model> for String {
    fn from(model: Model) -> String {
        model.name
    }
}
