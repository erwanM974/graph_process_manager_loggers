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
use graphviz_dot_builder::item::cluster::GraphVizCluster;
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyle, GraphvizNodeStyleItem, GvNodeShape};
use crate::graphviz::drawer::GraphVizProcessDrawer;
use crate::graphviz::format::GraphVizLoggerNodeFormat;
use crate::tests::fibo_proc::conf::{FiboConfig, FiboStaticProof};
use crate::tests::fibo_proc::context::{FiboContext, FiboParameterization};
use crate::tests::fibo_proc::node::FiboNodeKind;
use crate::tests::fibo_proc::step::FiboStepKind;
use crate::tests::fibo_proc::verdict::local::FiboLocalVerdict;

pub struct FiboProcessDrawer {
    pub temp_folder : String,
    pub node_format : GraphVizLoggerNodeFormat
}

impl FiboProcessDrawer {
    pub fn new(temp_folder: String) -> Self {
        FiboProcessDrawer { temp_folder, node_format:GraphVizLoggerNodeFormat::SimpleNode }
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

    fn make_static_analysis_as_gvcluster(&self,
                                         _context: &FiboContext,
                                         _param : &FiboParameterization,
                                         _parent_state_id: u32,
                                         _verdict: &FiboLocalVerdict,
                                         _data_proof: &FiboStaticProof) -> GraphVizCluster {
        panic!("should never be called")
    }

    fn make_step_gvnode(&self,
                        _context: &FiboContext,
                        _param : &FiboParameterization,
                        origin_state_id: u32,
                        target_state_id: u32,
                        _step: &FiboStepKind) -> GraphVizNode {

        let step_name = format!("s_{}_{}", origin_state_id, target_state_id);
        let gv_node_options : GraphvizNodeStyle = vec![
            GraphvizNodeStyleItem::Label( "next".to_string() ),
            GraphvizNodeStyleItem::FillColor( GraphvizColor::white ),
            GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle)];
        // ***
        GraphVizNode::new(step_name,gv_node_options)
    }

    fn make_node_gvitem_as_gvcluster(&self,
                                     _context: &FiboContext,
                                     _param: &FiboParameterization,
                                     _new_state_id: u32,
                                     _new_node: &FiboNodeKind) -> GraphVizCluster {
        panic!("should never be called")
    }

    fn make_node_gvitem_as_gvnode(&self,
                                  _context: &FiboContext,
                                  _param: &FiboParameterization,
                                  new_node_id: u32,
                                  new_node: &FiboNodeKind) -> GraphVizNode {
        let node_name = self.get_node_id(new_node_id);
        let gv_node_options : GraphvizNodeStyle = vec![
            GraphvizNodeStyleItem::Label( new_node.current.to_string() ),
            GraphvizNodeStyleItem::FillColor( GraphvizColor::white ),
            GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle)];
        // ***
        GraphVizNode::new(node_name,gv_node_options)
    }

    fn get_node_format(&self) -> &GraphVizLoggerNodeFormat {
        &self.node_format
    }

    fn get_anchor_id(&self,
                     id: u32) -> String {
        format!("a{:}", id)
    }

    fn get_node_id(&self,
                   id: u32) -> String {
        format!("n{:}", id)
    }

    fn get_verdict_id(&self, id: u32) -> String {
        format!("v{:}", id)
    }

    fn get_static_analysis_ids(&self, id: u32) -> (String, String) {
        (format!("stat{:}", id),format!("stat_anchor{:}", id))
    }
}