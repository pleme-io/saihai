//! `saihai-spec` (采配) — the typed border of the desktop action catalog.
//!
//! Every action a pleme-io desktop can perform is one `(defaction …)` row.
//! This crate is what those rows compile into; it **emits nothing**.
//!
//! The metaphor's load-bearing half is not the waving: a commander's baton has
//! a **closed, finite set of signals**. So does this.
//!
//! ## Three properties the types carry, not the prose
//!
//! 1. **Observability is derived, never authored.** [`ActionSpec::class`] is a
//!    function of `(kind, observed.is_empty())`. There is no `observability`
//!    field for an author to state wrongly, so "this action is convergeable"
//!    cannot be a claim — only a consequence of naming what can be read back.
//! 2. **Authority is a rung, from [`hashigo`].** The value ladder lives here;
//!    the type ladder stops an over-privileged call from compiling in the
//!    generated dispatch.
//! 3. **A blind action can never be counted as converged.** [`Class::Blind`] is
//!    a distinct arm, and the reconciler's fixpoint is defined over
//!    [`Class::Converging`] alone.
//!
//! ## Substrate facts this border relies on
//!
//! Measured, and pinned upstream in `tatara-lisp/tests/phase_f_constructs.rs`:
//! `#[tatara(domain)]` including `Vec<T>` works; `#[tatara(keyword_enum)]`
//! works; a typo'd kwarg is REJECTED with a did-you-mean; a typo'd **struct
//! attribute** is SILENT and yields a keyword computed from the struct name —
//! hence the literal `KEYWORD` assertions in the tests, which are load-bearing.
//! `KeywordSexp` lowercases the ident with **no separator**, so every enum
//! variant here is a single word.

pub mod bancada;

use serde::Deserialize;
use tatara_lisp::{DeriveKeywordSexp, DeriveTataraDomain, TataraDomain};

pub use hashigo::{AtLeast, Rung, RungWitness, Warrant, L0, L1, L2, L3};

// ── Newtypes ───────────────────────────────────────────────────────────────
//
// All reach the border through the derive's `Kind::Deserialize` fall-through,
// so there is no hand-written extractor to drift. NONE implements `Default`:
// the published derive decides "has a default" with a `tokens.contains("default")`
// substring test, and a type with no `Default` cannot produce a compiling
// program down that path.

/// A kebab-case action id: `window-focus`, `session-lock`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize)]
#[serde(try_from = "String")]
pub struct ActionId(String);

impl ActionId {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// `window-focus-direction` → `WindowFocusDirection`.
    #[must_use]
    pub fn to_pascal(&self) -> String {
        self.0
            .split('-')
            .map(|w| {
                let mut c = w.chars();
                c.next().map_or_else(String::new, |f| {
                    f.to_uppercase().collect::<String>() + c.as_str()
                })
            })
            .collect()
    }

    /// `window-focus-direction` → `window_focus_direction`.
    #[must_use]
    pub fn to_snake(&self) -> String {
        self.0.replace('-', "_")
    }
}

impl TryFrom<String> for ActionId {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        // ^[a-z][a-z0-9]*(-[a-z0-9]+)*$ — hand-checked rather than pulled in as
        // a regex dependency, because the grammar is small and a dependency in
        // the border is a dependency in every consumer.
        let ok = !s.is_empty()
            && s.starts_with(|c: char| c.is_ascii_lowercase())
            && !s.ends_with('-')
            && !s.contains("--")
            && s.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');
        if ok {
            Ok(Self(s))
        } else {
            Err(format!(
                "action id must be lowercase kebab-case matching \
                 ^[a-z][a-z0-9]*(-[a-z0-9]+)*$, got {s:?}"
            ))
        }
    }
}

/// A parameter name — legal as an identifier in EVERY target language after
/// case-mapping, computed once here so no emitter has to sanitise.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(try_from = "String")]
pub struct Ident(String);

