use asl_core_traits::CapabilityContext;
use asl_security::MockSecurityContext;
use asl_spec::AslError;

#[test]
fn test_fuel_exhaustion_halts_execution() {
    let ctx = MockSecurityContext::new(5);
    // An operation that consumes 10 fuel (e.g. http_request) must abort immediately
    let res = ctx.http_request("GET", "https://api.test", &[], None);
    assert!(matches!(res, Err(AslError::LimitExceeded(_))));
}
