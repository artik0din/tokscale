//! OpenRouter activity-cache parser.
//!
//! `tokscale openrouter sync` stores the response from OpenRouter's activity
//! endpoint in this cache. The endpoint returns one row per UTC day, model,
//! provider, and endpoint; rows therefore represent daily aggregates rather
//! than individual requests.

use super::{BillingIdentityOrigin, UnifiedMessage};
use crate::TokenBreakdown;
use chrono::NaiveDate;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct OpenRouterActivityCache {
    #[serde(default)]
    data: Vec<OpenRouterActivity>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterActivity {
    date: Option<String>,
    model: Option<String>,
    provider_name: Option<String>,
    endpoint_id: Option<String>,
    model_permaslug: Option<String>,
    requests: Option<i64>,
    prompt_tokens: Option<i64>,
    completion_tokens: Option<i64>,
    reasoning_tokens: Option<i64>,
    cached_tokens: Option<i64>,
    usage: Option<f64>,
}

/// Parse a cache written from OpenRouter's `GET /api/v1/activity` response.
///
/// API reference: <https://openrouter.ai/docs/api/api-reference/analytics/get-user-activity-grouped-by-endpoint>.
/// OpenRouter reports the actual billed amount in `usage`, so it is marked as
/// provider-reported instead of being recomputed from model price metadata.
pub fn parse_openrouter_file(path: &Path) -> Vec<UnifiedMessage> {
    let content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => return Vec::new(),
    };
    let cache: OpenRouterActivityCache = match serde_json::from_str(&content) {
        Ok(cache) => cache,
        Err(_) => return Vec::new(),
    };

    cache.data.iter().filter_map(activity_to_message).collect()
}

fn activity_to_message(activity: &OpenRouterActivity) -> Option<UnifiedMessage> {
    let date = activity.date.as_deref()?.trim();
    let model = activity.model.as_deref()?.trim();
    let provider = activity.provider_name.as_deref()?.trim();
    if model.is_empty() || provider.is_empty() {
        return None;
    }

    // Rows are UTC-day aggregates. Noon avoids assigning most aggregate rows
    // to an adjacent local day before the configured bucketing pass runs.
    let timestamp = NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .ok()?
        .and_hms_opt(12, 0, 0)?
        .and_utc()
        .timestamp_millis();
    let cost = finite_non_negative(activity.usage)?;
    let endpoint = activity
        .endpoint_id
        .as_deref()
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .unwrap_or("unknown-endpoint");
    let model_permaslug = activity
        .model_permaslug
        .as_deref()
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .unwrap_or(model);
    let prompt_tokens = non_negative(activity.prompt_tokens);
    let cache_read = non_negative(activity.cached_tokens).min(prompt_tokens);
    let completion_tokens = non_negative(activity.completion_tokens);
    let reasoning = non_negative(activity.reasoning_tokens).min(completion_tokens);

    // OpenRouter reports cached tokens within prompt and reasoning within completion.

    let mut message = UnifiedMessage::new(
        "openrouter",
        model,
        provider,
        format!("openrouter-{date}-{endpoint}-{model_permaslug}"),
        timestamp,
        TokenBreakdown {
            input: prompt_tokens - cache_read,
            output: completion_tokens - reasoning,
            cache_read,
            cache_write: 0,
            cache_write_1h: 0,
            reasoning,
        },
        cost,
    )
    .with_billing(
        Some("openrouter"),
        BillingIdentityOrigin::KnownHostedChannel,
    );
    message.message_count = non_negative(activity.requests).min(i32::MAX as i64) as i32;
    message.mark_provider_reported_cost();
    Some(message)
}

fn non_negative(value: Option<i64>) -> i64 {
    value.unwrap_or(0).max(0)
}

fn finite_non_negative(value: Option<f64>) -> Option<f64> {
    value.filter(|value| value.is_finite() && *value >= 0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_recorded_activity_fixture_with_authoritative_cost() {
        let fixture =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/openrouter/activity.json");

        let messages = parse_openrouter_file(&fixture);

        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].client, "openrouter");
        assert_eq!(messages[0].model_id, "openai/gpt-4.1");
        assert_eq!(messages[0].provider_id, "OpenAI");
        assert_eq!(messages[0].message_count, 5);
        assert_eq!(messages[0].tokens.input, 40);
        assert_eq!(messages[0].tokens.output, 100);
        assert_eq!(messages[0].tokens.reasoning, 25);
        assert_eq!(messages[0].tokens.cache_read, 10);
        assert_eq!(messages[0].tokens.cache_write, 0);
        assert_eq!(messages[0].cost, 0.015);
        assert_eq!(
            messages[0].cost_source,
            super::super::CostSource::ProviderReported
        );
        assert_eq!(messages[1].tokens.input, 0);
        assert_eq!(messages[1].tokens.output, 0);
        assert_eq!(messages[1].tokens.reasoning, 0);
        assert_eq!(messages[1].tokens.cache_read, 0);
    }

    #[test]
    fn clamps_overlapping_tokens_to_totals() {
        let activity = OpenRouterActivity {
            date: Some("2026-01-01".to_string()),
            model: Some("openai/gpt-4.1".to_string()),
            provider_name: Some("OpenAI".to_string()),
            endpoint_id: None,
            model_permaslug: None,
            requests: Some(1),
            prompt_tokens: Some(10),
            completion_tokens: Some(20),
            reasoning_tokens: Some(30),
            cached_tokens: Some(40),
            usage: Some(0.0),
        };

        let message = activity_to_message(&activity).expect("activity should parse");

        assert_eq!(message.tokens.input, 0);
        assert_eq!(message.tokens.cache_read, 10);
        assert_eq!(message.tokens.output, 0);
        assert_eq!(message.tokens.reasoning, 20);
    }

    #[test]
    fn skips_rows_without_a_valid_authoritative_cost() {
        let activity = OpenRouterActivity {
            date: Some("2026-01-01".to_string()),
            model: Some("openai/gpt-4.1".to_string()),
            provider_name: Some("OpenAI".to_string()),
            endpoint_id: None,
            model_permaslug: None,
            requests: Some(1),
            prompt_tokens: Some(1),
            completion_tokens: Some(1),
            reasoning_tokens: None,
            cached_tokens: None,
            usage: None,
        };

        assert!(activity_to_message(&activity).is_none());
    }
}
