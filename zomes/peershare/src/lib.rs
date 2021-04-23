use hdk::prelude::*;

#[hdk_extern]
pub fn who_am_i(_: ()) -> ExternResult<AgentPubKey> {
    Ok(agent_info()?.agent_latest_pubkey)
}

// You need to send your input arguments via a serializable struct
#[derive(Clone, Serialize, Deserialize, Debug, SerializedBytes)]
#[serde(rename_all = "camelCase")]
pub struct InputArgumentDTO {
    pub content: String,
}

// It is recommended to send back the zome function result via a serializable struct
#[derive(Clone, Serialize, Deserialize, Debug, SerializedBytes)]
#[serde(rename_all = "camelCase")]
pub struct HelloWordResultDTO {
    pub data: String,
}
#[hdk_extern]
pub fn hello_word(input: InputArgumentDTO) -> ExternResult<HelloWordResultDTO> {
    let result = HelloWordResultDTO {
        data: input.content,
    };
    Ok(result)
}
