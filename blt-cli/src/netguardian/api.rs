//! NetGuardian Distributed Security Scanning API Client.
//!
//! Provides the HTTP client for interacting with the NetGuardian Cloudflare Worker API:
//! - Domain queue leasing (`claim_domain`, `heartbeat`, `complete`, `fail`)
//! - Signed finding ingestion (`ingest_finding`)
//! - Target domain submission (`submit_domains`)

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// Represents a claimed domain scanning task from the queue.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClaimedJob {
    pub id: String,
    pub org_id: String,
    pub host_key: String,
    pub seed_url: String,
    #[serde(default)]
    pub status: String,
    pub claimed_by: Option<String>,
    pub claimed_at: Option<i64>,
    pub claim_until: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct ClaimResponse {
    job: Option<ClaimedJob>,
}

/// Errors returned by NetGuardian API operations.
#[derive(Debug)]
pub enum NetguardianError {
    Http(reqwest::Error),
    Json(serde_json::Error),
    Api { status: u16, message: String },
    Crypto(String),
}

impl fmt::Display for NetguardianError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetguardianError::Http(e) => write!(f, "HTTP transport error: {e}"),
            NetguardianError::Json(e) => write!(f, "JSON serialization error: {e}"),
            NetguardianError::Api { status, message } => {
                write!(f, "API error (HTTP {status}): {message}")
            }
            NetguardianError::Crypto(e) => write!(f, "Cryptographic error: {e}"),
        }
    }
}

impl std::error::Error for NetguardianError {}

impl From<reqwest::Error> for NetguardianError {
    fn from(e: reqwest::Error) -> Self {
        NetguardianError::Http(e)
    }
}

impl From<serde_json::Error> for NetguardianError {
    fn from(e: serde_json::Error) -> Self {
        NetguardianError::Json(e)
    }
}

/// Client for communicating with the NetGuardian API backend.
#[derive(Clone)]
pub struct NetguardianClient {
    client: Client,
    pub api_base: String,
    pub org_token: String,
    pub org_id: String,
    pub sender_id: String,
    pub secret_hex: String,
}

