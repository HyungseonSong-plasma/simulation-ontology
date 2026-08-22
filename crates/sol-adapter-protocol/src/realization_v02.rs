use serde::{Deserialize, Serialize};
use serde_json::Value;
use sol_public_contract::{
    BackendTargetDtoV02, MappingPlanDtoV02, RealizationSpecDtoV02, PUBLIC_CONTRACT_VERSION_0_2,
};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::{
    canonicalize_value, reject_transport_markers, AdapterProtocolVersion, Extensions,
    ProtocolError, ADAPTER_PROTOCOL_VERSION_0_2,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidatePlanRequestV02 {
    pub adapter_protocol_version: String,
    pub target: BackendTargetDtoV02,
    pub plan: MappingPlanDtoV02,
    pub realization_spec: RealizationSpecDtoV02,
    #[serde(flatten, default)]
    pub extensions: Extensions,
}

impl ValidatePlanRequestV02 {
    pub fn new(
        target: BackendTargetDtoV02,
        plan: MappingPlanDtoV02,
        realization_spec: RealizationSpecDtoV02,
    ) -> Result<Self, RealizationRequestError> {
        let mut request = Self {
            adapter_protocol_version: ADAPTER_PROTOCOL_VERSION_0_2.to_owned(),
            target,
            plan,
            realization_spec,
            extensions: BTreeMap::new(),
        };
        request.normalize()?;
        Ok(request)
    }

    pub fn from_json(input: &str) -> Result<Self, RealizationRequestError> {
        let value: Value = serde_json::from_str(input)
            .map_err(|error| RealizationRequestError::InvalidRequest(error.to_string()))?;
        reject_transport_markers(&value).map_err(RealizationRequestError::Protocol)?;
        reject_v02_forbidden_fields(&value)?;
        let mut request: Self = serde_json::from_value(value)
            .map_err(|error| RealizationRequestError::InvalidRequest(error.to_string()))?;
        request.normalize()?;
        Ok(request)
    }

    pub fn to_canonical_json(&self) -> Result<String, RealizationRequestError> {
        let mut normalized = self.clone();
        normalized.normalize()?;
        canonical_request_json(&normalized)
    }

    pub fn canonical_plan_identity(&self) -> Result<String, RealizationRequestError> {
        self.plan.to_canonical_json().map_err(|error| {
            RealizationRequestError::InvalidPublicPayload {
                field: "plan",
                detail: error.to_string(),
            }
        })
    }

    pub fn canonical_realization_identity(&self) -> Result<String, RealizationRequestError> {
        self.realization_spec.to_canonical_json().map_err(|error| {
            RealizationRequestError::InvalidPublicPayload {
                field: "realization_spec",
                detail: error.to_string(),
            }
        })
    }

    fn normalize(&mut self) -> Result<(), RealizationRequestError> {
        normalize_realization_request(
            &self.adapter_protocol_version,
            &mut self.target,
            &mut self.plan,
            &mut self.realization_spec,
            &self.extensions,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutePlanRequestV02 {
    pub adapter_protocol_version: String,
    pub target: BackendTargetDtoV02,
    pub plan: MappingPlanDtoV02,
    pub realization_spec: RealizationSpecDtoV02,
    #[serde(flatten, default)]
    pub extensions: Extensions,
}

impl ExecutePlanRequestV02 {
    pub fn new(
        target: BackendTargetDtoV02,
        plan: MappingPlanDtoV02,
        realization_spec: RealizationSpecDtoV02,
    ) -> Result<Self, RealizationRequestError> {
        let mut request = Self {
            adapter_protocol_version: ADAPTER_PROTOCOL_VERSION_0_2.to_owned(),
            target,
            plan,
            realization_spec,
            extensions: BTreeMap::new(),
        };
        request.normalize()?;
        Ok(request)
    }

    pub fn from_json(input: &str) -> Result<Self, RealizationRequestError> {
        let value: Value = serde_json::from_str(input)
            .map_err(|error| RealizationRequestError::InvalidRequest(error.to_string()))?;
        reject_transport_markers(&value).map_err(RealizationRequestError::Protocol)?;
        reject_v02_forbidden_fields(&value)?;
        let mut request: Self = serde_json::from_value(value)
            .map_err(|error| RealizationRequestError::InvalidRequest(error.to_string()))?;
        request.normalize()?;
        Ok(request)
    }

    pub fn to_canonical_json(&self) -> Result<String, RealizationRequestError> {
        let mut normalized = self.clone();
        normalized.normalize()?;
        canonical_request_json(&normalized)
    }

    pub fn canonical_plan_identity(&self) -> Result<String, RealizationRequestError> {
        self.plan.to_canonical_json().map_err(|error| {
            RealizationRequestError::InvalidPublicPayload {
                field: "plan",
                detail: error.to_string(),
            }
        })
    }

    pub fn canonical_realization_identity(&self) -> Result<String, RealizationRequestError> {
        self.realization_spec.to_canonical_json().map_err(|error| {
            RealizationRequestError::InvalidPublicPayload {
                field: "realization_spec",
                detail: error.to_string(),
            }
        })
    }

    fn normalize(&mut self) -> Result<(), RealizationRequestError> {
        normalize_realization_request(
            &self.adapter_protocol_version,
            &mut self.target,
            &mut self.plan,
            &mut self.realization_spec,
            &self.extensions,
        )
    }
}

fn normalize_realization_request(
    adapter_protocol_version: &str,
    target: &mut BackendTargetDtoV02,
    plan: &mut MappingPlanDtoV02,
    realization_spec: &mut RealizationSpecDtoV02,
    extensions: &Extensions,
) -> Result<(), RealizationRequestError> {
    require_v02_protocol_version(adapter_protocol_version)?;

    let extension_value = Value::Object(
        extensions
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect(),
    );
    reject_transport_markers(&extension_value).map_err(RealizationRequestError::Protocol)?;
    reject_v02_forbidden_fields(&extension_value)?;

    if target.public_contract_version != plan.public_contract_version
        || target.public_contract_version != realization_spec.public_contract_version
    {
        return Err(RealizationRequestError::IncoherentPublicContractVersions {
            target: target.public_contract_version.clone(),
            plan: plan.public_contract_version.clone(),
            realization_spec: realization_spec.public_contract_version.clone(),
        });
    }
    if target.public_contract_version != PUBLIC_CONTRACT_VERSION_0_2 {
        return Err(RealizationRequestError::UnsupportedPublicContractVersion(
            target.public_contract_version.clone(),
        ));
    }

    *target = BackendTargetDtoV02::from_json(&target.to_canonical_json().map_err(|error| {
        RealizationRequestError::InvalidPublicPayload {
            field: "target",
            detail: error.to_string(),
        }
    })?)
    .map_err(|error| RealizationRequestError::InvalidPublicPayload {
        field: "target",
        detail: error.to_string(),
    })?;

    *plan = MappingPlanDtoV02::from_json(&plan.to_canonical_json().map_err(|error| {
        RealizationRequestError::InvalidPublicPayload {
            field: "plan",
            detail: error.to_string(),
        }
    })?)
    .map_err(|error| RealizationRequestError::InvalidPublicPayload {
        field: "plan",
        detail: error.to_string(),
    })?;

    *realization_spec =
        RealizationSpecDtoV02::from_json(&realization_spec.to_canonical_json().map_err(
            |error| RealizationRequestError::InvalidPublicPayload {
                field: "realization_spec",
                detail: error.to_string(),
            },
        )?)
        .map_err(|error| RealizationRequestError::InvalidPublicPayload {
            field: "realization_spec",
            detail: error.to_string(),
        })?;

    realization_spec
        .validate_against_plan(plan)
        .map_err(|error| RealizationRequestError::PlanRealizationMismatch(error.to_string()))
}

fn require_v02_protocol_version(version: &str) -> Result<(), RealizationRequestError> {
    let parsed =
        AdapterProtocolVersion::parse(version).map_err(RealizationRequestError::Protocol)?;
    if parsed != AdapterProtocolVersion::realization_v02() {
        return Err(RealizationRequestError::UnsupportedProtocolVersion(
            version.to_owned(),
        ));
    }
    Ok(())
}

fn reject_v02_forbidden_fields(value: &Value) -> Result<(), RealizationRequestError> {
    const FORBIDDEN: &[&str] = &[
        "validation_token",
        "acceptance_id",
        "lease_id",
        "plan_hash",
        "plan_digest",
        "execution_authority",
        "replay_authority",
        "retry_authority",
        "backend_native_id",
        "backend_object",
        "native_object",
        "solver_object",
    ];

    match value {
        Value::Object(object) => {
            for (key, child) in object {
                if FORBIDDEN.contains(&key.as_str()) {
                    return Err(RealizationRequestError::ForbiddenField(key.clone()));
                }
                reject_v02_forbidden_fields(child)?;
            }
        }
        Value::Array(values) => {
            for child in values {
                reject_v02_forbidden_fields(child)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn canonical_request_json<T: Serialize>(value: &T) -> Result<String, RealizationRequestError> {
    let value = serde_json::to_value(value)
        .map_err(|error| RealizationRequestError::InvalidRequest(error.to_string()))?;
    serde_json::to_string(&canonicalize_value(value))
        .map_err(|error| RealizationRequestError::InvalidRequest(error.to_string()))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RealizationRequestError {
    Protocol(ProtocolError),
    InvalidRequest(String),
    InvalidPublicPayload {
        field: &'static str,
        detail: String,
    },
    UnsupportedProtocolVersion(String),
    UnsupportedPublicContractVersion(String),
    IncoherentPublicContractVersions {
        target: String,
        plan: String,
        realization_spec: String,
    },
    PlanRealizationMismatch(String),
    ForbiddenField(String),
}

impl Display for RealizationRequestError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Protocol(error) => write!(formatter, "{error}"),
            Self::InvalidRequest(detail) => {
                write!(formatter, "invalid Adapter Protocol 0.2 realization request: {detail}")
            }
            Self::InvalidPublicPayload { field, detail } => {
                write!(formatter, "invalid Public Contract 0.2 {field}: {detail}")
            }
            Self::UnsupportedProtocolVersion(version) => {
                write!(formatter, "unsupported Adapter Protocol version for realization request: {version}")
            }
            Self::UnsupportedPublicContractVersion(version) => {
                write!(formatter, "unsupported Public Contract version for realization request: {version}")
            }
            Self::IncoherentPublicContractVersions {
                target,
                plan,
                realization_spec,
            } => write!(
                formatter,
                "Adapter Protocol 0.2 request Public Contract versions differ: target={target}, plan={plan}, realization_spec={realization_spec}"
            ),
            Self::PlanRealizationMismatch(detail) => {
                write!(formatter, "MappingPlan/RealizationSpec mismatch: {detail}")
            }
            Self::ForbiddenField(field) => write!(
                formatter,
                "field is forbidden from Adapter Protocol 0.2 realization request semantics: {field}"
            ),
        }
    }
}

impl Error for RealizationRequestError {}
