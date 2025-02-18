// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_rescore_execution_plan_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateRescoreExecutionPlanInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("Description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.capacity_units {
        #[allow(unused_mut)]
        let mut object_4 = object.key("CapacityUnits").start_object();
        crate::json_ser::serialize_structure_crate_model_capacity_units_configuration(
            &mut object_4,
            var_3,
        )?;
        object_4.finish();
    }
    if let Some(var_5) = &input.tags {
        let mut array_6 = object.key("Tags").start_array();
        for item_7 in var_5 {
            {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.client_token {
        object.key("ClientToken").string(var_9.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_rescore_execution_plan_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteRescoreExecutionPlanInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_10) = &input.id {
        object.key("Id").string(var_10.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_rescore_execution_plan_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeRescoreExecutionPlanInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_11) = &input.id {
        object.key("Id").string(var_11.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_rescore_execution_plans_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListRescoreExecutionPlansInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_12) = &input.next_token {
        object.key("NextToken").string(var_12.as_str());
    }
    if let Some(var_13) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_13).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_for_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsForResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_14) = &input.resource_arn {
        object.key("ResourceARN").string(var_14.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_rescore_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RescoreInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_15) = &input.rescore_execution_plan_id {
        object.key("RescoreExecutionPlanId").string(var_15.as_str());
    }
    if let Some(var_16) = &input.search_query {
        object.key("SearchQuery").string(var_16.as_str());
    }
    if let Some(var_17) = &input.documents {
        let mut array_18 = object.key("Documents").start_array();
        for item_19 in var_17 {
            {
                #[allow(unused_mut)]
                let mut object_20 = array_18.value().start_object();
                crate::json_ser::serialize_structure_crate_model_document(&mut object_20, item_19)?;
                object_20.finish();
            }
        }
        array_18.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_21) = &input.resource_arn {
        object.key("ResourceARN").string(var_21.as_str());
    }
    if let Some(var_22) = &input.tags {
        let mut array_23 = object.key("Tags").start_array();
        for item_24 in var_22 {
            {
                #[allow(unused_mut)]
                let mut object_25 = array_23.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_25, item_24)?;
                object_25.finish();
            }
        }
        array_23.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_untag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_26) = &input.resource_arn {
        object.key("ResourceARN").string(var_26.as_str());
    }
    if let Some(var_27) = &input.tag_keys {
        let mut array_28 = object.key("TagKeys").start_array();
        for item_29 in var_27 {
            {
                array_28.value().string(item_29.as_str());
            }
        }
        array_28.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_rescore_execution_plan_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateRescoreExecutionPlanInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_30) = &input.id {
        object.key("Id").string(var_30.as_str());
    }
    if let Some(var_31) = &input.name {
        object.key("Name").string(var_31.as_str());
    }
    if let Some(var_32) = &input.description {
        object.key("Description").string(var_32.as_str());
    }
    if let Some(var_33) = &input.capacity_units {
        #[allow(unused_mut)]
        let mut object_34 = object.key("CapacityUnits").start_object();
        crate::json_ser::serialize_structure_crate_model_capacity_units_configuration(
            &mut object_34,
            var_33,
        )?;
        object_34.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_capacity_units_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CapacityUnitsConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_35) = &input.rescore_capacity_units {
        object.key("RescoreCapacityUnits").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_35).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_36) = &input.key {
        object.key("Key").string(var_36.as_str());
    }
    if let Some(var_37) = &input.value {
        object.key("Value").string(var_37.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_document(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Document,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_38) = &input.id {
        object.key("Id").string(var_38.as_str());
    }
    if let Some(var_39) = &input.group_id {
        object.key("GroupId").string(var_39.as_str());
    }
    if let Some(var_40) = &input.title {
        object.key("Title").string(var_40.as_str());
    }
    if let Some(var_41) = &input.body {
        object.key("Body").string(var_41.as_str());
    }
    if let Some(var_42) = &input.tokenized_title {
        let mut array_43 = object.key("TokenizedTitle").start_array();
        for item_44 in var_42 {
            {
                array_43.value().string(item_44.as_str());
            }
        }
        array_43.finish();
    }
    if let Some(var_45) = &input.tokenized_body {
        let mut array_46 = object.key("TokenizedBody").start_array();
        for item_47 in var_45 {
            {
                array_46.value().string(item_47.as_str());
            }
        }
        array_46.finish();
    }
    if let Some(var_48) = &input.original_score {
        object.key("OriginalScore").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((*var_48).into()),
        );
    }
    Ok(())
}
