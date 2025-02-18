// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `BatchAcknowledgeAlarm`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`batch_acknowledge_alarm`](crate::client::Client::batch_acknowledge_alarm).
///
/// See [`crate::client::fluent_builders::BatchAcknowledgeAlarm`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct BatchAcknowledgeAlarm {
    _private: (),
}
impl BatchAcknowledgeAlarm {
    /// Creates a new builder-style object to manufacture [`BatchAcknowledgeAlarmInput`](crate::input::BatchAcknowledgeAlarmInput).
    pub fn builder() -> crate::input::batch_acknowledge_alarm_input::Builder {
        crate::input::batch_acknowledge_alarm_input::Builder::default()
    }
    /// Creates a new `BatchAcknowledgeAlarm` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BatchAcknowledgeAlarm {
    type Output = std::result::Result<
        crate::output::BatchAcknowledgeAlarmOutput,
        crate::error::BatchAcknowledgeAlarmError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 202 {
            crate::operation_deser::parse_batch_acknowledge_alarm_error(response)
        } else {
            crate::operation_deser::parse_batch_acknowledge_alarm_response(response)
        }
    }
}

/// Operation shape for `BatchDeleteDetector`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`batch_delete_detector`](crate::client::Client::batch_delete_detector).
///
/// See [`crate::client::fluent_builders::BatchDeleteDetector`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct BatchDeleteDetector {
    _private: (),
}
impl BatchDeleteDetector {
    /// Creates a new builder-style object to manufacture [`BatchDeleteDetectorInput`](crate::input::BatchDeleteDetectorInput).
    pub fn builder() -> crate::input::batch_delete_detector_input::Builder {
        crate::input::batch_delete_detector_input::Builder::default()
    }
    /// Creates a new `BatchDeleteDetector` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BatchDeleteDetector {
    type Output = std::result::Result<
        crate::output::BatchDeleteDetectorOutput,
        crate::error::BatchDeleteDetectorError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_batch_delete_detector_error(response)
        } else {
            crate::operation_deser::parse_batch_delete_detector_response(response)
        }
    }
}

/// Operation shape for `BatchDisableAlarm`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`batch_disable_alarm`](crate::client::Client::batch_disable_alarm).
///
/// See [`crate::client::fluent_builders::BatchDisableAlarm`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct BatchDisableAlarm {
    _private: (),
}
impl BatchDisableAlarm {
    /// Creates a new builder-style object to manufacture [`BatchDisableAlarmInput`](crate::input::BatchDisableAlarmInput).
    pub fn builder() -> crate::input::batch_disable_alarm_input::Builder {
        crate::input::batch_disable_alarm_input::Builder::default()
    }
    /// Creates a new `BatchDisableAlarm` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BatchDisableAlarm {
    type Output = std::result::Result<
        crate::output::BatchDisableAlarmOutput,
        crate::error::BatchDisableAlarmError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 202 {
            crate::operation_deser::parse_batch_disable_alarm_error(response)
        } else {
            crate::operation_deser::parse_batch_disable_alarm_response(response)
        }
    }
}

/// Operation shape for `BatchEnableAlarm`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`batch_enable_alarm`](crate::client::Client::batch_enable_alarm).
///
/// See [`crate::client::fluent_builders::BatchEnableAlarm`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct BatchEnableAlarm {
    _private: (),
}
impl BatchEnableAlarm {
    /// Creates a new builder-style object to manufacture [`BatchEnableAlarmInput`](crate::input::BatchEnableAlarmInput).
    pub fn builder() -> crate::input::batch_enable_alarm_input::Builder {
        crate::input::batch_enable_alarm_input::Builder::default()
    }
    /// Creates a new `BatchEnableAlarm` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BatchEnableAlarm {
    type Output = std::result::Result<
        crate::output::BatchEnableAlarmOutput,
        crate::error::BatchEnableAlarmError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 202 {
            crate::operation_deser::parse_batch_enable_alarm_error(response)
        } else {
            crate::operation_deser::parse_batch_enable_alarm_response(response)
        }
    }
}

/// Operation shape for `BatchPutMessage`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`batch_put_message`](crate::client::Client::batch_put_message).
///
/// See [`crate::client::fluent_builders::BatchPutMessage`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct BatchPutMessage {
    _private: (),
}
impl BatchPutMessage {
    /// Creates a new builder-style object to manufacture [`BatchPutMessageInput`](crate::input::BatchPutMessageInput).
    pub fn builder() -> crate::input::batch_put_message_input::Builder {
        crate::input::batch_put_message_input::Builder::default()
    }
    /// Creates a new `BatchPutMessage` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BatchPutMessage {
    type Output = std::result::Result<
        crate::output::BatchPutMessageOutput,
        crate::error::BatchPutMessageError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_batch_put_message_error(response)
        } else {
            crate::operation_deser::parse_batch_put_message_response(response)
        }
    }
}

/// Operation shape for `BatchResetAlarm`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`batch_reset_alarm`](crate::client::Client::batch_reset_alarm).
///
/// See [`crate::client::fluent_builders::BatchResetAlarm`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct BatchResetAlarm {
    _private: (),
}
impl BatchResetAlarm {
    /// Creates a new builder-style object to manufacture [`BatchResetAlarmInput`](crate::input::BatchResetAlarmInput).
    pub fn builder() -> crate::input::batch_reset_alarm_input::Builder {
        crate::input::batch_reset_alarm_input::Builder::default()
    }
    /// Creates a new `BatchResetAlarm` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BatchResetAlarm {
    type Output = std::result::Result<
        crate::output::BatchResetAlarmOutput,
        crate::error::BatchResetAlarmError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 202 {
            crate::operation_deser::parse_batch_reset_alarm_error(response)
        } else {
            crate::operation_deser::parse_batch_reset_alarm_response(response)
        }
    }
}

