mod zlib_library;

pub use shared_library_builder::{CMakeLibrary, GitLocation, LibraryLocation};

pub use crate::zlib_library::ZLibLibrary;

pub fn libz(version: Option<&str>) -> CMakeLibrary {
    let library = match version {
        None => { ZLibLibrary::v1_2_11().into_cmake_library() }
        Some(version) => {
            match version {
                "v1.2.11" => ZLibLibrary::v1_2_11().into_cmake_library(),
                _ => { panic!("Unknown zlib version: {}", version) }
            }
        }
    };

    let library = library.with_release_location(version.map(|version| {
        LibraryLocation::Git(GitLocation::github("feenkcom", "libz").tag(version))
    }));

    library
}
