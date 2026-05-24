/// Phase 1 smoke test — validates the full mentor-turn request path.
///
/// Exit criterion (from 30-roadmap.md Phase 1):
///   "a smoke test that sends a mentor-turn command and streams a response from a local model"
///
/// By default uses MockProvider (no Ollama required).
/// Set `TEST_REAL_OLLAMA=1` to exercise a real Ollama instance.
use std::time::Duration;

use cognition_daemon_lib::{config::Config, start};

#[tokio::test]
async fn smoke_mentor_turn_streams_response() {
    std::env::set_var("DAEMON_TEST_MODE", "1");

    let config = Config::test_default();
    let addr = start(config).await.expect("daemon failed to start");

    // Brief wait for the server task to bind.
    tokio::time::sleep(Duration::from_millis(50)).await;

    let client = reqwest::Client::new();

    // ── 1. Health check ───────────────────────────────────────────────────────
    let health_resp = client
        .get(format!("http://{addr}/api/v1/health"))
        .send()
        .await
        .expect("health request failed");
    assert_eq!(health_resp.status(), 200, "health endpoint must return 200");
    let health_json: serde_json::Value = health_resp.json().await.unwrap();
    assert_eq!(health_json["status"], "ok");
    assert_eq!(health_json["daemon_ready"], true);

    // ── 2. Mentor turn — collect all SSE chunks ───────────────────────────────
    let turn_resp = client
        .post(format!("http://{addr}/api/v1/mentor/turn"))
        .header("content-type", "application/json")
        .body(r#"{"message":"Hello, what can you help me with?","routing":"local"}"#)
        .send()
        .await
        .expect("mentor turn request failed");

    assert_eq!(turn_resp.status(), 200, "mentor turn must return 200");
    assert!(
        turn_resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.contains("text/event-stream"))
            .unwrap_or(false),
        "response must be SSE (text/event-stream)"
    );

    let body = turn_resp.text().await.expect("failed to read SSE body");

    // Each SSE event is a line starting with "data: "
    let events: Vec<serde_json::Value> = body
        .lines()
        .filter(|l| l.starts_with("data: "))
        .map(|l| {
            serde_json::from_str::<serde_json::Value>(&l["data: ".len()..])
                .expect("SSE event must be valid JSON")
        })
        .collect();

    assert!(!events.is_empty(), "must receive at least one SSE event");

    // Verify streaming content accumulates to a non-empty response.
    let full_text: String = events
        .iter()
        .filter_map(|e| e["chunk"].as_str())
        .collect();
    assert!(
        !full_text.is_empty(),
        "accumulated response text must be non-empty; got: {events:?}"
    );

    // Verify the final event has done=true.
    let last_event = events.last().unwrap();
    assert_eq!(last_event["done"], true, "last SSE event must have done=true");
    assert!(last_event["error"].is_null(), "final event must not contain an error");

    // Verify a correlation_id is present.
    let corr_id = last_event["correlation_id"].as_str().expect("correlation_id must be present");
    assert!(!corr_id.is_empty(), "correlation_id must not be empty");

    // Verify conversation_id is present (a new conversation was created).
    let conv_id = last_event["conversation_id"]
        .as_str()
        .expect("conversation_id must be present in final chunk");
    assert!(!conv_id.is_empty(), "conversation_id must not be empty");

    // ── 3. Verify audit log has entries for this correlation_id ───────────────
    tokio::time::sleep(Duration::from_millis(100)).await; // let the spawn task finish

    let audit_resp = client
        .get(format!("http://{addr}/api/v1/audit/recent"))
        .send()
        .await
        .expect("audit request failed");
    assert_eq!(audit_resp.status(), 200);
    let audit_entries: Vec<serde_json::Value> = audit_resp.json().await.unwrap();

    let this_turn_entries: Vec<_> = audit_entries
        .iter()
        .filter(|e| e["correlation_id"].as_str() == Some(corr_id))
        .collect();

    assert!(
        !this_turn_entries.is_empty(),
        "at least one audit entry must exist for correlation_id={corr_id}"
    );

    let event_types: Vec<&str> = this_turn_entries
        .iter()
        .filter_map(|e| e["event_type"].as_str())
        .collect();

    assert!(
        event_types.contains(&"mentor_turn_received"),
        "audit log must contain mentor_turn_received; got: {event_types:?}"
    );
    assert!(
        event_types.contains(&"ai_call_started"),
        "audit log must contain ai_call_started; got: {event_types:?}"
    );
    assert!(
        event_types.contains(&"ai_call_completed"),
        "audit log must contain ai_call_completed; got: {event_types:?}"
    );
    assert!(
        event_types.contains(&"mentor_turn_completed"),
        "audit log must contain mentor_turn_completed; got: {event_types:?}"
    );
}
