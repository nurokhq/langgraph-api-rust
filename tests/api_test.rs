/*
 * Tests for API functions
 */

use langgraph_api::apis::UploadFile;
use langgraph_api::apis::configuration::Configuration;
use langgraph_api::apis::system_api::health_check_ok_get_request_builder;

#[tokio::test]
async fn test_health_check_request_builder() {
    // Create a test configuration
    let config = Configuration {
        base_path: "https://api.example.com".to_string(),
        user_agent: Some("test-agent/1.0".to_string()),
        ..Default::default()
    };

    // Test that the request builder is created successfully
    let result = health_check_ok_get_request_builder(&config, None);
    assert!(
        result.is_ok(),
        "Request builder should be created successfully"
    );

    // Verify the request builder can build a request
    let req_builder = result.unwrap();
    let req = req_builder.build();
    assert!(req.is_ok(), "Request should be built successfully");

    let request = req.unwrap();
    assert_eq!(
        request.url().to_string(),
        "https://api.example.com/ok",
        "URL should match expected health endpoint"
    );
    assert_eq!(
        request.method(),
        reqwest::Method::GET,
        "HTTP method should be GET"
    );
}

#[tokio::test]
async fn test_health_check_request_builder_with_default_config() {
    // Test with default configuration
    let config = Configuration::default();
    let result = health_check_ok_get_request_builder(&config, None);
    assert!(
        result.is_ok(),
        "Request builder should work with default config"
    );

    let req_builder = result.unwrap();
    let req = req_builder.build().unwrap();
    assert_eq!(
        req.url().to_string(),
        "http://localhost:8444/ok",
        "Default base path should be used"
    );
    assert_eq!(
        req.headers()
            .get(reqwest::header::USER_AGENT)
            .unwrap()
            .to_str()
            .unwrap(),
        format!(
            "{}-rust/{}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        ),
        "User agent should be set to the package name and version"
    );
}

#[tokio::test]
async fn test_health_check_with_check_db() {
    // Test health check with check_db parameter
    let config = Configuration {
        base_path: "http://localhost:8444".to_string(),
        user_agent: Some("test-agent/1.0".to_string()),
        ..Default::default()
    };

    let result = health_check_ok_get_request_builder(&config, Some(1));
    assert!(
        result.is_ok(),
        "Request builder should be created successfully with check_db"
    );

    let req_builder = result.unwrap();
    let req = req_builder.build().unwrap();
    assert_eq!(
        req.url().to_string(),
        "http://localhost:8444/ok?check_db=1",
        "URL should include check_db query parameter"
    );
}

#[tokio::test]
async fn test_upload_file_structure() {
    // Test that UploadFile can be created and used
    let file_data = std::fs::read("tests/files/weather.txt").unwrap();
    let file = UploadFile {
        name: "weather.txt".to_string(),
        content_type: "text/plain".to_string(),
        content: file_data,
    };

    assert_eq!(file.name, "weather.txt");
    assert_eq!(file.content_type, "text/plain");
    assert!(!file.content.is_empty(), "File content should not be empty");
}
