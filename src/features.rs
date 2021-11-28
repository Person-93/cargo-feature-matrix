use derive_more::{AsMut, AsRef, Deref, DerefMut};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeSet, HashSet},
    convert::Infallible,
    str::FromStr,
};

#[derive(
    Clone, Debug, Deref, DerefMut, AsRef, AsMut, Serialize, Deserialize,
)]
pub struct FeatureMatrix(HashSet<FeatureSet>);

#[derive(
    Clone,
    Debug,
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
pub struct FeatureSet(BTreeSet<Feature>);

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
#[as_ref(forward)]
#[as_mut(forward)]
#[serde(transparent)]
pub struct Feature(String);

impl FromStr for FeatureSet {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(FeatureSet(
            s.split(',').map(ToString::to_string).map(Feature).collect(),
        ))
    }
}
