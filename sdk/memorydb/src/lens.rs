// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_describe_ac_ls_output_next_token(
    input: &crate::output::DescribeAcLsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_clusters_output_next_token(
    input: &crate::output::DescribeClustersOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_engine_versions_output_next_token(
    input: &crate::output::DescribeEngineVersionsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_events_output_next_token(
    input: &crate::output::DescribeEventsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_parameter_groups_output_next_token(
    input: &crate::output::DescribeParameterGroupsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_parameters_output_next_token(
    input: &crate::output::DescribeParametersOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_reserved_nodes_output_next_token(
    input: &crate::output::DescribeReservedNodesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_reserved_nodes_offerings_output_next_token(
    input: &crate::output::DescribeReservedNodesOfferingsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_service_updates_output_next_token(
    input: &crate::output::DescribeServiceUpdatesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_snapshots_output_next_token(
    input: &crate::output::DescribeSnapshotsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_subnet_groups_output_next_token(
    input: &crate::output::DescribeSubnetGroupsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_users_output_next_token(
    input: &crate::output::DescribeUsersOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_ac_ls_output_ac_ls(
    input: crate::output::DescribeAcLsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::Acl>> {
    let input = match input.ac_ls {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_clusters_output_clusters(
    input: crate::output::DescribeClustersOutput,
) -> std::option::Option<std::vec::Vec<crate::model::Cluster>> {
    let input = match input.clusters {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_engine_versions_output_engine_versions(
    input: crate::output::DescribeEngineVersionsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::EngineVersionInfo>> {
    let input = match input.engine_versions {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_events_output_events(
    input: crate::output::DescribeEventsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::Event>> {
    let input = match input.events {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_parameter_groups_output_parameter_groups(
    input: crate::output::DescribeParameterGroupsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::ParameterGroup>> {
    let input = match input.parameter_groups {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_parameters_output_parameters(
    input: crate::output::DescribeParametersOutput,
) -> std::option::Option<std::vec::Vec<crate::model::Parameter>> {
    let input = match input.parameters {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_reserved_nodes_output_reserved_nodes(
    input: crate::output::DescribeReservedNodesOutput,
) -> std::option::Option<std::vec::Vec<crate::model::ReservedNode>> {
    let input = match input.reserved_nodes {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_reserved_nodes_offerings_output_reserved_nodes_offerings(
    input: crate::output::DescribeReservedNodesOfferingsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::ReservedNodesOffering>> {
    let input = match input.reserved_nodes_offerings {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_service_updates_output_service_updates(
    input: crate::output::DescribeServiceUpdatesOutput,
) -> std::option::Option<std::vec::Vec<crate::model::ServiceUpdate>> {
    let input = match input.service_updates {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_snapshots_output_snapshots(
    input: crate::output::DescribeSnapshotsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::Snapshot>> {
    let input = match input.snapshots {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_subnet_groups_output_subnet_groups(
    input: crate::output::DescribeSubnetGroupsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::SubnetGroup>> {
    let input = match input.subnet_groups {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_users_output_users(
    input: crate::output::DescribeUsersOutput,
) -> std::option::Option<std::vec::Vec<crate::model::User>> {
    let input = match input.users {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}
