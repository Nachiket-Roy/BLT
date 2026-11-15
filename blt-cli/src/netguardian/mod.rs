//! NetGuardian Distributed Security Scanning Integration.
//!
//! This module provides cryptographic signing and client capabilities
//! for participating in the BLT-NetGuardian distributed domain crawl and audit network.

pub mod api;
pub mod crypto;

pub use api::{ClaimedJob, NetguardianClient, NetguardianError};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
pub struct NetguardianArgs {
    #[command(subcommand)]
    pub action: Option<NetguardianAction>,

    /// NetGuardian API base URL
    #[arg(long, default_value = "http://localhost:8787", global = true)]
    pub api_base: String,

    /// Organization Bearer authentication token
    #[arg(
        long,
        env = "NG_ORG_TOKEN",
        default_value = "triage-token",
        global = true
    )]
    pub org_token: String,

    /// Sender HMAC secret hex key for signing findings
    #[arg(long, default_value = "736563726574", global = true)]
    pub secret_hex: String,

    /// Organization identifier
    #[arg(long, default_value = "org-demo", global = true)]
    pub org_id: String,

    /// Unique sender / node identifier
    #[arg(long, default_value = "rust-node-1", global = true)]
    pub sender_id: String,

    /// Concurrency level for worker pool
    #[arg(long, default_value_t = 1, global = true)]
    pub concurrency: usize,
}

#[derive(Subcommand, Debug, Clone, PartialEq)]
pub enum NetguardianAction {
    /// Claim a domain job from the queue and print details
    Claim,
    /// Run the headless worker node crawler daemon
    RunNode,
}

/// Run the NetGuardian subcommand based on parsed arguments.
pub async fn run(args: &NetguardianArgs) -> Result<(), Box<dyn std::error::Error>> {
    let action = args.action.as_ref().unwrap_or(&NetguardianAction::Claim);
    match action {
        NetguardianAction::Claim => run_claim(args).await,
        NetguardianAction::RunNode => {
            println!("NetGuardian node daemon runner will be wired in PR 4.");
            println!("Executing single domain claim check...");
            run_claim(args).await
        }
    }
}

async fn run_claim(args: &NetguardianArgs) -> Result<(), Box<dyn std::error::Error>> {
    let client = NetguardianClient::new(
        &args.api_base,
        &args.org_token,
        &args.org_id,
        &args.sender_id,
        &args.secret_hex,
    );

    println!(
        "Claiming domain job from {} (org: {}, sender: {})...",
        args.api_base, args.org_id, args.sender_id
    );

    match client.claim_domain().await? {
        Some(job) => {
            println!("Successfully claimed job:");
            println!("  ID:        {}", job.id);
            println!("  Host key:  {}", job.host_key);
            println!("  Seed URL:  {}", job.seed_url);
            println!("  Status:    {}", job.status);
            if let Some(claimed_by) = &job.claimed_by {
                println!("  Claimed by: {}", claimed_by);
            }
            if let Some(until) = job.claim_until {
                println!("  Lease until: {}", until);
            }
        }
        None => {
            println!("No claimable domain jobs in queue.");
        }
    }
    Ok(())
}