impl Ident {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
    #[must_use]
    pub fn to_camel(&self) -> String {
        let mut parts = self.0.split('_');
        let head = parts.next().unwrap_or_default().to_string();
        head + &parts
            .map(|w| {
                let mut c = w.chars();
                c.next().map_or_else(String::new, |f| {
                    f.to_uppercase().collect::<String>() + c.as_str()
                })
            })
            .collect::<String>()
    }
}

/// Reserved across the four target languages plus Rust. Checked at the border
/// so a catalog row naming `class` or `func` fails to parse rather than
/// emitting code that fails to compile in one language only.
const RESERVED: &[&str] = &[
    "type", "class", "func", "def", "return", "import", "from", "in", "is", "as", "if", "else",
    "for", "while", "match", "impl", "trait", "struct", "enum", "fn", "let", "mut", "const",
    "range", "map", "chan", "go", "defer", "package", "interface", "select", "var", "lambda",
    "pass", "None", "True", "False", "async", "await", "yield", "self", "super", "new", "delete",
];

impl TryFrom<String> for Ident {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        let shape = !s.is_empty()
            && s.starts_with(|c: char| c.is_ascii_lowercase())
            && !s.ends_with('_')
            && !s.contains("__")
            && s.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_');
        if !shape {
            return Err(format!(
                "identifier must be lowercase snake_case, got {s:?}"
            ));
        }
        if RESERVED.contains(&s.as_str()) {
            return Err(format!(
                "{s:?} is reserved in at least one target language; \
                 pick another name rather than letting one emitter sanitise it"
            ));
        }
        Ok(Self(s))
    }
}

// ── Closed axes ────────────────────────────────────────────────────────────
//
// Single-word variants throughout: `KeywordSexp` lowercases the ident with NO
// separator, so `WorkSpace` would be `:workspace` and `Work_Space` is not a
// legal ident. One word, one keyword, no surprises.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DeriveKeywordSexp)]
pub enum Category {
    Window,
    Workspace,
    Layout,
    Output,
    Input,
    Focus,
    Session,
    Login,
    Seat,
    Launch,
    Clipboard,
    Capture,
    Notify,
    Theme,
    System,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DeriveKeywordSexp)]
pub enum Kind {
    Observe,
    Mutate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DeriveKeywordSexp)]
pub enum Auth {
    L0,
    L1,
    L2,
    L3,
}

impl Auth {
    #[must_use]
    pub const fn rung(self) -> Rung {
        match self {
            Self::L0 => Rung::L0,
            Self::L1 => Rung::L1,
            Self::L2 => Rung::L2,
            Self::L3 => Rung::L3,
        }
    }
}

/// A parameter's type, closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DeriveKeywordSexp)]
pub enum ParamKind {
    Bool,
    Int,
    Str,
    Selector,
    Direction,
}

/// A state domain a reader can observe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DeriveKeywordSexp)]
pub enum ObsDomain {
    Windows,
    Workspaces,
    Outputs,
    Inputs,
    Focus,
    Session,
    Theme,
    Layout,
}

// ── Rows ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, DeriveTataraDomain)]
#[tatara(keyword = "defparam")]
pub struct Param {
    pub name: Ident,
    #[tatara(keyword_enum)]
    pub kind: ParamKind,
    pub required: bool,
}

/// A field of world-state this action's effect can be read back from.
///
/// The presence or absence of these rows is the WHOLE observability story —
/// there is no separate flag. An author names what can be read, and the class
/// follows.
#[derive(Debug, Clone, PartialEq, Eq, DeriveTataraDomain)]
#[tatara(keyword = "defobserved")]
pub struct Observed {
    #[tatara(keyword_enum)]
    pub domain: ObsDomain,
    pub field: Ident,
}

#[derive(Debug, Clone, PartialEq, Eq, DeriveTataraDomain)]
#[tatara(keyword = "defaction")]
pub struct ActionSpec {
    pub id: ActionId,
    pub gloss: String,
    #[tatara(keyword_enum)]
    pub category: Category,
    #[tatara(keyword_enum)]
    pub kind: Kind,
    #[tatara(keyword_enum)]
    pub auth: Auth,
    #[tatara(domain)]
    pub params: Vec<Param>,
    /// EMPTY means blind. There is deliberately no `observability` field that
    /// could disagree with this list.
    #[tatara(domain)]
    pub observed: Vec<Observed>,
}

