pub trait RpcTelemetry: Send + Sync {
    fn rpc_call(&self, method: &'static str, purpose: &'static str, ok: bool, ms: u64);
    fn rpc_error(&self, method: &'static str, purpose: &'static str);
}
