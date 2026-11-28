# Oxide 1.0 readiness checklist

This is the agreed implementation order, including the two additions placed
before the original audit. The initial release targets Windows x64/D3D12, Rust,
the C ABI (also used by C++), C#, and Python. Tests belong under `tests/`.

1. [x] Rect3D camera surfaces.
2. [x] Basic 2D rendering and menu text.
3. [x] Application startup and failure propagation.
4. [x] Window, input, and frame lifecycle.
5. [ ] Resource lifetime.
6. [ ] Raycasting.
7. [ ] API/ABI review.
8. [ ] Asset deployment.
9. [ ] Performance and memory.
10. [ ] Release validation.
11. [ ] Licensing and versioning.
12. [ ] Packages.
13. [ ] Reference game.
14. [ ] Documentation and release-candidate validation.

Items 1–4 are implemented; interactive window/focus checks for item 4 are described
in [the lifecycle guide](application-lifecycle.md). Completing implementation
items does not replace the later release-validation and release-candidate steps.
