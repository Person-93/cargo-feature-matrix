use derive_more::{AsMut, AsRef, Deref, DerefMut};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::{
    collections::{BTreeSet, HashSet},
    fmt::{Display, Formatter},
    ops::Deref,
};

#[derive(
    Clone, Debug, Default, Deref, DerefMut, AsRef, AsMut, Serialize, Deserialize,
)]
pub struct FeatureMatrix<'f>(#[serde(borrow)] HashSet<FeatureSet<'f>>);

#[derive(
    Clone, Debug, Default, Eq, PartialEq, Hash, Deref, DerefMut, AsRef, AsMut,
)]
pub struct FeatureSet<'f>(BTreeSet<Feature<'f>>);

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

impl Display for Feature<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
