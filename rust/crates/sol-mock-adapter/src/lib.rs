use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MockAdapterResponse {
    pub adapter: String,
    pub status: String,
    pub payload: Value,
}

pub fn echo(payload: Value) -> MockAdapterResponse {
    MockAdapterResponse {
        adapter: "mock-adapter".to_string(),
        status: "ok".to_string(),
        payload,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn mock_adapter_is_deterministic_for_same_payload() {
        let payload = json!({"case": "thermal"});
        assert_eq!(echo(payload.clone()), echo(payload));
    }
}
