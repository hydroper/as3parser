use crate::ns::*;
use by_address::ByAddress;

const LARGE_BYTES: usize = 26_000;

/// Represents a mapping of nodes to meaning (*symbols*).
/// 
/// A limited set of nodes may be mapped to symbols within this
/// structure through using the implemented `TreeSemanticsAccessor`
/// methods, such as `.get()` and `.set()`.
pub struct TreeSemantics<S> {
    common: TreeSemantics1<S>,
    large_units: RefCell<HashMap<ByAddress<Rc<CompilationUnit>>, TreeSemantics1<S>>>,
}

impl<S: Clone> TreeSemantics<S> {
    pub fn new() -> Self {
        Self {
            common: TreeSemantics1::new(),
            large_units: RefCell::new(HashMap::new()),
        }
    }
}

/// Defines access methods for the `TreeSemantics` structure,
/// used for attaching semantics to the syntactic tree,
/// where `T` is the node type, and `S` is the symbol type.
pub trait TreeSemanticsAccessor<T, S: Clone> {
    fn get(&self, node: &Rc<T>) -> Option<S>;
    fn set(&self, node: &Rc<T>, symbol: Option<S>);
    fn delete(&self, node: &Rc<T>) -> bool;
    fn has(&self, node: &Rc<T>) -> bool;
}

macro impl_semantics_with_loc_call {
    (struct $tree_semantics_id:ident, $($nodetype:ident),*$(,)?) => {
        $(
            impl<S: Clone> TreeSemanticsAccessor<$nodetype, S> for $tree_semantics_id<S> {
                fn get(&self, node: &Rc<$nodetype>) -> Option<S> {
                    let cu = node.location().compilation_unit();
                    if cu.text().len() < LARGE_BYTES {
                        self.common.get(node)
                    } else {
                        let large_units = self.large_units.borrow();
                        let m1 = large_units.get(&ByAddress(cu));
                        m1.and_then(|m1| m1.get(node))
                    }
                }
                fn set(&self, node: &Rc<$nodetype>, symbol: Option<S>) {
                    let cu = node.location().compilation_unit();
                    if cu.text().len() < LARGE_BYTES {
                        self.common.set(node, symbol);
                    } else {
                        let mut large_units = self.large_units.borrow_mut();
                        let m1 = large_units.get_mut(&ByAddress(cu.clone()));
                        if let Some(m1) = m1 {
                            m1.set(node, symbol);
                        } else {
                            let m1 = TreeSemantics1::new();
                            m1.set(node, symbol);
                            large_units.insert(ByAddress(cu), m1);
                        }
                    }
                }
                fn delete(&self, node: &Rc<$nodetype>) -> bool {
                    let cu = node.location().compilation_unit();
                    if cu.text().len() < LARGE_BYTES {
                        self.common.delete(node)
                    } else {
                        let mut large_units = self.large_units.borrow_mut();
                        let m1 = large_units.get_mut(&ByAddress(cu));
                        m1.map(|m1| m1.delete(node)).unwrap_or(false)
                    }
                }
                fn has(&self, node: &Rc<$nodetype>) -> bool {
                    let cu = node.location().compilation_unit();
                    if cu.text().len() < LARGE_BYTES {
                        self.common.has(node)
                    } else {
                        let large_units = self.large_units.borrow();
                        let m1 = large_units.get(&ByAddress(cu));
                        m1.map(|m1| m1.has(node)).unwrap_or(false)
                    }
                }
            }
        )*
    },
}

