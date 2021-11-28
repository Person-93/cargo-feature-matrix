use crate::features::{FeatureMatrix, FeatureSet};
use std::{collections::HashSet, num::NonZeroU8};

#[non_exhaustive]
pub struct Config {
    /// If this set is not empty, only these features will be used to construct the
    /// matrix.
    pub allow: FeatureSet,

    /// All of these features will be included in every feature set in the matrix.
    pub include: FeatureSet,

    /// Any feature set that includes any of these will be excluded from the matrix.
    /// This includes features enabled by other features.
    ///
    /// This can be used for things like having an "__unstable" feature that gets
    /// enabled by any other features that use unstable rust features and then
    /// excluding "__unstable" if not on nightly.
    pub deny: FeatureSet,

    /// These sets will be dropped from the matrix.
    pub skip: FeatureMatrix,

    /// Some crates prepend internal features with a double underscore. If this
    /// flag is set, those features will not be used to build the matrix, but
    /// will be allowed if they are enabled by other features.
    pub exclude_double_underscore: bool,

    /// List sets of features that can't be used together. Any generated feature
    /// set that is a superset of any of these sets will be dropped from the matrix.
    pub conflict: HashSet<FeatureSet>,

    /// See [`Choose`].
    pub choose: HashSet<Choose>,
}

/// Each set in the matrix must include `count` members in `set`. Any generated
/// features set that does not meet this requirement will be dropped from the
/// matrix.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Choose {
    pub count: NonZeroU8,
    pub set: FeatureSet,
}