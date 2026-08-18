//! `(defbancada …)` — the declared desktop.
//!
//! The workbench, declared: what the surface should look like, in the
//! operator's terms rather than in the reconciler's. It lowers to the flat
//! `key -> value` state the loop compares against the world, so the authoring
//! form can get richer without the loop learning anything new.
//!
//! ## Why lowering, rather than authoring keys directly
//!
//! A flat `("outputs.mode", "2560x1440")` is what the reconciler wants and the
//! worst thing to ask a person for: nothing checks the key exists, nothing
//! checks the value parses, and a typo is a silent no-op that reads as
//! "converged" forever. The typed form makes both a parse error, and the
//! lowering is one function with one test.
//!
//! ## What this deliberately does NOT express
//!
//! Anything blind. There is no `(defwantscreenshot …)`, because a screenshot
//! is not a state a desktop can be held at — firing it twice makes two files.
//! The declarative surface admits only what the loop can converge on, which is
//! the same partition [`crate::Class`] draws, enforced here by there being no
//! syntax for the other case.

use tatara_lisp::{DeriveTataraDomain, TataraDomain};

use crate::Ident;

/// The theme the desktop should be wearing.
#[derive(Debug, Clone, PartialEq, Eq, DeriveTataraDomain)]
#[tatara(keyword = "defwanttheme")]
pub struct WantTheme {
    pub name: String,
}

/// One output's declared geometry.
#[derive(Debug, Clone, PartialEq, Eq, DeriveTataraDomain)]
#[tatara(keyword = "defwantoutput")]
pub struct WantOutput {
    pub name: String,
    /// `WIDTHxHEIGHT`. Validated on lowering, not here — the value is one
    /// string in the wire format the action takes.
    pub mode: Option<String>,
    pub scale: Option<i64>,
    pub enabled: Option<bool>,
}

/// Keyboard behaviour — the half of "keyboard control" that is state rather
/// than an event.
#[derive(Debug, Clone, PartialEq, Eq, DeriveTataraDomain)]
#[tatara(keyword = "defwantinput")]
pub struct WantInput {
    pub repeat_rate: Option<i64>,
    pub repeat_delay: Option<i64>,
    pub layout: Option<String>,
    /// The active keymap mode — the modal-editing surface, declared.
    pub mode: Option<String>,
}

/// Session state the desktop should be holding.
#[derive(Debug, Clone, PartialEq, Eq, DeriveTataraDomain)]
#[tatara(keyword = "defwantsession")]
pub struct WantSession {
    pub locked: Option<bool>,
}

/// A key binding, declared. Present because "keyboard control" is a first-class
/// part of the goal, and a binding is genuinely state: it either exists or it
/// does not, and the loop can read it back.
#[derive(Debug, Clone, PartialEq, Eq, DeriveTataraDomain)]
#[tatara(keyword = "defwantbinding")]
pub struct WantBinding {
    pub chord: String,
    pub action: String,
    pub mode: Option<String>,
}

/// The declared desktop.
#[derive(Debug, Clone, PartialEq, Eq, DeriveTataraDomain)]
#[tatara(keyword = "defbancada")]
pub struct Bancada {
    /// Which node this declaration is for. Not decorative: a loop that reads
    /// the wrong node's declaration would converge a machine onto someone
    /// else's desktop, and the daemon refuses on mismatch.
    pub node: Ident,
    #[tatara(domain)]
    pub theme: Option<WantTheme>,
    #[tatara(domain)]
    pub outputs: Vec<WantOutput>,
    #[tatara(domain)]
    pub input: Option<WantInput>,
    #[tatara(domain)]
    pub session: Option<WantSession>,
    #[tatara(domain)]
    pub bindings: Vec<WantBinding>,
}

/// What a declaration can be wrong about that parsing cannot catch.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum BancadaError {
    #[error("output `{0}` declares mode {1:?}; expected WIDTHxHEIGHT, e.g. \"2560x1440\"")]
    BadMode(String, String),
    #[error("output `{0}` declares scale {1}; expected a positive integer")]
    BadScale(String, i64),
    #[error("two outputs both named `{0}`")]
    DuplicateOutput(String),
    #[error("two bindings both claim chord `{0}` in mode {1:?}")]
    DuplicateBinding(String, String),
    #[error("key repeat rate {0} is out of range 1..=1000")]
    BadRepeatRate(i64),
}

