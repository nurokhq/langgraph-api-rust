/*
 * Tests for model serialization and deserialization
 */

use langgraph_api::models::HealthResponse;
use langgraph_api::models::health_response::Ok;

#[test]
fn test_health_response_serialization() {
    // Create a HealthResponse instance
    let health_response = HealthResponse::new(Ok::True);

    // Test serialization
    let json = serde_json::to_string(&health_response);
    assert!(json.is_ok(), "HealthResponse should serialize successfully");

    let json_str = json.unwrap();
    assert!(
        json_str.contains("\"ok\":\"true\""),
        "Serialized JSON should contain ok field with value true"
    );
}

#[test]
fn test_health_response_deserialization() {
    // Test JSON string
    let json_str = r#"{"ok":"true"}"#;

    // Test deserialization
    let result: Result<HealthResponse, _> = serde_json::from_str(json_str);
    assert!(
        result.is_ok(),
        "HealthResponse should deserialize successfully"
    );

    let health_response = result.unwrap();
    assert_eq!(health_response.ok, Ok::True, "Deserialized ok should match");
}

#[test]
fn test_health_response_round_trip() {
    // Create a HealthResponse
    let original = HealthResponse::new(Ok::True);

    // Serialize to JSON
    let json_str = serde_json::to_string(&original).unwrap();

    // Deserialize back
    let deserialized: HealthResponse = serde_json::from_str(&json_str).unwrap();

    // Verify round trip
    assert_eq!(
        original.ok, deserialized.ok,
        "Ok should match after round trip"
    );
}

#[test]
fn test_health_response_new() {
    // Test the new() constructor
    let health_response = HealthResponse::new(Ok::True);

    assert_eq!(health_response.ok, Ok::True);
}

#[test]
fn test_health_response_default() {
    // Test the Default trait
    let health_response = HealthResponse::default();

    assert_eq!(health_response.ok, Ok::True);
}
