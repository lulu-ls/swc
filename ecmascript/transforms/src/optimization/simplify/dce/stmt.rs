use super::Dce;
use swc_common::{fold::FoldWith, Fold, Spanned};
use swc_ecma_ast::*;

impl Fold<ExprStmt> for Dce<'_> {
    fn fold(&mut self, node: ExprStmt) -> ExprStmt {
        if self.is_marked(node.span) {
            return node;
        }

        if self.should_include(&node.expr) {
            let stmt = ExprStmt {
                span: node.span.apply_mark(self.config.used_mark),
                expr: self.fold_in_marking_phase(node.expr),
            };
            return stmt;
        }

        node.fold_children(self)
    }
}

impl Fold<BlockStmt> for Dce<'_> {
    fn fold(&mut self, node: BlockStmt) -> BlockStmt {
        if self.is_marked(node.span) {
            return node;
        }

        let stmts = node.stmts.fold_with(self);

        let mut span = node.span;
        if stmts.iter().any(|stmt| self.is_marked(stmt.span())) {
            span = span.apply_mark(self.config.used_mark);
        }

        BlockStmt { span, stmts }
    }
}

impl Fold<IfStmt> for Dce<'_> {
    fn fold(&mut self, node: IfStmt) -> IfStmt {
        if self.is_marked(node.span) {
            return node;
        }

        let mut node: IfStmt = node.fold_children(self);

        if self.is_marked(node.test.span())
            || self.is_marked(node.cons.span())
            || self.is_marked(node.alt.span())
        {
            node.span = node.span.apply_mark(self.config.used_mark);

            node.test = self.fold_in_marking_phase(node.test);
            node.cons = self.fold_in_marking_phase(node.cons);
            node.alt = self.fold_in_marking_phase(node.alt);
        }

        node
    }
}

impl Fold<ReturnStmt> for Dce<'_> {
    fn fold(&mut self, mut node: ReturnStmt) -> ReturnStmt {
        if self.is_marked(node.span) {
            return node;
        }
        node.span = node.span.apply_mark(self.config.used_mark);

        let mut node = node.fold_children(self);

        if self.is_marked(node.arg.span()) {
            node.arg = self.fold_in_marking_phase(node.arg)
        }

        node
    }
}

impl Fold<ThrowStmt> for Dce<'_> {
    fn fold(&mut self, mut node: ThrowStmt) -> ThrowStmt {
        if self.is_marked(node.span) {
            return node;
        }
        node.span = node.span.apply_mark(self.config.used_mark);

        let mut node = node.fold_children(self);

        if self.is_marked(node.arg.span()) {
            node.arg = self.fold_in_marking_phase(node.arg)
        }

        node
    }
}

impl Fold<LabeledStmt> for Dce<'_> {
    fn fold(&mut self, mut node: LabeledStmt) -> LabeledStmt {
        if self.is_marked(node.span) {
            return node;
        }

        node.body = node.body.fold_with(self);

        if self.is_marked(node.body.span()) {
            node.span = node.span.apply_mark(self.config.used_mark);
            node.body = self.fold_in_marking_phase(node.body);
        }

        node
    }
}

impl Fold<SwitchStmt> for Dce<'_> {
    fn fold(&mut self, mut node: SwitchStmt) -> SwitchStmt {
        if self.is_marked(node.span) {
            return node;
        }

        node = node.fold_children(self);

        // TODO: Handle fallthrough
        //  Drop useless switch case.
        //        node.cases.retain(|case| {
        //            self.is_marked(case.span)
        //        });

        if self.is_marked(node.discriminant.span())
            || node.cases.iter().any(|case| self.is_marked(case.span))
        {
            node.span = node.span.apply_mark(self.config.used_mark);
            node.cases = self.fold_in_marking_phase(node.cases);
        }

        node
    }
}

impl Fold<SwitchCase> for Dce<'_> {
    fn fold(&mut self, mut node: SwitchCase) -> SwitchCase {
        if self.is_marked(node.span) {
            return node;
        }

        node = node.fold_children(self);

        if self.is_marked(node.test.span()) || node.cons.iter().any(|v| self.is_marked(v.span())) {
            node.span = node.span.apply_mark(self.config.used_mark);

            node.test = self.fold_in_marking_phase(node.test);
            node.cons = self.fold_in_marking_phase(node.cons);
        }

        node
    }
}