impl Bancada {
    /// Parse a declaration.
    ///
    /// # Errors
    /// The reader's or the derive's typed `LispError`.
    pub fn from_source(src: &str) -> tatara_lisp::Result<Self> {
        let forms = tatara_lisp::read(src)?;
        let first = forms
            .first()
            .ok_or_else(|| tatara_lisp::LispError::Compile {
                form: Self::KEYWORD.to_string(),
                message: "source contains no forms".into(),
            })?;
        Self::compile_from_sexp(first)
    }

    /// The checks the parse boundary cannot make.
    ///
    /// Tier-honest: **assertion-caught**. Uniqueness across a list and
    /// numeric ranges are not things this border expresses as types without
    /// more machinery than they are worth.
    ///
    /// # Errors
    /// The first violation found.
    pub fn validate(&self) -> Result<(), BancadaError> {
        let mut seen = std::collections::HashSet::new();
        for o in &self.outputs {
            if !seen.insert(o.name.clone()) {
                return Err(BancadaError::DuplicateOutput(o.name.clone()));
            }
            if let Some(m) = &o.mode {
                let ok = m.split_once('x').is_some_and(|(w, h)| {
                    !w.is_empty()
                        && !h.is_empty()
                        && w.chars().all(|c| c.is_ascii_digit())
                        && h.chars().all(|c| c.is_ascii_digit())
                });
                if !ok {
                    return Err(BancadaError::BadMode(o.name.clone(), m.clone()));
                }
            }
            if let Some(s) = o.scale {
                if s <= 0 {
                    return Err(BancadaError::BadScale(o.name.clone(), s));
                }
            }
        }
        if let Some(i) = &self.input {
            if let Some(r) = i.repeat_rate {
                if !(1..=1000).contains(&r) {
                    return Err(BancadaError::BadRepeatRate(r));
                }
            }
        }
        let mut chords = std::collections::HashSet::new();
        for b in &self.bindings {
            let key = (b.chord.clone(), b.mode.clone().unwrap_or_default());
            if !chords.insert(key) {
                return Err(BancadaError::DuplicateBinding(
                    b.chord.clone(),
                    b.mode.clone().unwrap_or_default(),
                ));
            }
        }
        Ok(())
    }

    /// Lower to the flat state the reconciler compares.
    ///
    /// Keys match the catalog's `observed` addressing (`domain.field`,
    /// lowercased), which is what makes a declared key routable: the routing
    /// table is built from the same rows. A key produced here that no action
    /// observes will surface as `Gap::NoAction` rather than silently doing
    /// nothing — visible, which is the point.
    #[must_use]
    pub fn lower(&self) -> std::collections::BTreeMap<String, String> {
        let mut m = std::collections::BTreeMap::new();
        if let Some(t) = &self.theme {
            m.insert("theme.name".into(), t.name.clone());
        }
        for o in &self.outputs {
            // Per-output keys are scoped by name, so two monitors do not
            // collapse into one desired value.
            if let Some(mode) = &o.mode {
                m.insert(format!("outputs.mode.{}", o.name), mode.clone());
            }
            if let Some(s) = o.scale {
                m.insert(format!("outputs.scale.{}", o.name), s.to_string());
            }
            if let Some(e) = o.enabled {
                m.insert(format!("outputs.enabled.{}", o.name), e.to_string());
            }
        }
        if let Some(i) = &self.input {
            if let Some(r) = i.repeat_rate {
                m.insert("inputs.repeat.rate".into(), r.to_string());
            }
            if let Some(d) = i.repeat_delay {
                m.insert("inputs.repeat.delay".into(), d.to_string());
            }
            if let Some(l) = &i.layout {
                m.insert("inputs.layout".into(), l.clone());
            }
            if let Some(md) = &i.mode {
                m.insert("inputs.mode".into(), md.clone());
            }
        }
        if let Some(s) = &self.session {
            if let Some(l) = s.locked {
                m.insert("session.locked".into(), l.to_string());
            }
        }
        for b in &self.bindings {
            let scope = b.mode.clone().unwrap_or_else(|| "default".into());
            m.insert(
                format!("inputs.bindings.{scope}.{}", b.chord),
                b.action.clone(),
            );
        }
        m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PLO: &str = include_str!("../../bancada/plo.bancada.lisp");

    fn plo() -> Bancada {
        Bancada::from_source(PLO).expect("plo's declaration must compile")
    }

    /// Load-bearing: a typo'd struct attribute is SILENT and yields a keyword
    /// computed from the struct name.
    #[test]
    fn every_keyword_is_what_was_written() {
        assert_eq!(Bancada::KEYWORD, "defbancada");
        assert_eq!(WantTheme::KEYWORD, "defwanttheme");
        assert_eq!(WantOutput::KEYWORD, "defwantoutput");
        assert_eq!(WantInput::KEYWORD, "defwantinput");
        assert_eq!(WantSession::KEYWORD, "defwantsession");
        assert_eq!(WantBinding::KEYWORD, "defwantbinding");
    }

    #[test]
    fn plos_declaration_compiles_and_validates() {
        let b = plo();
        assert_eq!(b.node.as_str(), "plo");
        b.validate().expect("must validate");
        assert!(b.theme.is_some(), "plo declares a theme");
    }

    /// ★ The lowering is total and addressed the way the catalog observes, so
    /// a declared key is routable.
    #[test]
    fn lowering_produces_catalog_addressed_keys() {
        let m = plo().lower();
        assert!(!m.is_empty());
        assert_eq!(m.get("theme.name").map(String::as_str), Some("nord"));
        for k in m.keys() {
            assert!(k.contains('.'), "key {k} is not domain.field addressed");
            // Only the DOMAIN prefix is lowercase. The tail may carry a
            // hardware proper noun — `outputs.mode.DP-1` — and lowercasing
            // that would stop it matching what the compositor reports, which
            // is the whole point of scoping per-output keys by name.
            let domain = k.split('.').next().unwrap();
            assert_eq!(
                domain.to_lowercase(),
                domain,
                "domain {domain} must be lowercase"
            );
        }
    }

    /// Two monitors must not collapse into one desired value.
    #[test]
    fn per_output_keys_are_scoped_by_name() {
        let src = r#"(defbancada :node "test"
          :outputs ((defwantoutput :name "DP-1" :mode "2560x1440")
                    (defwantoutput :name "HDMI-1" :mode "1920x1080")))"#;
        let m = Bancada::from_source(src).unwrap().lower();
        assert_eq!(
            m.get("outputs.mode.DP-1").map(String::as_str),
            Some("2560x1440")
        );
        assert_eq!(
            m.get("outputs.mode.HDMI-1").map(String::as_str),
            Some("1920x1080")
        );
    }

