use derive_more::{AsMut, AsRef, Deref, DerefMut};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::{
    collections::{BTreeSet, HashSet},
    convert::Infallible,
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(
    Clone, Debug, Default, Deref, DerefMut, AsRef, AsMut, Serialize, Deserialize,
)]
pub struct FeatureMatrix(HashSet<FeatureSet>);

#[derive(
    Clone, Debug, Default, Eq, PartialEq, Hash, Deref, DerefMut, AsRef, AsMut,
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

impl Serialize for FeatureSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for FeatureSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        return deserializer.deserialize_str(Visitor);

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = FeatureSet;

            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                write!(f, "a valid feature set")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(v.parse().unwrap())
            }
        }
    }
}

impl Display for FeatureSet {
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

impl FromStr for FeatureSet {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.split(',').map(ToString::to_string).map(Feature).collect())
    }
}

impl FromIterator<Feature> for FeatureSet {
    fn from_iter<T: IntoIterator<Item = Feature>>(iter: T) -> Self {
        FeatureSet(iter.into_iter().collect())
    }
}

impl Display for Feature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
