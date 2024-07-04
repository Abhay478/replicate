use std::{cell::RefCell, collections::HashMap};

use crate::ast::Type;

pub struct VariableSymbolTable<'a> {
    tables: HashMap<Scope<'a>, InnerVST<'a>>,
}

impl<'a> VariableSymbolTable<'a> {
    /// Gets the variable entry for the given name in the given scope, or any parent scopes.
    /// Uses RefCell to avoid borrowing problems. Returns None if the variable is not found.
    pub fn get(&'a self, name: &str, scope: &'a Scope) -> Option<&RefCell<Variable<'a>>> {
        // The scope has to exist.
        let init_vst = self.tables.get(scope).unwrap();
        let mut q = init_vst.variables.get(name);
        while q.is_none() {
            // The scope may not have a parent.
            let Some(parent) = scope.parent else {
                return None;
            };
            let vst = self.tables.get(parent).unwrap();
            q = vst.variables.get(name);
        }

        return q;
    }

    pub fn new_scope(
        &mut self,
        parent: Option<&'a Scope>,
        init: Option<HashMap<String, Variable<'a>>>,
    ) -> Scope<'a> {
        let scope = Scope {
            id: self.tables.len(),
            parent,
        };
        let ivst = if let Some(init) = init {
            let variables = init
                .into_iter()
                .map(|(k, v)| (k, RefCell::new(v)))
                .collect();
            InnerVST { variables }
        } else {
            InnerVST {
                variables: HashMap::new(),
            }
        };
        self.tables.insert(scope, ivst);
        return scope;
    }

    pub fn install(&mut self, name: &str, scope: &'a Scope, v: Variable<'a>) {
        let vst = self.tables.get_mut(scope).unwrap();
        vst.variables.insert(name.to_string(), RefCell::new(v));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Scope<'a> {
    id: usize,
    parent: Option<&'a Scope<'a>>,
}

pub struct InnerVST<'a> {
    variables: HashMap<String, RefCell<Variable<'a>>>,
}

pub struct Variable<'a> {
    // Is key of HashMap, don't need.
    // name: String,
    typ: Option<Type<'a>>, // we inferring this.
    // scope: Scope,
    mutable: bool,
    initialized: bool,
}

impl<'a> Variable<'a> {
    pub fn new(typ: Option<Type<'a>>, mutable: bool) -> Self {
        Variable {
            typ,
            mutable,
            initialized: false,
        }
    }

    pub fn check_type(&mut self, other: &Type<'a>) -> bool {
        if let Some(ref t) = self.typ {
            return t == other;
        }
        self.typ = Some(other.clone());
        return true;
    }
}

pub struct FunctionSymbolTable<'a> {
    tables: HashMap<String, Function<'a>>,
}

impl<'a> FunctionSymbolTable<'a> {
    pub fn new() -> Self {
        FunctionSymbolTable {
            tables: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Function<'a>> {
        self.tables.get(name)
    }

    pub fn install(
        &mut self,
        name: &str, // func name
        rtyp: Type<'a>, // return type
        args: HashMap<String, Variable<'a>>, // arguments
        vst: &'a mut VariableSymbolTable<'a>,
    ) -> Scope<'a> {
        // Create a new scope for the function parameters.
        let new_scope = vst.new_scope(None, Some(args));
        // Add the function to the symbol table.
        let f = Function::new(rtyp, new_scope);
        self.tables.insert(name.to_string(), f);

        // return the function's scope. Why? I don't know, TODO.
        return new_scope;
    }
}

pub struct Function<'a> {
    // Is key of HashMap, don't need.
    // name: String,
    ret_typ: Type<'a>,
    // args: HashMap<Variable<'a>>,
    /// Points to the scope in the VST for the function parameters.
    scope: Scope<'a>,
    // body: Vec<Statement>,
}

impl<'a> Function<'a> {
    pub fn new(ret_typ: Type<'a>, scope: Scope<'a>) -> Self {
        Function {
            ret_typ,
            scope,
        }
    }
}
