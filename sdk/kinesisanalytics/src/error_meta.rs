// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>User-provided application code (query) is invalid. This can be a simple syntax error.</p>
    CodeValidationException(crate::error::CodeValidationException),
    /// <p>Exception thrown as a result of concurrent modification to an application. For example, two individuals attempting to edit the same application at the same time.</p>
    ConcurrentModificationException(crate::error::ConcurrentModificationException),
    /// <p>User-provided application configuration is not valid.</p>
    InvalidApplicationConfigurationException(
        crate::error::InvalidApplicationConfigurationException,
    ),
    /// <p>Specified input parameter value is invalid.</p>
    InvalidArgumentException(crate::error::InvalidArgumentException),
    /// <p>Exceeded the number of applications allowed.</p>
    LimitExceededException(crate::error::LimitExceededException),
    /// <p>Application is not available for this operation.</p>
    ResourceInUseException(crate::error::ResourceInUseException),
    /// <p>Specified application can't be found.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>Discovery failed to get a record from the streaming source because of the Amazon Kinesis Streams ProvisionedThroughputExceededException. For more information, see <a href="https://docs.aws.amazon.com/kinesis/latest/APIReference/API_GetRecords.html">GetRecords</a> in the Amazon Kinesis Streams API Reference.</p>
    ResourceProvisionedThroughputExceededException(
        crate::error::ResourceProvisionedThroughputExceededException,
    ),
    /// <p>The service is unavailable. Back off and retry the operation. </p>
    ServiceUnavailableException(crate::error::ServiceUnavailableException),
    /// <p>Application created with too many tags, or too many tags added to an application. Note that the maximum number of application tags includes system tags. The maximum number of user-defined application tags is 50.</p>
    TooManyTagsException(crate::error::TooManyTagsException),
    /// <p>Data format is not valid. Amazon Kinesis Analytics is not able to detect schema for the given streaming source.</p>
    UnableToDetectSchemaException(crate::error::UnableToDetectSchemaException),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation. </p>
    UnsupportedOperationException(crate::error::UnsupportedOperationException),
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
            Error::CodeValidationException(inner) => inner.fmt(f),
            Error::ConcurrentModificationException(inner) => inner.fmt(f),
            Error::InvalidApplicationConfigurationException(inner) => inner.fmt(f),
            Error::InvalidArgumentException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::ResourceInUseException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ResourceProvisionedThroughputExceededException(inner) => inner.fmt(f),
            Error::ServiceUnavailableException(inner) => inner.fmt(f),
            Error::TooManyTagsException(inner) => inner.fmt(f),
            Error::UnableToDetectSchemaException(inner) => inner.fmt(f),
            Error::UnsupportedOperationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::error::AddApplicationCloudWatchLoggingOptionError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::AddApplicationCloudWatchLoggingOptionError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::AddApplicationCloudWatchLoggingOptionError> for Error {
    fn from(err: crate::error::AddApplicationCloudWatchLoggingOptionError) -> Self {
        match err.kind {
            crate::error::AddApplicationCloudWatchLoggingOptionErrorKind::ConcurrentModificationException(inner) => Error::ConcurrentModificationException(inner),
            crate::error::AddApplicationCloudWatchLoggingOptionErrorKind::InvalidArgumentException(inner) => Error::InvalidArgumentException(inner),
            crate::error::AddApplicationCloudWatchLoggingOptionErrorKind::ResourceInUseException(inner) => Error::ResourceInUseException(inner),
            crate::error::AddApplicationCloudWatchLoggingOptionErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::AddApplicationCloudWatchLoggingOptionErrorKind::UnsupportedOperationException(inner) => Error::UnsupportedOperationException(inner),
            crate::error::AddApplicationCloudWatchLoggingOptionErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::AddApplicationInputError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::AddApplicationInputError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::AddApplicationInputError> for Error {
    fn from(err: crate::error::AddApplicationInputError) -> Self {
        match err.kind {
            crate::error::AddApplicationInputErrorKind::CodeValidationException(inner) => {
                Error::CodeValidationException(inner)
            }
            crate::error::AddApplicationInputErrorKind::ConcurrentModificationException(inner) => {
                Error::ConcurrentModificationException(inner)
            }
            crate::error::AddApplicationInputErrorKind::InvalidArgumentException(inner) => {
                Error::InvalidArgumentException(inner)
            }
            crate::error::AddApplicationInputErrorKind::ResourceInUseException(inner) => {
                Error::ResourceInUseException(inner)
            }
            crate::error::AddApplicationInputErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::AddApplicationInputErrorKind::UnsupportedOperationException(inner) => {
                Error::UnsupportedOperationException(inner)
            }
            crate::error::AddApplicationInputErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::error::AddApplicationInputProcessingConfigurationError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::AddApplicationInputProcessingConfigurationError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::AddApplicationInputProcessingConfigurationError> for Error {
    fn from(err: crate::error::AddApplicationInputProcessingConfigurationError) -> Self {
        match err.kind {
            crate::error::AddApplicationInputProcessingConfigurationErrorKind::ConcurrentModificationException(inner) => Error::ConcurrentModificationException(inner),
            crate::error::AddApplicationInputProcessingConfigurationErrorKind::InvalidArgumentException(inner) => Error::InvalidArgumentException(inner),
            crate::error::AddApplicationInputProcessingConfigurationErrorKind::ResourceInUseException(inner) => Error::ResourceInUseException(inner),
            crate::error::AddApplicationInputProcessingConfigurationErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::AddApplicationInputProcessingConfigurationErrorKind::UnsupportedOperationException(inner) => Error::UnsupportedOperationException(inner),
            crate::error::AddApplicationInputProcessingConfigurationErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::AddApplicationOutputError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::AddApplicationOutputError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::AddApplicationOutputError> for Error {
    fn from(err: crate::error::AddApplicationOutputError) -> Self {
        match err.kind {
            crate::error::AddApplicationOutputErrorKind::ConcurrentModificationException(inner) => {
                Error::ConcurrentModificationException(inner)
            }
            crate::error::AddApplicationOutputErrorKind::InvalidArgumentException(inner) => {
                Error::InvalidArgumentException(inner)
            }
            crate::error::AddApplicationOutputErrorKind::ResourceInUseException(inner) => {
                Error::ResourceInUseException(inner)
            }
            crate::error::AddApplicationOutputErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::AddApplicationOutputErrorKind::UnsupportedOperationException(inner) => {
                Error::UnsupportedOperationException(inner)
            }
            crate::error::AddApplicationOutputErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::error::AddApplicationReferenceDataSourceError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::AddApplicationReferenceDataSourceError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::AddApplicationReferenceDataSourceError> for Error {
    fn from(err: crate::error::AddApplicationReferenceDataSourceError) -> Self {
        match err.kind {
            crate::error::AddApplicationReferenceDataSourceErrorKind::ConcurrentModificationException(inner) => Error::ConcurrentModificationException(inner),
            crate::error::AddApplicationReferenceDataSourceErrorKind::InvalidArgumentException(inner) => Error::InvalidArgumentException(inner),
            crate::error::AddApplicationReferenceDataSourceErrorKind::ResourceInUseException(inner) => Error::ResourceInUseException(inner),
            crate::error::AddApplicationReferenceDataSourceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::AddApplicationReferenceDataSourceErrorKind::UnsupportedOperationException(inner) => Error::UnsupportedOperationException(inner),
            crate::error::AddApplicationReferenceDataSourceErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateApplicationError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CreateApplicationError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateApplicationError> for Error {
    fn from(err: crate::error::CreateApplicationError) -> Self {
        match err.kind {
            crate::error::CreateApplicationErrorKind::CodeValidationException(inner) => {
                Error::CodeValidationException(inner)
            }
            crate::error::CreateApplicationErrorKind::ConcurrentModificationException(inner) => {
                Error::ConcurrentModificationException(inner)
            }
            crate::error::CreateApplicationErrorKind::InvalidArgumentException(inner) => {
                Error::InvalidArgumentException(inner)
            }
            crate::error::CreateApplicationErrorKind::LimitExceededException(inner) => {
                Error::LimitExceededException(inner)
            }
            crate::error::CreateApplicationErrorKind::ResourceInUseException(inner) => {
                Error::ResourceInUseException(inner)
            }
            crate::error::CreateApplicationErrorKind::TooManyTagsException(inner) => {
                Error::TooManyTagsException(inner)
            }
            crate::error::CreateApplicationErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteApplicationError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteApplicationError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteApplicationError> for Error {
    fn from(err: crate::error::DeleteApplicationError) -> Self {
        match err.kind {
            crate::error::DeleteApplicationErrorKind::ConcurrentModificationException(inner) => {
                Error::ConcurrentModificationException(inner)
            }
            crate::error::DeleteApplicationErrorKind::ResourceInUseException(inner) => {
                Error::ResourceInUseException(inner)
            }
            crate::error::DeleteApplicationErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::DeleteApplicationErrorKind::UnsupportedOperationException(inner) => {
                Error::UnsupportedOperationException(inner)
            }
            crate::error::DeleteApplicationErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::error::DeleteApplicationCloudWatchLoggingOptionError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::DeleteApplicationCloudWatchLoggingOptionError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteApplicationCloudWatchLoggingOptionError> for Error {
    fn from(err: crate::error::DeleteApplicationCloudWatchLoggingOptionError) -> Self {
        match err.kind {
            crate::error::DeleteApplicationCloudWatchLoggingOptionErrorKind::ConcurrentModificationException(inner) => Error::ConcurrentModificationException(inner),
            crate::error::DeleteApplicationCloudWatchLoggingOptionErrorKind::InvalidArgumentException(inner) => Error::InvalidArgumentException(inner),
            crate::error::DeleteApplicationCloudWatchLoggingOptionErrorKind::ResourceInUseException(inner) => Error::ResourceInUseException(inner),
            crate::error::DeleteApplicationCloudWatchLoggingOptionErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DeleteApplicationCloudWatchLoggingOptionErrorKind::UnsupportedOperationException(inner) => Error::UnsupportedOperationException(inner),
            crate::error::DeleteApplicationCloudWatchLoggingOptionErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::error::DeleteApplicationInputProcessingConfigurationError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::DeleteApplicationInputProcessingConfigurationError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteApplicationInputProcessingConfigurationError> for Error {
    fn from(err: crate::error::DeleteApplicationInputProcessingConfigurationError) -> Self {
        match err.kind {
            crate::error::DeleteApplicationInputProcessingConfigurationErrorKind::ConcurrentModificationException(inner) => Error::ConcurrentModificationException(inner),
            crate::error::DeleteApplicationInputProcessingConfigurationErrorKind::InvalidArgumentException(inner) => Error::InvalidArgumentException(inner),
            crate::error::DeleteApplicationInputProcessingConfigurationErrorKind::ResourceInUseException(inner) => Error::ResourceInUseException(inner),
            crate::error::DeleteApplicationInputProcessingConfigurationErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DeleteApplicationInputProcessingConfigurationErrorKind::UnsupportedOperationException(inner) => Error::UnsupportedOperationException(inner),
            crate::error::DeleteApplicationInputProcessingConfigurationErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteApplicationOutputError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteApplicationOutputError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteApplicationOutputError> for Error {
    fn from(err: crate::error::DeleteApplicationOutputError) -> Self {
        match err.kind {
            crate::error::DeleteApplicationOutputErrorKind::ConcurrentModificationException(
                inner,
            ) => Error::ConcurrentModificationException(inner),
            crate::error::DeleteApplicationOutputErrorKind::InvalidArgumentException(inner) => {
                Error::InvalidArgumentException(inner)
            }
            crate::error::DeleteApplicationOutputErrorKind::ResourceInUseException(inner) => {
                Error::ResourceInUseException(inner)
            }
            crate::error::DeleteApplicationOutputErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::DeleteApplicationOutputErrorKind::UnsupportedOperationException(
                inner,
            ) => Error::UnsupportedOperationException(inner),
            crate::error::DeleteApplicationOutputErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::error::DeleteApplicationReferenceDataSourceError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::DeleteApplicationReferenceDataSourceError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteApplicationReferenceDataSourceError> for Error {
    fn from(err: crate::error::DeleteApplicationReferenceDataSourceError) -> Self {
        match err.kind {
            crate::error::DeleteApplicationReferenceDataSourceErrorKind::ConcurrentModificationException(inner) => Error::ConcurrentModificationException(inner),
            crate::error::DeleteApplicationReferenceDataSourceErrorKind::InvalidArgumentException(inner) => Error::InvalidArgumentException(inner),
            crate::error::DeleteApplicationReferenceDataSourceErrorKind::ResourceInUseException(inner) => Error::ResourceInUseException(inner),
            crate::error::DeleteApplicationReferenceDataSourceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DeleteApplicationReferenceDataSourceErrorKind::UnsupportedOperationException(inner) => Error::UnsupportedOperationException(inner),
            crate::error::DeleteApplicationReferenceDataSourceErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeApplicationError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeApplicationError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeApplicationError> for Error {
    fn from(err: crate::error::DescribeApplicationError) -> Self {
        match err.kind {
            crate::error::DescribeApplicationErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::DescribeApplicationErrorKind::UnsupportedOperationException(inner) => {
                Error::UnsupportedOperationException(inner)
            }
            crate::error::DescribeApplicationErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DiscoverInputSchemaError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DiscoverInputSchemaError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DiscoverInputSchemaError> for Error {
    fn from(err: crate::error::DiscoverInputSchemaError) -> Self {
        match err.kind {
            crate::error::DiscoverInputSchemaErrorKind::InvalidArgumentException(inner) => Error::InvalidArgumentException(inner),
            crate::error::DiscoverInputSchemaErrorKind::ResourceProvisionedThroughputExceededException(inner) => Error::ResourceProvisionedThroughputExceededException(inner),
            crate::error::DiscoverInputSchemaErrorKind::ServiceUnavailableException(inner) => Error::ServiceUnavailableException(inner),
            crate::error::DiscoverInputSchemaErrorKind::UnableToDetectSchemaException(inner) => Error::UnableToDetectSchemaException(inner),
            crate::error::DiscoverInputSchemaErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListApplicationsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListApplicationsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListApplicationsError> for Error {
    fn from(err: crate::error::ListApplicationsError) -> Self {
        match err.kind {
            crate::error::ListApplicationsErrorKind::Unhandled(inner) => {
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
            crate::error::ListTagsForResourceErrorKind::ConcurrentModificationException(inner) => {
                Error::ConcurrentModificationException(inner)
            }
            crate::error::ListTagsForResourceErrorKind::InvalidArgumentException(inner) => {
                Error::InvalidArgumentException(inner)
            }
            crate::error::ListTagsForResourceErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::ListTagsForResourceErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StartApplicationError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::StartApplicationError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::StartApplicationError> for Error {
    fn from(err: crate::error::StartApplicationError) -> Self {
        match err.kind {
            crate::error::StartApplicationErrorKind::InvalidApplicationConfigurationException(
                inner,
            ) => Error::InvalidApplicationConfigurationException(inner),
            crate::error::StartApplicationErrorKind::InvalidArgumentException(inner) => {
                Error::InvalidArgumentException(inner)
            }
            crate::error::StartApplicationErrorKind::ResourceInUseException(inner) => {
                Error::ResourceInUseException(inner)
            }
            crate::error::StartApplicationErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::StartApplicationErrorKind::UnsupportedOperationException(inner) => {
                Error::UnsupportedOperationException(inner)
            }
            crate::error::StartApplicationErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StopApplicationError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::StopApplicationError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::StopApplicationError> for Error {
    fn from(err: crate::error::StopApplicationError) -> Self {
        match err.kind {
            crate::error::StopApplicationErrorKind::ResourceInUseException(inner) => {
                Error::ResourceInUseException(inner)
            }
            crate::error::StopApplicationErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::StopApplicationErrorKind::UnsupportedOperationException(inner) => {
                Error::UnsupportedOperationException(inner)
            }
            crate::error::StopApplicationErrorKind::Unhandled(inner) => {
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
            crate::error::TagResourceErrorKind::ConcurrentModificationException(inner) => {
                Error::ConcurrentModificationException(inner)
            }
            crate::error::TagResourceErrorKind::InvalidArgumentException(inner) => {
                Error::InvalidArgumentException(inner)
            }
            crate::error::TagResourceErrorKind::ResourceInUseException(inner) => {
                Error::ResourceInUseException(inner)
            }
            crate::error::TagResourceErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::TagResourceErrorKind::TooManyTagsException(inner) => {
                Error::TooManyTagsException(inner)
            }
            crate::error::TagResourceErrorKind::Unhandled(inner) => {
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
            crate::error::UntagResourceErrorKind::ConcurrentModificationException(inner) => {
                Error::ConcurrentModificationException(inner)
            }
            crate::error::UntagResourceErrorKind::InvalidArgumentException(inner) => {
                Error::InvalidArgumentException(inner)
            }
            crate::error::UntagResourceErrorKind::ResourceInUseException(inner) => {
                Error::ResourceInUseException(inner)
            }
            crate::error::UntagResourceErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::UntagResourceErrorKind::TooManyTagsException(inner) => {
                Error::TooManyTagsException(inner)
            }
            crate::error::UntagResourceErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateApplicationError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdateApplicationError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateApplicationError> for Error {
    fn from(err: crate::error::UpdateApplicationError) -> Self {
        match err.kind {
            crate::error::UpdateApplicationErrorKind::CodeValidationException(inner) => {
                Error::CodeValidationException(inner)
            }
            crate::error::UpdateApplicationErrorKind::ConcurrentModificationException(inner) => {
                Error::ConcurrentModificationException(inner)
            }
            crate::error::UpdateApplicationErrorKind::InvalidArgumentException(inner) => {
                Error::InvalidArgumentException(inner)
            }
            crate::error::UpdateApplicationErrorKind::ResourceInUseException(inner) => {
                Error::ResourceInUseException(inner)
            }
            crate::error::UpdateApplicationErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::UpdateApplicationErrorKind::UnsupportedOperationException(inner) => {
                Error::UnsupportedOperationException(inner)
            }
            crate::error::UpdateApplicationErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl std::error::Error for Error {}
