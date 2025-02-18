// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Error type for the `UpdateServiceSettings` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct UpdateServiceSettingsError {
    /// Kind of error that occurred.
    pub kind: UpdateServiceSettingsErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
impl aws_smithy_http::result::CreateUnhandledError for UpdateServiceSettingsError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: UpdateServiceSettingsErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default(),
        }
    }
}
/// Types of errors that can occur for the `UpdateServiceSettings` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum UpdateServiceSettingsErrorKind {
    /// <p>An exception occurred with the service.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>The request was denied due to request throttling.</p>
    ThrottlingException(crate::error::ThrottlingException),
    /// <p>The provided input is not valid. Try your request again.</p>
    ValidationException(crate::error::ValidationException),
    ///
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    ///
    /// When logging an error from the SDK, it is recommended that you either wrap the error in
    /// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
    /// error reporter library that visits the error's cause/source chain, or call
    /// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
    ///
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for UpdateServiceSettingsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            UpdateServiceSettingsErrorKind::InternalServerException(_inner) => _inner.fmt(f),
            UpdateServiceSettingsErrorKind::ThrottlingException(_inner) => _inner.fmt(f),
            UpdateServiceSettingsErrorKind::ValidationException(_inner) => _inner.fmt(f),
            UpdateServiceSettingsErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for UpdateServiceSettingsError {
    fn code(&self) -> Option<&str> {
        UpdateServiceSettingsError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl UpdateServiceSettingsError {
    /// Creates a new `UpdateServiceSettingsError`.
    pub fn new(kind: UpdateServiceSettingsErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `UpdateServiceSettingsError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: UpdateServiceSettingsErrorKind::Unhandled(crate::error::Unhandled::new(
                err.into(),
            )),
            meta: Default::default(),
        }
    }

    /// Creates the `UpdateServiceSettingsError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: UpdateServiceSettingsErrorKind::Unhandled(crate::error::Unhandled::new(
                err.into(),
            )),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `UpdateServiceSettingsErrorKind::InternalServerException`.
    pub fn is_internal_server_exception(&self) -> bool {
        matches!(
            &self.kind,
            UpdateServiceSettingsErrorKind::InternalServerException(_)
        )
    }
    /// Returns `true` if the error kind is `UpdateServiceSettingsErrorKind::ThrottlingException`.
    pub fn is_throttling_exception(&self) -> bool {
        matches!(
            &self.kind,
            UpdateServiceSettingsErrorKind::ThrottlingException(_)
        )
    }
    /// Returns `true` if the error kind is `UpdateServiceSettingsErrorKind::ValidationException`.
    pub fn is_validation_exception(&self) -> bool {
        matches!(
            &self.kind,
            UpdateServiceSettingsErrorKind::ValidationException(_)
        )
    }
}
impl std::error::Error for UpdateServiceSettingsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            UpdateServiceSettingsErrorKind::InternalServerException(_inner) => Some(_inner),
            UpdateServiceSettingsErrorKind::ThrottlingException(_inner) => Some(_inner),
            UpdateServiceSettingsErrorKind::ValidationException(_inner) => Some(_inner),
            UpdateServiceSettingsErrorKind::Unhandled(_inner) => Some(_inner),
        }
    }
}

