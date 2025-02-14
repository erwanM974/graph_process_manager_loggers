/*
Copyright 2020 Erwan Mahe (github.com/erwanM974)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use std::path::{Path, PathBuf};

use graph_process_manager_core::process::config::AbstractProcessConfiguration;
use graphviz_dot_builder::{colors::GraphvizColor, item::node::style::GraphvizNodeStyle};

use crate::graphviz::item::BuiltinGraphvizLoggerItemStyle;




pub trait CustomAllTheRestDrawerForGraphvizLogger<Conf : AbstractProcessConfiguration> {

    fn get_step_node_inner_style_and_draw_if_needed(
        &self,
        context_and_param : &Conf::ContextAndParameterization,
        step : &Conf::DomainSpecificStep,
        image_file_path : &Path
    ) -> BuiltinGraphvizLoggerItemStyle;

    fn get_step_edge_color(
        &self,
        context_and_param : &Conf::ContextAndParameterization,
        step : &Conf::DomainSpecificStep,
    ) -> GraphvizColor;

    fn get_filter_node_inner_style_and_draw_if_needed(
        &self,
        context_and_param : &Conf::ContextAndParameterization,
        filtration_result: &Conf::FiltrationResult,
        image_file_path : &Path
    ) -> BuiltinGraphvizLoggerItemStyle;

    fn get_filter_edge_color(
        &self,
        context_and_param : &Conf::ContextAndParameterization,
        filtration_result: &Conf::FiltrationResult,
    ) -> GraphvizColor;

    fn get_node_phase_id(
        &self,
        context_and_param : &Conf::ContextAndParameterization,
        new_node: &Conf::DomainSpecificNode
    ) -> Option<usize>;
        
    /**
     The colors of the background of the cluster in which to draw a specific phase of the process.
    **/
    fn get_phase_color(&self, phase_id : usize) -> GraphvizColor;
}


pub fn get_step_gvnode_style_and_edge_color<Conf : AbstractProcessConfiguration>(
    all_the_rest_drawer : &dyn CustomAllTheRestDrawerForGraphvizLogger<Conf>,
    temp_folder : &str,
    context_and_param: &Conf::ContextAndParameterization,
    step: &Conf::DomainSpecificStep,
    step_name : &str
) -> (GraphvizNodeStyle,GraphvizColor) {
    let step_color = all_the_rest_drawer.get_step_edge_color(context_and_param, step);
    let image_file_path: PathBuf = [temp_folder, &format!("{}.png", step_name)]
        .iter()
        .collect();
    // ***
    let item = all_the_rest_drawer.get_step_node_inner_style_and_draw_if_needed(
        context_and_param,
        step, 
        &image_file_path
    );
    let node_gv_options = item.to_graphviz_node_styte(&image_file_path);
    // ***
    (node_gv_options,step_color)
}



pub fn get_filtration_result_as_gvnode_style_and_edge_color<Conf : AbstractProcessConfiguration>(
    all_the_rest_drawer : &dyn CustomAllTheRestDrawerForGraphvizLogger<Conf>,
    temp_folder : &str,
    context_and_param: &Conf::ContextAndParameterization,
    filtration_result: &Conf::FiltrationResult,
    filtration_node_name : &str 
) -> (GraphvizNodeStyle,GraphvizColor) {
    let image_file_path: PathBuf = [temp_folder, &format!("{}.png", filtration_node_name)]
            .iter()
            .collect();
    let filtration_item = all_the_rest_drawer.get_filter_node_inner_style_and_draw_if_needed(
        context_and_param,
        filtration_result, 
        &image_file_path
    );
    // ***
    (
        filtration_item.to_graphviz_node_styte(&image_file_path), 
        all_the_rest_drawer.get_filter_edge_color(context_and_param, filtration_result)
    )
}