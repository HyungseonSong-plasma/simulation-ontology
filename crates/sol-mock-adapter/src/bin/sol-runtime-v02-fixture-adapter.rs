use sol_adapter_protocol::{
    AdapterDescription, ExecutePlanRequestV02, ExecutePlanResponseV02, ProtocolFailure,
    ValidatePlanRequestV02, ValidatePlanResponseV02,
};
use sol_adapter_transport::{
    decode_request, AdapterTransportMethod, JsonRpcResponse, RequestDisposition, StdioFrameDecoder,
    TransportRequest,
};
use std::io::{self, Read, Write};
use std::process;

const DESCRIPTION: &str = include_str!(
    "../../../../fixtures/adapter-protocol/0.2/realization-compatible-description.json"
);
const THERMAL_REQUEST: &str =
    include_str!("../../../../fixtures/adapter-protocol/0.2/thermal-realization-request.json");
const EXECUTE_RESPONSE: &str = include_str!(
    "../../../../fixtures/adapter-protocol/0.2/execute-plan-exact-response.json"
);

fn main() {
    if let Err(error) = run() {
        eprintln!("runtime v0.2 fixture adapter transport error: {error}");
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut input = stdin.lock();
    let mut output = stdout.lock();
    let mut decoder = StdioFrameDecoder::new();
    let mut buffer = [0_u8; 8192];

    loop {
        let count = input.read(&mut buffer).map_err(|error| error.to_string())?;
        if count == 0 {
            decoder.finish().map_err(|error| error.to_string())?;
            return Ok(());
        }

        for frame in decoder.push(&buffer[..count]) {
            let frame = frame.map_err(|error| error.to_string())?;
            if let Some(response) = handle_frame(&frame)? {
                output
                    .write_all(&response)
                    .map_err(|error| error.to_string())?;
                output.flush().map_err(|error| error.to_string())?;
            }
        }
    }
}

fn handle_frame(frame: &str) -> Result<Option<Vec<u8>>, String> {
    match decode_request(frame) {
        RequestDisposition::Dispatch(request) => Ok(Some(dispatch(&request)?.to_stdio_frame())),
        RequestDisposition::Reject(response) => Ok(Some(response.to_stdio_frame())),
        RequestDisposition::IgnoreNotification => Ok(None),
    }
}

fn dispatch(request: &TransportRequest) -> Result<JsonRpcResponse, String> {
    match request.method() {
        AdapterTransportMethod::DescribeAdapter => {
            let description = AdapterDescription::from_json(DESCRIPTION)
                .map_err(|error| error.to_string())?;
            success_response(
                request,
                description
                    .to_canonical_json()
                    .map_err(|error| error.to_string())?,
            )
        }
        AdapterTransportMethod::ValidatePlan => {
            let outcome = parse_validate_request(request).and_then(|received| {
                require_exact_validate_fixture(&received)?;
                Ok(ValidatePlanResponseV02::accepted())
            });
            match outcome {
                Ok(response) => success_response(
                    request,
                    response
                        .to_canonical_json()
                        .map_err(|error| error.to_string())?,
                ),
                Err(failure) => failure_response(request, &failure),
            }
        }
        AdapterTransportMethod::ExecutePlan => {
            let outcome = parse_execute_request(request).and_then(|received| {
                require_exact_execute_fixture(&received)?;
                let mut response = ExecutePlanResponseV02::from_json(EXECUTE_RESPONSE)
                    .map_err(invalid_request_failure)?;
                response
                    .validate_against(&received)
                    .map_err(invalid_request_failure)?;
                Ok(response)
            });
            match outcome {
                Ok(response) => success_response(
                    request,
                    response
                        .to_canonical_json()
                        .map_err(|error| error.to_string())?,
                ),
                Err(failure) => failure_response(request, &failure),
            }
        }
    }
}

fn parse_validate_request(
    request: &TransportRequest,
) -> Result<ValidatePlanRequestV02, ProtocolFailure> {
    let json = params_json(request);
    ValidatePlanRequestV02::from_json(&json).map_err(invalid_request_failure)
}

fn parse_execute_request(
    request: &TransportRequest,
) -> Result<ExecutePlanRequestV02, ProtocolFailure> {
    let json = params_json(request);
    ExecutePlanRequestV02::from_json(&json).map_err(invalid_request_failure)
}

fn params_json(request: &TransportRequest) -> String {
    serde_json::to_string(
        request
            .params()
            .expect("validate/execute transport requests always have params"),
    )
    .expect("JSON Value always serializes")
}

fn require_exact_validate_fixture(
    received: &ValidatePlanRequestV02,
) -> Result<(), ProtocolFailure> {
    let expected = ValidatePlanRequestV02::from_json(THERMAL_REQUEST)
        .map_err(invalid_request_failure)?;
    if received
        .to_canonical_json()
        .map_err(invalid_request_failure)?
        != expected
            .to_canonical_json()
            .map_err(invalid_request_failure)?
    {
        return Err(invalid_request_failure(
            "runtime changed the canonical Protocol 0.2 thermal validate request",
        ));
    }
    Ok(())
}

fn require_exact_execute_fixture(
    received: &ExecutePlanRequestV02,
) -> Result<(), ProtocolFailure> {
    let expected =
        ExecutePlanRequestV02::from_json(THERMAL_REQUEST).map_err(invalid_request_failure)?;
    if received
        .to_canonical_json()
        .map_err(invalid_request_failure)?
        != expected
            .to_canonical_json()
            .map_err(invalid_request_failure)?
    {
        return Err(invalid_request_failure(
            "runtime changed the canonical Protocol 0.2 thermal execute request",
        ));
    }
    Ok(())
}

fn invalid_request_failure(error: impl ToString) -> ProtocolFailure {
    ProtocolFailure::invalid_request(error.to_string())
        .expect("fixture adapter error detail produces a valid ProtocolFailure")
}

fn success_response(
    request: &TransportRequest,
    canonical_payload: String,
) -> Result<JsonRpcResponse, String> {
    let payload = serde_json::from_str(&canonical_payload).map_err(|error| error.to_string())?;
    JsonRpcResponse::protocol_success(request.id(), payload).map_err(|error| error.to_string())
}

fn failure_response(
    request: &TransportRequest,
    failure: &ProtocolFailure,
) -> Result<JsonRpcResponse, String> {
    JsonRpcResponse::protocol_failure(request.id(), request.method(), failure)
        .map_err(|error| error.to_string())
}