/// <p>The provided input is not valid. Try your request again.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ValidationException {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl ValidationException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ValidationException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ValidationException")?;
        if let Some(inner_1) = &self.message {
            {
                write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for ValidationException {}
/// See [`ValidationException`](crate::error::ValidationException).
pub mod validation_exception {

    /// A builder for [`ValidationException`](crate::error::ValidationException).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ValidationException`](crate::error::ValidationException).
        pub fn build(self) -> crate::error::ValidationException {
            crate::error::ValidationException {
                message: self.message,
            }
        }
    }
}
impl ValidationException {
    /// Creates a new builder-style object to manufacture [`ValidationException`](crate::error::ValidationException).
    pub fn builder() -> crate::error::validation_exception::Builder {
        crate::error::validation_exception::Builder::default()
    }
}

/// <p>The request was denied due to request throttling.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ThrottlingException {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl ThrottlingException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ThrottlingException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ThrottlingException")?;
        if let Some(inner_2) = &self.message {
            {
                write!(f, ": {}", inner_2)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for ThrottlingException {}
/// See [`ThrottlingException`](crate::error::ThrottlingException).
pub mod throttling_exception {

    /// A builder for [`ThrottlingException`](crate::error::ThrottlingException).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ThrottlingException`](crate::error::ThrottlingException).
        pub fn build(self) -> crate::error::ThrottlingException {
            crate::error::ThrottlingException {
                message: self.message,
            }
        }
    }
}
impl ThrottlingException {
    /// Creates a new builder-style object to manufacture [`ThrottlingException`](crate::error::ThrottlingException).
    pub fn builder() -> crate::error::throttling_exception::Builder {
        crate::error::throttling_exception::Builder::default()
    }
}

/// <p>An exception occurred with the service.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct InternalServerException {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl InternalServerException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InternalServerException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InternalServerException")?;
        if let Some(inner_3) = &self.message {
            {
                write!(f, ": {}", inner_3)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for InternalServerException {}
/// See [`InternalServerException`](crate::error::InternalServerException).
pub mod internal_server_exception {

    /// A builder for [`InternalServerException`](crate::error::InternalServerException).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`InternalServerException`](crate::error::InternalServerException).
        pub fn build(self) -> crate::error::InternalServerException {
            crate::error::InternalServerException {
                message: self.message,
            }
        }
    }
}
impl InternalServerException {
    /// Creates a new builder-style object to manufacture [`InternalServerException`](crate::error::InternalServerException).
    pub fn builder() -> crate::error::internal_server_exception::Builder {
        crate::error::internal_server_exception::Builder::default()
    }
}

/// Error type for the `ListLinuxSubscriptions` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct ListLinuxSubscriptionsError {
    /// Kind of error that occurred.
    pub kind: ListLinuxSubscriptionsErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
impl aws_smithy_http::result::CreateUnhandledError for ListLinuxSubscriptionsError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: ListLinuxSubscriptionsErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default(),
        }
    }
}
/// Types of errors that can occur for the `ListLinuxSubscriptions` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum ListLinuxSubscriptionsErrorKind {
    /// <p>An exception occurred with the service.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>The request was denied due to request throttling.</p>
    ThrottlingException(crate::error::ThrottlingException),
    /// <p>The provided input is not valid. Try your request again.</p>
    ValidationException(crate::error::ValidationException),
    ///
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    ///
    /// When logging an error from the SDK, it is recommended that you either wrap the error in
    /// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
    /// error reporter library that visits the error's cause/source chain, or call
    /// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
    ///
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for ListLinuxSubscriptionsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ListLinuxSubscriptionsErrorKind::InternalServerException(_inner) => _inner.fmt(f),
            ListLinuxSubscriptionsErrorKind::ThrottlingException(_inner) => _inner.fmt(f),
            ListLinuxSubscriptionsErrorKind::ValidationException(_inner) => _inner.fmt(f),
            ListLinuxSubscriptionsErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for ListLinuxSubscriptionsError {
    fn code(&self) -> Option<&str> {
        ListLinuxSubscriptionsError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl ListLinuxSubscriptionsError {
    /// Creates a new `ListLinuxSubscriptionsError`.
    pub fn new(kind: ListLinuxSubscriptionsErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `ListLinuxSubscriptionsError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: ListLinuxSubscriptionsErrorKind::Unhandled(crate::error::Unhandled::new(
                err.into(),
            )),
            meta: Default::default(),
        }
    }

    /// Creates the `ListLinuxSubscriptionsError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: ListLinuxSubscriptionsErrorKind::Unhandled(crate::error::Unhandled::new(
                err.into(),
            )),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `ListLinuxSubscriptionsErrorKind::InternalServerException`.
    pub fn is_internal_server_exception(&self) -> bool {
        matches!(
            &self.kind,
            ListLinuxSubscriptionsErrorKind::InternalServerException(_)
        )
    }
    /// Returns `true` if the error kind is `ListLinuxSubscriptionsErrorKind::ThrottlingException`.
    pub fn is_throttling_exception(&self) -> bool {
        matches!(
            &self.kind,
            ListLinuxSubscriptionsErrorKind::ThrottlingException(_)
        )
    }
    /// Returns `true` if the error kind is `ListLinuxSubscriptionsErrorKind::ValidationException`.
    pub fn is_validation_exception(&self) -> bool {
        matches!(
            &self.kind,
            ListLinuxSubscriptionsErrorKind::ValidationException(_)
        )
    }
}
impl std::error::Error for ListLinuxSubscriptionsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ListLinuxSubscriptionsErrorKind::InternalServerException(_inner) => Some(_inner),
            ListLinuxSubscriptionsErrorKind::ThrottlingException(_inner) => Some(_inner),
            ListLinuxSubscriptionsErrorKind::ValidationException(_inner) => Some(_inner),
            ListLinuxSubscriptionsErrorKind::Unhandled(_inner) => Some(_inner),
        }
    }
}

