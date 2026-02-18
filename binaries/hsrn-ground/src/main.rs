//! HERMES MISSION CONTROL
//!
//! Ground station command and telemetry interface.

use hsrn_common::{init_logging, mission_log, LogLevel};

fn main() {
    init_logging(LogLevel::INFO);

    mission_log!(info, "═══════════════════════════════════════════════════════════════");
    mission_log!(info, "  HERMES SPACE RELAY NETWORK - MISSION CONTROL v0.1.0");
    mission_log!(info, "═══════════════════════════════════════════════════════════════");

    // TODO: ground station initialization

    mission_log!(info, "MISSION CONTROL ONLINE");
}