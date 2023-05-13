use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub(crate) document: Option<Document>,
    pub(crate) components: Option<ComponentSets>,
    #[serde(rename = "componentSets")]
    pub(crate) component_sets: Option<ComponentSets>,
    #[serde(rename = "schemaVersion")]
    pub(crate) schema_version: Option<i64>,
    pub(crate) styles: Option<ComponentSets>,
    pub(crate) name: Option<String>,
    #[serde(rename = "lastModified")]
    pub(crate) last_modified: Option<String>,
    #[serde(rename = "thumbnailUrl")]
    pub(crate) thumbnail_url: Option<String>,
    pub(crate) version: Option<String>,
    pub(crate) role: Option<String>,
    #[serde(rename = "editorType")]
    pub(crate) editor_type: Option<String>,
    #[serde(rename = "linkAccess")]
    pub(crate) link_access: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComponentSets {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub(crate) id: Option<String>,
    pub(crate) name: Option<String>,
    #[serde(rename = "type")]
    pub(crate) document_type: Option<String>,
    #[serde(rename = "scrollBehavior")]
    pub(crate) scroll_behavior: Option<ScrollBehavior>,
    pub(crate) children: Option<Vec<Child>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Child {
    pub(crate) id: Option<String>,
    pub(crate) name: Option<String>,
    #[serde(rename = "type")]
    pub(crate) child_type: Option<Type>,
    #[serde(rename = "scrollBehavior")]
    pub(crate) scroll_behavior: Option<ScrollBehavior>,
    pub(crate) children: Option<Vec<Option<serde_json::Value>>>,
    #[serde(rename = "backgroundColor")]
    pub(crate) background_color: Option<BackgroundColor>,
    #[serde(rename = "prototypeStartNodeID")]
    pub(crate) prototype_start_node_id: Option<serde_json::Value>,
    #[serde(rename = "flowStartingPoints")]
    pub(crate) flow_starting_points: Option<Vec<Option<serde_json::Value>>>,
    #[serde(rename = "prototypeDevice")]
    pub(crate) prototype_device: Option<PrototypeDevice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackgroundColor {
    pub(crate) r: Option<f64>,
    pub(crate) g: Option<f64>,
    pub(crate) b: Option<f64>,
    pub(crate) a: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrototypeDevice {
    #[serde(rename = "type")]
    pub(crate) prototype_device_type: Option<Rotation>,
    pub(crate) rotation: Option<Rotation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "CANVAS")]
    Canvas,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Rotation {
    #[serde(rename = "NONE")]
    None,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ScrollBehavior {
    #[serde(rename = "SCROLLS")]
    Scrolls,
}

