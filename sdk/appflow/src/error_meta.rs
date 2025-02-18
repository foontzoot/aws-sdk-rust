// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>AppFlow/Requester has invalid or missing permissions.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p> There was a conflict when processing the request (for example, a flow with the given name already exists within the account. Check for conflicting resource names and try again. </p>
    ConflictException(crate::error::ConflictException),
    /// <p> An error occurred when authenticating with the connector endpoint. </p>
    ConnectorAuthenticationException(crate::error::ConnectorAuthenticationException),
    /// <p> An error occurred when retrieving data from the connector endpoint. </p>
    ConnectorServerException(crate::error::ConnectorServerException),
    /// <p> An internal service error occurred during the processing of your request. Try again later. </p>
    InternalServerException(crate::error::InternalServerException),
    /// <p> The resource specified in the request (such as the source or destination connector profile) is not found. </p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p> The request would cause a service quota (such as the number of flows) to be exceeded. </p>
    ServiceQuotaExceededException(crate::error::ServiceQuotaExceededException),
    /// <p>API calls have exceeded the maximum allowed API request rate per account and per Region. </p>
    ThrottlingException(crate::error::ThrottlingException),
    /// <p> The requested operation is not supported for the current flow. </p>
    UnsupportedOperationException(crate::error::UnsupportedOperationException),
    /// <p> The request has invalid or missing parameters. </p>
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
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::ConnectorAuthenticationException(inner) => inner.fmt(f),
            Error::ConnectorServerException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceQuotaExceededException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::UnsupportedOperationException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateConnectorProfileError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CreateConnectorProfileError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateConnectorProfileError> for Error {
    fn from(err: crate::error::CreateConnectorProfileError) -> Self {
        match err.kind {
            crate::error::CreateConnectorProfileErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::CreateConnectorProfileErrorKind::ConnectorAuthenticationException(
                inner,
            ) => Error::ConnectorAuthenticationException(inner),
            crate::error::CreateConnectorProfileErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::CreateConnectorProfileErrorKind::ServiceQuotaExceededException(inner) => {
                Error::ServiceQuotaExceededException(inner)
            }
            crate::error::CreateConnectorProfileErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::CreateConnectorProfileErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateFlowError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateFlowError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateFlowError> for Error {
    fn from(err: crate::error::CreateFlowError) -> Self {
        match err.kind {
            crate::error::CreateFlowErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::CreateFlowErrorKind::ConnectorAuthenticationException(inner) => {
                Error::ConnectorAuthenticationException(inner)
            }
            crate::error::CreateFlowErrorKind::ConnectorServerException(inner) => {
                Error::ConnectorServerException(inner)
            }
            crate::error::CreateFlowErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::CreateFlowErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::CreateFlowErrorKind::ServiceQuotaExceededException(inner) => {
                Error::ServiceQuotaExceededException(inner)
            }
            crate::error::CreateFlowErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::CreateFlowErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteConnectorProfileError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteConnectorProfileError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteConnectorProfileError> for Error {
    fn from(err: crate::error::DeleteConnectorProfileError) -> Self {
        match err.kind {
            crate::error::DeleteConnectorProfileErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::DeleteConnectorProfileErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::DeleteConnectorProfileErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::DeleteConnectorProfileErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteFlowError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteFlowError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteFlowError> for Error {
    fn from(err: crate::error::DeleteFlowError) -> Self {
        match err.kind {
            crate::error::DeleteFlowErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::DeleteFlowErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::DeleteFlowErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::DeleteFlowErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeConnectorError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeConnectorError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeConnectorError> for Error {
    fn from(err: crate::error::DescribeConnectorError) -> Self {
        match err.kind {
            crate::error::DescribeConnectorErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::DescribeConnectorErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::DescribeConnectorErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::DescribeConnectorErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeConnectorEntityError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeConnectorEntityError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeConnectorEntityError> for Error {
    fn from(err: crate::error::DescribeConnectorEntityError) -> Self {
        match err.kind {
            crate::error::DescribeConnectorEntityErrorKind::ConnectorAuthenticationException(
                inner,
            ) => Error::ConnectorAuthenticationException(inner),
            crate::error::DescribeConnectorEntityErrorKind::ConnectorServerException(inner) => {
                Error::ConnectorServerException(inner)
            }
            crate::error::DescribeConnectorEntityErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::DescribeConnectorEntityErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::DescribeConnectorEntityErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::DescribeConnectorEntityErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeConnectorProfilesError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeConnectorProfilesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeConnectorProfilesError> for Error {
    fn from(err: crate::error::DescribeConnectorProfilesError) -> Self {
        match err.kind {
            crate::error::DescribeConnectorProfilesErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::DescribeConnectorProfilesErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::DescribeConnectorProfilesErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeConnectorsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeConnectorsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeConnectorsError> for Error {
    fn from(err: crate::error::DescribeConnectorsError) -> Self {
        match err.kind {
            crate::error::DescribeConnectorsErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::DescribeConnectorsErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::DescribeConnectorsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeFlowError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeFlowError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeFlowError> for Error {
    fn from(err: crate::error::DescribeFlowError) -> Self {
        match err.kind {
            crate::error::DescribeFlowErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::DescribeFlowErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::DescribeFlowErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeFlowExecutionRecordsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeFlowExecutionRecordsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeFlowExecutionRecordsError> for Error {
    fn from(err: crate::error::DescribeFlowExecutionRecordsError) -> Self {
        match err.kind {
            crate::error::DescribeFlowExecutionRecordsErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::DescribeFlowExecutionRecordsErrorKind::ResourceNotFoundException(
                inner,
            ) => Error::ResourceNotFoundException(inner),
            crate::error::DescribeFlowExecutionRecordsErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::DescribeFlowExecutionRecordsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListConnectorEntitiesError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListConnectorEntitiesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListConnectorEntitiesError> for Error {
    fn from(err: crate::error::ListConnectorEntitiesError) -> Self {
        match err.kind {
            crate::error::ListConnectorEntitiesErrorKind::ConnectorAuthenticationException(
                inner,
            ) => Error::ConnectorAuthenticationException(inner),
            crate::error::ListConnectorEntitiesErrorKind::ConnectorServerException(inner) => {
                Error::ConnectorServerException(inner)
            }
            crate::error::ListConnectorEntitiesErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::ListConnectorEntitiesErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::ListConnectorEntitiesErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::ListConnectorEntitiesErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListConnectorsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListConnectorsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListConnectorsError> for Error {
    fn from(err: crate::error::ListConnectorsError) -> Self {
        match err.kind {
            crate::error::ListConnectorsErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::ListConnectorsErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::ListConnectorsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListFlowsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListFlowsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListFlowsError> for Error {
    fn from(err: crate::error::ListFlowsError) -> Self {
        match err.kind {
            crate::error::ListFlowsErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::ListFlowsErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::ListFlowsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListTagsForResourceError> for Error {
    fn from(err: crate::error::ListTagsForResourceError) -> Self {
        match err.kind {
            crate::error::ListTagsForResourceErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::ListTagsForResourceErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::ListTagsForResourceErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::ListTagsForResourceErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::RegisterConnectorError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::RegisterConnectorError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::RegisterConnectorError> for Error {
    fn from(err: crate::error::RegisterConnectorError) -> Self {
        match err.kind {
            crate::error::RegisterConnectorErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::RegisterConnectorErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::RegisterConnectorErrorKind::ConnectorAuthenticationException(inner) => {
                Error::ConnectorAuthenticationException(inner)
            }
            crate::error::RegisterConnectorErrorKind::ConnectorServerException(inner) => {
                Error::ConnectorServerException(inner)
            }
            crate::error::RegisterConnectorErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::RegisterConnectorErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::RegisterConnectorErrorKind::ServiceQuotaExceededException(inner) => {
                Error::ServiceQuotaExceededException(inner)
            }
            crate::error::RegisterConnectorErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::RegisterConnectorErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::RegisterConnectorErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StartFlowError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::StartFlowError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::StartFlowError> for Error {
    fn from(err: crate::error::StartFlowError) -> Self {
        match err.kind {
            crate::error::StartFlowErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::StartFlowErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::StartFlowErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::StartFlowErrorKind::ServiceQuotaExceededException(inner) => {
                Error::ServiceQuotaExceededException(inner)
            }
            crate::error::StartFlowErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StopFlowError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::StopFlowError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::StopFlowError> for Error {
    fn from(err: crate::error::StopFlowError) -> Self {
        match err.kind {
            crate::error::StopFlowErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::StopFlowErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::StopFlowErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::StopFlowErrorKind::UnsupportedOperationException(inner) => {
                Error::UnsupportedOperationException(inner)
            }
            crate::error::StopFlowErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::TagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::TagResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::TagResourceError> for Error {
    fn from(err: crate::error::TagResourceError) -> Self {
        match err.kind {
            crate::error::TagResourceErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::TagResourceErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::TagResourceErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::TagResourceErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UnregisterConnectorError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UnregisterConnectorError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UnregisterConnectorError> for Error {
    fn from(err: crate::error::UnregisterConnectorError) -> Self {
        match err.kind {
            crate::error::UnregisterConnectorErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::UnregisterConnectorErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::UnregisterConnectorErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::UnregisterConnectorErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UntagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UntagResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UntagResourceError> for Error {
    fn from(err: crate::error::UntagResourceError) -> Self {
        match err.kind {
            crate::error::UntagResourceErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::UntagResourceErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::UntagResourceErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::UntagResourceErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateConnectorProfileError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdateConnectorProfileError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateConnectorProfileError> for Error {
    fn from(err: crate::error::UpdateConnectorProfileError) -> Self {
        match err.kind {
            crate::error::UpdateConnectorProfileErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::UpdateConnectorProfileErrorKind::ConnectorAuthenticationException(
                inner,
            ) => Error::ConnectorAuthenticationException(inner),
            crate::error::UpdateConnectorProfileErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::UpdateConnectorProfileErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::UpdateConnectorProfileErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::UpdateConnectorProfileErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateConnectorRegistrationError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdateConnectorRegistrationError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateConnectorRegistrationError> for Error {
    fn from(err: crate::error::UpdateConnectorRegistrationError) -> Self {
        match err.kind {
            crate::error::UpdateConnectorRegistrationErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::UpdateConnectorRegistrationErrorKind::ConflictException(inner) => Error::ConflictException(inner),
            crate::error::UpdateConnectorRegistrationErrorKind::ConnectorAuthenticationException(inner) => Error::ConnectorAuthenticationException(inner),
            crate::error::UpdateConnectorRegistrationErrorKind::ConnectorServerException(inner) => Error::ConnectorServerException(inner),
            crate::error::UpdateConnectorRegistrationErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::UpdateConnectorRegistrationErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::UpdateConnectorRegistrationErrorKind::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
            crate::error::UpdateConnectorRegistrationErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::UpdateConnectorRegistrationErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::UpdateConnectorRegistrationErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateFlowError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateFlowError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateFlowError> for Error {
    fn from(err: crate::error::UpdateFlowError) -> Self {
        match err.kind {
            crate::error::UpdateFlowErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::UpdateFlowErrorKind::ConnectorAuthenticationException(inner) => {
                Error::ConnectorAuthenticationException(inner)
            }
            crate::error::UpdateFlowErrorKind::ConnectorServerException(inner) => {
                Error::ConnectorServerException(inner)
            }
            crate::error::UpdateFlowErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::UpdateFlowErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::UpdateFlowErrorKind::ServiceQuotaExceededException(inner) => {
                Error::ServiceQuotaExceededException(inner)
            }
            crate::error::UpdateFlowErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::UpdateFlowErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl std::error::Error for Error {}
