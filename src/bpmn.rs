use serde::Deserialize;
#[derive(Debug, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Definitions {
    #[serde(rename = "process", default)]
    pub processes: Vec<Process>,
}

#[derive(Debug, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Process {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@isExecutable")]
    pub is_executable: bool,

    #[serde(rename = "startEvent", default)]
    pub start_events: Vec<StartEvent>,
    #[serde(rename = "sequenceFlow", default)]
    pub sequence_flows: Vec<SequenceFlow>,
    #[serde(rename = "scriptTask", default)]
    pub script_tasks: Vec<ScriptTask>,
    #[serde(rename = "userTask", default)]
    pub user_tasks: Vec<UserTask>,
    #[serde(rename = "serviceTask", default)]
    pub service_tasks: Vec<ServiceTask>,
    #[serde(rename = "exclusiveGateway", default)]
    pub exclusive_gateways: Vec<ExclusiveGateway>,
    #[serde(rename = "endEvent", default)]
    pub end_events: Vec<EndEvent>,
}

#[derive(Debug, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StartEvent {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(default)]
    pub outgoing: Vec<String>,
}

#[derive(Debug, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SequenceFlow {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@sourceRef")]
    pub source_ref: String,
    #[serde(rename = "@targetRef")]
    pub target_ref: String,
}

#[derive(Debug, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ScriptTask {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(default)]
    pub incoming: Vec<String>,
    #[serde(default)]
    pub outgoing: Vec<String>,
}

#[derive(Debug, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UserTask {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(default)]
    pub incoming: Vec<String>,
    #[serde(default)]
    pub outgoing: Vec<String>,
}

#[derive(Debug, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ServiceTask {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(default)]
    pub incoming: Vec<String>,
    #[serde(default)]
    pub outgoing: Vec<String>,
}

#[derive(Debug, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExclusiveGateway {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(default)]
    pub incoming: Vec<String>,
    #[serde(default)]
    pub outgoing: Vec<String>,
}

#[derive(Debug, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EndEvent {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(default)]
    pub incoming: Vec<String>,
}
