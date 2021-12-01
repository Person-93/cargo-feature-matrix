use crate::Config;
use cargo_metadata::Package;
use derive_more::{AsMut, AsRef, Deref, DerefMut};
use itertools::Itertools;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::{
    collections::{BTreeSet, HashSet},
    fmt::{Display, Formatter},
    ops::Deref,
};
use thiserror::Error;

#[derive(
    Clone, Debug, Default, Deref, DerefMut, AsRef, AsMut, Serialize, Deserialize,
)]
pub struct FeatureMatrix<'f>(#[serde(borrow)] HashSet<FeatureSet<'f>>);

impl<'f> FeatureMatrix<'f> {
    pub(crate) fn new(
        package: &'f Package,
        config: &'f Config<'f>,
    ) -> Result<Self, MissingFeature> {
        let mut include = config.include.clone();
        include.add_transitive_features(package)?;
        let include = include;

        extract_seed(package, config)
            .into_iter()
            .powerset()
            .map(FeatureSet::from_iter)
            .map(|mut set| -> Result<_, MissingFeature> {
                set.extend(include.clone());
                set.add_transitive_features(package)?;
                Ok(set)
            })
            .filter_ok(|set| set.is_disjoint(&config.deny))
            .filter_ok(|set| !config.skip.iter().any(|skip| skip == set))
            .filter_ok(|set| {
                !config
                    .conflict
                    .iter()
                    .any(|conflict| set.is_superset(conflict))
            })
            .collect()
    }
}

/// Reads the package + config and outputs the set of features that should be used to
/// seed the matrix.
fn extract_seed<'f>(
    package: &'f Package,
    config: &'f Config<'f>,
) -> FeatureSet<'f> {
    if !config.seed.is_empty() {
        config.seed.clone()
    } else {
        package
            .features
            .keys()
            .map(|feature| Feature(feature))
            // exclude default feature
            .filter(|feature| feature.0 != "default")
            // exclude deny list because they will all end up denied anyways
            .filter(|package| !config.deny.iter().contains(package))
            // exclude the include list because it'll be easier to just add them all at once
            .filter(|package| !config.include.iter().contains(package))
            // exclude hidden features by default
            .filter(|feature| {
                config.include_hidden || !feature.starts_with("__")
            })
            // add the optional dependencies to the list
            .chain(
                package
                    .dependencies
                    .iter()
                    .filter_map(|dependency| {
                        dependency.optional.then(|| {
                            dependency
                                .rename
                                .as_deref()
                                .unwrap_or(&dependency.name)
                        })
                    })
                    .map(Feature),
            )
            .collect()
    }
}

#[derive(
    Clone, Debug, Default, Eq, PartialEq, Hash, Deref, DerefMut, AsRef, AsMut,
)]
pub struct FeatureSet<'f>(BTreeSet<Feature<'f>>);

impl<'f> FeatureSet<'f> {
    fn add_transitive_features(
        &mut self,
        package: &'f Package,
    ) -> Result<(), MissingFeature> {
        let raw_features = &package.features;
        let transitive = self
            .iter()
            .map(|feature| -> Result<_, _> {
                Ok(raw_features
                    .get(*feature.as_ref())
                    .ok_or_else(|| MissingFeature(feature.to_string()))?
                    .iter()
                    .map(AsRef::<str>::as_ref))
            })
            .flatten_ok()
            .map_ok(Feature)
            .collect::<Result<Vec<_>, _>>()?;
        self.extend(transitive);
        Ok(())
    }
}

#[derive(
    Clone,
    Debug,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Hash,
    Deref,
    DerefMut,
    AsRef,
    AsMut,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct Feature<'f>(pub(crate) &'f str);

impl Serialize for FeatureSet<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'f> FromIterator<FeatureSet<'f>> for FeatureMatrix<'f> {
    fn from_iter<T: IntoIterator<Item = FeatureSet<'f>>>(iter: T) -> Self {
        FeatureMatrix(iter.into_iter().collect())
    }
}

impl<'f> IntoIterator for FeatureMatrix<'f> {
    type Item = FeatureSet<'f>;
    type IntoIter = <<Self as Deref>::Target as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'de: 'f, 'f> Deserialize<'de> for FeatureSet<'f> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        return deserializer.deserialize_str(Visitor);

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = FeatureSet<'de>;

            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                write!(f, "a valid feature set")
            }

            fn visit_borrowed_str<E>(
                self,
                v: &'de str,
            ) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(FeatureSet::from(v))
            }
        }
    }
}

impl Display for FeatureSet<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.iter();
        if let Some(feature) = iter.next() {
            Display::fmt(feature, f)?;
        }
        for feature in iter {
            write!(f, ",{}", feature)?;
        }
        Ok(())
    }
}

impl<'f> From<&'f str> for FeatureSet<'f> {
    fn from(s: &'f str) -> Self {
        s.split(',').map(Feature).collect()
    }
}

impl<'f> FromIterator<Feature<'f>> for FeatureSet<'f> {
    fn from_iter<T: IntoIterator<Item = Feature<'f>>>(iter: T) -> Self {
        FeatureSet(iter.into_iter().collect())
    }
}

impl<'f> IntoIterator for FeatureSet<'f> {
    type Item = Feature<'f>;
    type IntoIter = <<Self as Deref>::Target as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Display for Feature<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

#[derive(Debug, Error)]
#[error("feature `{}` not present in Cargo.toml", _0)]
pub struct MissingFeature(String);
