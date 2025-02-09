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

use std::path::Path;
use graph_process_manager_core::process::config::AbstractProcessConfiguration;
use graphviz_dot_builder::{colors::GraphvizColor, item::node::style::GvNodeShape};


/** 
 * Draw the filtration result:
 * - either as a Graphviz Node with:
 *   + a specific shape
 *   + a label
 *   + a fill color
 *   + a font size
 *   + a font name
 *   + a font color
 * - or as a custom image
 * **/
pub enum FiltrationResultStyle {
    ShapeAndLabel(GvNodeShape,String,GraphvizColor,u32,&'static str,GraphvizColor),
    CustomImage
}

pub trait FiltrationDrawerForGraphvizLogger<Conf : AbstractProcessConfiguration> {

    fn get_filter_node_inner_style_and_draw_if_needed(
        &self,
        context_and_param : &Conf::ContextAndParameterization,
        filtration_result: &Conf::FiltrationResult,
        image_file_path : &Path
    ) -> FiltrationResultStyle;

    fn get_filter_edge_color(
        &self,
        context_and_param : &Conf::ContextAndParameterization,
        filtration_result: &Conf::FiltrationResult,
    ) -> GraphvizColor;

}



