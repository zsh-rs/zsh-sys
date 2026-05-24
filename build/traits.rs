pub trait WriteToFile {
    /// Write these bindings as source text to a file.
    fn write_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> std::io::Result<()>;
}

impl WriteToFile for bindgen::Bindings {
    fn write_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> std::io::Result<()> {
        bindgen::Bindings::write_to_file(self, path)
    }
}

pub trait MaybeSplitToStubs {
    fn maybe_split_to_stubs(self) -> std::io::Result<impl WriteToFile>;
}

#[cfg(not(feature = "test-stubs"))]
impl MaybeSplitToStubs for bindgen::Bindings {
    fn maybe_split_to_stubs(self) -> std::io::Result<impl WriteToFile> {
        Ok(self)
    }
}
