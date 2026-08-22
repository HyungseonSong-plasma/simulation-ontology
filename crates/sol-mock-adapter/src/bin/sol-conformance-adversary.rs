use sol_adapter_protocol::{ExecutePlanRequest, ExecutionProvenance, OpaqueExecutionReference};
use sol_adapter_transport::{
    decode_request, AdapterTransportMethod, JsonRpcResponse, RequestDisposition, TransportRequest,
};
use sol_mock_adapter::MockAdapter;
use std::env;
use std::io::{self, BufRead, BufReader, Write};
use std::process;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AdversaryMode {
    DependencySchedule,
    AggregateEffect,
    ProvenanceIdentity,
}

impl AdversaryMode {
    fn parse(value: &str) -> Option<Self> {
        Some(match value {
            "dependency-schedule" => Self::DependencySchedule,
            "aggregate-effect" => Self::AggregateEffect,
            "provenance-identity" => Self::ProvenanceIdentity,
            _ => return None,
        })
    }
}

fn main() {
    let Some(mode) = env::args()
        .nth(1)
        .and_then(|value| AdversaryMode::parse(&value))
    else {
        eprintln!("conformance adversary requires one recognized mode");
        process::exit(2);
    };

    if let Err(error) = run(mode) {
        eprintln!("conformance adversary failed: {error}");
        process::exit(2);
    }
}

fn run(mode: AdversaryMode) -> Result<(), String> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut input = BufReader::new(stdin.lock());
    let mut output = stdout.lock();

    let bootstrap = read_request(&mut input)?;
    require_method(&bootstrap, AdapterTransportMethod::DescribeAdapter)?;
    let description = MockAdapter::thermal()
        .describe_adapter_operation()
        .map_err(|failure| failure.detail)?;
    let payload = serde_json::from_str(
        &description
            .to_canonical_json()
            .map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())?;
    let response = JsonRpcResponse::protocol_success(bootstrap.id(), payload)
        .map_err(|error| error.to_string())?;
    write_frame(&mut output, &response.to_stdio_frame())?;

    let request = read_request(&mut input)?;
    require_method(&request, AdapterTransportMethod::ExecutePlan)?;
    let params = request
        .params()
        .expect("execute_plan transport request always has params");
    let typed_request = ExecutePlanRequest::from_json(
        &serde_json::to_string(params).expect("JSON Value always serializes"),
    )
    .map_err(|error| error.to_string())?;

    let mut result = MockAdapter::thermal()
        .execute_plan_operation(&typed_request)
        .map_err(|failure| failure.detail)?;

    match mode {
        AdversaryMode::DependencySchedule => {
            result.execution_batches.reverse();
        }
        AdversaryMode::AggregateEffect => {
            result.effects.clear();
        }
        AdversaryMode::ProvenanceIdentity => {
            let semantic_identity = typed_request
                .plan
                .actions
                .first()
                .expect("published execute fixture contains actions")
                .id
                .clone();
            result.provenance = Some(ExecutionProvenance {
                producer: "conformance-adversary".to_owned(),
                opaque_references: vec![OpaqueExecutionReference {
                    namespace: "test-only".to_owned(),
                    reference: semantic_identity,
                    extensions: Default::default(),
                }],
                extensions: Default::default(),
            });
        }
    }

    let payload = serde_json::from_str(
        &result
            .to_canonical_json()
            .map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())?;
    let response = JsonRpcResponse::protocol_success(request.id(), payload)
        .map_err(|error| error.to_string())?;
    write_frame(&mut output, &response.to_stdio_frame())
}

fn read_request(input: &mut impl BufRead) -> Result<TransportRequest, String> {
    let mut frame = String::new();
    if input
        .read_line(&mut frame)
        .map_err(|error| error.to_string())?
        == 0
    {
        return Err("stdin reached EOF before the expected request".to_owned());
    }
    let frame = frame.trim_end_matches(&['\r', '\n'][..]);
    match decode_request(frame) {
        RequestDisposition::Dispatch(request) => Ok(request),
        RequestDisposition::Reject(response) => Err(format!(
            "adversary received a rejected request: {}",
            response.to_json()
        )),
        RequestDisposition::IgnoreNotification => {
            Err("adversary received a notification instead of a request".to_owned())
        }
    }
}

fn require_method(
    request: &TransportRequest,
    expected: AdapterTransportMethod,
) -> Result<(), String> {
    if request.method() == expected {
        Ok(())
    } else {
        Err(format!(
            "adversary expected {} but received {}",
            expected.wire_name(),
            request.method().wire_name()
        ))
    }
}

fn write_frame(output: &mut impl Write, bytes: &[u8]) -> Result<(), String> {
    output.write_all(bytes).map_err(|error| error.to_string())?;
    output.flush().map_err(|error| error.to_string())
}
