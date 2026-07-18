use std::path::{Path, PathBuf};

/// Bundled fallback preview, shown for any shader that has no sibling `.bmp`.
/// Compiled in so a preview is always available even if the file is missing.
const DEFAULT_PREVIEW: &[u8] = include_bytes!("../assets/shaders/shader-default.bmp");

#[derive(Debug)]
pub struct ShaderEntry {
    /// File stem, e.g. "shader-001".
    pub name: String,
    /// Path to the `.glsl` fragment shader.
    pub path: PathBuf,
    /// Bytes of the sibling `<stem>.bmp` preview, if one exists on disk.
    pub preview: Option<Vec<u8>>,
}

/// Shaders discovered in a directory, scanned once at startup and shared
/// immutably (via `Arc`) across threads.
#[derive(Debug)]
pub struct ShaderLibrary {
    pub entries: Vec<ShaderEntry>,
}

impl ShaderLibrary {
    /// Scans `dir` for `*.glsl` files (sorted by name) and pairs each with its
    /// sibling `<stem>.bmp` preview if present. Non-`.glsl` files (e.g. `.bmp`,
    /// `.DS_Store`) are ignored.
    pub fn scan(dir: &Path) -> std::io::Result<Self> {
        let mut entries = Vec::new();

        for entry in std::fs::read_dir(dir)? {
            let path = entry?.path();

            let is_glsl = path.is_file()
                && path.extension().and_then(|e| e.to_str()) == Some("glsl");
            if !is_glsl {
                continue;
            }

            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();

            // Sibling preview: same path with the extension swapped to `.bmp`.
            let preview = std::fs::read(path.with_extension("bmp")).ok();

            entries.push(ShaderEntry { name, path, preview });
        }

        entries.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(Self { entries })
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// File stem of the shader at `index`, or `""` if out of range.
    pub fn name(&self, index: usize) -> &str {
        self.entries.get(index).map(|e| e.name.as_str()).unwrap_or("")
    }

    /// Path to the `.glsl` at `index`, or `None` if out of range.
    pub fn path(&self, index: usize) -> Option<&Path> {
        self.entries.get(index).map(|e| e.path.as_path())
    }

    /// Preview bytes for the shader at `index`, falling back to the bundled
    /// default when the shader has no `.bmp` (or the index is out of range).
    pub fn preview(&self, index: usize) -> &[u8] {
        self.entries
            .get(index)
            .and_then(|e| e.preview.as_deref())
            .unwrap_or(DEFAULT_PREVIEW)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_graphics::pixelcolor::BinaryColor;
    use tinybmp::Bmp;

    #[test]
    fn scans_and_decodes_shader_folder() {
        let lib = ShaderLibrary::scan(Path::new("assets/shaders")).expect("scan failed");

        // The three shipped shaders must be discovered and sorted by name.
        assert!(lib.len() >= 3, "expected at least 3 shaders, got {}", lib.len());
        assert_eq!(lib.name(0), "shader-001");

        // Every shader's `.glsl` reads and every preview (own bmp or the
        // bundled default) decodes onto the 1-bit OLED color type.
        for i in 0..lib.len() {
            std::fs::read_to_string(lib.path(i).unwrap())
                .unwrap_or_else(|e| panic!("shader {i} unreadable: {e}"));
            Bmp::<BinaryColor>::from_slice(lib.preview(i))
                .unwrap_or_else(|e| panic!("preview {i} failed to decode: {e:?}"));
        }
    }

    #[test]
    fn default_preview_decodes() {
        // Out-of-range index falls back to the compiled-in default, which must
        // itself be a valid 1-bit-decodable BMP.
        let lib = ShaderLibrary { entries: Vec::new() };
        Bmp::<BinaryColor>::from_slice(lib.preview(0)).expect("default preview must decode");
    }
}
