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

use graph_process_manager_core::manager::config::AbstractProcessConfiguration;
use graphviz_dot_builder::item::item::GraphVizGraphItem;
use graphviz_dot_builder::item::node::style::GraphvizNodeStyle;

use crate::graphviz::format::GraphVizLoggerNodeFormat;

pub trait GraphVizProcessDrawer<Conf : AbstractProcessConfiguration> {

    /**
     Returns whether or not we want to represent static analyses
    (the verification of predicates on the nodes
    to stop the process prematurely on that node i.e. not explore its children)
    visually on the Graphviz Graph
     **/
    fn repr_static_analysis(&self) -> bool;

    /**
     Returns the temporary folder on which to store temporary files.
     **/
    fn get_temp_folder(&self) -> &str;

    /**
     The colors of the verdicts of the process
     **/
    fn get_verdict_color(&self,
                         local_verdict : &Conf::LocalVerdict) -> GraphvizColor;

    /**
     In case self.repr_static_analysis() returns True,
    this will be called to actually draw the node representing the
    static analysis
     **/
    fn get_static_analysis_as_gvnode_style(
        &self,
        context: &Conf::Context,
        param: &Conf::Parameterization,
        verdict: &Conf::LocalVerdict,
        data_proof : &Conf::StaticLocalVerdictAnalysisProof,
        static_analysis_name : &str
    ) -> GraphvizNodeStyle;

    /**
     A step of the process between two nodes of the process
    May be represented in more detailed on the Graphviz representation
    via a dedicated Graphviz node.
     **/
    fn get_step_gvnode_style(
        &self,
        context: &Conf::Context,
        param: &Conf::Parameterization,
        step: &Conf::StepKind,
        step_name : &str
    ) -> GraphvizNodeStyle;

    /**
     In case nodes of the process must be represented using GraphViz Clusters
    instead of simple Graphviz Nodes
    (i.e, when self.get_node_format() returns GraphVizLoggerNodeFormat::AnchoredCluster),
    this function specifies how this is to be done.
     **/
    fn get_node_as_gvcluster_style(
        &self,
        context: &Conf::Context,
        param: &Conf::Parameterization,
        new_node: &Conf::NodeKind,
        cluster_name : &str
    ) -> (GraphvizNodeStyle,Vec<Box<GraphVizGraphItem>>);

    /**
     In case nodes of the process are represented using Graphviz Nodes
    (i.e, when self.get_node_format() returns GraphVizLoggerNodeFormat::SimpleNodes),
    this function specifies how this is to be done.
     **/
    fn get_node_as_gvnode_style(
        &self,
        context: &Conf::Context,
        param: &Conf::Parameterization,
        new_node: &Conf::NodeKind,
        node_name : &str
    ) -> GraphvizNodeStyle;

    /**
     In case the process is made of several consecutive phases,
    which we want to highlight in the Graph,
    this method returns the id of the phase to which a given node belongs to.
     **/
    fn get_node_phase_id(&self,
                         context: &Conf::Context,
                         param: &Conf::Parameterization,
                         new_node: &Conf::NodeKind) -> Option<u32>;

    /**
     The colors of the background of the cluster in which to draw a specific phase of the process.
    **/
    fn get_phase_color(&self, phase_id : u32) -> GraphvizColor;

    /**
     Specifies whether nodes of the process must be represented in the Graphviz graph
    as GraphViz Clusters or simple Graphviz Nodes
     **/
    fn get_node_format(&self) -> GraphVizLoggerNodeFormat;

    fn get_anchor_id(&self, id: u32) -> String {
        format!("a{:}", id)
    }

    fn get_node_id(&self, id: u32) -> String {
        format!("n{:}", id)
    }

    fn get_verdict_id(&self, id: u32) -> String {
        format!("v{:}", id)
    }

    fn get_static_analysis_id(&self, id: u32) -> String {
        format!("stat{:}", id)
    }

    fn get_step_id(&self,
                       origin_id: u32,
                       target_id: u32) -> String {
        format!("s_{:}_{:}", origin_id, target_id)
    }
}