/// Operation shape for `BatchSnoozeAlarm`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`batch_snooze_alarm`](crate::client::Client::batch_snooze_alarm).
///
/// See [`crate::client::fluent_builders::BatchSnoozeAlarm`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct BatchSnoozeAlarm {
    _private: (),
}
impl BatchSnoozeAlarm {
    /// Creates a new builder-style object to manufacture [`BatchSnoozeAlarmInput`](crate::input::BatchSnoozeAlarmInput).
    pub fn builder() -> crate::input::batch_snooze_alarm_input::Builder {
        crate::input::batch_snooze_alarm_input::Builder::default()
    }
    /// Creates a new `BatchSnoozeAlarm` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BatchSnoozeAlarm {
    type Output = std::result::Result<
        crate::output::BatchSnoozeAlarmOutput,
        crate::error::BatchSnoozeAlarmError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 202 {
            crate::operation_deser::parse_batch_snooze_alarm_error(response)
        } else {
            crate::operation_deser::parse_batch_snooze_alarm_response(response)
        }
    }
}

/// Operation shape for `BatchUpdateDetector`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`batch_update_detector`](crate::client::Client::batch_update_detector).
///
/// See [`crate::client::fluent_builders::BatchUpdateDetector`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct BatchUpdateDetector {
    _private: (),
}
impl BatchUpdateDetector {
    /// Creates a new builder-style object to manufacture [`BatchUpdateDetectorInput`](crate::input::BatchUpdateDetectorInput).
    pub fn builder() -> crate::input::batch_update_detector_input::Builder {
        crate::input::batch_update_detector_input::Builder::default()
    }
    /// Creates a new `BatchUpdateDetector` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BatchUpdateDetector {
    type Output = std::result::Result<
        crate::output::BatchUpdateDetectorOutput,
        crate::error::BatchUpdateDetectorError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_batch_update_detector_error(response)
        } else {
            crate::operation_deser::parse_batch_update_detector_response(response)
        }
    }
}

/// Operation shape for `DescribeAlarm`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_alarm`](crate::client::Client::describe_alarm).
///
/// See [`crate::client::fluent_builders::DescribeAlarm`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeAlarm {
    _private: (),
}
impl DescribeAlarm {
    /// Creates a new builder-style object to manufacture [`DescribeAlarmInput`](crate::input::DescribeAlarmInput).
    pub fn builder() -> crate::input::describe_alarm_input::Builder {
        crate::input::describe_alarm_input::Builder::default()
    }
    /// Creates a new `DescribeAlarm` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeAlarm {
    type Output =
        std::result::Result<crate::output::DescribeAlarmOutput, crate::error::DescribeAlarmError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_alarm_error(response)
        } else {
            crate::operation_deser::parse_describe_alarm_response(response)
        }
    }
}

/// Operation shape for `DescribeDetector`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_detector`](crate::client::Client::describe_detector).
///
/// See [`crate::client::fluent_builders::DescribeDetector`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeDetector {
    _private: (),
}
impl DescribeDetector {
    /// Creates a new builder-style object to manufacture [`DescribeDetectorInput`](crate::input::DescribeDetectorInput).
    pub fn builder() -> crate::input::describe_detector_input::Builder {
        crate::input::describe_detector_input::Builder::default()
    }
    /// Creates a new `DescribeDetector` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeDetector {
    type Output = std::result::Result<
        crate::output::DescribeDetectorOutput,
        crate::error::DescribeDetectorError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_detector_error(response)
        } else {
            crate::operation_deser::parse_describe_detector_response(response)
        }
    }
}

/// Operation shape for `ListAlarms`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_alarms`](crate::client::Client::list_alarms).
///
/// See [`crate::client::fluent_builders::ListAlarms`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListAlarms {
    _private: (),
}
impl ListAlarms {
    /// Creates a new builder-style object to manufacture [`ListAlarmsInput`](crate::input::ListAlarmsInput).
    pub fn builder() -> crate::input::list_alarms_input::Builder {
        crate::input::list_alarms_input::Builder::default()
    }
    /// Creates a new `ListAlarms` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListAlarms {
    type Output =
        std::result::Result<crate::output::ListAlarmsOutput, crate::error::ListAlarmsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_alarms_error(response)
        } else {
            crate::operation_deser::parse_list_alarms_response(response)
        }
    }
}

/// Operation shape for `ListDetectors`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_detectors`](crate::client::Client::list_detectors).
///
/// See [`crate::client::fluent_builders::ListDetectors`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListDetectors {
    _private: (),
}
impl ListDetectors {
    /// Creates a new builder-style object to manufacture [`ListDetectorsInput`](crate::input::ListDetectorsInput).
    pub fn builder() -> crate::input::list_detectors_input::Builder {
        crate::input::list_detectors_input::Builder::default()
    }
    /// Creates a new `ListDetectors` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListDetectors {
    type Output =
        std::result::Result<crate::output::ListDetectorsOutput, crate::error::ListDetectorsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_detectors_error(response)
        } else {
            crate::operation_deser::parse_list_detectors_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
