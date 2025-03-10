use clap::Parser;
use serde::Deserialize;

/// Telemetry and metrics related options.
#[derive(Parser, Debug, Clone, Deserialize)]
pub struct TelemetryOpts {
    /// The port on which to expose Prometheus metrics
    #[clap(long, env = "BOLT_SIDECAR_METRICS_PORT", default_value_t = 3300)]
    metrics_port: u16,
    #[clap(long, env = "BOLT_SIDECAR_DISABLE_METRICS", default_value_t = false)]
    disable_metrics: bool,
}

impl TelemetryOpts {
    /// Get the metrics port if metrics are enabled or None if they are disabled.
    pub fn metrics_port(&self) -> Option<u16> {
        if self.disable_metrics {
            None
        } else {
            Some(self.metrics_port)
        }
    }
}
