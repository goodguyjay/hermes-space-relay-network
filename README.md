# HERMES SPACE RELAY NETWORK
```
           _     |   |     _
          | |____|   |____| |
          | | H S R N - 0 1 | |
          |_|____|   |____|_|
                 |___|
       L4         / \         L5
        .        /   \        .
  [HSRN-2]      / SUN \      [HSRN-3]
        '--.   \       /   .--'
            '--.\_____/.--'
                 \   /
                  ' '
                 EARTH
```

**Lagrange Point Delay-Tolerant Networking Constellation**

Est. 2077 - Mission Year 1 (WIP, prolly will take like 3 years to "finish")

## Mission Objectives

Establish continuous communication relay between Earth and Mars using a constellation of autonomous satellites positioned at Sun-Earth-Mars Lagrange points. Implement delay-tolerant networking protocols to handle 4-24 minute light-time delays and intermittent connectivity.

## System Architecture

- **3 satellites** (HSRN-1, HSRN-2, HSRN-3) in solar orbit at Lagrange points.
- **Bundle protocol v7** for delay-tolerant networking
- **Contact Graph Routing** for dynamic path selection
- **Realistic orbital mechanics** with power/thermal/failure simulation
- **Mission control interface** for command uplink and telemetry downlink

## Technical Stack

- Rust (v2024)
- Static binaries with cgroups resource management
- Async runtime
- HTMX frontend for mission control dashboard

## Building
```bash
cargo build --release
```

## Running
```bash
# Terminal 1: Start satellites
./target/release/hsrn-sat --config config/hsrn-1.toml
./target/release/hsrn-sat --config config/hsrn-2.toml
./target/release/hsrn-sat --config config/hsrn-3.toml

# Terminal 2: Start mission control
./target/release/hsrn-ground --config config/ground-earth.toml
```

## License

GPL-3.0 - See LICENSE file

## References

See docs/REFERENCES.md