#![allow(
    clippy::doc_link_with_quotes, // https://github.com/rust-lang/rust-clippy/issues/8961
    clippy::iter_not_returning_iterator, // https://github.com/rust-lang/rust-clippy/issues/8285
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::new_without_default
)]

mod de;
mod path;
mod ser;
mod wrap;

use std::cell::Cell;
use std::error::Error as StdError;
use std::fmt::{self, Display};

pub use crate::de::{deserialize, Deserializer};
pub use crate::path::{Path, Segment, Segments};
pub use crate::ser::{serialize, Serializer};

/// Original deserializer error together with the path at which it occurred.
#[derive(Clone, Debug)]
pub struct Error<E> {
    path: Path,
    original: E,
}

impl<E> Error<E> {
    /// Element path at which this deserialization error occurred.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// The Deserializer's underlying error that occurred.
    pub fn into_inner(self) -> E {
        self.original
    }

    /// Reference to the Deserializer's underlying error that occurred.
    pub fn inner(&self) -> &E {
        &self.original
    }
}

impl<E: Display> Display for Error<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.path(), self.inner())
    }
}

impl<E: StdError + 'static> StdError for Error<E> {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self.inner())
    }
}

/// State for bookkeeping across nested deserializer calls.
///
/// You don't need this if you are using `serde_path_to_error::deserializer`. If
/// you are managing your own `Deserializer`, see the usage example on
/// [`Deserializer`].
pub struct Track {
    path: Cell<Option<Path>>,
}

impl Track {
    /// Empty state with no error having happened yet.
    pub fn new() -> Self {
        Track {
            path: Cell::new(None),
        }
    }

    /// Gets path at which the error occurred. Only meaningful after we know
    /// that an error has occurred. Returns an empty path otherwise.
    pub fn path(self) -> Path {
        self.path.into_inner().unwrap_or_else(Path::empty)
    }

    #[inline]
    fn trigger<E>(&self, chain: &Chain, err: E) -> E {
        self.trigger_impl(chain);
        err
    }

    fn trigger_impl(&self, chain: &Chain) {
        self.path.set(Some(match self.path.take() {
            Some(already_set) => already_set,
            None => Path::from_chain(chain),
        }));
    }
}

#[derive(Clone)]
enum Chain<'a> {
    Root,
    Seq {
        parent: &'a Chain<'a>,
        index: usize,
    },
    Map {
        parent: &'a Chain<'a>,
        key: String,
    },
    Struct {
        parent: &'a Chain<'a>,
        key: &'static str,
    },
    Enum {
        parent: &'a Chain<'a>,
        variant: String,
    },
    Some {
        parent: &'a Chain<'a>,
    },
    NewtypeStruct {
        parent: &'a Chain<'a>,
    },
    NewtypeVariant {
        parent: &'a Chain<'a>,
    },
    NonStringKey {
        parent: &'a Chain<'a>,
    },
}