/// What a generated surface may do with an action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Class {
    /// A read. Never planned, never applied.
    Read,
    /// A write whose effect can be read back — the reconciler may converge on it.
    Converging,
    /// A write with no read-back path. May be fired; may **never** be counted
    /// toward a fixpoint.
    Blind,
}

impl ActionSpec {
    /// The trichotomy every generated surface is partitioned by.
    ///
    /// DERIVED from data. No author states it, so no author can state it
    /// wrongly — which is the difference between this and a boolean field
    /// somebody sets to `true` because the action feels important.
    #[must_use]
    pub fn class(&self) -> Class {
        match (self.kind, self.observed.is_empty()) {
            (Kind::Observe, _) => Class::Read,
            (Kind::Mutate, false) => Class::Converging,
            (Kind::Mutate, true) => Class::Blind,
        }
    }

    /// The rung a caller must hold.
    #[must_use]
    pub const fn rung(&self) -> Rung {
        self.auth.rung()
    }
}

/// What a catalog can be wrong about in a way the parser cannot catch.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum SpecError {
    #[error("duplicate action id `{0}`")]
    DuplicateId(String),
    #[error("`{0}` is an observe action but names no observed state; a read that reads nothing is a typo, not a blind read")]
    ReaderObservesNothing(String),
    #[error("`{0}` is an observe action at rung {1:?}; a read must not require break-glass")]
    ReaderNeedsBreakGlass(String, Rung),
    #[error("`{0}` has a required parameter after an optional one, which several target languages cannot express")]
    RequiredAfterOptional(String),
}

/// A whole catalog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Catalog {
    pub actions: Vec<ActionSpec>,
}

impl Catalog {
    /// Parse every `(defaction …)` form in a source file.
    ///
    /// # Errors
    /// Returns the reader's or the derive's typed `LispError`.
    pub fn from_source(src: &str) -> tatara_lisp::Result<Self> {
        let forms = tatara_lisp::read(src)?;
        let actions = forms
            .iter()
            .map(ActionSpec::compile_from_sexp)
            .collect::<tatara_lisp::Result<Vec<_>>>()?;
        Ok(Self { actions })
    }

