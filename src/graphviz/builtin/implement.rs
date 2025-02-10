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

use graph_process_manager_core::process::config::AbstractProcessConfiguration;

use crate::graphviz::builtin::builtin_process_drawer_trait::BuiltinProcessDrawer;
use crate::graphviz::format::GraphVizLoggerNodeFormat;
use crate::graphviz::process_drawer_trait::GraphVizProcessDrawer;
use graphviz_dot_builder::colors::GraphvizColor;
use graphviz_dot_builder::item::item::GraphVizGraphItem;
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::{
    GraphvizNodeStyle, GraphvizNodeStyleItem, GvNodeShape, GvNodeStyleKind,
};
use std::path::PathBuf;

use super::filtration_drawer::FiltrationResultStyle;

impl<ProcessDrawer, Conf> GraphVizProcessDrawer<Conf> for ProcessDrawer
where
    Conf: 'static + AbstractProcessConfiguration,
    ProcessDrawer: BuiltinProcessDrawer<Conf>,
{

    fn get_temp_folder(&self) -> &str {
        self.get_temp_folder()
    }

    fn get_final_legend_gvnode_style(
        &self,
        context_and_param: &Conf::ContextAndParameterization,
        final_global_state : &Conf::MutablePersistentState
    ) -> GraphvizNodeStyle {
        let legs = self.get_final_global_state_description_for_legend(context_and_param,final_global_state);
        let legend_node_gv_options : GraphvizNodeStyle = vec![
            GraphvizNodeStyleItem::Label( legs.join(r"\l") + r"\l" ),
            GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle),
            GraphvizNodeStyleItem::Style(vec![GvNodeStyleKind::Bold,GvNodeStyleKind::Rounded]),
            GraphvizNodeStyleItem::FontSize( 18 )];
        // ***
        legend_node_gv_options
    }

    fn get_step_gvnode_style_and_edge_color(
        &self,
        context_and_param: &Conf::ContextAndParameterization,
        step: &Conf::DomainSpecificStep,
        step_name : &str
    ) -> (GraphvizNodeStyle,GraphvizColor) {
        let step_color = self.get_step_drawer().get_step_color(context_and_param, step);
        let mut node_gv_options = vec![];
        node_gv_options.push(GraphvizNodeStyleItem::Label("".to_string()));
        node_gv_options.push(GraphvizNodeStyleItem::Color(step_color.clone()));
        node_gv_options.push(GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle));
        let image_file_path: PathBuf = [self.get_temp_folder(), &format!("{}.png", step_name)]
            .iter()
            .collect();
        // ***
        self.get_step_drawer()
            .draw(context_and_param,step, &image_file_path);
        // ***
        node_gv_options.push(GraphvizNodeStyleItem::Image(
            image_file_path
                .into_os_string()
                .to_str()
                .unwrap()
                .to_string(),
        ));
        // **
        (node_gv_options,step_color)
    }

    fn get_node_as_gvcluster_style(
        &self,
        context_and_param: &Conf::ContextAndParameterization,
        new_node: &Conf::DomainSpecificNode,
        cluster_name : &str
    ) -> (GraphvizNodeStyle,Vec<Box<GraphVizGraphItem>>) {
        let cluster_gv_options = vec![
            GraphvizNodeStyleItem::FillColor(GraphvizColor::lightgrey),
            GraphvizNodeStyleItem::Label("".to_string()),
        ];
        let mut sub_nodes = vec![];
        for (drawer_id, drawer) in self.get_node_drawers().iter().enumerate() {
            let sub_node_name = format!("{}_drawn{}", cluster_name, drawer_id);
            let mut sub_node_style = vec![];
            let image_file_path: PathBuf =
                [self.get_temp_folder(), &format!("{}.png", &sub_node_name)]
                    .iter()
                    .collect();
            // ***
            drawer.draw(context_and_param,new_node, &image_file_path);
            // ***
            sub_node_style.push(GraphvizNodeStyleItem::Image(
                image_file_path
                    .into_os_string()
                    .to_str()
                    .unwrap()
                    .to_string(),
            ));
            sub_node_style.push(GraphvizNodeStyleItem::Label("".to_string()));
            sub_node_style.push(GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle));
            // ***
            let sub_gv_node = GraphVizNode::new(sub_node_name, sub_node_style);
            sub_nodes.push(Box::new(GraphVizGraphItem::Node(sub_gv_node)));
        }
        (cluster_gv_options, sub_nodes)
    }

    fn get_node_as_gvnode_style(
        &self,
        context_and_param: &Conf::ContextAndParameterization,
        new_node: &Conf::DomainSpecificNode,
        node_name : &str
    ) -> GraphvizNodeStyle {
        let mut node_gv_options = vec![];
        node_gv_options.push(GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle));
        node_gv_options.push(GraphvizNodeStyleItem::FillColor(GraphvizColor::white));
        node_gv_options.push(GraphvizNodeStyleItem::Label("".to_string()));
        if let Some(concrete_drawer) = self.get_node_drawers().first() {
            let image_file_path: PathBuf = [self.get_temp_folder(), &format!("{}.png", node_name)]
                .iter()
                .collect();
            // ***
            (*concrete_drawer).draw(context_and_param,new_node,&image_file_path);
            // ***
            node_gv_options.push(GraphvizNodeStyleItem::Image(
                image_file_path
                    .into_os_string()
                    .to_str()
                    .unwrap()
                    .to_string(),
            ));
        }
        node_gv_options
    }

    fn get_filtration_result_as_gvnode_style_and_edge_color(
        &self,
        context_and_param: &Conf::ContextAndParameterization,
        filtration_result: &Conf::FiltrationResult,
        filtration_node_name : &str 
    ) -> (GraphvizNodeStyle,GraphvizColor) {
        let image_file_path: PathBuf = [self.get_temp_folder(), &format!("{}.png", filtration_node_name)]
                .iter()
                .collect();
        let node_style = match self.get_filter_drawer().get_filter_node_inner_style_and_draw_if_needed(
            context_and_param, 
            filtration_result, 
            &image_file_path
        ) {
            FiltrationResultStyle::CustomImage => {
                vec![
                    GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle),
                    GraphvizNodeStyleItem::FillColor(GraphvizColor::white),
                    GraphvizNodeStyleItem::Label("".to_string()),
                    GraphvizNodeStyleItem::Image(
                        image_file_path
                            .into_os_string()
                            .to_str()
                            .unwrap()
                            .to_string(),
                    )
                ]
            },
            FiltrationResultStyle::ShapeAndLabel(
                shape, 
                label,
                fill_color,
                font_size,
                font_name,
                font_color) => {
                    vec![
                        GraphvizNodeStyleItem::Shape( shape ),
                        GraphvizNodeStyleItem::Label( label ),
                        GraphvizNodeStyleItem::Color( fill_color ),
                        GraphvizNodeStyleItem::FontSize( font_size ),
                        GraphvizNodeStyleItem::FontName( font_name.to_string() ),
                        GraphvizNodeStyleItem::FontColor( font_color ),
                        GraphvizNodeStyleItem::Style(vec![GvNodeStyleKind::Filled])
                    ]
            }
        };
        (node_style, self.get_filter_drawer().get_filter_edge_color(context_and_param, filtration_result))
    }

    fn get_node_phase_id(
        &self,
        context_and_param: &Conf::ContextAndParameterization,
        new_node: &Conf::DomainSpecificNode
    ) -> Option<u32> {
        self.get_node_phase_id(context_and_param, new_node)
    }

    fn get_phase_color(&self, phase_id: u32) -> GraphvizColor {
        self.get_phase_color(phase_id)
    }

    fn get_node_format(&self) -> GraphVizLoggerNodeFormat {
        if self.get_node_drawers().len() <= 1 {
            GraphVizLoggerNodeFormat::SimpleNode
        } else {
            GraphVizLoggerNodeFormat::AnchoredCluster
        }
    }
}
