use crate::{
    exec::Executable,
    gc::{Finalize, Trace},
    syntax::ast::node::{join_nodes, FormalParameter, Node, StatementList},
    Context, Result, Value,
};
use std::fmt;

#[cfg(feature = "deser")]
use serde::{Deserialize, Serialize};

/// An async arrow function expression is a syntactically compact alternative to an async function.
///
/// More information:
///  - [ECMAScript reference][spec]
///  - [MDN documentation][mdn]
///
/// [spec]: https://tc39.es/ecma262/#prod-AsyncArrowFunction
/// [mdn]:
#[cfg_attr(feature = "deser", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Trace, Finalize, PartialEq)]
pub struct AsyncArrowFunctionDecl {
    params: Box<[FormalParameter]>,
    body: StatementList,
}

impl AsyncArrowFunctionDecl {
    /// Creates a new `AsyncArrowFunctionDecl` AST node.
    pub(in crate::syntax) fn new<P, B>(params: P, body: B) -> Self
    where
        P: Into<Box<[FormalParameter]>>,
        B: Into<StatementList>,
    {
        Self {
            params: params.into(),
            body: body.into(),
        }
    }

    /// Gets the list of parameters of the arrow function.
    pub fn params(&self) -> &[FormalParameter] {
        &self.params
    }

    /// Gets the body of the arrow function.
    pub fn body(&self) -> &[Node] {
        &self.body.items()
    }

    /// Implements the display formatting with indentation.
    pub(in crate::syntax::ast::node) fn display(
        &self,
        f: &mut fmt::Formatter<'_>,
        indentation: usize,
    ) -> fmt::Result {
        write!(f, "async (")?;
        join_nodes(f, &self.params)?;
        f.write_str(") => ")?;
        self.body.display(f, indentation)
    }
}

impl Executable for AsyncArrowFunctionDecl {
    fn run(&self, _: &mut Context) -> Result<Value> {
        // TODO: Implement AsyncFunctionExpr
        Ok(Value::Undefined)
    }
}

impl fmt::Display for AsyncArrowFunctionDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(f, 0)
    }
}

impl From<AsyncArrowFunctionDecl> for Node {
    fn from(decl: AsyncArrowFunctionDecl) -> Self {
        Self::AsyncArrowFunctionDecl(decl)
    }
}
