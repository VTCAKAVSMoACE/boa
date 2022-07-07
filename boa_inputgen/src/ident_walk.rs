use boa_engine::syntax::ast::node::visit::Visitor;
use boa_engine::syntax::ast::node::StatementList;
use boa_interner::Sym;

struct SymReplacer<'a> {
    syms: &'a [Sym],
}

impl<'a, 'ast> Visitor<'ast> for SymReplacer<'a> {
    type Output = ();
    type Error = ();

    fn visit_sym_mut(&mut self, sym: &'ast mut Sym) -> Result<Self::Output, Self::Error> {
        *sym = self.syms[sym.get() % self.syms.len()];
        Ok(())
    }

    fn get_default_ok() -> Result<Self::Output, Self::Error> {
        Ok(())
    }
}

pub(crate) fn replace_syms(syms: &[Sym], sample: &mut StatementList) {
    let mut replacer = SymReplacer { syms };
    replacer
        .visit_statement_list_mut(sample)
        .expect("No error cases provided.");
}
