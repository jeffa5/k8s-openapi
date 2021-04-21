// Generated from definition io.k8s.api.core.v1.ContainerStatus

/// ContainerStatus contains details for the current status of this container.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ContainerStatus {
    /// Container's ID in the format 'docker://\<container_id\>'.
    pub container_id: Option<String>,

    /// The image the container is running. More info: https://kubernetes.io/docs/concepts/containers/images
    pub image: String,

    /// ImageID of the container's image.
    pub image_id: String,

    /// Details about the container's last termination condition.
    pub last_state: Option<crate::api::core::v1::ContainerState>,

    /// This must be a DNS_LABEL. Each container in a pod must have a unique name. Cannot be updated.
    pub name: String,

    /// Specifies whether the container has passed its readiness probe.
    pub ready: bool,

    /// The number of times the container has been restarted, currently based on the number of dead containers that have not yet been removed. Note that this is calculated from dead containers. But those containers are subject to garbage collection. This value will get capped at 5 by GC.
    pub restart_count: i32,

    /// Specifies whether the container has passed its startup probe. Initialized as false, becomes true after startupProbe is considered successful. Resets to false when the container is restarted, or if kubelet loses state temporarily. Is always true when no startupProbe is defined.
    pub started: Option<bool>,

    /// Details about the container's current condition.
    pub state: Option<crate::api::core::v1::ContainerState>,
}

impl<'de> serde::Deserialize<'de> for ContainerStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_container_id,
            Key_image,
            Key_image_id,
            Key_last_state,
            Key_name,
            Key_ready,
            Key_restart_count,
            Key_started,
            Key_state,
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
                            "containerID" => Field::Key_container_id,
                            "image" => Field::Key_image,
                            "imageID" => Field::Key_image_id,
                            "lastState" => Field::Key_last_state,
                            "name" => Field::Key_name,
                            "ready" => Field::Key_ready,
                            "restartCount" => Field::Key_restart_count,
                            "started" => Field::Key_started,
                            "state" => Field::Key_state,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContainerStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ContainerStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_container_id: Option<String> = None;
                let mut value_image: Option<String> = None;
                let mut value_image_id: Option<String> = None;
                let mut value_last_state: Option<crate::api::core::v1::ContainerState> = None;
                let mut value_name: Option<String> = None;
                let mut value_ready: Option<bool> = None;
                let mut value_restart_count: Option<i32> = None;
                let mut value_started: Option<bool> = None;
                let mut value_state: Option<crate::api::core::v1::ContainerState> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_container_id => value_container_id = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_image => value_image = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_image_id => value_image_id = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_last_state => value_last_state = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_ready => value_ready = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_restart_count => value_restart_count = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_started => value_started = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_state => value_state = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ContainerStatus {
                    container_id: value_container_id,
                    image: value_image.ok_or_else(|| serde::de::Error::missing_field("image"))?,
                    image_id: value_image_id.ok_or_else(|| serde::de::Error::missing_field("imageID"))?,
                    last_state: value_last_state,
                    name: value_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    ready: value_ready.ok_or_else(|| serde::de::Error::missing_field("ready"))?,
                    restart_count: value_restart_count.ok_or_else(|| serde::de::Error::missing_field("restartCount"))?,
                    started: value_started,
                    state: value_state,
                })
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: serde::de::SeqAccess<'de> {
                Ok(ContainerStatus {
                    container_id: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("container_id"))?,
                    image: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("image"))?,
                    image_id: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("image_id"))?,
                    last_state: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("last_state"))?,
                    name: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    ready: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("ready"))?,
                    restart_count: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("restart_count"))?,
                    started: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("started"))?,
                    state: serde::de::SeqAccess::next_element(&mut seq)?.ok_or_else(|| serde::de::Error::missing_field("state"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ContainerStatus",
            &[
                "containerID",
                "image",
                "imageID",
                "lastState",
                "name",
                "ready",
                "restartCount",
                "started",
                "state",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ContainerStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ContainerStatus",
            5 +
            self.container_id.as_ref().map_or(0, |_| 1) +
            self.last_state.as_ref().map_or(0, |_| 1) +
            self.started.as_ref().map_or(0, |_| 1) +
            self.state.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.container_id {
            serde::ser::SerializeStruct::serialize_field(&mut state, "containerID", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "containerID")?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "image", &self.image)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "imageID", &self.image_id)?;
        if let Some(value) = &self.last_state {
            serde::ser::SerializeStruct::serialize_field(&mut state, "lastState", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "lastState")?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "ready", &self.ready)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "restartCount", &self.restart_count)?;
        if let Some(value) = &self.started {
            serde::ser::SerializeStruct::serialize_field(&mut state, "started", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "started")?;
        }
        if let Some(value) = &self.state {
            serde::ser::SerializeStruct::serialize_field(&mut state, "state", &Some(value))?;
        }
        else {
            serde::ser::SerializeStruct::skip_field(&mut state, "state")?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
