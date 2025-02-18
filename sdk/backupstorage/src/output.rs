// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct StartObjectOutput {
    /// Upload Id for a given upload.
    #[doc(hidden)]
    pub upload_id: std::option::Option<std::string::String>,
}
impl StartObjectOutput {
    /// Upload Id for a given upload.
    pub fn upload_id(&self) -> std::option::Option<&str> {
        self.upload_id.as_deref()
    }
}
/// See [`StartObjectOutput`](crate::output::StartObjectOutput).
pub mod start_object_output {

    /// A builder for [`StartObjectOutput`](crate::output::StartObjectOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) upload_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Upload Id for a given upload.
        pub fn upload_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.upload_id = Some(input.into());
            self
        }
        /// Upload Id for a given upload.
        pub fn set_upload_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.upload_id = input;
            self
        }
        /// Consumes the builder and constructs a [`StartObjectOutput`](crate::output::StartObjectOutput).
        pub fn build(self) -> crate::output::StartObjectOutput {
            crate::output::StartObjectOutput {
                upload_id: self.upload_id,
            }
        }
    }
}
impl StartObjectOutput {
    /// Creates a new builder-style object to manufacture [`StartObjectOutput`](crate::output::StartObjectOutput).
    pub fn builder() -> crate::output::start_object_output::Builder {
        crate::output::start_object_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct PutObjectOutput {
    /// Inline chunk checksum
    #[doc(hidden)]
    pub inline_chunk_checksum: std::option::Option<std::string::String>,
    /// Inline chunk checksum algorithm
    #[doc(hidden)]
    pub inline_chunk_checksum_algorithm: std::option::Option<crate::model::DataChecksumAlgorithm>,
    /// object checksum
    #[doc(hidden)]
    pub object_checksum: std::option::Option<std::string::String>,
    /// object checksum algorithm
    #[doc(hidden)]
    pub object_checksum_algorithm: std::option::Option<crate::model::SummaryChecksumAlgorithm>,
}
impl PutObjectOutput {
    /// Inline chunk checksum
    pub fn inline_chunk_checksum(&self) -> std::option::Option<&str> {
        self.inline_chunk_checksum.as_deref()
    }
    /// Inline chunk checksum algorithm
    pub fn inline_chunk_checksum_algorithm(
        &self,
    ) -> std::option::Option<&crate::model::DataChecksumAlgorithm> {
        self.inline_chunk_checksum_algorithm.as_ref()
    }
    /// object checksum
    pub fn object_checksum(&self) -> std::option::Option<&str> {
        self.object_checksum.as_deref()
    }
    /// object checksum algorithm
    pub fn object_checksum_algorithm(
        &self,
    ) -> std::option::Option<&crate::model::SummaryChecksumAlgorithm> {
        self.object_checksum_algorithm.as_ref()
    }
}
/// See [`PutObjectOutput`](crate::output::PutObjectOutput).
pub mod put_object_output {

    /// A builder for [`PutObjectOutput`](crate::output::PutObjectOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) inline_chunk_checksum: std::option::Option<std::string::String>,
        pub(crate) inline_chunk_checksum_algorithm:
            std::option::Option<crate::model::DataChecksumAlgorithm>,
        pub(crate) object_checksum: std::option::Option<std::string::String>,
        pub(crate) object_checksum_algorithm:
            std::option::Option<crate::model::SummaryChecksumAlgorithm>,
    }
    impl Builder {
        /// Inline chunk checksum
        pub fn inline_chunk_checksum(mut self, input: impl Into<std::string::String>) -> Self {
            self.inline_chunk_checksum = Some(input.into());
            self
        }
        /// Inline chunk checksum
        pub fn set_inline_chunk_checksum(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inline_chunk_checksum = input;
            self
        }
        /// Inline chunk checksum algorithm
        pub fn inline_chunk_checksum_algorithm(
            mut self,
            input: crate::model::DataChecksumAlgorithm,
        ) -> Self {
            self.inline_chunk_checksum_algorithm = Some(input);
            self
        }
        /// Inline chunk checksum algorithm
        pub fn set_inline_chunk_checksum_algorithm(
            mut self,
            input: std::option::Option<crate::model::DataChecksumAlgorithm>,
        ) -> Self {
            self.inline_chunk_checksum_algorithm = input;
            self
        }
        /// object checksum
        pub fn object_checksum(mut self, input: impl Into<std::string::String>) -> Self {
            self.object_checksum = Some(input.into());
            self
        }
        /// object checksum
        pub fn set_object_checksum(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.object_checksum = input;
            self
        }
        /// object checksum algorithm
        pub fn object_checksum_algorithm(
            mut self,
            input: crate::model::SummaryChecksumAlgorithm,
        ) -> Self {
            self.object_checksum_algorithm = Some(input);
            self
        }
        /// object checksum algorithm
        pub fn set_object_checksum_algorithm(
            mut self,
            input: std::option::Option<crate::model::SummaryChecksumAlgorithm>,
        ) -> Self {
            self.object_checksum_algorithm = input;
            self
        }
        /// Consumes the builder and constructs a [`PutObjectOutput`](crate::output::PutObjectOutput).
        pub fn build(self) -> crate::output::PutObjectOutput {
            crate::output::PutObjectOutput {
                inline_chunk_checksum: self.inline_chunk_checksum,
                inline_chunk_checksum_algorithm: self.inline_chunk_checksum_algorithm,
                object_checksum: self.object_checksum,
                object_checksum_algorithm: self.object_checksum_algorithm,
            }
        }
    }
}
impl PutObjectOutput {
    /// Creates a new builder-style object to manufacture [`PutObjectOutput`](crate::output::PutObjectOutput).
    pub fn builder() -> crate::output::put_object_output::Builder {
        crate::output::put_object_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct PutChunkOutput {
    /// Chunk checksum
    #[doc(hidden)]
    pub chunk_checksum: std::option::Option<std::string::String>,
    /// Checksum algorithm
    #[doc(hidden)]
    pub chunk_checksum_algorithm: std::option::Option<crate::model::DataChecksumAlgorithm>,
}
impl PutChunkOutput {
    /// Chunk checksum
    pub fn chunk_checksum(&self) -> std::option::Option<&str> {
        self.chunk_checksum.as_deref()
    }
    /// Checksum algorithm
    pub fn chunk_checksum_algorithm(
        &self,
    ) -> std::option::Option<&crate::model::DataChecksumAlgorithm> {
        self.chunk_checksum_algorithm.as_ref()
    }
}
/// See [`PutChunkOutput`](crate::output::PutChunkOutput).
pub mod put_chunk_output {

    /// A builder for [`PutChunkOutput`](crate::output::PutChunkOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) chunk_checksum: std::option::Option<std::string::String>,
        pub(crate) chunk_checksum_algorithm:
            std::option::Option<crate::model::DataChecksumAlgorithm>,
    }
    impl Builder {
        /// Chunk checksum
        pub fn chunk_checksum(mut self, input: impl Into<std::string::String>) -> Self {
            self.chunk_checksum = Some(input.into());
            self
        }
        /// Chunk checksum
        pub fn set_chunk_checksum(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.chunk_checksum = input;
            self
        }
        /// Checksum algorithm
        pub fn chunk_checksum_algorithm(
            mut self,
            input: crate::model::DataChecksumAlgorithm,
        ) -> Self {
            self.chunk_checksum_algorithm = Some(input);
            self
        }
        /// Checksum algorithm
        pub fn set_chunk_checksum_algorithm(
            mut self,
            input: std::option::Option<crate::model::DataChecksumAlgorithm>,
        ) -> Self {
            self.chunk_checksum_algorithm = input;
            self
        }
        /// Consumes the builder and constructs a [`PutChunkOutput`](crate::output::PutChunkOutput).
        pub fn build(self) -> crate::output::PutChunkOutput {
            crate::output::PutChunkOutput {
                chunk_checksum: self.chunk_checksum,
                chunk_checksum_algorithm: self.chunk_checksum_algorithm,
            }
        }
    }
}
impl PutChunkOutput {
    /// Creates a new builder-style object to manufacture [`PutChunkOutput`](crate::output::PutChunkOutput).
    pub fn builder() -> crate::output::put_chunk_output::Builder {
        crate::output::put_chunk_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct NotifyObjectCompleteOutput {
    /// Object checksum
    #[doc(hidden)]
    pub object_checksum: std::option::Option<std::string::String>,
    /// Checksum algorithm
    #[doc(hidden)]
    pub object_checksum_algorithm: std::option::Option<crate::model::SummaryChecksumAlgorithm>,
}
impl NotifyObjectCompleteOutput {
    /// Object checksum
    pub fn object_checksum(&self) -> std::option::Option<&str> {
        self.object_checksum.as_deref()
    }
    /// Checksum algorithm
    pub fn object_checksum_algorithm(
        &self,
    ) -> std::option::Option<&crate::model::SummaryChecksumAlgorithm> {
        self.object_checksum_algorithm.as_ref()
    }
}
/// See [`NotifyObjectCompleteOutput`](crate::output::NotifyObjectCompleteOutput).
pub mod notify_object_complete_output {

    /// A builder for [`NotifyObjectCompleteOutput`](crate::output::NotifyObjectCompleteOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) object_checksum: std::option::Option<std::string::String>,
        pub(crate) object_checksum_algorithm:
            std::option::Option<crate::model::SummaryChecksumAlgorithm>,
    }
    impl Builder {
        /// Object checksum
        pub fn object_checksum(mut self, input: impl Into<std::string::String>) -> Self {
            self.object_checksum = Some(input.into());
            self
        }
        /// Object checksum
        pub fn set_object_checksum(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.object_checksum = input;
            self
        }
        /// Checksum algorithm
        pub fn object_checksum_algorithm(
            mut self,
            input: crate::model::SummaryChecksumAlgorithm,
        ) -> Self {
            self.object_checksum_algorithm = Some(input);
            self
        }
        /// Checksum algorithm
        pub fn set_object_checksum_algorithm(
            mut self,
            input: std::option::Option<crate::model::SummaryChecksumAlgorithm>,
        ) -> Self {
            self.object_checksum_algorithm = input;
            self
        }
        /// Consumes the builder and constructs a [`NotifyObjectCompleteOutput`](crate::output::NotifyObjectCompleteOutput).
        pub fn build(self) -> crate::output::NotifyObjectCompleteOutput {
            crate::output::NotifyObjectCompleteOutput {
                object_checksum: self.object_checksum,
                object_checksum_algorithm: self.object_checksum_algorithm,
            }
        }
    }
}
impl NotifyObjectCompleteOutput {
    /// Creates a new builder-style object to manufacture [`NotifyObjectCompleteOutput`](crate::output::NotifyObjectCompleteOutput).
    pub fn builder() -> crate::output::notify_object_complete_output::Builder {
        crate::output::notify_object_complete_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListObjectsOutput {
    /// Object list
    #[doc(hidden)]
    pub object_list: std::option::Option<std::vec::Vec<crate::model::BackupObject>>,
    /// Pagination token
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl ListObjectsOutput {
    /// Object list
    pub fn object_list(&self) -> std::option::Option<&[crate::model::BackupObject]> {
        self.object_list.as_deref()
    }
    /// Pagination token
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
/// See [`ListObjectsOutput`](crate::output::ListObjectsOutput).
pub mod list_objects_output {

    /// A builder for [`ListObjectsOutput`](crate::output::ListObjectsOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) object_list: std::option::Option<std::vec::Vec<crate::model::BackupObject>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `object_list`.
        ///
        /// To override the contents of this collection use [`set_object_list`](Self::set_object_list).
        ///
        /// Object list
        pub fn object_list(mut self, input: crate::model::BackupObject) -> Self {
            let mut v = self.object_list.unwrap_or_default();
            v.push(input);
            self.object_list = Some(v);
            self
        }
        /// Object list
        pub fn set_object_list(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::BackupObject>>,
        ) -> Self {
            self.object_list = input;
            self
        }
        /// Pagination token
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// Pagination token
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListObjectsOutput`](crate::output::ListObjectsOutput).
        pub fn build(self) -> crate::output::ListObjectsOutput {
            crate::output::ListObjectsOutput {
                object_list: self.object_list,
                next_token: self.next_token,
            }
        }
    }
}
impl ListObjectsOutput {
    /// Creates a new builder-style object to manufacture [`ListObjectsOutput`](crate::output::ListObjectsOutput).
    pub fn builder() -> crate::output::list_objects_output::Builder {
        crate::output::list_objects_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListChunksOutput {
    /// List of chunks
    #[doc(hidden)]
    pub chunk_list: std::option::Option<std::vec::Vec<crate::model::Chunk>>,
    /// Pagination token
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl ListChunksOutput {
    /// List of chunks
    pub fn chunk_list(&self) -> std::option::Option<&[crate::model::Chunk]> {
        self.chunk_list.as_deref()
    }
    /// Pagination token
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
/// See [`ListChunksOutput`](crate::output::ListChunksOutput).
pub mod list_chunks_output {

    /// A builder for [`ListChunksOutput`](crate::output::ListChunksOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) chunk_list: std::option::Option<std::vec::Vec<crate::model::Chunk>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `chunk_list`.
        ///
        /// To override the contents of this collection use [`set_chunk_list`](Self::set_chunk_list).
        ///
        /// List of chunks
        pub fn chunk_list(mut self, input: crate::model::Chunk) -> Self {
            let mut v = self.chunk_list.unwrap_or_default();
            v.push(input);
            self.chunk_list = Some(v);
            self
        }
        /// List of chunks
        pub fn set_chunk_list(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Chunk>>,
        ) -> Self {
            self.chunk_list = input;
            self
        }
        /// Pagination token
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// Pagination token
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListChunksOutput`](crate::output::ListChunksOutput).
        pub fn build(self) -> crate::output::ListChunksOutput {
            crate::output::ListChunksOutput {
                chunk_list: self.chunk_list,
                next_token: self.next_token,
            }
        }
    }
}
impl ListChunksOutput {
    /// Creates a new builder-style object to manufacture [`ListChunksOutput`](crate::output::ListChunksOutput).
    pub fn builder() -> crate::output::list_chunks_output::Builder {
        crate::output::list_chunks_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct GetObjectMetadataOutput {
    /// Metadata string.
    #[doc(hidden)]
    pub metadata_string: std::option::Option<std::string::String>,
    /// Metadata blob.
    pub metadata_blob: aws_smithy_http::byte_stream::ByteStream,
    /// The size of MetadataBlob.
    #[doc(hidden)]
    pub metadata_blob_length: i64,
    /// MetadataBlob checksum.
    #[doc(hidden)]
    pub metadata_blob_checksum: std::option::Option<std::string::String>,
    /// Checksum algorithm.
    #[doc(hidden)]
    pub metadata_blob_checksum_algorithm: std::option::Option<crate::model::DataChecksumAlgorithm>,
}
impl GetObjectMetadataOutput {
    /// Metadata string.
    pub fn metadata_string(&self) -> std::option::Option<&str> {
        self.metadata_string.as_deref()
    }
    /// Metadata blob.
    pub fn metadata_blob(&self) -> &aws_smithy_http::byte_stream::ByteStream {
        &self.metadata_blob
    }
    /// The size of MetadataBlob.
    pub fn metadata_blob_length(&self) -> i64 {
        self.metadata_blob_length
    }
    /// MetadataBlob checksum.
    pub fn metadata_blob_checksum(&self) -> std::option::Option<&str> {
        self.metadata_blob_checksum.as_deref()
    }
    /// Checksum algorithm.
    pub fn metadata_blob_checksum_algorithm(
        &self,
    ) -> std::option::Option<&crate::model::DataChecksumAlgorithm> {
        self.metadata_blob_checksum_algorithm.as_ref()
    }
}
/// See [`GetObjectMetadataOutput`](crate::output::GetObjectMetadataOutput).
pub mod get_object_metadata_output {

    /// A builder for [`GetObjectMetadataOutput`](crate::output::GetObjectMetadataOutput).
    #[derive(std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) metadata_string: std::option::Option<std::string::String>,
        pub(crate) metadata_blob: std::option::Option<aws_smithy_http::byte_stream::ByteStream>,
        pub(crate) metadata_blob_length: std::option::Option<i64>,
        pub(crate) metadata_blob_checksum: std::option::Option<std::string::String>,
        pub(crate) metadata_blob_checksum_algorithm:
            std::option::Option<crate::model::DataChecksumAlgorithm>,
    }
    impl Builder {
        /// Metadata string.
        pub fn metadata_string(mut self, input: impl Into<std::string::String>) -> Self {
            self.metadata_string = Some(input.into());
            self
        }
        /// Metadata string.
        pub fn set_metadata_string(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.metadata_string = input;
            self
        }
        /// Metadata blob.
        pub fn metadata_blob(mut self, input: aws_smithy_http::byte_stream::ByteStream) -> Self {
            self.metadata_blob = Some(input);
            self
        }
        /// Metadata blob.
        pub fn set_metadata_blob(
            mut self,
            input: std::option::Option<aws_smithy_http::byte_stream::ByteStream>,
        ) -> Self {
            self.metadata_blob = input;
            self
        }
        /// The size of MetadataBlob.
        pub fn metadata_blob_length(mut self, input: i64) -> Self {
            self.metadata_blob_length = Some(input);
            self
        }
        /// The size of MetadataBlob.
        pub fn set_metadata_blob_length(mut self, input: std::option::Option<i64>) -> Self {
            self.metadata_blob_length = input;
            self
        }
        /// MetadataBlob checksum.
        pub fn metadata_blob_checksum(mut self, input: impl Into<std::string::String>) -> Self {
            self.metadata_blob_checksum = Some(input.into());
            self
        }
        /// MetadataBlob checksum.
        pub fn set_metadata_blob_checksum(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.metadata_blob_checksum = input;
            self
        }
        /// Checksum algorithm.
        pub fn metadata_blob_checksum_algorithm(
            mut self,
            input: crate::model::DataChecksumAlgorithm,
        ) -> Self {
            self.metadata_blob_checksum_algorithm = Some(input);
            self
        }
        /// Checksum algorithm.
        pub fn set_metadata_blob_checksum_algorithm(
            mut self,
            input: std::option::Option<crate::model::DataChecksumAlgorithm>,
        ) -> Self {
            self.metadata_blob_checksum_algorithm = input;
            self
        }
        /// Consumes the builder and constructs a [`GetObjectMetadataOutput`](crate::output::GetObjectMetadataOutput).
        pub fn build(self) -> crate::output::GetObjectMetadataOutput {
            crate::output::GetObjectMetadataOutput {
                metadata_string: self.metadata_string,
                metadata_blob: self.metadata_blob.unwrap_or_default(),
                metadata_blob_length: self.metadata_blob_length.unwrap_or_default(),
                metadata_blob_checksum: self.metadata_blob_checksum,
                metadata_blob_checksum_algorithm: self.metadata_blob_checksum_algorithm,
            }
        }
    }
}
impl GetObjectMetadataOutput {
    /// Creates a new builder-style object to manufacture [`GetObjectMetadataOutput`](crate::output::GetObjectMetadataOutput).
    pub fn builder() -> crate::output::get_object_metadata_output::Builder {
        crate::output::get_object_metadata_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct GetChunkOutput {
    /// Chunk data
    pub data: aws_smithy_http::byte_stream::ByteStream,
    /// Data length
    #[doc(hidden)]
    pub length: i64,
    /// Data checksum
    #[doc(hidden)]
    pub checksum: std::option::Option<std::string::String>,
    /// Checksum algorithm
    #[doc(hidden)]
    pub checksum_algorithm: std::option::Option<crate::model::DataChecksumAlgorithm>,
}
impl GetChunkOutput {
    /// Chunk data
    pub fn data(&self) -> &aws_smithy_http::byte_stream::ByteStream {
        &self.data
    }
    /// Data length
    pub fn length(&self) -> i64 {
        self.length
    }
    /// Data checksum
    pub fn checksum(&self) -> std::option::Option<&str> {
        self.checksum.as_deref()
    }
    /// Checksum algorithm
    pub fn checksum_algorithm(&self) -> std::option::Option<&crate::model::DataChecksumAlgorithm> {
        self.checksum_algorithm.as_ref()
    }
}
/// See [`GetChunkOutput`](crate::output::GetChunkOutput).
pub mod get_chunk_output {

    /// A builder for [`GetChunkOutput`](crate::output::GetChunkOutput).
    #[derive(std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) data: std::option::Option<aws_smithy_http::byte_stream::ByteStream>,
        pub(crate) length: std::option::Option<i64>,
        pub(crate) checksum: std::option::Option<std::string::String>,
        pub(crate) checksum_algorithm: std::option::Option<crate::model::DataChecksumAlgorithm>,
    }
    impl Builder {
        /// Chunk data
        pub fn data(mut self, input: aws_smithy_http::byte_stream::ByteStream) -> Self {
            self.data = Some(input);
            self
        }
        /// Chunk data
        pub fn set_data(
            mut self,
            input: std::option::Option<aws_smithy_http::byte_stream::ByteStream>,
        ) -> Self {
            self.data = input;
            self
        }
        /// Data length
        pub fn length(mut self, input: i64) -> Self {
            self.length = Some(input);
            self
        }
        /// Data length
        pub fn set_length(mut self, input: std::option::Option<i64>) -> Self {
            self.length = input;
            self
        }
        /// Data checksum
        pub fn checksum(mut self, input: impl Into<std::string::String>) -> Self {
            self.checksum = Some(input.into());
            self
        }
        /// Data checksum
        pub fn set_checksum(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.checksum = input;
            self
        }
        /// Checksum algorithm
        pub fn checksum_algorithm(mut self, input: crate::model::DataChecksumAlgorithm) -> Self {
            self.checksum_algorithm = Some(input);
            self
        }
        /// Checksum algorithm
        pub fn set_checksum_algorithm(
            mut self,
            input: std::option::Option<crate::model::DataChecksumAlgorithm>,
        ) -> Self {
            self.checksum_algorithm = input;
            self
        }
        /// Consumes the builder and constructs a [`GetChunkOutput`](crate::output::GetChunkOutput).
        pub fn build(self) -> crate::output::GetChunkOutput {
            crate::output::GetChunkOutput {
                data: self.data.unwrap_or_default(),
                length: self.length.unwrap_or_default(),
                checksum: self.checksum,
                checksum_algorithm: self.checksum_algorithm,
            }
        }
    }
}
impl GetChunkOutput {
    /// Creates a new builder-style object to manufacture [`GetChunkOutput`](crate::output::GetChunkOutput).
    pub fn builder() -> crate::output::get_chunk_output::Builder {
        crate::output::get_chunk_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DeleteObjectOutput {}
/// See [`DeleteObjectOutput`](crate::output::DeleteObjectOutput).
pub mod delete_object_output {

    /// A builder for [`DeleteObjectOutput`](crate::output::DeleteObjectOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`DeleteObjectOutput`](crate::output::DeleteObjectOutput).
        pub fn build(self) -> crate::output::DeleteObjectOutput {
            crate::output::DeleteObjectOutput {}
        }
    }
}
impl DeleteObjectOutput {
    /// Creates a new builder-style object to manufacture [`DeleteObjectOutput`](crate::output::DeleteObjectOutput).
    pub fn builder() -> crate::output::delete_object_output::Builder {
        crate::output::delete_object_output::Builder::default()
    }
}