    #[test]
    fn a_malformed_mode_is_a_validate_error() {
        let src = r#"(defbancada :node "test"
          :outputs ((defwantoutput :name "DP-1" :mode "2560by1440")))"#;
        let b = Bancada::from_source(src).expect("parses");
        assert!(matches!(b.validate(), Err(BancadaError::BadMode(..))));
    }

    #[test]
    fn duplicate_outputs_and_bindings_are_validate_errors() {
        let dup_out = r#"(defbancada :node "test"
          :outputs ((defwantoutput :name "DP-1") (defwantoutput :name "DP-1")))"#;
        assert!(matches!(
            Bancada::from_source(dup_out).unwrap().validate(),
            Err(BancadaError::DuplicateOutput(_))
        ));

        let dup_bind = r#"(defbancada :node "test"
          :bindings ((defwantbinding :chord "Mod+Return" :action "launch-spawn")
                     (defwantbinding :chord "Mod+Return" :action "window-close")))"#;
        assert!(matches!(
            Bancada::from_source(dup_bind).unwrap().validate(),
            Err(BancadaError::DuplicateBinding(..))
        ));
    }

    #[test]
    fn an_out_of_range_repeat_rate_is_caught_by_validate_not_by_parse() {
        let src = r#"(defbancada :node "test"
          :input (defwantinput :repeat-rate 99999))"#;
        // It PARSES — tatara-lisp cannot bound an integer — and validate is
        // what rejects it. Pinned so the tier is not later described as
        // unrepresentability.
        let b = Bancada::from_source(src).expect("an out-of-range rate still parses");
        assert!(matches!(
            b.validate(),
            Err(BancadaError::BadRepeatRate(99999))
        ));
    }

    /// ★ There is no syntax for a blind action. The declarative surface admits
    /// only what the loop can converge on, and that is enforced by absence
    /// rather than by a check — you cannot declare a screenshot.
    #[test]
    fn the_form_has_no_syntax_for_a_blind_action() {
        let src = r#"(defbancada :node "test" :screenshot "yes")"#;
        let err = Bancada::from_source(src)
            .expect_err("there is no such field")
            .to_string();
        assert!(err.contains("screenshot"), "should name the bad key: {err}");
    }
}
