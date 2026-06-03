//! The test-running harness: an HTTP client, a check abstraction, and a report.

use std::future::Future;
use std::pin::Pin;

use reqwest::Client;

/// A check receives a configured [`Context`] and returns `Ok(())` on success.
pub type CheckResult = anyhow::Result<()>;

/// Boxed future returned by a check body.
pub type CheckFuture<'a> = Pin<Box<dyn Future<Output = CheckResult> + Send + 'a>>;

/// A single named conformance check.
pub struct Check {
    pub name: &'static str,
    pub run: for<'a> fn(&'a Context) -> CheckFuture<'a>,
}

/// Everything a check needs to talk to the endpoint under test.
pub struct Context {
    pub client: Client,
    pub endpoint: String,
}

impl Context {
    /// Join a path onto the configured endpoint.
    pub fn url(&self, path: &str) -> String {
        format!(
            "{}/{}",
            self.endpoint.trim_end_matches('/'),
            path.trim_start_matches('/')
        )
    }
}

/// Drives a set of [`Check`]s against one endpoint.
pub struct Runner {
    ctx: Context,
}

impl Runner {
    pub fn new(endpoint: &str) -> anyhow::Result<Self> {
        let client = Client::builder().build()?;
        Ok(Self {
            ctx: Context {
                client,
                endpoint: endpoint.to_string(),
            },
        })
    }

    /// Run every check sequentially and collect the outcomes.
    pub async fn run(&self, checks: Vec<Check>) -> Report {
        let mut outcomes = Vec::with_capacity(checks.len());
        for check in checks {
            tracing::info!(check = check.name, "running");
            let result = (check.run)(&self.ctx).await;
            outcomes.push(Outcome {
                name: check.name,
                error: result.err().map(|e| format!("{e:#}")),
            });
        }
        Report { outcomes }
    }
}

/// The result of a single check.
pub struct Outcome {
    pub name: &'static str,
    pub error: Option<String>,
}

impl Outcome {
    pub fn passed(&self) -> bool {
        self.error.is_none()
    }
}

/// The aggregate result of a run.
pub struct Report {
    pub outcomes: Vec<Outcome>,
}

impl Report {
    pub fn all_passed(&self) -> bool {
        self.outcomes.iter().all(Outcome::passed)
    }

    /// Print a one-line-per-check summary plus a total.
    pub fn print(&self) {
        let mut passed = 0;
        for outcome in &self.outcomes {
            match &outcome.error {
                None => {
                    passed += 1;
                    println!("PASS  {}", outcome.name);
                }
                Some(err) => println!("FAIL  {}\n      {}", outcome.name, err),
            }
        }
        println!("\n{passed}/{} checks passed", self.outcomes.len());
    }
}
