// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_cluster_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateClusterInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.admin_user_name {
        object.key("adminUserName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.admin_user_password {
        object.key("adminUserPassword").string(var_2.as_str());
    }
    if let Some(var_3) = &input.auth_type {
        object.key("authType").string(var_3.as_str());
    }
    if let Some(var_4) = &input.client_token {
        object.key("clientToken").string(var_4.as_str());
    }
    if let Some(var_5) = &input.cluster_name {
        object.key("clusterName").string(var_5.as_str());
    }
    if let Some(var_6) = &input.kms_key_id {
        object.key("kmsKeyId").string(var_6.as_str());
    }
    if let Some(var_7) = &input.preferred_maintenance_window {
        object
            .key("preferredMaintenanceWindow")
            .string(var_7.as_str());
    }
    if let Some(var_8) = &input.shard_capacity {
        object.key("shardCapacity").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_8).into()),
        );
    }
    if let Some(var_9) = &input.shard_count {
        object.key("shardCount").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_9).into()),
        );
    }
    if let Some(var_10) = &input.subnet_ids {
        let mut array_11 = object.key("subnetIds").start_array();
        for item_12 in var_10 {
            {
                array_11.value().string(item_12.as_str());
            }
        }
        array_11.finish();
    }
    if let Some(var_13) = &input.tags {
        #[allow(unused_mut)]
        let mut object_14 = object.key("tags").start_object();
        for (key_15, value_16) in var_13 {
            {
                object_14.key(key_15.as_str()).string(value_16.as_str());
            }
        }
        object_14.finish();
    }
    if let Some(var_17) = &input.vpc_security_group_ids {
        let mut array_18 = object.key("vpcSecurityGroupIds").start_array();
        for item_19 in var_17 {
            {
                array_18.value().string(item_19.as_str());
            }
        }
        array_18.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_cluster_snapshot_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateClusterSnapshotInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_20) = &input.cluster_arn {
        object.key("clusterArn").string(var_20.as_str());
    }
    if let Some(var_21) = &input.snapshot_name {
        object.key("snapshotName").string(var_21.as_str());
    }
    if let Some(var_22) = &input.tags {
        #[allow(unused_mut)]
        let mut object_23 = object.key("tags").start_object();
        for (key_24, value_25) in var_22 {
            {
                object_23.key(key_24.as_str()).string(value_25.as_str());
            }
        }
        object_23.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_restore_cluster_from_snapshot_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RestoreClusterFromSnapshotInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_26) = &input.cluster_name {
        object.key("clusterName").string(var_26.as_str());
    }
    if let Some(var_27) = &input.kms_key_id {
        object.key("kmsKeyId").string(var_27.as_str());
    }
    if let Some(var_28) = &input.subnet_ids {
        let mut array_29 = object.key("subnetIds").start_array();
        for item_30 in var_28 {
            {
                array_29.value().string(item_30.as_str());
            }
        }
        array_29.finish();
    }
    if let Some(var_31) = &input.tags {
        #[allow(unused_mut)]
        let mut object_32 = object.key("tags").start_object();
        for (key_33, value_34) in var_31 {
            {
                object_32.key(key_33.as_str()).string(value_34.as_str());
            }
        }
        object_32.finish();
    }
    if let Some(var_35) = &input.vpc_security_group_ids {
        let mut array_36 = object.key("vpcSecurityGroupIds").start_array();
        for item_37 in var_35 {
            {
                array_36.value().string(item_37.as_str());
            }
        }
        array_36.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_38) = &input.tags {
        #[allow(unused_mut)]
        let mut object_39 = object.key("tags").start_object();
        for (key_40, value_41) in var_38 {
            {
                object_39.key(key_40.as_str()).string(value_41.as_str());
            }
        }
        object_39.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_cluster_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateClusterInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_42) = &input.admin_user_password {
        object.key("adminUserPassword").string(var_42.as_str());
    }
    if let Some(var_43) = &input.auth_type {
        object.key("authType").string(var_43.as_str());
    }
    if let Some(var_44) = &input.client_token {
        object.key("clientToken").string(var_44.as_str());
    }
    if let Some(var_45) = &input.preferred_maintenance_window {
        object
            .key("preferredMaintenanceWindow")
            .string(var_45.as_str());
    }
    if let Some(var_46) = &input.shard_capacity {
        object.key("shardCapacity").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_46).into()),
        );
    }
    if let Some(var_47) = &input.shard_count {
        object.key("shardCount").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_47).into()),
        );
    }
    if let Some(var_48) = &input.subnet_ids {
        let mut array_49 = object.key("subnetIds").start_array();
        for item_50 in var_48 {
            {
                array_49.value().string(item_50.as_str());
            }
        }
        array_49.finish();
    }
    if let Some(var_51) = &input.vpc_security_group_ids {
        let mut array_52 = object.key("vpcSecurityGroupIds").start_array();
        for item_53 in var_51 {
            {
                array_52.value().string(item_53.as_str());
            }
        }
        array_52.finish();
    }
    Ok(())
}
