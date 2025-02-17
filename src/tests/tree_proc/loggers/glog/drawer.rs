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



use graphviz_dot_builder::colors::GraphvizColor;
use graphviz_dot_builder::item::item::GraphVizGraphItem;
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyle, GraphvizNodeStyleItem, GvNodeShape, GvNodeStyleKind};
use crate::graphviz::process_drawer_trait::GraphVizProcessDrawer;
use crate::graphviz::format::GraphVizLoggerNodeFormat;
use crate::tests::tree_proc::conf::TreeConfig;
use crate::tests::tree_proc::context::TreeContextAndParameterization;
use crate::tests::tree_proc::filtration::TreeFiltrationResult;
use crate::tests::tree_proc::node::TreeNodeKind;
use crate::tests::tree_proc::state::TreePersistentState;
use crate::tests::tree_proc::step::TreeStepKind;

pub struct TreeProcessDrawer {
    pub temp_folder : String
}

impl TreeProcessDrawer {
    pub fn new(temp_folder: String) -> Self {
        TreeProcessDrawer { temp_folder }
    }
}


impl GraphVizProcessDrawer<TreeConfig> for TreeProcessDrawer {

    fn get_temp_folder(&self) -> &str {
        &self.temp_folder
    }

    fn get_final_legend_gvnode_style(
        &self,
        _context_and_param: &TreeContextAndParameterization,
        _final_global_state : &TreePersistentState
    ) -> GraphvizNodeStyle {
        vec![
            GraphvizNodeStyleItem::Label("".to_string()),
            GraphvizNodeStyleItem::Style(vec![GvNodeStyleKind::Invis])
        ]
    }

    fn get_step_gvnode_style_and_edge_color(
        &self,
        _context_and_param: &TreeContextAndParameterization,
        step: &TreeStepKind,
        _step_name : &str
    ) -> (GraphvizNodeStyle,GraphvizColor) {
        let gv_node_options : GraphvizNodeStyle = vec![
            GraphvizNodeStyleItem::Label( step.to_string() ),
            GraphvizNodeStyleItem::FillColor( GraphvizColor::white ),
            GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle)];
        // ***
        (gv_node_options,GraphvizColor::black)
    }

    fn get_node_as_gvcluster_style(
        &self,
        _context_and_param: &TreeContextAndParameterization,
        _new_node: &TreeNodeKind,
        _cluster_name : &str
    ) -> (GraphvizNodeStyle,Vec<Box<GraphVizGraphItem>>) {
        panic!("should never be called")
    }

    fn get_node_as_gvnode_style(
        &self,
        _context_and_param: &TreeContextAndParameterization,
        new_node: &TreeNodeKind,
        _node_name : &str
    ) -> GraphvizNodeStyle {
        let gv_node_options : GraphvizNodeStyle = vec![
            GraphvizNodeStyleItem::Label( new_node.word.clone() ),
            GraphvizNodeStyleItem::FillColor( GraphvizColor::white ),
            GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle)];
        gv_node_options
    }

    fn get_filtration_result_as_gvnode_style_and_edge_color(
        &self,
        _context_and_param: &TreeContextAndParameterization,
        filtration_result: &TreeFiltrationResult,
        _filtration_node_name : &str 
    ) -> (GraphvizNodeStyle,GraphvizColor) {
        let gv_node_options : GraphvizNodeStyle = vec![
            GraphvizNodeStyleItem::Label( filtration_result.to_string() ),
            GraphvizNodeStyleItem::FillColor( GraphvizColor::burlywood4 ),
            GraphvizNodeStyleItem::Shape(GvNodeShape::Pentagon)];
        // ***
        (gv_node_options,GraphvizColor::burlywood4)
    }

    fn get_node_phase_id(
        &self,
        _context_and_param: &TreeContextAndParameterization,
        _new_node: &TreeNodeKind,
    ) -> Option<u32> {
        None
    }
    
    fn get_phase_color(&self, _phase_id : u32) -> GraphvizColor {
        GraphvizColor::black
    }

    fn get_node_format(&self) -> GraphVizLoggerNodeFormat {
        GraphVizLoggerNodeFormat::SimpleNode
    }

}