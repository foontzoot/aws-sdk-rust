// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_get_vehicle_status_output_next_token(
    input: &crate::output::GetVehicleStatusOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_campaigns_output_next_token(
    input: &crate::output::ListCampaignsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_decoder_manifest_network_interfaces_output_next_token(
    input: &crate::output::ListDecoderManifestNetworkInterfacesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_decoder_manifests_output_next_token(
    input: &crate::output::ListDecoderManifestsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_decoder_manifest_signals_output_next_token(
    input: &crate::output::ListDecoderManifestSignalsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_fleets_output_next_token(
    input: &crate::output::ListFleetsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_fleets_for_vehicle_output_next_token(
    input: &crate::output::ListFleetsForVehicleOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_model_manifest_nodes_output_next_token(
    input: &crate::output::ListModelManifestNodesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_model_manifests_output_next_token(
    input: &crate::output::ListModelManifestsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_signal_catalog_nodes_output_next_token(
    input: &crate::output::ListSignalCatalogNodesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_signal_catalogs_output_next_token(
    input: &crate::output::ListSignalCatalogsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_vehicles_output_next_token(
    input: &crate::output::ListVehiclesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_vehicles_in_fleet_output_next_token(
    input: &crate::output::ListVehiclesInFleetOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_get_vehicle_status_output_campaigns(
    input: crate::output::GetVehicleStatusOutput,
) -> std::option::Option<std::vec::Vec<crate::model::VehicleStatus>> {
    let input = match input.campaigns {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_campaigns_output_campaign_summaries(
    input: crate::output::ListCampaignsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::CampaignSummary>> {
    let input = match input.campaign_summaries {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_decoder_manifest_network_interfaces_output_network_interfaces(
    input: crate::output::ListDecoderManifestNetworkInterfacesOutput,
) -> std::option::Option<std::vec::Vec<crate::model::NetworkInterface>> {
    let input = match input.network_interfaces {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_decoder_manifests_output_summaries(
    input: crate::output::ListDecoderManifestsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::DecoderManifestSummary>> {
    let input = match input.summaries {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_decoder_manifest_signals_output_signal_decoders(
    input: crate::output::ListDecoderManifestSignalsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::SignalDecoder>> {
    let input = match input.signal_decoders {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_fleets_output_fleet_summaries(
    input: crate::output::ListFleetsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::FleetSummary>> {
    let input = match input.fleet_summaries {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_fleets_for_vehicle_output_fleets(
    input: crate::output::ListFleetsForVehicleOutput,
) -> std::option::Option<std::vec::Vec<std::string::String>> {
    let input = match input.fleets {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_model_manifest_nodes_output_nodes(
    input: crate::output::ListModelManifestNodesOutput,
) -> std::option::Option<std::vec::Vec<crate::model::Node>> {
    let input = match input.nodes {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_model_manifests_output_summaries(
    input: crate::output::ListModelManifestsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::ModelManifestSummary>> {
    let input = match input.summaries {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_signal_catalog_nodes_output_nodes(
    input: crate::output::ListSignalCatalogNodesOutput,
) -> std::option::Option<std::vec::Vec<crate::model::Node>> {
    let input = match input.nodes {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_signal_catalogs_output_summaries(
    input: crate::output::ListSignalCatalogsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::SignalCatalogSummary>> {
    let input = match input.summaries {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_vehicles_output_vehicle_summaries(
    input: crate::output::ListVehiclesOutput,
) -> std::option::Option<std::vec::Vec<crate::model::VehicleSummary>> {
    let input = match input.vehicle_summaries {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_vehicles_in_fleet_output_vehicles(
    input: crate::output::ListVehiclesInFleetOutput,
) -> std::option::Option<std::vec::Vec<std::string::String>> {
    let input = match input.vehicles {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}
