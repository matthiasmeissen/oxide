# Changelog

All notable changes to this project are documented here.
Format based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/);
this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added
- Folder-driven shaders: any `.glsl` in `assets/shaders/` is discovered at startup, with an optional sibling `<name>.bmp` preview (a bundled default is shown when none is provided). No recompile needed to add a shader.
- New shader: `mmn-gl-141-260109`
- Shader template: `shader-template.glsl` as minimal starting point with all uniforms declared.

### Changed
- Shader previews now load from disk per shader instead of a compiled-in spritesheet.

### Removed
- `NUM_SHADERS` constant — the shader count is now derived from the folder.

### Fixed
- Startup no longer panics on non-`.glsl` files (e.g. `.DS_Store`) in the shader folder.
- OLED desktop preview is no longer skewed in x on non-16:9 displays; the preview quad now keeps a true 2:1 aspect ratio and is recomputed on resize.

## [0.1.0] - 2026-01-04
- Initial release.
