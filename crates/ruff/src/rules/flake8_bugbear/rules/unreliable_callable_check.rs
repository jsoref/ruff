use rustpython_parser::ast::{self, Constant, Expr, ExprKind};

use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};

use crate::checkers::ast::Checker;

#[violation]
pub struct UnreliableCallableCheck;

impl Violation for UnreliableCallableCheck {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!(
            "Using `hasattr(x, '__call__')` to test if x is callable is unreliable. Use \
             `callable(x)` for consistent results."
        )
    }
}

/// B004
pub(crate) fn unreliable_callable_check(
    checker: &mut Checker,
    expr: &Expr,
    func: &Expr,
    args: &[Expr],
) {
    let ExprKind::Name(ast::ExprName { id, .. }) = &func.node else {
        return;
    };
    if id != "getattr" && id != "hasattr" {
        return;
    }
    if args.len() < 2 {
        return;
    };
    let ExprKind::Constant(ast::ExprConstant {
        value: Constant::Str(s),
        ..
    }) = &args[1].node else
    {
        return;
    };
    if s != "__call__" {
        return;
    }
    checker
        .diagnostics
        .push(Diagnostic::new(UnreliableCallableCheck, expr.range()));
}
