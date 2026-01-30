//! XML parsing abstraction layer
//!
//! This module provides a unified interface for XML deserialization
//! that can use either serde-xml-rs or quick-xml as the backend.

use std::io;

/// Deserialize XML data into a Rust struct
///
/// This function provides a unified interface for XML deserialization
/// that works with either serde-xml-rs or quick-xml backend.
pub fn from_str<'de, T>(s: &'de str) -> io::Result<T>
where
    T: serde::Deserialize<'de>,
{
    #[cfg(feature = "serde-xml")]
    {
        serde_xml_rs::from_str(s).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("XML deserialization failed: {}", e),
            )
        })
    }

    #[cfg(all(feature = "quick-xml", not(feature = "serde-xml")))]
    {
        quick_xml::de::from_str(s).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("XML deserialization failed: {}", e),
            )
        })
    }

    #[cfg(not(any(feature = "serde-xml", feature = "quick-xml")))]
    compile_error!(
        "Either the 'serde-xml' or 'quick-xml' feature must be enabled. \
         Add --features serde-xml or --features quick-xml when building."
    );
}

/// Serialize a Rust struct to XML
///
/// This function provides a unified interface for XML serialization
/// that works with either serde-xml-rs or quick-xml backend.
pub fn to_string<T>(value: &T) -> io::Result<String>
where
    T: serde::Serialize,
{
    #[cfg(feature = "serde-xml")]
    {
        serde_xml_rs::to_string(value).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("XML serialization failed: {}", e),
            )
        })
    }

    #[cfg(all(feature = "quick-xml", not(feature = "serde-xml")))]
    {
        quick_xml::se::to_string(value).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("XML serialization failed: {}", e),
            )
        })
    }

    #[cfg(not(any(feature = "serde-xml", feature = "quick-xml")))]
    compile_error!(
        "Either the 'serde-xml' or 'quick-xml' feature must be enabled. \
         Add --features serde-xml or --features quick-xml when building."
    );
}
