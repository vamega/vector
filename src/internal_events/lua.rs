use super::InternalEvent;
use crate::transforms::lua::v1::format_error;
use metrics::{counter, gauge};

define_events_processed!(LuaEventProcessed, "transform", "lua");

#[derive(Debug)]
pub struct LuaGcTriggered {
    pub used_memory: usize,
}

impl InternalEvent for LuaGcTriggered {
    fn emit_metrics(&self) {
        gauge!("memory_used", self.used_memory as i64,
            "component_kind" => "transform",
            "component_type" => "lua",
        );
    }
}

#[derive(Debug)]
pub struct LuaScriptError {
    pub error: rlua::Error,
}

impl InternalEvent for LuaScriptError {
    fn emit_logs(&self) {
        let error = format_error(&self.error);
        error!(message = "Error in lua script; discarding event.", %error, rate_limit_secs = 30);
    }

    fn emit_metrics(&self) {
        counter!("processing_errors", 1,
            "component_kind" => "transform",
            "component_type" => "lua",
        );
    }
}
