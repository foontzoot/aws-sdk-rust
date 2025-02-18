// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p> The AWS user account does not have permission to perform the action. Check the AWS Identity and Access Management (IAM) policy associated with this account.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p> Exception to indicate that there is an ongoing task when a new task is created. Return when once the existing tasks are complete. </p>
    ConflictException(crate::error::ConflictException),
    /// <p>Dependency encountered an error.</p>
    DependencyException(crate::error::DependencyException),
    /// <p> The server experienced an internal error. Try again. </p>
    InternalServerException(crate::error::InternalServerException),
    /// <p> The specified ID in the request is not found. </p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p> Exception to indicate that the service-linked role (SLR) is locked. </p>
    ServiceLinkedRoleLockClientException(crate::error::ServiceLinkedRoleLockClientException),
    /// <p> The AWS account has reached its quota of imports. Contact AWS Support to increase the quota for this account. </p>
    ServiceQuotaExceededException(crate::error::ServiceQuotaExceededException),
    /// <p> The request was denied due to request throttling. </p>
    ThrottlingException(crate::error::ThrottlingException),
    /// <p> The request body isn't valid. </p>
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
            Error::DependencyException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceLinkedRoleLockClientException(inner) => inner.fmt(f),
            Error::ServiceQuotaExceededException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::error::GetApplicationComponentDetailsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::GetApplicationComponentDetailsError,
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
impl From<crate::error::GetApplicationComponentDetailsError> for Error {
    fn from(err: crate::error::GetApplicationComponentDetailsError) -> Self {
        match err.kind {
            crate::error::GetApplicationComponentDetailsErrorKind::InternalServerException(
                inner,
            ) => Error::InternalServerException(inner),
            crate::error::GetApplicationComponentDetailsErrorKind::ResourceNotFoundException(
                inner,
            ) => Error::ResourceNotFoundException(inner),
            crate::error::GetApplicationComponentDetailsErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::GetApplicationComponentDetailsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::error::GetApplicationComponentStrategiesError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::GetApplicationComponentStrategiesError,
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
impl From<crate::error::GetApplicationComponentStrategiesError> for Error {
    fn from(err: crate::error::GetApplicationComponentStrategiesError) -> Self {
        match err.kind {
            crate::error::GetApplicationComponentStrategiesErrorKind::InternalServerException(
                inner,
            ) => Error::InternalServerException(inner),
            crate::error::GetApplicationComponentStrategiesErrorKind::ResourceNotFoundException(
                inner,
            ) => Error::ResourceNotFoundException(inner),
            crate::error::GetApplicationComponentStrategiesErrorKind::ThrottlingException(
                inner,
            ) => Error::ThrottlingException(inner),
            crate::error::GetApplicationComponentStrategiesErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetAssessmentError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetAssessmentError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetAssessmentError> for Error {
    fn from(err: crate::error::GetAssessmentError) -> Self {
        match err.kind {
            crate::error::GetAssessmentErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::GetAssessmentErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::GetAssessmentErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::GetAssessmentErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::GetAssessmentErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetImportFileTaskError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetImportFileTaskError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetImportFileTaskError> for Error {
    fn from(err: crate::error::GetImportFileTaskError) -> Self {
        match err.kind {
            crate::error::GetImportFileTaskErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::GetImportFileTaskErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::GetImportFileTaskErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::GetImportFileTaskErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::GetImportFileTaskErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::GetImportFileTaskErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetLatestAssessmentIdError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetLatestAssessmentIdError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetLatestAssessmentIdError> for Error {
    fn from(err: crate::error::GetLatestAssessmentIdError) -> Self {
        match err.kind {
            crate::error::GetLatestAssessmentIdErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::GetLatestAssessmentIdErrorKind::DependencyException(inner) => {
                Error::DependencyException(inner)
            }
            crate::error::GetLatestAssessmentIdErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::GetLatestAssessmentIdErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::GetLatestAssessmentIdErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetPortfolioPreferencesError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetPortfolioPreferencesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetPortfolioPreferencesError> for Error {
    fn from(err: crate::error::GetPortfolioPreferencesError) -> Self {
        match err.kind {
            crate::error::GetPortfolioPreferencesErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::GetPortfolioPreferencesErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::GetPortfolioPreferencesErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::GetPortfolioPreferencesErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::GetPortfolioPreferencesErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetPortfolioSummaryError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetPortfolioSummaryError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetPortfolioSummaryError> for Error {
    fn from(err: crate::error::GetPortfolioSummaryError) -> Self {
        match err.kind {
            crate::error::GetPortfolioSummaryErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::GetPortfolioSummaryErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::GetPortfolioSummaryErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::GetPortfolioSummaryErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::error::GetRecommendationReportDetailsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::GetRecommendationReportDetailsError,
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
impl From<crate::error::GetRecommendationReportDetailsError> for Error {
    fn from(err: crate::error::GetRecommendationReportDetailsError) -> Self {
        match err.kind {
            crate::error::GetRecommendationReportDetailsErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::GetRecommendationReportDetailsErrorKind::InternalServerException(
                inner,
            ) => Error::InternalServerException(inner),
            crate::error::GetRecommendationReportDetailsErrorKind::ResourceNotFoundException(
                inner,
            ) => Error::ResourceNotFoundException(inner),
            crate::error::GetRecommendationReportDetailsErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::GetRecommendationReportDetailsErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::GetRecommendationReportDetailsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetServerDetailsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetServerDetailsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetServerDetailsError> for Error {
    fn from(err: crate::error::GetServerDetailsError) -> Self {
        match err.kind {
            crate::error::GetServerDetailsErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::GetServerDetailsErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::GetServerDetailsErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::GetServerDetailsErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::GetServerDetailsErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::GetServerDetailsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetServerStrategiesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetServerStrategiesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetServerStrategiesError> for Error {
    fn from(err: crate::error::GetServerStrategiesError) -> Self {
        match err.kind {
            crate::error::GetServerStrategiesErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::GetServerStrategiesErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::GetServerStrategiesErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::GetServerStrategiesErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::GetServerStrategiesErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::GetServerStrategiesErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListApplicationComponentsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListApplicationComponentsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListApplicationComponentsError> for Error {
    fn from(err: crate::error::ListApplicationComponentsError) -> Self {
        match err.kind {
            crate::error::ListApplicationComponentsErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::ListApplicationComponentsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::ListApplicationComponentsErrorKind::ServiceLinkedRoleLockClientException(inner) => Error::ServiceLinkedRoleLockClientException(inner),
            crate::error::ListApplicationComponentsErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::ListApplicationComponentsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListCollectorsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListCollectorsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListCollectorsError> for Error {
    fn from(err: crate::error::ListCollectorsError) -> Self {
        match err.kind {
            crate::error::ListCollectorsErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::ListCollectorsErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::ListCollectorsErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::ListCollectorsErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::ListCollectorsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListImportFileTaskError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListImportFileTaskError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListImportFileTaskError> for Error {
    fn from(err: crate::error::ListImportFileTaskError) -> Self {
        match err.kind {
            crate::error::ListImportFileTaskErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::ListImportFileTaskErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::ListImportFileTaskErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::ListImportFileTaskErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::ListImportFileTaskErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListServersError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListServersError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListServersError> for Error {
    fn from(err: crate::error::ListServersError) -> Self {
        match err.kind {
            crate::error::ListServersErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::ListServersErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::ListServersErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::ListServersErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::ListServersErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PutPortfolioPreferencesError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::PutPortfolioPreferencesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::PutPortfolioPreferencesError> for Error {
    fn from(err: crate::error::PutPortfolioPreferencesError) -> Self {
        match err.kind {
            crate::error::PutPortfolioPreferencesErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::PutPortfolioPreferencesErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::PutPortfolioPreferencesErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::PutPortfolioPreferencesErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::PutPortfolioPreferencesErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::PutPortfolioPreferencesErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StartAssessmentError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::StartAssessmentError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::StartAssessmentError> for Error {
    fn from(err: crate::error::StartAssessmentError) -> Self {
        match err.kind {
            crate::error::StartAssessmentErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::StartAssessmentErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::StartAssessmentErrorKind::ServiceQuotaExceededException(inner) => {
                Error::ServiceQuotaExceededException(inner)
            }
            crate::error::StartAssessmentErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::StartAssessmentErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StartImportFileTaskError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::StartImportFileTaskError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::StartImportFileTaskError> for Error {
    fn from(err: crate::error::StartImportFileTaskError) -> Self {
        match err.kind {
            crate::error::StartImportFileTaskErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::StartImportFileTaskErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::StartImportFileTaskErrorKind::ServiceQuotaExceededException(inner) => {
                Error::ServiceQuotaExceededException(inner)
            }
            crate::error::StartImportFileTaskErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::StartImportFileTaskErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::StartImportFileTaskErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::error::StartRecommendationReportGenerationError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::StartRecommendationReportGenerationError,
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
impl From<crate::error::StartRecommendationReportGenerationError> for Error {
    fn from(err: crate::error::StartRecommendationReportGenerationError) -> Self {
        match err.kind {
            crate::error::StartRecommendationReportGenerationErrorKind::AccessDeniedException(
                inner,
            ) => Error::AccessDeniedException(inner),
            crate::error::StartRecommendationReportGenerationErrorKind::ConflictException(
                inner,
            ) => Error::ConflictException(inner),
            crate::error::StartRecommendationReportGenerationErrorKind::InternalServerException(
                inner,
            ) => Error::InternalServerException(inner),
            crate::error::StartRecommendationReportGenerationErrorKind::ThrottlingException(
                inner,
            ) => Error::ThrottlingException(inner),
            crate::error::StartRecommendationReportGenerationErrorKind::ValidationException(
                inner,
            ) => Error::ValidationException(inner),
            crate::error::StartRecommendationReportGenerationErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StopAssessmentError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::StopAssessmentError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::StopAssessmentError> for Error {
    fn from(err: crate::error::StopAssessmentError) -> Self {
        match err.kind {
            crate::error::StopAssessmentErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::StopAssessmentErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::StopAssessmentErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::StopAssessmentErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::StopAssessmentErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::error::UpdateApplicationComponentConfigError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::UpdateApplicationComponentConfigError,
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
impl From<crate::error::UpdateApplicationComponentConfigError> for Error {
    fn from(err: crate::error::UpdateApplicationComponentConfigError) -> Self {
        match err.kind {
            crate::error::UpdateApplicationComponentConfigErrorKind::InternalServerException(
                inner,
            ) => Error::InternalServerException(inner),
            crate::error::UpdateApplicationComponentConfigErrorKind::ResourceNotFoundException(
                inner,
            ) => Error::ResourceNotFoundException(inner),
            crate::error::UpdateApplicationComponentConfigErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::UpdateApplicationComponentConfigErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::UpdateApplicationComponentConfigErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateServerConfigError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdateServerConfigError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateServerConfigError> for Error {
    fn from(err: crate::error::UpdateServerConfigError) -> Self {
        match err.kind {
            crate::error::UpdateServerConfigErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::UpdateServerConfigErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::UpdateServerConfigErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::UpdateServerConfigErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::UpdateServerConfigErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl std::error::Error for Error {}