macro impl_semantics_with_loc_field {
    (struct $tree_semantics_id:ident, $($nodetype:ident),*$(,)?) => {
        $(
            impl<S: Clone> TreeSemanticsAccessor<$nodetype, S> for $tree_semantics_id<S> {
                fn get(&self, node: &Rc<$nodetype>) -> Option<S> {
                    let cu = node.location.compilation_unit();
                    if cu.text().len() < LARGE_BYTES {
                        self.common.get(node)
                    } else {
                        let large_units = self.large_units.borrow();
                        let m1 = large_units.get(&ByAddress(cu));
                        m1.and_then(|m1| m1.get(node))
                    }
                }
                fn set(&self, node: &Rc<$nodetype>, symbol: Option<S>) {
                    let cu = node.location.compilation_unit();
                    if cu.text().len() < LARGE_BYTES {
                        self.common.set(node, symbol);
                    } else {
                        let mut large_units = self.large_units.borrow_mut();
                        let m1 = large_units.get_mut(&ByAddress(cu.clone()));
                        if let Some(m1) = m1 {
                            m1.set(node, symbol);
                        } else {
                            let m1 = TreeSemantics1::new();
                            m1.set(node, symbol);
                            large_units.insert(ByAddress(cu), m1);
                        }
                    }
                }
                fn delete(&self, node: &Rc<$nodetype>) -> bool {
                    let cu = node.location.compilation_unit();
                    if cu.text().len() < LARGE_BYTES {
                        self.common.delete(node)
                    } else {
                        let mut large_units = self.large_units.borrow_mut();
                        let m1 = large_units.get_mut(&ByAddress(cu));
                        m1.map(|m1| m1.delete(node)).unwrap_or(false)
                    }
                }
                fn has(&self, node: &Rc<$nodetype>) -> bool {
                    let cu = node.location.compilation_unit();
                    if cu.text().len() < LARGE_BYTES {
                        self.common.has(node)
                    } else {
                        let large_units = self.large_units.borrow();
                        let m1 = large_units.get(&ByAddress(cu));
                        m1.map(|m1| m1.has(node)).unwrap_or(false)
                    }
                }
            }
        )*
    },
}

macro impl_semantics_1 {
    (struct $tree_semantics_1_id:ident, fn $new_id:ident, $($nodetype:ident),*$(,)?) => {
        #[allow(non_snake_case)]
        struct $tree_semantics_1_id<S> {
            $($nodetype: RefCell<HashMap<NodeAsKey<Rc<$nodetype>>, Option<S>>>,)*
        }

        impl<S: Clone> $tree_semantics_1_id<S> {
            pub fn $new_id() -> Self {
                Self {
                    $($nodetype: RefCell::new(HashMap::new()),)*
                }
            }
        }

        $(
            impl<S: Clone> TreeSemanticsAccessor<$nodetype, S> for $tree_semantics_1_id<S> {
                fn get(&self, node: &Rc<$nodetype>) -> Option<S> {
                    self.$nodetype.borrow().get(&NodeAsKey(node.clone())).map(|v| v.clone().unwrap())
                }
                fn set(&self, node: &Rc<$nodetype>, symbol: Option<S>) {
                    self.$nodetype.borrow_mut().insert(NodeAsKey(node.clone()), symbol);
                }
                fn delete(&self, node: &Rc<$nodetype>) -> bool {
                    self.$nodetype.borrow_mut().remove(&NodeAsKey(node.clone())).is_some()
                }
                fn has(&self, node: &Rc<$nodetype>) -> bool {
                    self.$nodetype.borrow().contains_key(&NodeAsKey(node.clone()))
                }
            }
        )*
    },
}

impl_semantics_with_loc_call!(
    struct TreeSemantics,
    Expression,
    Directive,
    MxmlContent,
);

impl_semantics_with_loc_field!(
    struct TreeSemantics,
    FunctionCommon,
    Block,
    Program,
    SimpleVariableDefinition,
    MxmlDocument,
    MxmlElement,
    MxmlAttribute,
);

impl_semantics_1!(
    struct TreeSemantics1,
    fn new,
    Expression,
    Directive,
    FunctionCommon,
    Block,
    Program,
    SimpleVariableDefinition,
    MxmlDocument,
    MxmlContent,
    MxmlElement,
    MxmlAttribute,
);