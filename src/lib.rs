// mod strict;
// mod reserved;

pub enum Category {
    Strict,
    Reserved,
    Weak,
}

pub struct Keyword {
    token: &'static str,
    category: Category,
}

macro_rules! keywords_consts {
    ($cat:path, ) => {};
    ($cat:path, $name:ident : $value:tt $($rest:tt)*) => {
        pub const $name: Keyword =
        Keyword{
            token: stringify!($value), category: $cat
        };
        keywords_consts!($cat, $($rest)*);
    };
}

macro_rules! keywords_itter {
    () => {};
    ($cat:path => { $($name:ident : $value:tt)* } $($rest:tt)*) => {
        //let all = [$($($name),*),*]
        //stringify!($($cat => { $($name: $value),*}),*)
        // $(enum $cat{ $($name),* }),*

        keywords_consts![$cat, $($name : $value)*];


        // pub enum $cat {
        //     $($name),*
        // }
        // impl $cat {
        //     pub fn value(&self) -> &str {
        //         match self{
        //             $(Self::$name => $name),*
        //         }
        //     }
        // }

        //pub const $cat: &[&str] = &[$($name),*];



        keywords_itter![$($rest)*];
    };
}

macro_rules! keywords {
    ($($cat:path => { $($name:ident : $value:tt)* })*) => {
        keywords_itter![$($cat => { $($name : $value)* })*];
        pub const KEYWORDS: &[&Keyword] = &[$($(&$name),*),*];
    };
}

// https://doc.rust-lang.org/reference/keywords.html

use std::borrow::Cow;

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
    KEYWORDS.iter().any(|keyword| keyword.token == token)
}

lazy_static! {
    static ref HASHMAP: HashMap<&'static str, &Keyword> = 
}


pub fn escape_ident(ident: &str) -> Cow<str> {
    KEYWORDS.iter().map(|keyword|)
    
    Cow::Borrowed(ident)
}

#[test]
fn test() {
    // let cool =

    // let nice = dict![1=>1, 2=>2];

    println!("{:?}", Weak::KW_UNION.value())
}
