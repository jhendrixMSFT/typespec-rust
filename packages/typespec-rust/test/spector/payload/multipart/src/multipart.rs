// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Multipart form data encoding utilities.
//!
//! These types facilitate building `multipart/form-data` request bodies.
//!
//! **NOTE:** In a production SDK, `Part` and `MultipartFormData` should live
//! in `azure_core` and integrate directly with `Request`.

use std::io::Write;

/// Represents binary data with optional file metadata for a multipart form part.
///
/// Used for `HttpPart<bytes>` and file-based parts in multipart form requests.
/// The builder pattern allows optionally setting filename and content type.
///
/// # Examples
///
/// ```
/// use spector_multipart::multipart::Part;
///
/// // Simple binary data
/// let part = Part::new(vec![1, 2, 3]);
///
/// // Binary data with file metadata
/// let part = Part::new(vec![1, 2, 3])
///     .filename("image.jpg")
///     .content_type("image/jpeg");
/// ```
#[derive(Clone, Debug)]
pub struct Part {
    contents: Vec<u8>,
    filename: Option<String>,
    content_type: Option<String>,
}

impl Part {
    /// Creates a new part with the given binary contents.
    pub fn new(contents: impl Into<Vec<u8>>) -> Self {
        Self {
            contents: contents.into(),
            filename: None,
            content_type: None,
        }
    }

    /// Sets the filename for this part's `Content-Disposition` header.
    pub fn filename(mut self, filename: impl Into<String>) -> Self {
        self.filename = Some(filename.into());
        self
    }

    /// Sets the `Content-Type` header for this part.
    pub fn content_type(mut self, content_type: impl Into<String>) -> Self {
        self.content_type = Some(content_type.into());
        self
    }
}

impl From<Vec<u8>> for Part {
    fn from(contents: Vec<u8>) -> Self {
        Self::new(contents)
    }
}

impl<'a> From<&'a [u8]> for Part {
    fn from(contents: &'a [u8]) -> Self {
        Self::new(contents.to_vec())
    }
}

/// Builder for `multipart/form-data` request bodies.
///
/// Encodes parts with proper boundaries, `Content-Disposition` headers,
/// and optional `Content-Type` headers per part.
///
/// # Examples
///
/// ```
/// use spector_multipart::multipart::{MultipartFormData, Part};
///
/// let form = MultipartFormData::new()
///     .text("id", "123")
///     .part("file", Part::new(vec![0xFF, 0xD8])
///         .filename("photo.jpg")
///         .content_type("image/jpeg"));
///
/// let content_type = form.content_type_header();
/// let body = form.into_bytes();
/// ```
pub struct MultipartFormData {
    boundary: String,
    parts: Vec<(String, FormPartKind)>,
}

enum FormPartKind {
    Text {
        value: String,
        content_type: Option<String>,
    },
    Binary {
        data: Vec<u8>,
        content_type: Option<String>,
        filename: Option<String>,
    },
}

impl MultipartFormData {
    /// Creates a new empty multipart form with an auto-generated boundary.
    pub fn new() -> Self {
        // Use a combination of timestamp and a counter for uniqueness.
        // A production implementation in azure_core should use a
        // cryptographically random source.
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        let count = COUNTER.fetch_add(1, Ordering::Relaxed);
        let boundary = format!("AzureRustFormBoundary{ts:016x}{count:04x}");
        Self {
            boundary,
            parts: Vec::new(),
        }
    }

    /// Adds a text part with no explicit `Content-Type`.
    pub fn text(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.parts.push((
            name.into(),
            FormPartKind::Text {
                value: value.into(),
                content_type: None,
            },
        ));
        self
    }

    /// Adds a text part with an explicit `Content-Type` header.
    pub fn text_with_content_type(
        mut self,
        name: impl Into<String>,
        value: impl Into<String>,
        content_type: impl Into<String>,
    ) -> Self {
        self.parts.push((
            name.into(),
            FormPartKind::Text {
                value: value.into(),
                content_type: Some(content_type.into()),
            },
        ));
        self
    }

    /// Adds a binary part from a [`Part`].
    pub fn part(mut self, name: impl Into<String>, part: Part) -> Self {
        self.parts.push((
            name.into(),
            FormPartKind::Binary {
                data: part.contents,
                content_type: part.content_type,
                filename: part.filename,
            },
        ));
        self
    }

    /// Adds a JSON-serialized part with `Content-Type: application/json`.
    pub fn json<T: serde::Serialize>(
        mut self,
        name: impl Into<String>,
        value: &T,
    ) -> azure_core::Result<Self> {
        let json = serde_json::to_string(value)?;
        self.parts.push((
            name.into(),
            FormPartKind::Text {
                value: json,
                content_type: Some("application/json".to_string()),
            },
        ));
        Ok(self)
    }

    /// Returns the `Content-Type` header value including the boundary.
    pub fn content_type_header(&self) -> String {
        format!("multipart/form-data; boundary={}", self.boundary)
    }

    /// Encodes the multipart form data into bytes suitable for a request body.
    pub fn into_bytes(self) -> Vec<u8> {
        let mut body = Vec::new();
        for (name, kind) in &self.parts {
            write!(body, "--{}\r\n", self.boundary).unwrap();
            match kind {
                FormPartKind::Text {
                    value,
                    content_type,
                } => {
                    write!(body, "Content-Disposition: form-data; name=\"{name}\"\r\n").unwrap();
                    if let Some(ct) = content_type {
                        write!(body, "Content-Type: {ct}\r\n").unwrap();
                    }
                    write!(body, "\r\n").unwrap();
                    body.extend_from_slice(value.as_bytes());
                }
                FormPartKind::Binary {
                    data,
                    content_type,
                    filename,
                } => {
                    let f = filename.as_deref().unwrap_or("blob");
                    write!(
                        body,
                        "Content-Disposition: form-data; name=\"{name}\"; filename=\"{f}\"\r\n"
                    )
                    .unwrap();
                    match content_type {
                        Some(ct) => write!(body, "Content-Type: {ct}\r\n").unwrap(),
                        None => write!(body, "Content-Type: application/octet-stream\r\n").unwrap(),
                    }
                    write!(body, "\r\n").unwrap();
                    body.extend_from_slice(data);
                }
            }
            write!(body, "\r\n").unwrap();
        }
        write!(body, "--{}--\r\n", self.boundary).unwrap();
        body
    }
}