/// Error type for the `ListLinuxSubscriptionInstances` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct ListLinuxSubscriptionInstancesError {
    /// Kind of error that occurred.
    pub kind: ListLinuxSubscriptionInstancesErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
impl aws_smithy_http::result::CreateUnhandledError for ListLinuxSubscriptionInstancesError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: ListLinuxSubscriptionInstancesErrorKind::Unhandled(crate::error::Unhandled::new(
                source,
            )),
            meta: Default::default(),
        }
    }
}
/// Types of errors that can occur for the `ListLinuxSubscriptionInstances` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum ListLinuxSubscriptionInstancesErrorKind {
    /// <p>An exception occurred with the service.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>The request was denied due to request throttling.</p>
    ThrottlingException(crate::error::ThrottlingException),
    /// <p>The provided input is not valid. Try your request again.</p>
    ValidationException(crate::error::ValidationException),
    ///
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    ///
    /// When logging an error from the SDK, it is recommended that you either wrap the error in
    /// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
    /// error reporter library that visits the error's cause/source chain, or call
    /// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
    ///
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for ListLinuxSubscriptionInstancesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ListLinuxSubscriptionInstancesErrorKind::InternalServerException(_inner) => {
                _inner.fmt(f)
            }
            ListLinuxSubscriptionInstancesErrorKind::ThrottlingException(_inner) => _inner.fmt(f),
            ListLinuxSubscriptionInstancesErrorKind::ValidationException(_inner) => _inner.fmt(f),
            ListLinuxSubscriptionInstancesErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for ListLinuxSubscriptionInstancesError {
    fn code(&self) -> Option<&str> {
        ListLinuxSubscriptionInstancesError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl ListLinuxSubscriptionInstancesError {
    /// Creates a new `ListLinuxSubscriptionInstancesError`.
    pub fn new(
        kind: ListLinuxSubscriptionInstancesErrorKind,
        meta: aws_smithy_types::Error,
    ) -> Self {
        Self { kind, meta }
    }

    /// Creates the `ListLinuxSubscriptionInstancesError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: ListLinuxSubscriptionInstancesErrorKind::Unhandled(crate::error::Unhandled::new(
                err.into(),
            )),
            meta: Default::default(),
        }
    }

    /// Creates the `ListLinuxSubscriptionInstancesError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: ListLinuxSubscriptionInstancesErrorKind::Unhandled(crate::error::Unhandled::new(
                err.into(),
            )),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `ListLinuxSubscriptionInstancesErrorKind::InternalServerException`.
    pub fn is_internal_server_exception(&self) -> bool {
        matches!(
            &self.kind,
            ListLinuxSubscriptionInstancesErrorKind::InternalServerException(_)
        )
    }
    /// Returns `true` if the error kind is `ListLinuxSubscriptionInstancesErrorKind::ThrottlingException`.
    pub fn is_throttling_exception(&self) -> bool {
        matches!(
            &self.kind,
            ListLinuxSubscriptionInstancesErrorKind::ThrottlingException(_)
        )
    }
    /// Returns `true` if the error kind is `ListLinuxSubscriptionInstancesErrorKind::ValidationException`.
    pub fn is_validation_exception(&self) -> bool {
        matches!(
            &self.kind,
            ListLinuxSubscriptionInstancesErrorKind::ValidationException(_)
        )
    }
}
impl std::error::Error for ListLinuxSubscriptionInstancesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ListLinuxSubscriptionInstancesErrorKind::InternalServerException(_inner) => {
                Some(_inner)
            }
            ListLinuxSubscriptionInstancesErrorKind::ThrottlingException(_inner) => Some(_inner),
            ListLinuxSubscriptionInstancesErrorKind::ValidationException(_inner) => Some(_inner),
            ListLinuxSubscriptionInstancesErrorKind::Unhandled(_inner) => Some(_inner),
        }
    }
}

