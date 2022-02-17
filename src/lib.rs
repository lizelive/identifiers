// mod strict;
// mod reserved;

#[derive(Debug, Clone, Copy)]
pub enum Category {
    Strict,
    Reserved,
    Weak,
}

/// a keyword
#[derive(Debug, Clone, Copy)]
pub struct Keyword {
    token: &'static str,
    category: Category,
    escaped: &'static str,
}

impl std::ops::Deref for Keyword {
    type Target = &'static str;

    fn deref(&self) -> &Self::Target {
        &self.token
    }
}


impl Keyword {
    /// Get the keyword's category.
    pub fn category(&self) -> Category {
        self.category
    }

    /// Get the keyword's token.
    pub fn token(&self) -> &str {
        self.token
    }
}

macro_rules! keywords {
    ($($cat:path => { $($name:ident : $value:tt)* })*) => {
        pub const KEYWORDS : &[Keyword] = &[$($(Keyword{
            // id: stringify!($name),
            token: stringify!($value),
            category: $cat,
            escaped: concat!("r#", stringify!($value))
        }),*),*];
    };
}

use std::{collections::HashMap};

// https://doc.rust-lang.org/reference/keywords.html
keywords! {
    Category::Strict => {
        KW_AS : as
        KW_BREAK : break
        KW_CONST : const
        KW_CONTINUE : continue
        KW_CRATE : crate
        KW_ELSE : else
        KW_ENUM : enum
        KW_EXTERN : extern
        KW_FALSE : false
        KW_FN : fn
        KW_FOR : for
        KW_IF : if
        KW_IMPL : impl
        KW_IN : in
        KW_LET : let
        KW_LOOP : loop
        KW_MATCH : match
        KW_MOD : mod
        KW_MOVE : move
        KW_MUT : mut
        KW_PUB : pub
        KW_REF : ref
        KW_RETURN : return
        KW_SELFVALUE : self
        KW_SELFTYPE : Self
        KW_STATIC : static
        KW_STRUCT : struct
        KW_SUPER : super
        KW_TRAIT : trait
        KW_TRUE : true
        KW_TYPE : type
        KW_UNSAFE : unsafe
        KW_USE : use
        KW_WHERE : where
        KW_WHILE : while
        KW_ASYNC : async
        KW_AWAIT : await
        KW_DYN : dyn
    }
    Category::Reserved => {
        KW_ABSTRACT : abstract
        KW_ALIGNOF : alignof
        KW_BECOME : become
        KW_DO : do
        KW_FINAL : final
        KW_MACRO : macro
        KW_OFFSETOF : offsetof
        KW_OVERRIDE : override
        KW_PRIV : priv
        KW_PROC : proc
        KW_PURE : pure
        KW_SIZEOF : sizeof
        KW_TYPEOF : typeof
        KW_UNSIZED : unsized
        KW_VIRTUAL : virtual
        KW_YIELD : yield
    }
    Category::Weak => {
        KW_UNION : union
        KW_STATICLIFETIME : 'static
    }
}

pub fn is_keyword(token: &str) -> bool {
    KEYWORDS_BY_TOKEN.contains_key(token)
}
use lazy_static::lazy_static;

lazy_static! {
    static ref KEYWORDS_BY_TOKEN: HashMap<&'static str, &'static Keyword> = KEYWORDS
        .iter()
        .map(|keyword| (keyword.token, keyword))
        .collect();
}

pub fn get_keyword(keyword: &str) -> Option<&'static Keyword> {
    KEYWORDS_BY_TOKEN.get(keyword).cloned()
}

fn is_valid_ident_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn is_ident_shaped(ident: &str) -> bool {
    ident.chars().all(is_valid_ident_char)
        && match ident.chars().next() {
            Some('_') => ident.len() >= 2,
            Some(c) => c.is_ascii_alphabetic(),
            None => false,
        }
}


/// is this an identifier?
pub fn is_ident(ident: &str) -> bool {
    // http://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/reference/identifiers.html
    is_ident_shaped(ident) && !is_keyword(ident)
        || (ident.starts_with("r#") && is_ident_shaped(ident.split_at(2).1))
}


// escape keyword in an identifier
pub fn escape_ident(ident: &str) -> &str {
    if let Some(keyword) = get_keyword(ident) {
        keyword.escaped
    } else {
        ident
    }
}

#[cfg(test)]
mod test;