    /// The checks the parse boundary cannot make.
    ///
    /// Tier-honest: **assertion-caught**, not unrepresentable. Uniqueness across
    /// a list and cross-field coherence are not things a Nix-free Rust type
    /// expresses without a great deal more machinery than they are worth.
    ///
    /// # Errors
    /// Returns the first violation.
    pub fn validate(&self) -> Result<(), SpecError> {
        let mut seen = std::collections::HashSet::new();
        for a in &self.actions {
            if !seen.insert(a.id.as_str()) {
                return Err(SpecError::DuplicateId(a.id.as_str().to_string()));
            }
            if a.kind == Kind::Observe && a.observed.is_empty() {
                return Err(SpecError::ReaderObservesNothing(a.id.as_str().to_string()));
            }
            if a.kind == Kind::Observe && a.auth == Auth::L3 {
                return Err(SpecError::ReaderNeedsBreakGlass(
                    a.id.as_str().to_string(),
                    a.rung(),
                ));
            }
            // Positional emitters (Go, and Python without kwargs) cannot put a
            // required parameter after an optional one. Caught here so it is a
            // catalog error rather than three separate emitter bugs.
            if a.params
                .iter()
                .skip_while(|p| p.required)
                .any(|p| p.required)
            {
                return Err(SpecError::RequiredAfterOptional(a.id.as_str().to_string()));
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn by_class(&self, c: Class) -> Vec<&ActionSpec> {
        self.actions.iter().filter(|a| a.class() == c).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CATALOG: &str = include_str!("../../catalog/desktop.saihai.lisp");

    fn catalog() -> Catalog {
        Catalog::from_source(CATALOG).expect("the authored catalog must compile")
    }

    /// ★ Load-bearing, not ceremony: a typo'd struct attribute is SILENT and
    /// yields a keyword computed from the struct name.
    #[test]
    fn every_keyword_is_what_was_written() {
        assert_eq!(ActionSpec::KEYWORD, "defaction");
        assert_eq!(Param::KEYWORD, "defparam");
        assert_eq!(Observed::KEYWORD, "defobserved");
    }

    #[test]
    fn the_authored_catalog_compiles_and_validates() {
        let c = catalog();
        assert!(c.actions.len() >= 60, "got {}", c.actions.len());
        c.validate().expect("authored catalog must validate");
    }

    /// ★ The property the whole border exists for: observability is DERIVED.
    /// An action that names no observed state is blind, and nothing an author
    /// can write says otherwise.
    #[test]
    fn class_is_derived_from_observed_and_cannot_be_authored() {
        let c = catalog();
        for a in &c.actions {
            let expected = match (a.kind, a.observed.is_empty()) {
                (Kind::Observe, _) => Class::Read,
                (Kind::Mutate, false) => Class::Converging,
                (Kind::Mutate, true) => Class::Blind,
            };
            assert_eq!(a.class(), expected, "{}", a.id.as_str());
        }
        // All three classes must be populated, or the catalog is not modelling
        // the distinction it claims to.
        for cl in [Class::Read, Class::Converging, Class::Blind] {
            assert!(!c.by_class(cl).is_empty(), "no actions in {cl:?}");
        }
    }

    #[test]
    fn every_category_and_every_rung_is_represented() {
        let c = catalog();
        for cat in [
            Category::Window,
            Category::Workspace,
            Category::Output,
            Category::Input,
            Category::Session,
            Category::Login,
            Category::Launch,
            Category::Theme,
            Category::System,
        ] {
            assert!(
                c.actions.iter().any(|a| a.category == cat),
                "no action in {cat:?}"
            );
        }
        for r in [Rung::L0, Rung::L1, Rung::L2] {
            assert!(c.actions.iter().any(|a| a.rung() == r), "no action at {r:?}");
        }
    }

    #[test]
    fn a_malformed_action_id_is_rejected_at_the_border() {
        assert!(ActionId::try_from("window-focus".to_string()).is_ok());
        for bad in ["Window-Focus", "window_focus", "-lead", "trail-", "double--dash", ""] {
            assert!(
                ActionId::try_from(bad.to_string()).is_err(),
                "{bad:?} should be rejected"
            );
        }
    }

    /// A parameter named `class` or `type` compiles in Rust and breaks Python
    /// or Go. Caught once, at the border.
    #[test]
    fn a_reserved_word_parameter_is_rejected_once_rather_than_per_emitter() {
        assert!(Ident::try_from("target".to_string()).is_ok());
        for bad in ["class", "type", "func", "range", "None"] {
            assert!(Ident::try_from(bad.to_string()).is_err(), "{bad:?}");
        }
    }

    #[test]
    fn a_reader_that_observes_nothing_is_a_validate_error() {
        let src = r#"(defaction :id "bogus-read" :gloss "reads nothing"
                       :category :window :kind :observe :auth :l0
                       :params () :observed ())"#;
        let c = Catalog::from_source(src).expect("parses");
        assert!(matches!(
            c.validate(),
            Err(SpecError::ReaderObservesNothing(_))
        ));
    }

    #[test]
    fn a_duplicate_id_is_a_validate_error() {
        let one = r#"(defaction :id "dup" :gloss "a" :category :window :kind :mutate
                       :auth :l1 :params () :observed ())"#;
        let c = Catalog::from_source(&format!("{one}\n{one}")).expect("parses");
        assert!(matches!(c.validate(), Err(SpecError::DuplicateId(_))));
    }

    #[test]
    fn identifier_case_mapping_is_stable() {
        let id = ActionId::try_from("window-focus-direction".to_string()).unwrap();
        assert_eq!(id.to_pascal(), "WindowFocusDirection");
        assert_eq!(id.to_snake(), "window_focus_direction");
        let p = Ident::try_from("follow_focus".to_string()).unwrap();
        assert_eq!(p.to_camel(), "followFocus");
    }
}
