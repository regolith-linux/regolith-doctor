//! Check identifiers.
//!
//! Every check has a stable identifier made of an area code and a three digit
//! number, for example `ENV-001` or `CFG-014`. Identifiers never change once
//! published; a retired check keeps its number.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// The area a check belongs to. The variants are in alphabetical order of
/// their codes, so sorting by `Area` sorts like the text form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Area {
    /// Apt sources and keys.
    Apt,
    /// Status bars and their blocks.
    Bar,
    /// Window manager configuration (partials, bindings).
    Cfg,
    /// Compositor.
    Comp,
    /// Displays and outputs.
    Disp,
    /// The platform Regolith runs on.
    Env,
    /// Pointer and touchpad input.
    Input,
    /// Keyboard layouts and shortcuts.
    Kbd,
    /// Launchers and desktop entries.
    Lnch,
    /// Looks, themes, fonts and wallpaper.
    Look,
    /// Notifications.
    Notif,
    /// Installed packages.
    Pkg,
    /// Desktop portals.
    Port,
    /// Power, lid and lock.
    Pwr,
    /// Session startup and environment.
    Ses,
    /// Xresources.
    Xres,
}

impl Area {
    /// Every area, in sort order.
    pub const ALL: [Area; 16] = [
        Area::Apt,
        Area::Bar,
        Area::Cfg,
        Area::Comp,
        Area::Disp,
        Area::Env,
        Area::Input,
        Area::Kbd,
        Area::Lnch,
        Area::Look,
        Area::Notif,
        Area::Pkg,
        Area::Port,
        Area::Pwr,
        Area::Ses,
        Area::Xres,
    ];

    /// The upper-case code used in identifiers, such as `"ENV"`.
    pub const fn code(self) -> &'static str {
        match self {
            Area::Apt => "APT",
            Area::Bar => "BAR",
            Area::Cfg => "CFG",
            Area::Comp => "COMP",
            Area::Disp => "DISP",
            Area::Env => "ENV",
            Area::Input => "INPUT",
            Area::Kbd => "KBD",
            Area::Lnch => "LNCH",
            Area::Look => "LOOK",
            Area::Notif => "NOTIF",
            Area::Pkg => "PKG",
            Area::Port => "PORT",
            Area::Pwr => "PWR",
            Area::Ses => "SES",
            Area::Xres => "XRES",
        }
    }
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

impl FromStr for Area {
    type Err = ParseIdError;

    /// Parses an area code, ignoring ASCII case.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Area::ALL
            .into_iter()
            .find(|a| a.code().eq_ignore_ascii_case(s))
            .ok_or_else(|| ParseIdError::UnknownArea(s.to_owned()))
    }
}

/// A check identifier such as `ENV-001`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CheckId {
    area: Area,
    number: u16,
}

impl CheckId {
    /// Builds an identifier. `number` must be between 1 and 999.
    ///
    /// # Panics
    ///
    /// Panics if `number` is out of range; use [`str::parse`] for untrusted
    /// input.
    pub const fn new(area: Area, number: u16) -> Self {
        assert!(number >= 1 && number <= 999, "check number out of range");
        CheckId { area, number }
    }

    /// The area part.
    pub const fn area(self) -> Area {
        self.area
    }

    /// The number part.
    pub const fn number(self) -> u16 {
        self.number
    }
}

impl fmt::Display for CheckId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{:03}", self.area.code(), self.number)
    }
}

/// Why a string is not a valid area or check identifier.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ParseIdError {
    /// The area code is not one of [`Area::ALL`].
    #[error("unknown check area `{0}`")]
    UnknownArea(String),
    /// The text is not of the form `AREA-NNN`.
    #[error("`{0}` is not a check id (expected AREA-NNN, for example ENV-001)")]
    Malformed(String),
}

impl FromStr for CheckId {
    type Err = ParseIdError;

    /// Parses `AREA-NNN`. The area ignores ASCII case; the number must have
    /// exactly three digits and must not be `000`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let malformed = || ParseIdError::Malformed(s.to_owned());
        let (area, number) = s.split_once('-').ok_or_else(malformed)?;
        let area: Area = area.parse()?;
        if number.len() != 3 || !number.bytes().all(|b| b.is_ascii_digit()) {
            return Err(malformed());
        }
        let number: u16 = number.parse().map_err(|_| malformed())?;
        if number == 0 {
            return Err(malformed());
        }
        Ok(CheckId { area, number })
    }
}

impl Serialize for CheckId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for CheckId {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        text.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_pads_the_number() {
        assert_eq!(CheckId::new(Area::Env, 1).to_string(), "ENV-001");
        assert_eq!(CheckId::new(Area::Notif, 12).to_string(), "NOTIF-012");
    }

    #[test]
    fn parse_round_trips_and_ignores_area_case() {
        let id: CheckId = "env-001".parse().unwrap();
        assert_eq!(id, CheckId::new(Area::Env, 1));
        assert_eq!(
            "XRES-009".parse::<CheckId>().unwrap().to_string(),
            "XRES-009"
        );
    }

    #[test]
    fn parse_rejects_bad_ids() {
        for bad in [
            "ENV", "ENV-1", "ENV-0001", "ENV-000", "ENV-01a", "FOO-001", "-001", "",
        ] {
            assert!(bad.parse::<CheckId>().is_err(), "{bad} should not parse");
        }
    }

    #[test]
    fn area_order_matches_code_order() {
        let mut codes: Vec<&str> = Area::ALL.iter().map(|a| a.code()).collect();
        let before = codes.clone();
        codes.sort_unstable();
        assert_eq!(codes, before);
    }

    #[test]
    fn serde_uses_the_text_form() {
        let id = CheckId::new(Area::Cfg, 14);
        assert_eq!(serde_json::to_string(&id).unwrap(), "\"CFG-014\"");
        let back: CheckId = serde_json::from_str("\"CFG-014\"").unwrap();
        assert_eq!(back, id);
        assert!(serde_json::from_str::<CheckId>("\"CFG-14\"").is_err());
    }
}
