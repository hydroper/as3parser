use crate::ns::*;

/// Context used to control the parsing of an expression.
#[derive(Clone)]
pub struct ParserExpressionContext {
    pub min_precedence: OperatorPrecedence,
    pub allow_in: bool,
    pub allow_assignment: bool,
}

impl Default for ParserExpressionContext {
    fn default() -> Self {
        Self {
            min_precedence: OperatorPrecedence::List,
            allow_in: true,
            allow_assignment: true,
        }
    }
}

#[derive(Clone)]
pub enum ParserDirectiveContext {
    Default,
    TopLevel,
    PackageBlock,
    ClassBlock {
        name: String,
    },
    InterfaceBlock,
    EnumBlock,
    ConstructorBlock {
        super_statement_found: Rc<Cell<bool>>,
    },
    WithControl {
        super_statement_found: Option<Rc<Cell<bool>>>,
        to_be_labeled: Option<String>,
        control_context: ParserControlFlowContext,
        labels: HashMap<String, ParserControlFlowContext>,
    },
}

impl ParserDirectiveContext {
    pub fn may_contain_super_statement(&self) -> bool {
        matches!(self, Self::ConstructorBlock { .. }) || matches!(self, Self::WithControl { .. })
    }

    pub fn super_statement_found(&self) -> bool {
        match self {
            Self::ConstructorBlock { super_statement_found } => super_statement_found.get(),
            Self::WithControl { super_statement_found, .. } => super_statement_found.as_ref().or(Some(&Rc::new(Cell::new(false)))).unwrap().get(),
            _ => false,
        }
    }

    pub fn set_super_statement_found(&self, value: bool) {
        match self {
            Self::ConstructorBlock { super_statement_found } => { super_statement_found.set(value) },
            Self::WithControl { super_statement_found, .. } => {
                if let Some(found) = super_statement_found.as_ref() {
                    found.set(value);
                }
            },
            _ => {},
        }
    }

    pub fn function_name_is_constructor(&self, name: &(String, Location)) -> bool {
        if let ParserDirectiveContext::ClassBlock { name: ref name_1 } = self {
            &name.0 == name_1
        } else {
            false
        }
    }

    pub fn is_top_level_or_package(&self) -> bool {
        matches!(self, ParserDirectiveContext::TopLevel) || matches!(self, ParserDirectiveContext::PackageBlock)
    }

    pub fn is_type_block(&self) -> bool {
        match self {
            Self::ClassBlock { .. } |
            Self::InterfaceBlock |
            Self::EnumBlock => true,
            _ => false,
        }
    }

    pub fn clone_control(&self) -> Self {
        match self {
            Self::WithControl { .. } => self.clone(),
            _ => Self::Default,
        }
    }

    pub fn override_control_context(&self, label_only: bool, mut context: ParserControlFlowContext) -> Self {
        let mut prev_context = None;
        let mut label = None;
        let mut super_statement_found: Option<Rc<Cell<bool>>> = None;
        let mut labels = match self {
            Self::WithControl { control_context, labels, to_be_labeled: label1, super_statement_found: super_found_1 } => {
                prev_context = Some(control_context.clone());
                label = label1.clone();
                super_statement_found = super_found_1.clone();
                labels.clone()
            },
            _ => HashMap::new(),
        };
        if let Some(label) = label.clone() {
            labels.insert(label, context.clone());
        }
        if label_only {
            context = prev_context.unwrap_or(ParserControlFlowContext {
                breakable: false,
                iteration: false,
            });
        }
        Self::WithControl { control_context: context, labels, to_be_labeled: None, super_statement_found }
    }

    pub fn put_label(&self, label: String) -> Self {
        match self {
            Self::WithControl { control_context, labels, to_be_labeled: _, super_statement_found } => Self::WithControl {
                to_be_labeled: Some(label),
                control_context: control_context.clone(),
                labels: labels.clone(),
                super_statement_found: super_statement_found.clone(),
            },
            _ => Self::WithControl {
                to_be_labeled: Some(label),
                control_context: ParserControlFlowContext {
                    breakable: false,
                    iteration: false,
                },
                labels: HashMap::new(),
                super_statement_found: match self {
                    Self::ConstructorBlock { super_statement_found } => Some(super_statement_found.clone()),
                    _ => None,
                },
            },
        }
    }

    pub fn is_label_defined(&self, label: String) -> bool {
        self.resolve_label(label).is_some()
    }

    pub fn resolve_label(&self, label: String) -> Option<ParserControlFlowContext> {
        if let Self::WithControl { labels, .. } = &self { labels.get(&label).map(|c| c.clone()) } else { None }
    }

    pub fn is_break_allowed(&self, label: Option<String>) -> bool {
        if let Some(label) = label {
            let context = self.resolve_label(label);
            if let Some(context) = context { context.breakable } else { false }
        } else {
            if let Self::WithControl { control_context, .. } = &self { control_context.breakable } else { false }
        }
    }

    pub fn is_continue_allowed(&self, label: Option<String>) -> bool {
        if let Some(label) = label {
            let context = self.resolve_label(label);
            if let Some(context) = context { context.iteration } else { false }
        } else {
            if let Self::WithControl { control_context, .. } = &self { control_context.iteration } else { false }
        }
    }
}

#[derive(Clone)]
pub struct ParserControlFlowContext {
    pub breakable: bool,
    pub iteration: bool,
}