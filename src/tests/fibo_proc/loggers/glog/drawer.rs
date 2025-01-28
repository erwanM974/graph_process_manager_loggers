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
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyle, GraphvizNodeStyleItem, GvNodeShape};
use crate::graphviz::process_drawer_trait::GraphVizProcessDrawer;
use crate::graphviz::format::GraphVizLoggerNodeFormat;
use crate::tests::fibo_proc::conf::{FiboConfig, FiboStaticProof};
use crate::tests::fibo_proc::context::{FiboContext, FiboParameterization};
use crate::tests::fibo_proc::node::FiboNodeKind;
use crate::tests::fibo_proc::step::FiboStepKind;
use crate::tests::fibo_proc::verdict::local::FiboLocalVerdict;

pub struct FiboProcessDrawer {
    pub temp_folder : String
}

impl FiboProcessDrawer {
    pub fn new(temp_folder: String) -> Self {
        FiboProcessDrawer { temp_folder }
    }
}


impl GraphVizProcessDrawer<FiboConfig> for FiboProcessDrawer {

    fn repr_static_analysis(&self) -> bool {
        false
    }

    fn get_temp_folder(&self) -> &str {
        &self.temp_folder
    }

    fn get_verdict_color(&self,
                         _local_verdict: &FiboLocalVerdict) -> GraphvizColor {
        GraphvizColor::black
    }

    fn get_static_analysis_as_gvnode_style(&self,
                                         _context: &FiboContext,
                                         _param : &FiboParameterization,
                                         _verdict: &FiboLocalVerdict,
                                         _data_proof: &FiboStaticProof,
    _static_name : &str) -> GraphvizNodeStyle {
        panic!("should never be called")
    }

    fn get_step_gvnode_style(&self,
                        _context: &FiboContext,
                        _param : &FiboParameterization,
                        _step: &FiboStepKind,
                             _step_name : &str) -> GraphvizNodeStyle {
        let gv_node_options : GraphvizNodeStyle = vec![
            GraphvizNodeStyleItem::Label( "next".to_string() ),
            GraphvizNodeStyleItem::FillColor( GraphvizColor::white ),
            GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle)];
        // ***
        gv_node_options
    }

    fn get_node_as_gvcluster_style(&self,
                                   _context: &FiboContext,
                                   _param: &FiboParameterization,
                                   _new_node: &FiboNodeKind,
                                   _cluster_name : &str) -> (GraphvizNodeStyle, Vec<Box<GraphVizGraphItem>>) {
        panic!("should never be called")
    }

    fn get_node_as_gvnode_style(&self,
                                  _context: &FiboContext,
                                  _param: &FiboParameterization,
                                  new_node: &FiboNodeKind,
                                _node_name : &str) -> GraphvizNodeStyle {
        let gv_node_options : GraphvizNodeStyle = vec![
            GraphvizNodeStyleItem::Label( new_node.current.to_string() ),
            GraphvizNodeStyleItem::FillColor( GraphvizColor::white ),
            GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle)];
        // ***
        gv_node_options
    }

    fn get_node_format(&self) -> GraphVizLoggerNodeFormat {
        GraphVizLoggerNodeFormat::SimpleNode
    }
    
    fn get_node_phase_id(&self,
                         _context: &<FiboConfig as graph_process_manager_core::manager::config::AbstractProcessConfiguration>::Context,
                         _param: &<FiboConfig as graph_process_manager_core::manager::config::AbstractProcessConfiguration>::Parameterization,
                         _new_node: &<FiboConfig as graph_process_manager_core::manager::config::AbstractProcessConfiguration>::NodeKind) -> Option<u32> {
        None 
    }
    
    fn get_phase_color(&self, _phase_id : u32) -> GraphvizColor {
        GraphvizColor::black
    }

}