use crate::features::{FeatureMatrix, FeatureSet};
use figment::{
    value::{Dict, Map},
    Error, Figment, Metadata, Profile, Provider,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Config {
    /// If this set is not empty, only these features will be used to construct the
    /// matrix.
    pub seed: FeatureSet,

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
}

impl Config {
    pub fn from<T: Provider>(provider: T) -> Result<Config, Error> {
        Figment::from(provider).extract()
    }

    pub fn figment() -> Figment {
        Figment::from(Config::default())
    }
}

impl Provider for Config {
    fn metadata(&self) -> Metadata {
        Metadata::named("Config object")
    }

    fn data(&self) -> Result<Map<Profile, Dict>, Error> {
        figment::providers::Serialized::defaults(self).data()
    }
}
