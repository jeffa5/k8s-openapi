// Generated from definition io.k8s.api.apps.v1beta2.DaemonSetUpdateStrategy

/// DaemonSetUpdateStrategy is a struct used to control the update strategy for a DaemonSet.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DaemonSetUpdateStrategy {
    /// Rolling update config params. Present only if type = "RollingUpdate".
    pub rolling_update: Option<crate::api::apps::v1beta2::RollingUpdateDaemonSet>,

    /// Type of daemon set update. Can be "RollingUpdate" or "OnDelete". Default is RollingUpdate.
    pub type_: Option<String>,
}

impl<'de> serde::Deserialize<'de> for DaemonSetUpdateStrategy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_rolling_update,
            Key_type_,
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
                            "rollingUpdate" => Field::Key_rolling_update,
                            "type" => Field::Key_type_,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DaemonSetUpdateStrategy;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("DaemonSetUpdateStrategy")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_rolling_update: Option<crate::api::apps::v1beta2::RollingUpdateDaemonSet> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_rolling_update => value_rolling_update = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DaemonSetUpdateStrategy {
                    rolling_update: value_rolling_update,
                    type_: value_type_,
                })
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: serde::de::SeqAccess<'de> {
                Ok(DaemonSetUpdateStrategy {
                    rolling_update: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("rolling_update"))?,
                    type_: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("type_"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "DaemonSetUpdateStrategy",
            &[
                "rollingUpdate",
                "type",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for DaemonSetUpdateStrategy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DaemonSetUpdateStrategy",
            self.rolling_update.as_ref().map_or(0, |_| 1) +
            self.type_.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.rolling_update {
            serde::ser::SerializeStruct::serialize_field(&mut state, "rollingUpdate", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "rollingUpdate")?;
        }
        if let Some(value) = &self.type_ {
            serde::ser::SerializeStruct::serialize_field(&mut state, "type", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "type")?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
