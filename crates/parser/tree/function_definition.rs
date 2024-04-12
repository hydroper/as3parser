use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub location: Location,
    pub asdoc: Option<Rc<AsDoc>>,
    pub attributes: Vec<Attribute>,
    pub name: FunctionName,
    pub common: Rc<FunctionCommon>,
}

impl FunctionDefinition {
    /// Indicates whether the function definition is not a getter, setter,
    /// or constructor.
    pub fn is_normal(&self) -> bool {
        matches!(self.name, FunctionName::Identifier(_))
    }
    pub fn is_getter(&self) -> bool {
        matches!(self.name, FunctionName::Getter(_))
    }
    pub fn is_setter(&self) -> bool {
        matches!(self.name, FunctionName::Setter(_))
    }
    pub fn is_constructor(&self) -> bool {
        matches!(self.name, FunctionName::Constructor(_))
    }
    pub fn name_identifier(&self) -> (String, Location) {
        match &self.name {
            FunctionName::Identifier(name) => name.clone(),
            FunctionName::Getter(name) => name.clone(),
            FunctionName::Setter(name) => name.clone(),
            FunctionName::Constructor(name) => name.clone(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum FunctionName {
    Identifier((String, Location)),
    Getter((String, Location)),
    Setter((String, Location)),
    /// A `FunctionName` is a `Constructor` variant
    /// when the corresponding function definition is a constructor.
    Constructor((String, Location)),
}

impl FunctionName {
    pub fn location(&self) -> Location {
        match self {
            Self::Identifier((_, l)) => l.clone(),
            Self::Getter((_, l)) => l.clone(),
            Self::Setter((_, l)) => l.clone(),
            Self::Constructor((_, l)) => l.clone(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FunctionCommon {
    pub location: Location,
    /// Indicates whether the corresponding function
    /// contains the `yield` operator.
    pub contains_yield: bool,
    /// Indicates whether the corresponding function
    /// contains the `await` operator.
    pub contains_await: bool,
    pub signature: FunctionSignature,
    pub body: Option<FunctionBody>,
}

impl FunctionCommon {
    pub(crate) fn has_block_body(&self) -> bool {
        if let Some(ref body) = self.body { matches!(body, FunctionBody::Block(_)) } else { false }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FunctionSignature {
    pub location: Location,
    pub parameters: Vec<Rc<Parameter>>,
    pub result_type: Option<Rc<Expression>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub location: Location,
    pub kind: ParameterKind,
    pub destructuring: TypedDestructuring,
    pub default_value: Option<Rc<Expression>>,
}

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[repr(u32)]
pub enum ParameterKind {
    Required = 1,
    Optional = 2,
    Rest = 3,
}

impl ParameterKind {
    pub fn may_be_followed_by(&self, other: Self) -> bool {
        (*self as u32) <= (other as u32)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum FunctionBody {
    Expression(Rc<Expression>),
    Block(Rc<Block>),
}