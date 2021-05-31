use lookup::LookupBuf;
use vrl::Value;

#[derive(Debug)]
pub struct GrokPattern {
    pub match_fn: Function,
    pub destination: Option<Destination>,
}

#[derive(Clone, Debug)]
pub struct Destination {
    pub path: LookupBuf,
    pub filter_fn: Option<Function>,
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub args: Option<Vec<FunctionArgument>>,
}

#[derive(Clone, Debug)]
pub enum FunctionArgument {
    FUNCTION(Function),
    ARG(Value),
}
// #[derive(Debug)]
// pub struct RawRegex {
//     pub(crate) pattern: String,
// }
//
// #[derive(Debug)]
// pub enum Rule {
//     RawRegex(String),
//     FunctionOrRef(Vec<String>),
// }
