# Aevyrn Engine

A serious, long-term, cross-platform AI-native game engine.

## Vision

Aevyrn Engine is designed to help developers create any kind of game:

- **Genres**: 2D, 2.5D, 3D, FPS, TPS, RPG, ARPG, horror, survival, strategy, RTS, simulation, racing, sports, platformers, fighting games, roguelikes, city builders, open-world games, multiplayer games, and more.
- **Visual Styles**: pixel art, low-poly, stylized, cartoon, anime-inspired, realistic, photorealistic.

The long-term goal is for **Aevyrn AI**, the built-in local AI system, to understand the entire game project and be capable of creating and modifying almost every aspect of a game through a structured tool/API system.

## Architecture

The engine is organized into major subsystems:

```
Aevyrn Engine
├── Core
├── Runtime
├── Renderer
├── Scene System
├── Entity/Component System
├── Physics
├── Audio
├── Animation
├── Input
├── Asset Pipeline
├── Terrain
├── UI
├── Networking
├── Scripting
├── Editor
├── AI
├── Project System
├── Build System
├── Export System
└── Tools
```

## Technology

- **Primary Language**: Rust
- **Graphics**: wgpu (cross-platform graphics abstraction)
- **Window Management**: winit
- **Target Platforms**: macOS (primary), Windows

## Development Strategy

The engine is built vertically and incrementally:

1. **Phase 1**: Engine foundation (current) - compilation, window, basic rendering
2. **Phase 2**: Rendering improvements and scene management
3. **Phase 3**: Entity/component system and editor
4. **Phase 4**: Assets, terrain, scripting
5. **Phase 5**: AI tool system and integration
6. **Phase 6**: Advanced graphics, physics, audio, animation
7. **Phase 7**: Networking, profiling, optimization
8. **Phase 8**: Platform exports

## Building

```bash
cargo build
```

## Running

```bash
cargo run -p aevyrn_cli
```

## Testing

```bash
cargo test
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