/// Error type for the `GetServiceSettings` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct GetServiceSettingsError {
    /// Kind of error that occurred.
    pub kind: GetServiceSettingsErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
impl aws_smithy_http::result::CreateUnhandledError for GetServiceSettingsError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: GetServiceSettingsErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default(),
        }
    }
}
/// Types of errors that can occur for the `GetServiceSettings` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum GetServiceSettingsErrorKind {
    /// <p>An exception occurred with the service.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>The request was denied due to request throttling.</p>
    ThrottlingException(crate::error::ThrottlingException),
    /// <p>The provided input is not valid. Try your request again.</p>
    ValidationException(crate::error::ValidationException),
    ///
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    ///
    /// When logging an error from the SDK, it is recommended that you either wrap the error in
    /// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
    /// error reporter library that visits the error's cause/source chain, or call
    /// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
    ///
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for GetServiceSettingsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            GetServiceSettingsErrorKind::InternalServerException(_inner) => _inner.fmt(f),
            GetServiceSettingsErrorKind::ThrottlingException(_inner) => _inner.fmt(f),
            GetServiceSettingsErrorKind::ValidationException(_inner) => _inner.fmt(f),
            GetServiceSettingsErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for GetServiceSettingsError {
    fn code(&self) -> Option<&str> {
        GetServiceSettingsError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl GetServiceSettingsError {
    /// Creates a new `GetServiceSettingsError`.
    pub fn new(kind: GetServiceSettingsErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `GetServiceSettingsError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: GetServiceSettingsErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
            meta: Default::default(),
        }
    }

    /// Creates the `GetServiceSettingsError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: GetServiceSettingsErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `GetServiceSettingsErrorKind::InternalServerException`.
    pub fn is_internal_server_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetServiceSettingsErrorKind::InternalServerException(_)
        )
    }
    /// Returns `true` if the error kind is `GetServiceSettingsErrorKind::ThrottlingException`.
    pub fn is_throttling_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetServiceSettingsErrorKind::ThrottlingException(_)
        )
    }
    /// Returns `true` if the error kind is `GetServiceSettingsErrorKind::ValidationException`.
    pub fn is_validation_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetServiceSettingsErrorKind::ValidationException(_)
        )
    }
}
impl std::error::Error for GetServiceSettingsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            GetServiceSettingsErrorKind::InternalServerException(_inner) => Some(_inner),
            GetServiceSettingsErrorKind::ThrottlingException(_inner) => Some(_inner),
            GetServiceSettingsErrorKind::ValidationException(_inner) => Some(_inner),
            GetServiceSettingsErrorKind::Unhandled(_inner) => Some(_inner),
        }
    }
}

///
/// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
///
/// When logging an error from the SDK, it is recommended that you either wrap the error in
/// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
/// error reporter library that visits the error's cause/source chain, or call
/// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
///
#[derive(Debug)]
pub struct Unhandled {
    source: Box<dyn std::error::Error + Send + Sync + 'static>,
}
impl Unhandled {
    #[allow(unused)]
    pub(crate) fn new(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self { source }
    }
}
impl std::fmt::Display for Unhandled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "unhandled error")
    }
}
impl std::error::Error for Unhandled {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.source.as_ref() as _)
    }
}