impl Fold<TryStmt> for Dce<'_> {
    fn fold(&mut self, mut node: TryStmt) -> TryStmt {
        if self.is_marked(node.span) {
            return node;
        }

        node = node.fold_children(self);

        if self.is_marked(node.block.span())
            || self.is_marked(node.handler.span())
            || self.is_marked(node.finalizer.span())
        {
            node.span = node.span.apply_mark(self.config.used_mark);

            node.block = self.fold_in_marking_phase(node.block);
            node.handler = self.fold_in_marking_phase(node.handler);
            node.finalizer = self.fold_in_marking_phase(node.finalizer);
        }

        node
    }
}

impl Fold<WhileStmt> for Dce<'_> {
    fn fold(&mut self, mut node: WhileStmt) -> WhileStmt {
        if self.is_marked(node.span) {
            return node;
        }

        node = node.fold_children(self);

        if self.is_marked(node.test.span()) || self.is_marked(node.body.span()) {
            node.span = node.span.apply_mark(self.config.used_mark);

            node.test = self.fold_in_marking_phase(node.test);
            node.body = self.fold_in_marking_phase(node.body);
        }

        node
    }
}

impl Fold<DoWhileStmt> for Dce<'_> {
    fn fold(&mut self, mut node: DoWhileStmt) -> DoWhileStmt {
        if self.is_marked(node.span) {
            return node;
        }

        node = node.fold_children(self);

        if self.is_marked(node.test.span()) || self.is_marked(node.body.span()) {
            node.span = node.span.apply_mark(self.config.used_mark);

            node.test = self.fold_in_marking_phase(node.test);
            node.body = self.fold_in_marking_phase(node.body);
        }

        node
    }
}

impl Fold<ForStmt> for Dce<'_> {
    fn fold(&mut self, mut node: ForStmt) -> ForStmt {
        if self.is_marked(node.span) {
            return node;
        }

        node = node.fold_children(self);

        if node.test.is_none()
            || self.is_marked(node.init.span())
            || self.is_marked(node.test.span())
            || self.is_marked(node.update.span())
            || self.is_marked(node.body.span())
        {
            node.span = node.span.apply_mark(self.config.used_mark);

            node.test = self.fold_in_marking_phase(node.test);
            node.init = self.fold_in_marking_phase(node.init);
            node.update = self.fold_in_marking_phase(node.update);
            node.body = self.fold_in_marking_phase(node.body);
        }

        node
    }
}

impl Fold<ForInStmt> for Dce<'_> {
    fn fold(&mut self, mut node: ForInStmt) -> ForInStmt {
        if self.is_marked(node.span) {
            return node;
        }

        node = node.fold_children(self);

        if self.is_marked(node.left.span())
            || self.is_marked(node.right.span())
            || self.is_marked(node.body.span())
        {
            node.span = node.span.apply_mark(self.config.used_mark);

            node.left = self.fold_in_marking_phase(node.left);
            node.right = self.fold_in_marking_phase(node.right);
            node.body = self.fold_in_marking_phase(node.body);
        }

        node
    }
}

impl Fold<ForOfStmt> for Dce<'_> {
    fn fold(&mut self, mut node: ForOfStmt) -> ForOfStmt {
        if self.is_marked(node.span) {
            return node;
        }

        node = node.fold_children(self);

        if self.is_marked(node.left.span())
            || self.is_marked(node.right.span())
            || self.is_marked(node.body.span())
        {
            node.span = node.span.apply_mark(self.config.used_mark);

            node.left = self.fold_in_marking_phase(node.left);
            node.right = self.fold_in_marking_phase(node.right);
            node.body = self.fold_in_marking_phase(node.body);
        }

        node
    }
}

preserve!(DebuggerStmt);
preserve!(WithStmt);
preserve!(BreakStmt);
preserve!(ContinueStmt);
