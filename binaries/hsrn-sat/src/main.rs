//! HERMES SATELLITE NODE
//! 
//! Autonomous satellite operating in deep space.

use hsrn_common::{init_logging, mission_log, LogLevel};

fn main() {
    init_logging(LogLevel::INFO);

    mission_log!(info, "═══════════════════════════════════════════════════════════════");
    mission_log!(info, "  HERMES SPACE RELAY NETWORK - SATELLITE NODE v0.1.0");
    mission_log!(info, "═══════════════════════════════════════════════════════════════");

    // TODO: satellite initialization
    
    mission_log!(info, "SYSTEM INITIALIZATION COMPLETE");
}