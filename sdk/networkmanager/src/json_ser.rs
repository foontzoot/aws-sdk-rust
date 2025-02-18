// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_associate_connect_peer_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateConnectPeerInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.connect_peer_id {
        object.key("ConnectPeerId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.device_id {
        object.key("DeviceId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.link_id {
        object.key("LinkId").string(var_3.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_associate_customer_gateway_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateCustomerGatewayInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_4) = &input.customer_gateway_arn {
        object.key("CustomerGatewayArn").string(var_4.as_str());
    }
    if let Some(var_5) = &input.device_id {
        object.key("DeviceId").string(var_5.as_str());
    }
    if let Some(var_6) = &input.link_id {
        object.key("LinkId").string(var_6.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_associate_link_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateLinkInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_7) = &input.device_id {
        object.key("DeviceId").string(var_7.as_str());
    }
    if let Some(var_8) = &input.link_id {
        object.key("LinkId").string(var_8.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_associate_transit_gateway_connect_peer_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateTransitGatewayConnectPeerInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_9) = &input.device_id {
        object.key("DeviceId").string(var_9.as_str());
    }
    if let Some(var_10) = &input.link_id {
        object.key("LinkId").string(var_10.as_str());
    }
    if let Some(var_11) = &input.transit_gateway_connect_peer_arn {
        object
            .key("TransitGatewayConnectPeerArn")
            .string(var_11.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_connect_attachment_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateConnectAttachmentInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_12) = &input.client_token {
        object.key("ClientToken").string(var_12.as_str());
    }
    if let Some(var_13) = &input.core_network_id {
        object.key("CoreNetworkId").string(var_13.as_str());
    }
    if let Some(var_14) = &input.edge_location {
        object.key("EdgeLocation").string(var_14.as_str());
    }
    if let Some(var_15) = &input.options {
        #[allow(unused_mut)]
        let mut object_16 = object.key("Options").start_object();
        crate::json_ser::serialize_structure_crate_model_connect_attachment_options(
            &mut object_16,
            var_15,
        )?;
        object_16.finish();
    }
    if let Some(var_17) = &input.tags {
        let mut array_18 = object.key("Tags").start_array();
        for item_19 in var_17 {
            {
                #[allow(unused_mut)]
                let mut object_20 = array_18.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_20, item_19)?;
                object_20.finish();
            }
        }
        array_18.finish();
    }
    if let Some(var_21) = &input.transport_attachment_id {
        object.key("TransportAttachmentId").string(var_21.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_connection_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateConnectionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_22) = &input.connected_device_id {
        object.key("ConnectedDeviceId").string(var_22.as_str());
    }
    if let Some(var_23) = &input.connected_link_id {
        object.key("ConnectedLinkId").string(var_23.as_str());
    }
    if let Some(var_24) = &input.description {
        object.key("Description").string(var_24.as_str());
    }
    if let Some(var_25) = &input.device_id {
        object.key("DeviceId").string(var_25.as_str());
    }
    if let Some(var_26) = &input.link_id {
        object.key("LinkId").string(var_26.as_str());
    }
    if let Some(var_27) = &input.tags {
        let mut array_28 = object.key("Tags").start_array();
        for item_29 in var_27 {
            {
                #[allow(unused_mut)]
                let mut object_30 = array_28.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_30, item_29)?;
                object_30.finish();
            }
        }
        array_28.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_connect_peer_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateConnectPeerInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_31) = &input.bgp_options {
        #[allow(unused_mut)]
        let mut object_32 = object.key("BgpOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_bgp_options(&mut object_32, var_31)?;
        object_32.finish();
    }
    if let Some(var_33) = &input.client_token {
        object.key("ClientToken").string(var_33.as_str());
    }
    if let Some(var_34) = &input.connect_attachment_id {
        object.key("ConnectAttachmentId").string(var_34.as_str());
    }
    if let Some(var_35) = &input.core_network_address {
        object.key("CoreNetworkAddress").string(var_35.as_str());
    }
    if let Some(var_36) = &input.inside_cidr_blocks {
        let mut array_37 = object.key("InsideCidrBlocks").start_array();
        for item_38 in var_36 {
            {
                array_37.value().string(item_38.as_str());
            }
        }
        array_37.finish();
    }
    if let Some(var_39) = &input.peer_address {
        object.key("PeerAddress").string(var_39.as_str());
    }
    if let Some(var_40) = &input.tags {
        let mut array_41 = object.key("Tags").start_array();
        for item_42 in var_40 {
            {
                #[allow(unused_mut)]
                let mut object_43 = array_41.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_43, item_42)?;
                object_43.finish();
            }
        }
        array_41.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_core_network_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateCoreNetworkInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_44) = &input.client_token {
        object.key("ClientToken").string(var_44.as_str());
    }
    if let Some(var_45) = &input.description {
        object.key("Description").string(var_45.as_str());
    }
    if let Some(var_46) = &input.global_network_id {
        object.key("GlobalNetworkId").string(var_46.as_str());
    }
    if let Some(var_47) = &input.policy_document {
        object.key("PolicyDocument").string(var_47.as_str());
    }
    if let Some(var_48) = &input.tags {
        let mut array_49 = object.key("Tags").start_array();
        for item_50 in var_48 {
            {
                #[allow(unused_mut)]
                let mut object_51 = array_49.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_51, item_50)?;
                object_51.finish();
            }
        }
        array_49.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_device_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDeviceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_52) = &input.aws_location {
        #[allow(unused_mut)]
        let mut object_53 = object.key("AWSLocation").start_object();
        crate::json_ser::serialize_structure_crate_model_aws_location(&mut object_53, var_52)?;
        object_53.finish();
    }
    if let Some(var_54) = &input.description {
        object.key("Description").string(var_54.as_str());
    }
    if let Some(var_55) = &input.location {
        #[allow(unused_mut)]
        let mut object_56 = object.key("Location").start_object();
        crate::json_ser::serialize_structure_crate_model_location(&mut object_56, var_55)?;
        object_56.finish();
    }
    if let Some(var_57) = &input.model {
        object.key("Model").string(var_57.as_str());
    }
    if let Some(var_58) = &input.serial_number {
        object.key("SerialNumber").string(var_58.as_str());
    }
    if let Some(var_59) = &input.site_id {
        object.key("SiteId").string(var_59.as_str());
    }
    if let Some(var_60) = &input.tags {
        let mut array_61 = object.key("Tags").start_array();
        for item_62 in var_60 {
            {
                #[allow(unused_mut)]
                let mut object_63 = array_61.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_63, item_62)?;
                object_63.finish();
            }
        }
        array_61.finish();
    }
    if let Some(var_64) = &input.r#type {
        object.key("Type").string(var_64.as_str());
    }
    if let Some(var_65) = &input.vendor {
        object.key("Vendor").string(var_65.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_global_network_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateGlobalNetworkInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_66) = &input.description {
        object.key("Description").string(var_66.as_str());
    }
    if let Some(var_67) = &input.tags {
        let mut array_68 = object.key("Tags").start_array();
        for item_69 in var_67 {
            {
                #[allow(unused_mut)]
                let mut object_70 = array_68.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_70, item_69)?;
                object_70.finish();
            }
        }
        array_68.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_link_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateLinkInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_71) = &input.bandwidth {
        #[allow(unused_mut)]
        let mut object_72 = object.key("Bandwidth").start_object();
        crate::json_ser::serialize_structure_crate_model_bandwidth(&mut object_72, var_71)?;
        object_72.finish();
    }
    if let Some(var_73) = &input.description {
        object.key("Description").string(var_73.as_str());
    }
    if let Some(var_74) = &input.provider {
        object.key("Provider").string(var_74.as_str());
    }
    if let Some(var_75) = &input.site_id {
        object.key("SiteId").string(var_75.as_str());
    }
    if let Some(var_76) = &input.tags {
        let mut array_77 = object.key("Tags").start_array();
        for item_78 in var_76 {
            {
                #[allow(unused_mut)]
                let mut object_79 = array_77.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_79, item_78)?;
                object_79.finish();
            }
        }
        array_77.finish();
    }
    if let Some(var_80) = &input.r#type {
        object.key("Type").string(var_80.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_site_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateSiteInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_81) = &input.description {
        object.key("Description").string(var_81.as_str());
    }
    if let Some(var_82) = &input.location {
        #[allow(unused_mut)]
        let mut object_83 = object.key("Location").start_object();
        crate::json_ser::serialize_structure_crate_model_location(&mut object_83, var_82)?;
        object_83.finish();
    }
    if let Some(var_84) = &input.tags {
        let mut array_85 = object.key("Tags").start_array();
        for item_86 in var_84 {
            {
                #[allow(unused_mut)]
                let mut object_87 = array_85.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_87, item_86)?;
                object_87.finish();
            }
        }
        array_85.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_site_to_site_vpn_attachment_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateSiteToSiteVpnAttachmentInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_88) = &input.client_token {
        object.key("ClientToken").string(var_88.as_str());
    }
    if let Some(var_89) = &input.core_network_id {
        object.key("CoreNetworkId").string(var_89.as_str());
    }
    if let Some(var_90) = &input.tags {
        let mut array_91 = object.key("Tags").start_array();
        for item_92 in var_90 {
            {
                #[allow(unused_mut)]
                let mut object_93 = array_91.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_93, item_92)?;
                object_93.finish();
            }
        }
        array_91.finish();
    }
    if let Some(var_94) = &input.vpn_connection_arn {
        object.key("VpnConnectionArn").string(var_94.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_transit_gateway_peering_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateTransitGatewayPeeringInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_95) = &input.client_token {
        object.key("ClientToken").string(var_95.as_str());
    }
    if let Some(var_96) = &input.core_network_id {
        object.key("CoreNetworkId").string(var_96.as_str());
    }
    if let Some(var_97) = &input.tags {
        let mut array_98 = object.key("Tags").start_array();
        for item_99 in var_97 {
            {
                #[allow(unused_mut)]
                let mut object_100 = array_98.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_100, item_99)?;
                object_100.finish();
            }
        }
        array_98.finish();
    }
    if let Some(var_101) = &input.transit_gateway_arn {
        object.key("TransitGatewayArn").string(var_101.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_transit_gateway_route_table_attachment_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateTransitGatewayRouteTableAttachmentInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_102) = &input.client_token {
        object.key("ClientToken").string(var_102.as_str());
    }
    if let Some(var_103) = &input.peering_id {
        object.key("PeeringId").string(var_103.as_str());
    }
    if let Some(var_104) = &input.tags {
        let mut array_105 = object.key("Tags").start_array();
        for item_106 in var_104 {
            {
                #[allow(unused_mut)]
                let mut object_107 = array_105.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_107, item_106)?;
                object_107.finish();
            }
        }
        array_105.finish();
    }
    if let Some(var_108) = &input.transit_gateway_route_table_arn {
        object
            .key("TransitGatewayRouteTableArn")
            .string(var_108.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_vpc_attachment_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateVpcAttachmentInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_109) = &input.client_token {
        object.key("ClientToken").string(var_109.as_str());
    }
    if let Some(var_110) = &input.core_network_id {
        object.key("CoreNetworkId").string(var_110.as_str());
    }
    if let Some(var_111) = &input.options {
        #[allow(unused_mut)]
        let mut object_112 = object.key("Options").start_object();
        crate::json_ser::serialize_structure_crate_model_vpc_options(&mut object_112, var_111)?;
        object_112.finish();
    }
    if let Some(var_113) = &input.subnet_arns {
        let mut array_114 = object.key("SubnetArns").start_array();
        for item_115 in var_113 {
            {
                array_114.value().string(item_115.as_str());
            }
        }
        array_114.finish();
    }
    if let Some(var_116) = &input.tags {
        let mut array_117 = object.key("Tags").start_array();
        for item_118 in var_116 {
            {
                #[allow(unused_mut)]
                let mut object_119 = array_117.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_119, item_118)?;
                object_119.finish();
            }
        }
        array_117.finish();
    }
    if let Some(var_120) = &input.vpc_arn {
        object.key("VpcArn").string(var_120.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_network_routes_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetNetworkRoutesInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_121) = &input.destination_filters {
        #[allow(unused_mut)]
        let mut object_122 = object.key("DestinationFilters").start_object();
        for (key_123, value_124) in var_121 {
            {
                let mut array_125 = object_122.key(key_123.as_str()).start_array();
                for item_126 in value_124 {
                    {
                        array_125.value().string(item_126.as_str());
                    }
                }
                array_125.finish();
            }
        }
        object_122.finish();
    }
    if let Some(var_127) = &input.exact_cidr_matches {
        let mut array_128 = object.key("ExactCidrMatches").start_array();
        for item_129 in var_127 {
            {
                array_128.value().string(item_129.as_str());
            }
        }
        array_128.finish();
    }
    if let Some(var_130) = &input.longest_prefix_matches {
        let mut array_131 = object.key("LongestPrefixMatches").start_array();
        for item_132 in var_130 {
            {
                array_131.value().string(item_132.as_str());
            }
        }
        array_131.finish();
    }
    if let Some(var_133) = &input.prefix_list_ids {
        let mut array_134 = object.key("PrefixListIds").start_array();
        for item_135 in var_133 {
            {
                array_134.value().string(item_135.as_str());
            }
        }
        array_134.finish();
    }
    if let Some(var_136) = &input.route_table_identifier {
        #[allow(unused_mut)]
        let mut object_137 = object.key("RouteTableIdentifier").start_object();
        crate::json_ser::serialize_structure_crate_model_route_table_identifier(
            &mut object_137,
            var_136,
        )?;
        object_137.finish();
    }
    if let Some(var_138) = &input.states {
        let mut array_139 = object.key("States").start_array();
        for item_140 in var_138 {
            {
                array_139.value().string(item_140.as_str());
            }
        }
        array_139.finish();
    }
    if let Some(var_141) = &input.subnet_of_matches {
        let mut array_142 = object.key("SubnetOfMatches").start_array();
        for item_143 in var_141 {
            {
                array_142.value().string(item_143.as_str());
            }
        }
        array_142.finish();
    }
    if let Some(var_144) = &input.supernet_of_matches {
        let mut array_145 = object.key("SupernetOfMatches").start_array();
        for item_146 in var_144 {
            {
                array_145.value().string(item_146.as_str());
            }
        }
        array_145.finish();
    }
    if let Some(var_147) = &input.types {
        let mut array_148 = object.key("Types").start_array();
        for item_149 in var_147 {
            {
                array_148.value().string(item_149.as_str());
            }
        }
        array_148.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_core_network_policy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutCoreNetworkPolicyInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_150) = &input.client_token {
        object.key("ClientToken").string(var_150.as_str());
    }
    if let Some(var_151) = &input.description {
        object.key("Description").string(var_151.as_str());
    }
    if let Some(var_152) = &input.latest_version_id {
        object.key("LatestVersionId").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_152).into()),
        );
    }
    if let Some(var_153) = &input.policy_document {
        object.key("PolicyDocument").string(var_153.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_resource_policy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutResourcePolicyInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_154) = &input.policy_document {
        object.key("PolicyDocument").string(var_154.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_register_transit_gateway_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RegisterTransitGatewayInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_155) = &input.transit_gateway_arn {
        object.key("TransitGatewayArn").string(var_155.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_organization_service_access_update_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartOrganizationServiceAccessUpdateInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_156) = &input.action {
        object.key("Action").string(var_156.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_route_analysis_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartRouteAnalysisInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_157) = &input.destination {
        #[allow(unused_mut)]
        let mut object_158 = object.key("Destination").start_object();
        crate::json_ser::serialize_structure_crate_model_route_analysis_endpoint_options_specification(&mut object_158, var_157)?;
        object_158.finish();
    }
    if input.include_return_path {
        object
            .key("IncludeReturnPath")
            .boolean(input.include_return_path);
    }
    if let Some(var_159) = &input.source {
        #[allow(unused_mut)]
        let mut object_160 = object.key("Source").start_object();
        crate::json_ser::serialize_structure_crate_model_route_analysis_endpoint_options_specification(&mut object_160, var_159)?;
        object_160.finish();
    }
    if input.use_middleboxes {
        object.key("UseMiddleboxes").boolean(input.use_middleboxes);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_161) = &input.tags {
        let mut array_162 = object.key("Tags").start_array();
        for item_163 in var_161 {
            {
                #[allow(unused_mut)]
                let mut object_164 = array_162.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_164, item_163)?;
                object_164.finish();
            }
        }
        array_162.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_connection_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateConnectionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_165) = &input.connected_link_id {
        object.key("ConnectedLinkId").string(var_165.as_str());
    }
    if let Some(var_166) = &input.description {
        object.key("Description").string(var_166.as_str());
    }
    if let Some(var_167) = &input.link_id {
        object.key("LinkId").string(var_167.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_core_network_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateCoreNetworkInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_168) = &input.description {
        object.key("Description").string(var_168.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_device_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateDeviceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_169) = &input.aws_location {
        #[allow(unused_mut)]
        let mut object_170 = object.key("AWSLocation").start_object();
        crate::json_ser::serialize_structure_crate_model_aws_location(&mut object_170, var_169)?;
        object_170.finish();
    }
    if let Some(var_171) = &input.description {
        object.key("Description").string(var_171.as_str());
    }
    if let Some(var_172) = &input.location {
        #[allow(unused_mut)]
        let mut object_173 = object.key("Location").start_object();
        crate::json_ser::serialize_structure_crate_model_location(&mut object_173, var_172)?;
        object_173.finish();
    }
    if let Some(var_174) = &input.model {
        object.key("Model").string(var_174.as_str());
    }
    if let Some(var_175) = &input.serial_number {
        object.key("SerialNumber").string(var_175.as_str());
    }
    if let Some(var_176) = &input.site_id {
        object.key("SiteId").string(var_176.as_str());
    }
    if let Some(var_177) = &input.r#type {
        object.key("Type").string(var_177.as_str());
    }
    if let Some(var_178) = &input.vendor {
        object.key("Vendor").string(var_178.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_global_network_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateGlobalNetworkInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_179) = &input.description {
        object.key("Description").string(var_179.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_link_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateLinkInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_180) = &input.bandwidth {
        #[allow(unused_mut)]
        let mut object_181 = object.key("Bandwidth").start_object();
        crate::json_ser::serialize_structure_crate_model_bandwidth(&mut object_181, var_180)?;
        object_181.finish();
    }
    if let Some(var_182) = &input.description {
        object.key("Description").string(var_182.as_str());
    }
    if let Some(var_183) = &input.provider {
        object.key("Provider").string(var_183.as_str());
    }
    if let Some(var_184) = &input.r#type {
        object.key("Type").string(var_184.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_network_resource_metadata_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateNetworkResourceMetadataInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_185) = &input.metadata {
        #[allow(unused_mut)]
        let mut object_186 = object.key("Metadata").start_object();
        for (key_187, value_188) in var_185 {
            {
                object_186.key(key_187.as_str()).string(value_188.as_str());
            }
        }
        object_186.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_site_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateSiteInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_189) = &input.description {
        object.key("Description").string(var_189.as_str());
    }
    if let Some(var_190) = &input.location {
        #[allow(unused_mut)]
        let mut object_191 = object.key("Location").start_object();
        crate::json_ser::serialize_structure_crate_model_location(&mut object_191, var_190)?;
        object_191.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_vpc_attachment_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateVpcAttachmentInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_192) = &input.add_subnet_arns {
        let mut array_193 = object.key("AddSubnetArns").start_array();
        for item_194 in var_192 {
            {
                array_193.value().string(item_194.as_str());
            }
        }
        array_193.finish();
    }
    if let Some(var_195) = &input.options {
        #[allow(unused_mut)]
        let mut object_196 = object.key("Options").start_object();
        crate::json_ser::serialize_structure_crate_model_vpc_options(&mut object_196, var_195)?;
        object_196.finish();
    }
    if let Some(var_197) = &input.remove_subnet_arns {
        let mut array_198 = object.key("RemoveSubnetArns").start_array();
        for item_199 in var_197 {
            {
                array_198.value().string(item_199.as_str());
            }
        }
        array_198.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_connect_attachment_options(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ConnectAttachmentOptions,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_200) = &input.protocol {
        object.key("Protocol").string(var_200.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_201) = &input.key {
        object.key("Key").string(var_201.as_str());
    }
    if let Some(var_202) = &input.value {
        object.key("Value").string(var_202.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_bgp_options(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::BgpOptions,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_203) = &input.peer_asn {
        object.key("PeerAsn").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_203).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_aws_location(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AwsLocation,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_204) = &input.zone {
        object.key("Zone").string(var_204.as_str());
    }
    if let Some(var_205) = &input.subnet_arn {
        object.key("SubnetArn").string(var_205.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_location(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Location,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_206) = &input.address {
        object.key("Address").string(var_206.as_str());
    }
    if let Some(var_207) = &input.latitude {
        object.key("Latitude").string(var_207.as_str());
    }
    if let Some(var_208) = &input.longitude {
        object.key("Longitude").string(var_208.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_bandwidth(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Bandwidth,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_209) = &input.upload_speed {
        object.key("UploadSpeed").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_209).into()),
        );
    }
    if let Some(var_210) = &input.download_speed {
        object.key("DownloadSpeed").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_210).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_vpc_options(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::VpcOptions,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.ipv6_support {
        object.key("Ipv6Support").boolean(input.ipv6_support);
    }
    if input.appliance_mode_support {
        object
            .key("ApplianceModeSupport")
            .boolean(input.appliance_mode_support);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_route_table_identifier(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RouteTableIdentifier,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_211) = &input.transit_gateway_route_table_arn {
        object
            .key("TransitGatewayRouteTableArn")
            .string(var_211.as_str());
    }
    if let Some(var_212) = &input.core_network_segment_edge {
        #[allow(unused_mut)]
        let mut object_213 = object.key("CoreNetworkSegmentEdge").start_object();
        crate::json_ser::serialize_structure_crate_model_core_network_segment_edge_identifier(
            &mut object_213,
            var_212,
        )?;
        object_213.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_route_analysis_endpoint_options_specification(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RouteAnalysisEndpointOptionsSpecification,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_214) = &input.transit_gateway_attachment_arn {
        object
            .key("TransitGatewayAttachmentArn")
            .string(var_214.as_str());
    }
    if let Some(var_215) = &input.ip_address {
        object.key("IpAddress").string(var_215.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_core_network_segment_edge_identifier(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CoreNetworkSegmentEdgeIdentifier,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_216) = &input.core_network_id {
        object.key("CoreNetworkId").string(var_216.as_str());
    }
    if let Some(var_217) = &input.segment_name {
        object.key("SegmentName").string(var_217.as_str());
    }
    if let Some(var_218) = &input.edge_location {
        object.key("EdgeLocation").string(var_218.as_str());
    }
    Ok(())
}
