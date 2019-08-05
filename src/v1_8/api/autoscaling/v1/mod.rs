
mod cross_version_object_reference;
pub use self::cross_version_object_reference::{
    CrossVersionObjectReference,
};

mod horizontal_pod_autoscaler;
pub use self::horizontal_pod_autoscaler::{
    HorizontalPodAutoscaler,
    CreateNamespacedHorizontalPodAutoscalerOptional, CreateNamespacedHorizontalPodAutoscalerResponse,
    DeleteCollectionNamespacedHorizontalPodAutoscalerResponse,
    DeleteNamespacedHorizontalPodAutoscalerResponse,
    ListHorizontalPodAutoscalerForAllNamespacesResponse,
    ListNamespacedHorizontalPodAutoscalerResponse,
    PatchNamespacedHorizontalPodAutoscalerResponse,
    PatchNamespacedHorizontalPodAutoscalerStatusResponse,
    ReadNamespacedHorizontalPodAutoscalerOptional, ReadNamespacedHorizontalPodAutoscalerResponse,
    ReadNamespacedHorizontalPodAutoscalerStatusOptional, ReadNamespacedHorizontalPodAutoscalerStatusResponse,
    ReplaceNamespacedHorizontalPodAutoscalerOptional, ReplaceNamespacedHorizontalPodAutoscalerResponse,
    ReplaceNamespacedHorizontalPodAutoscalerStatusOptional, ReplaceNamespacedHorizontalPodAutoscalerStatusResponse,
    WatchHorizontalPodAutoscalerForAllNamespacesResponse,
    WatchNamespacedHorizontalPodAutoscalerResponse,
};

mod horizontal_pod_autoscaler_list;
pub use self::horizontal_pod_autoscaler_list::{
    HorizontalPodAutoscalerList,
};

mod horizontal_pod_autoscaler_spec;
pub use self::horizontal_pod_autoscaler_spec::{
    HorizontalPodAutoscalerSpec,
};

mod horizontal_pod_autoscaler_status;
pub use self::horizontal_pod_autoscaler_status::{
    HorizontalPodAutoscalerStatus,
};

mod scale;
pub use self::scale::{
    Scale,
    PatchNamespacedReplicationControllerScaleResponse,
    ReadNamespacedReplicationControllerScaleOptional, ReadNamespacedReplicationControllerScaleResponse,
    ReplaceNamespacedReplicationControllerScaleOptional, ReplaceNamespacedReplicationControllerScaleResponse,
};

mod scale_spec;
pub use self::scale_spec::{
    ScaleSpec,
};

mod scale_status;
pub use self::scale_status::{
    ScaleStatus,
};