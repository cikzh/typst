//! Data loading.

#[cfg(feature = "fat")]
#[path = "cbor.rs"]
mod cbor_;
#[path = "csv.rs"]
mod csv_;
#[path = "json.rs"]
mod json_;
#[path = "read.rs"]
mod read_;
#[path = "toml.rs"]
mod toml_;
#[path = "xml.rs"]
mod xml_;
#[cfg(feature = "fat")]
#[path = "yaml.rs"]
mod yaml_;

#[cfg(feature = "fat")]
pub use self::cbor_::*;
pub use self::csv_::*;
pub use self::json_::*;
pub use self::read_::*;
pub use self::toml_::*;
pub use self::xml_::*;
#[cfg(feature = "fat")]
pub use self::yaml_::*;

use crate::foundations::{cast, category, Bytes, Category, Scope, Str};

/// Data loading from external files.
///
/// These functions help you with loading and embedding data, for example from
/// the results of an experiment.
#[category]
pub static DATA_LOADING: Category;

/// Hook up all `data-loading` definitions.
pub(super) fn define(global: &mut Scope) {
    global.category(DATA_LOADING);
    global.define_func::<read>();
    global.define_func::<csv>();
    global.define_func::<json>();
    global.define_func::<toml>();
    #[cfg(feature = "fat")]
    global.define_func::<yaml>();
    #[cfg(feature = "fat")]
    global.define_func::<cbor>();
    global.define_func::<xml>();
}

/// A value that can be read from a file.
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Readable {
    /// A decoded string.
    Str(Str),
    /// Raw bytes.
    Bytes(Bytes),
}

impl Readable {
    fn as_slice(&self) -> &[u8] {
        match self {
            Readable::Bytes(v) => v,
            Readable::Str(v) => v.as_bytes(),
        }
    }
}

cast! {
    Readable,
    self => match self {
        Self::Str(v) => v.into_value(),
        Self::Bytes(v) => v.into_value(),
    },
    v: Str => Self::Str(v),
    v: Bytes => Self::Bytes(v),
}

impl From<Readable> for Bytes {
    fn from(value: Readable) -> Self {
        match value {
            Readable::Bytes(v) => v,
            Readable::Str(v) => v.as_bytes().into(),
        }
    }
}
