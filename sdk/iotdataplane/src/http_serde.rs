// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn add_headers_publish(
    input: &crate::input::PublishInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_1) = &input.user_properties {
        let formatted_2 = aws_smithy_types::base64::encode(&inner_1);
        if !formatted_2.is_empty() {
            let header_value = formatted_2;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::error::BuildError::invalid_field(
                        "user_properties",
                        format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    )
                })?;
            builder = builder.header("x-amz-mqtt5-user-properties", header_value);
        }
    }
    if let Some(inner_3) = &input.payload_format_indicator {
        let formatted_4 = inner_3.as_str();
        if !formatted_4.is_empty() {
            let header_value = formatted_4;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::error::BuildError::invalid_field(
                        "payload_format_indicator",
                        format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    )
                })?;
            builder = builder.header("x-amz-mqtt5-payload-format-indicator", header_value);
        }
    }
    if let Some(inner_5) = &input.correlation_data {
        let formatted_6 = inner_5.as_str();
        if !formatted_6.is_empty() {
            let header_value = formatted_6;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::error::BuildError::invalid_field(
                        "correlation_data",
                        format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    )
                })?;
            builder = builder.header("x-amz-mqtt5-correlation-data", header_value);
        }
    }
    Ok(builder)
}

pub fn deser_payload_delete_thing_shadow_delete_thing_shadow_output_payload(
    body: &[u8],
) -> std::result::Result<
    std::option::Option<aws_smithy_types::Blob>,
    crate::error::DeleteThingShadowError,
> {
    (!body.is_empty())
        .then(|| Ok(aws_smithy_types::Blob::new(body)))
        .transpose()
}

pub fn deser_payload_get_thing_shadow_get_thing_shadow_output_payload(
    body: &[u8],
) -> std::result::Result<
    std::option::Option<aws_smithy_types::Blob>,
    crate::error::GetThingShadowError,
> {
    (!body.is_empty())
        .then(|| Ok(aws_smithy_types::Blob::new(body)))
        .transpose()
}

pub fn deser_payload_update_thing_shadow_update_thing_shadow_output_payload(
    body: &[u8],
) -> std::result::Result<
    std::option::Option<aws_smithy_types::Blob>,
    crate::error::UpdateThingShadowError,
> {
    (!body.is_empty())
        .then(|| Ok(aws_smithy_types::Blob::new(body)))
        .transpose()
}
