//! Fuzz data generation.

use std::collections::HashSet;

use arbitrary::{size_hint, Arbitrary, Unstructured};
use spin::lazy::Lazy;

use boa_engine::syntax::ast::node::StatementList;
use boa_interner::{Interner, ToInternedString};

use crate::replace_syms;

static ALPHA: Lazy<Vec<u8>> = Lazy::new(|| {
    let mut all = Vec::new();
    all.extend(b'A'..b'Z');
    all.extend(b'a'..b'z');
    all
});

static ALPHANUM: Lazy<Vec<u8>> = Lazy::new(|| {
    let mut all = Vec::new();
    all.extend(b'0'..b'9');
    all.extend(b'A'..b'Z');
    all.extend(b'a'..b'z');
    all
});

/// A valid name for use as an identifier.
#[derive(Debug, PartialEq, Eq, Hash)]
struct Name {
    name: String,
}

impl<'a> arbitrary::Arbitrary<'a> for Name {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        // generate a valid identifier; starts with at least one alphabetic character
        let first = u8::arbitrary(u)?;
        let first = ALPHA[(first as usize) % ALPHA.len()];

        // remaining characters are alphanumeric
        let mut chars: Vec<u8> = vec![first];
        let mut second: Vec<u8> = Arbitrary::arbitrary(u)?;
        second
            .iter_mut()
            .for_each(|c| *c = ALPHANUM[(*c as usize) % ALPHANUM.len()]);
        chars.extend(second);

        Ok(Self {
            name: String::from_utf8(chars).expect("Only valid characters used."),
        })
    }

    // size is at least one u8 and a vec of u8s for the rest
    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        size_hint::and(u8::size_hint(depth), Vec::<u8>::size_hint(depth))
    }
}

/// Fuzz data which can be arbitrarily generated and used to test boa's parser, compiler, and vm.
#[derive(Debug, Clone)]
pub struct FuzzData {
    source: String,
}

impl<'a> Arbitrary<'a> for FuzzData {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        // we need at least one name or we'll mod by zero when trying to get a name
        let first_name = Name::arbitrary(u)?;
        // generate the rest
        let mut vars = HashSet::<Name>::arbitrary(u)?;
        vars.insert(first_name);

        // generate a javascript sample
        let mut sample = StatementList::arbitrary(u)?;

        // notify the interner of the symbols we're using
        let mut interner = Interner::with_capacity(vars.len());
        let syms = vars
            .into_iter()
            .map(|var| interner.get_or_intern(var.name))
            .collect::<Vec<_>>();

        // walk the AST and ensure that all identifiers are valid
        replace_syms(&syms, &mut sample);
        Ok(Self {
            source: sample.to_interned_string(&interner),
        })
    }

    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        size_hint::and_all(&[
            Name::size_hint(depth),
            HashSet::<Name>::size_hint(depth),
            StatementList::size_hint(depth),
        ])
    }
}

impl FuzzData {
    /// Get the source represented by this fuzz data
    pub fn get_source(&self) -> &str {
        &self.source
    }
}
