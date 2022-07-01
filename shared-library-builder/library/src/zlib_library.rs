use file_matcher::FileNamed;
use shared_library_builder::{
    CMakeLibrary, CompiledLibraryName, GitLocation, Library, LibraryLocation,
};

#[derive(Debug, Clone)]
pub struct ZLibLibrary(CMakeLibrary);

impl ZLibLibrary {
    pub fn new(version: impl Into<String>) -> Self {
        Self(
            CMakeLibrary::new(
                "zlib",
                LibraryLocation::Git(GitLocation::github("madler", "zlib").tag(version)),
            )
            .with_exported_name("z")
            .compiled_name(CompiledLibraryName::Matching("libz".to_string()))
            .define_static("BUILD_SHARED_LIBS", "OFF")
            .define_shared("BUILD_SHARED_LIBS", "ON")
            .delete(FileNamed::any_named(vec![
                FileNamed::wildmatch("*zlib.*"),  // windows
                FileNamed::wildmatch("*.dylib"),  // mac
                FileNamed::wildmatch("libz.so*"), // linux
            ]))
            .into(),
        )
    }

    pub fn v1_2_11() -> Self {
        Self::new("v1.2.11")
    }

    pub fn into_cmake_library(self) -> CMakeLibrary {
        self.0
    }
}

impl From<ZLibLibrary> for Box<dyn Library> {
    fn from(library: ZLibLibrary) -> Self {
        library.0.into()
    }
}
