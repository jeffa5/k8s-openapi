// Generated from definition io.k8s.api.core.v1.NodeSpec

/// NodeSpec describes the attributes that a node is created with.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeSpec {
    /// If specified, the source to get node configuration from The DynamicKubeletConfig feature gate must be enabled for the Kubelet to use this field
    pub config_source: Option<crate::api::core::v1::NodeConfigSource>,

    /// Deprecated. Not all kubelets will set this field. Remove field after 1.13. see: https://issues.k8s.io/61966
    pub external_id: Option<String>,

    /// PodCIDR represents the pod IP range assigned to the node.
    pub pod_cidr: Option<String>,

    /// podCIDRs represents the IP ranges assigned to the node for usage by Pods on that node. If this field is specified, the 0th entry must match the podCIDR field. It may contain at most 1 value for each of IPv4 and IPv6.
    pub pod_cidrs: Option<Vec<String>>,

    /// ID of the node assigned by the cloud provider in the format: \<ProviderName\>://\<ProviderSpecificNodeID\>
    pub provider_id: Option<String>,

    /// If specified, the node's taints.
    pub taints: Option<Vec<crate::api::core::v1::Taint>>,

    /// Unschedulable controls node schedulability of new pods. By default, node is schedulable. More info: https://kubernetes.io/docs/concepts/nodes/node/#manual-node-administration
    pub unschedulable: Option<bool>,
}

impl<'de> serde::Deserialize<'de> for NodeSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_config_source,
            Key_external_id,
            Key_pod_cidr,
            Key_pod_cidrs,
            Key_provider_id,
            Key_taints,
            Key_unschedulable,
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
                            "configSource" => Field::Key_config_source,
                            "externalID" => Field::Key_external_id,
                            "podCIDR" => Field::Key_pod_cidr,
                            "podCIDRs" => Field::Key_pod_cidrs,
                            "providerID" => Field::Key_provider_id,
                            "taints" => Field::Key_taints,
                            "unschedulable" => Field::Key_unschedulable,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NodeSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("NodeSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_config_source: Option<crate::api::core::v1::NodeConfigSource> = None;
                let mut value_external_id: Option<String> = None;
                let mut value_pod_cidr: Option<String> = None;
                let mut value_pod_cidrs: Option<Vec<String>> = None;
                let mut value_provider_id: Option<String> = None;
                let mut value_taints: Option<Vec<crate::api::core::v1::Taint>> = None;
                let mut value_unschedulable: Option<bool> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_config_source => value_config_source = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_external_id => value_external_id = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_cidr => value_pod_cidr = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pod_cidrs => value_pod_cidrs = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_provider_id => value_provider_id = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_taints => value_taints = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_unschedulable => value_unschedulable = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NodeSpec {
                    config_source: value_config_source,
                    external_id: value_external_id,
                    pod_cidr: value_pod_cidr,
                    pod_cidrs: value_pod_cidrs,
                    provider_id: value_provider_id,
                    taints: value_taints,
                    unschedulable: value_unschedulable,
                })
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: serde::de::SeqAccess<'de> {
                Ok(NodeSpec {
                    config_source: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("config_source"))?,
                    external_id: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("external_id"))?,
                    pod_cidr: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("pod_cidr"))?,
                    pod_cidrs: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("pod_cidrs"))?,
                    provider_id: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("provider_id"))?,
                    taints: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("taints"))?,
                    unschedulable: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("unschedulable"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "NodeSpec",
            &[
                "configSource",
                "externalID",
                "podCIDR",
                "podCIDRs",
                "providerID",
                "taints",
                "unschedulable",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for NodeSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NodeSpec",
            self.config_source.as_ref().map_or(0, |_| 1) +
            self.external_id.as_ref().map_or(0, |_| 1) +
            self.pod_cidr.as_ref().map_or(0, |_| 1) +
            self.pod_cidrs.as_ref().map_or(0, |_| 1) +
            self.provider_id.as_ref().map_or(0, |_| 1) +
            self.taints.as_ref().map_or(0, |_| 1) +
            self.unschedulable.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.config_source {
            serde::ser::SerializeStruct::serialize_field(&mut state, "configSource", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "configSource")?;
        }
        if let Some(value) = &self.external_id {
            serde::ser::SerializeStruct::serialize_field(&mut state, "externalID", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "externalID")?;
        }
        if let Some(value) = &self.pod_cidr {
            serde::ser::SerializeStruct::serialize_field(&mut state, "podCIDR", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "podCIDR")?;
        }
        if let Some(value) = &self.pod_cidrs {
            serde::ser::SerializeStruct::serialize_field(&mut state, "podCIDRs", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "podCIDRs")?;
        }
        if let Some(value) = &self.provider_id {
            serde::ser::SerializeStruct::serialize_field(&mut state, "providerID", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "providerID")?;
        }
        if let Some(value) = &self.taints {
            serde::ser::SerializeStruct::serialize_field(&mut state, "taints", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "taints")?;
        }
        if let Some(value) = &self.unschedulable {
            serde::ser::SerializeStruct::serialize_field(&mut state, "unschedulable", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "unschedulable")?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