impl NetguardianClient {
    pub fn new(
        api_base: &str,
        org_token: &str,
        org_id: &str,
        sender_id: &str,
        secret_hex: &str,
    ) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(15))
            .build()
            .unwrap_or_else(|_| Client::new());

        Self {
            client,
            api_base: api_base.trim_end_matches('/').to_string(),
            org_token: org_token.trim().to_string(),
            org_id: org_id.trim().to_string(),
            sender_id: sender_id.trim().to_string(),
            secret_hex: secret_hex.trim().to_string(),
        }
    }

    /// Claim a domain lease from `POST /api/domains/claim`.
    pub async fn claim_domain(&self) -> Result<Option<ClaimedJob>, NetguardianError> {
        let url = format!("{}/api/domains/claim", self.api_base);
        let resp = self
            .client
            .post(&url)
            .bearer_auth(&self.org_token)
            .json(&serde_json::json!({
                "sender_id": self.sender_id
            }))
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let message = resp.text().await.unwrap_or_default();
            return Err(NetguardianError::Api { status, message });
        }

        let claim_resp: ClaimResponse = resp.json().await?;
        Ok(claim_resp.job)
    }

    /// Send a lease heartbeat to extend the claim on `POST /api/domains/{id}/heartbeat`.
    pub async fn heartbeat(&self, job_id: &str) -> Result<(), NetguardianError> {
        let url = format!("{}/api/domains/{}/heartbeat", self.api_base, job_id);
        let resp = self
            .client
            .post(&url)
            .bearer_auth(&self.org_token)
            .json(&serde_json::json!({
                "sender_id": self.sender_id
            }))
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let message = resp.text().await.unwrap_or_default();
            return Err(NetguardianError::Api { status, message });
        }
        Ok(())
    }

    /// Complete a domain scan job on `POST /api/domains/{id}/complete`.
    pub async fn complete(&self, job_id: &str, result: Value) -> Result<(), NetguardianError> {
        let url = format!("{}/api/domains/{}/complete", self.api_base, job_id);
        let resp = self
            .client
            .post(&url)
            .bearer_auth(&self.org_token)
            .json(&serde_json::json!({
                "sender_id": self.sender_id,
                "result": result
            }))
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let message = resp.text().await.unwrap_or_default();
            return Err(NetguardianError::Api { status, message });
        }
        Ok(())
    }

    /// Mark a job as failed on `POST /api/domains/{id}/fail`.
    pub async fn fail(&self, job_id: &str, error: &str) -> Result<(), NetguardianError> {
        let url = format!("{}/api/domains/{}/fail", self.api_base, job_id);
        let resp = self
            .client
            .post(&url)
            .bearer_auth(&self.org_token)
            .json(&serde_json::json!({
                "sender_id": self.sender_id,
                "error": error
            }))
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let message = resp.text().await.unwrap_or_default();
            return Err(NetguardianError::Api { status, message });
        }
        Ok(())
    }

    /// Ingest a signed `ztr-finding-1` envelope on `POST /api/ingest`.
    ///
    /// Computes and sends the required `X-BLT-Body-Digest` header matching
    /// the SHA-256 of the raw transmitted body bytes.
    pub async fn ingest_finding(&self, envelope: &Value) -> Result<(), NetguardianError> {
        let url = format!("{}/api/ingest", self.api_base);
        let raw_body = serde_json::to_vec(envelope)?;
        let digest_header = format!(
            "sha256={}",
            crate::netguardian::crypto::body_digest_hex(&raw_body)
        );

        let resp = self
            .client
            .post(&url)
            .bearer_auth(&self.org_token)
            .header("Content-Type", "application/json")
            .header("X-BLT-Body-Digest", digest_header)
            .body(raw_body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let message = resp.text().await.unwrap_or_default();
            return Err(NetguardianError::Api { status, message });
        }
        Ok(())
    }

    /// Helper to submit domain URLs to the queue on `POST /api/domains`.
    pub async fn submit_domains(&self, domains: &[String]) -> Result<Value, NetguardianError> {
        let url = format!("{}/api/domains", self.api_base);
        let resp = self
            .client
            .post(&url)
            .bearer_auth(&self.org_token)
            .json(&serde_json::json!({
                "sender_id": self.sender_id,
                "domains": domains
            }))
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let message = resp.text().await.unwrap_or_default();
            return Err(NetguardianError::Api { status, message });
        }

        let body: Value = resp.json().await?;
        Ok(body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    #[test]
    fn test_claimed_job_deserialization() {
        let raw = json!({
            "id": "job-101",
            "org_id": "org-demo",
            "host_key": "example.com",
            "seed_url": "https://example.com/",
            "status": "in_progress",
            "claimed_by": "scanner-1",
            "claimed_at": 1729000000,
            "claim_until": 1729000180
        });

        let job: ClaimedJob = serde_json::from_value(raw).unwrap();
        assert_eq!(job.id, "job-101");
        assert_eq!(job.org_id, "org-demo");
        assert_eq!(job.host_key, "example.com");
        assert_eq!(job.seed_url, "https://example.com/");
        assert_eq!(job.status, "in_progress");
        assert_eq!(job.claimed_by.as_deref(), Some("scanner-1"));
        assert_eq!(job.claim_until, Some(1729000180));
    }

    #[test]
    fn test_claim_response_empty_deserialization() {
        let raw = json!({
            "job": null,
            "org_id": "org-demo",
            "sender_id": "scanner-1"
        });

        let resp: ClaimResponse = serde_json::from_value(raw).unwrap();
        assert!(resp.job.is_none());
    }

    #[test]
    fn test_error_display() {
        let err = NetguardianError::Api {
            status: 404,
            message: "Not found".into(),
        };
        assert_eq!(format!("{err}"), "API error (HTTP 404): Not found");

        let crypto_err = NetguardianError::Crypto("invalid signature".into());
        assert_eq!(
            format!("{crypto_err}"),
            "Cryptographic error: invalid signature"
        );
    }

    #[tokio::test]
    async fn test_client_claim_domain_mock_server() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let base_url = format!("http://127.0.0.1:{port}");

        tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buf = vec![0u8; 2048];
            let n = socket.read(&mut buf).await.unwrap();
            let req_str = String::from_utf8_lossy(&buf[..n]);

            assert!(req_str.starts_with("POST /api/domains/claim "));
            assert!(req_str.contains("authorization: Bearer test-token"));
            assert!(req_str.contains(r#""sender_id":"test-sender""#));

            let body = json!({
                "org_id": "org-test",
                "sender_id": "test-sender",
                "job": {
                    "id": "job-mock-1",
                    "org_id": "org-test",
                    "host_key": "owasp.org",
                    "seed_url": "https://owasp.org/",
                    "status": "in_progress",
                    "claimed_by": "test-sender",
                    "claimed_at": 1729000000,
                    "claim_until": 1729000180
                }
            })
            .to_string();

            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            socket.write_all(response.as_bytes()).await.unwrap();
        });

        let client = NetguardianClient::new(
            &base_url,
            "test-token",
            "org-test",
            "test-sender",
            "736563726574",
        );

        let job = client.claim_domain().await.unwrap().expect("job claimed");
        assert_eq!(job.id, "job-mock-1");
        assert_eq!(job.host_key, "owasp.org");
        assert_eq!(job.seed_url, "https://owasp.org/");
    }

    #[tokio::test]
    async fn test_client_ingest_finding_mock_server() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let base_url = format!("http://127.0.0.1:{port}");

        tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buf = vec![0u8; 4096];
            let n = socket.read(&mut buf).await.unwrap();
            let req_str = String::from_utf8_lossy(&buf[..n]);

            assert!(req_str.starts_with("POST /api/ingest "));
            assert!(req_str.contains("authorization: Bearer test-token"));
            assert!(req_str.contains("x-blt-body-digest: sha256="));

            let body = json!({
                "status": "created",
                "finding_id": "f-123"
            })
            .to_string();

            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            socket.write_all(response.as_bytes()).await.unwrap();
        });

        let client = NetguardianClient::new(
            &base_url,
            "test-token",
            "org-test",
            "test-sender",
            "736563726574",
        );

        let envelope = json!({
            "version": "ztr-finding-1",
            "org_id": "org-test",
            "sender_id": "test-sender",
            "kid": "k1",
            "alg": "hmac-sha256",
            "issued_at": "2026-11-15T12:00:00Z",
            "nonce": "test-nonce-1",
            "plaintext_mode": true,
            "payload_plaintext": {
                "rule_id": "http.missing-csp",
                "severity": "medium",
                "title": "Missing Content-Security-Policy",
                "target": "https://owasp.org/"
            },
            "payload_digest": "849d3d642065b4722904d0333ee8b2d435475e7e3ab9c6f7e44ae203a24e9bef",
            "signature": "007e45fd05c38d9a7bce3a7ea80cf5edff7a7d2fa5199d93ccdf5fb243a1b3ce"
        });

        let res = client.ingest_finding(&envelope).await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_client_heartbeat_and_complete_mock_server() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let base_url = format!("http://127.0.0.1:{port}");

        tokio::spawn(async move {
            // 1. Heartbeat
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buf = vec![0u8; 2048];
            let n = socket.read(&mut buf).await.unwrap();
            let req_str = String::from_utf8_lossy(&buf[..n]);
            assert!(req_str.starts_with("POST /api/domains/job-xyz/heartbeat "));
            let response = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 14\r\nConnection: close\r\n\r\n{\"job\":{}}";
            socket.write_all(response.as_bytes()).await.unwrap();

            // 2. Complete
            let (mut socket2, _) = listener.accept().await.unwrap();
            let mut buf2 = vec![0u8; 2048];
            let n2 = socket2.read(&mut buf2).await.unwrap();
            let req_str2 = String::from_utf8_lossy(&buf2[..n2]);
            assert!(req_str2.starts_with("POST /api/domains/job-xyz/complete "));
            let response2 = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 14\r\nConnection: close\r\n\r\n{\"job\":{}}";
            socket2.write_all(response2.as_bytes()).await.unwrap();
        });

        let client = NetguardianClient::new(
            &base_url,
            "test-token",
            "org-test",
            "test-sender",
            "736563726574",
        );

        assert!(client.heartbeat("job-xyz").await.is_ok());
        assert!(client
            .complete("job-xyz", json!({"findings_count": 0}))
            .await
            .is_ok());
    }
}
