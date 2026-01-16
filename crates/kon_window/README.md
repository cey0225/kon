# kon_window

**kon_window** is the windowing subsystem of Kon Engine - it manages the display surface, event loop integration, and acts as the primary source for raw OS events using `winit`.

## Scope

- **Window Management:** Configurable window creation and resizing.
- **Event Loop Integration:** Drives the application loop and polls system events.
- **Raw Event Capture:** Captures raw input events (keyboard, mouse, focus) and forwards them to the core event queue.
- 
```bash
# Part of the kon-engine ecosystem
cargo add kon-engine
```

## License

MIT OR Apache-2.0
