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
use graphviz_dot_builder::{colors::GraphvizColor, item::{item::GraphVizGraphItem, node::{node::GraphVizNode, style::{GraphvizNodeStyle, GraphvizNodeStyleItem, GvNodeShape}}}};

use crate::graphviz::item::BuiltinGraphvizLoggerItemStyle;



pub trait CustomNodeDrawerForGraphvizLogger<Conf : AbstractProcessConfiguration> {

    fn get_node_node_inner_style_and_draw_if_needed(
        &self,
        context_and_param : &Conf::ContextAndParameterization,
        node : &Conf::DomainSpecificNode,
        full_path : &Path
    ) -> BuiltinGraphvizLoggerItemStyle;

}



pub fn get_node_as_gvcluster_style<Conf : AbstractProcessConfiguration>(
    node_drawers : &Vec<Box<dyn CustomNodeDrawerForGraphvizLogger<Conf>>>,
    temp_folder : &str,
    context_and_param: &Conf::ContextAndParameterization,
    new_node: &Conf::DomainSpecificNode,
    cluster_name : &str
) -> (GraphvizNodeStyle,Vec<Box<GraphVizGraphItem>>) {
    let cluster_gv_options = vec![
        GraphvizNodeStyleItem::FillColor(GraphvizColor::lightgrey),
        GraphvizNodeStyleItem::Label("".to_string()),
    ];
    let mut sub_nodes = vec![];
    for (node_drawer_id, node_drawer) in node_drawers.iter().enumerate() {
        let sub_node_name = format!("{}_drawn{}", cluster_name, node_drawer_id);
        let image_file_path: PathBuf =
            [temp_folder, &format!("{}.png", &sub_node_name)]
                .iter()
                .collect();
        // ***
        let sub_node_item = node_drawer.get_node_node_inner_style_and_draw_if_needed(
            context_and_param,
            new_node, 
            &image_file_path
        );
        // ***
        let sub_node_style = sub_node_item.to_graphviz_node_styte(&image_file_path);
        // ***
        let sub_gv_node = GraphVizNode::new(sub_node_name, sub_node_style);
        sub_nodes.push(Box::new(GraphVizGraphItem::Node(sub_gv_node)));
    }
    (cluster_gv_options, sub_nodes)
}






pub fn get_node_as_gvnode_style<Conf : AbstractProcessConfiguration>(
    node_drawers : &Vec<Box<dyn CustomNodeDrawerForGraphvizLogger<Conf>>>,
    temp_folder : &str,
    context_and_param: &Conf::ContextAndParameterization,
    new_node: &Conf::DomainSpecificNode,
    node_name : &str
) -> GraphvizNodeStyle {
    if let Some(node_drawer) = node_drawers.first() {
        let image_file_path: PathBuf = [temp_folder, &format!("{}.png", node_name)]
            .iter()
            .collect();
        // ***
        let sub_node_item = node_drawer.get_node_node_inner_style_and_draw_if_needed(
            context_and_param,
            new_node, 
            &image_file_path
        );
        // ***
        sub_node_item.to_graphviz_node_styte(&image_file_path)
    } else {
        vec![
            GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle),
            GraphvizNodeStyleItem::FillColor(GraphvizColor::white),
            GraphvizNodeStyleItem::Label("".to_string())
        ]
    }
}