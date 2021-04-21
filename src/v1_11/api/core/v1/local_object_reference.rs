// Generated from definition io.k8s.api.core.v1.LocalObjectReference

/// LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LocalObjectReference {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    pub name: Option<String>,
}

impl<'de> serde::Deserialize<'de> for LocalObjectReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "name" => Field::Key_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LocalObjectReference;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("LocalObjectReference")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_name: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(LocalObjectReference {
                    name: value_name,
                })
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: serde::de::SeqAccess<'de> {
                Ok(LocalObjectReference {
                    name: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "LocalObjectReference",
            &[
                "name",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for LocalObjectReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "LocalObjectReference",
            self.name.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.name {
            serde::ser::SerializeStruct::serialize_field(&mut state, "name", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "name")?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
