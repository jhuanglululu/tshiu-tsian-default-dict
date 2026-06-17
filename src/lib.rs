//! Character→bopomofo (注音) lookup data for Mandarin.
//!
//! The pronunciation data is extracted from the g0v moedict-data project
//! (Ministry of Education dictionaries).
//! See `LICENSE` / [`license_notice`].

/// The bundled dictionary as JSON: `[{"c": "一", "bopomofo": ["ㄧ"]}, …]`.
pub static DEFAULT_DICT_STR: &str = include_str!("../dict.json");

/// Attribution notice for the bundled data (MOE / moedict-data, CC BY-ND 3.0 TW).
///
/// Surface this in an about/credits screen if you want the attribution to
/// travel with your application.
pub fn license_notice() -> &'static str {
    include_str!("../LICENSE")
}
