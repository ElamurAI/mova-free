//! Crab external-role adapter: status/start/marker over the Whale pool or the continuous swarm.
//! The embedding binary installs it with `mova_crab::angel::serve_with_roles`.
use crate::Result;
use serde_json::Value;
use std::sync::Arc;

/// Whale handler for the public Crab control protocol. Crab passes its server-side
/// configuration after its own request, pin and journal gates; clients choose no paths.
#[derive(Clone, Copy, Debug, Default)]
pub struct WhaleRoles;
impl WhaleRoles {
    pub fn new() -> Self {
        Self
    }
    /// Shared handle in the form `serve_with_roles` accepts.
    pub fn handler() -> Arc<dyn mova_crab::angel::ExternalRoleHandler> {
        Arc::new(Self)
    }
}
impl mova_crab::angel::ExternalRoleHandler for WhaleRoles {
    fn status(&self, config: &Value, swarm: Option<&Value>) -> Result<Value> {
        match swarm {
            Some(s) => crate::swarm::status(s),
            None => crate::pool::status(config),
        }
    }
    fn start(&self, config: &Value, swarm: bool) -> Result<Value> {
        if swarm {
            crate::swarm::start(config)
        } else {
            crate::pool::start(config)
        }
    }
    /// Crab handles swarm markers itself; only the pool path reaches this hook.
    fn marker(&self, config: &Value, operation: &str) -> Result<Value> {
        crate::pool::marker(config, operation)
    }
}
/// Serve Crab with Whale roles installed.
pub fn serve(config: &Value) -> Result<()> {
    mova_crab::angel::serve_with_roles(config, Arc::new(WhaleRoles))
}
#[cfg(test)]
mod tests {
    use super::*;
    use mova_crab::angel::ExternalRoleHandler;
    use serde_json::json;
    #[test]
    fn handler_delegates_and_fails_closed_without_trusted_configuration() {
        let h = WhaleRoles::new();
        assert!(h.status(&json!({}), None).unwrap_err().to_string().contains("missing string run_root"));
        assert!(h.start(&json!({}), true).unwrap_err().to_string().contains("whale_swarm_config"));
        assert!(h.marker(&json!({"run_root":"relative","whale_config":"/x","whale_executable":"/y"}), "pause").is_err());
        assert!(h.status(&json!({}), Some(&json!({"run_root":"relative"}))).is_err());
        let _shared = WhaleRoles::handler();
    }
}
