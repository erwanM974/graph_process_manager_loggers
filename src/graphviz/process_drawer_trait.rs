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


use graph_process_manager_core::process::filter::GenericFiltersManager;
use graph_process_manager_core::queue::priorities::GenericProcessPriorities;
use graph_process_manager_core::queue::strategy::QueueSearchStrategy;
use graphviz_dot_builder::colors::GraphvizColor;

use graph_process_manager_core::process::config::AbstractProcessConfiguration;
use graphviz_dot_builder::item::item::GraphVizGraphItem;
use graphviz_dot_builder::item::node::style::GraphvizNodeStyle;

use crate::graphviz::format::GraphVizLoggerNodeFormat;

pub trait GraphVizProcessDrawer<Conf : AbstractProcessConfiguration> {

    /**
     Returns the temporary folder on which to store temporary files.
     **/
    fn get_temp_folder(&self) -> &str;

    /** 
     * Returns a legend node in which to put the description of the algorithm and its initial parameterization.
     * **/
    fn get_initial_legend_gvnode_style(
        &self,
        context_and_param: &Conf::ContextAndParameterization,
        strategy: &QueueSearchStrategy,
        priorities: &GenericProcessPriorities<Conf::Priorities>,
        filters_manager : &GenericFiltersManager<Conf>,
        use_memoization : bool
    ) -> GraphvizNodeStyle;

    /** 
     * Returns a legend node in which to describe the end result of the algorithm.
     * **/
    fn get_final_legend_gvnode_style(
        &self,
        context_and_param: &Conf::ContextAndParameterization,
        final_global_state : &Conf::MutablePersistentState
    ) -> GraphvizNodeStyle;

    /**
     We consider a "step" of the process between two nodes of the process : "node1" -"step"> "node2"
    We represented it in more detailed on the Graphviz representation via a dedicated Graphviz node so that we have:
    "node1_as_gv_node" -> "step_as_gv_node" -> "node2_as_gv_node".
    This returns the style of "step_as_gv_node" and the color of the two edges.
     **/
    fn get_step_gvnode_style_and_edge_color(
        &self,
        context_and_param: &Conf::ContextAndParameterization,
        step: &Conf::DomainSpecificStep,
        step_name : &str
    ) -> (GraphvizNodeStyle,GraphvizColor);

    /**
     In case nodes of the process must be represented using GraphViz Clusters
    instead of simple Graphviz Nodes
    (i.e, when self.get_node_format() returns GraphVizLoggerNodeFormat::AnchoredCluster),
    this function specifies how this is to be done.
     **/
    fn get_node_as_gvcluster_style(
        &self,
        context_and_param: &Conf::ContextAndParameterization,
        new_node: &Conf::DomainSpecificNode,
        cluster_name : &str
    ) -> (GraphvizNodeStyle,Vec<Box<GraphVizGraphItem>>);

    /**
     In case nodes of the process are represented using Graphviz Nodes
    (i.e, when self.get_node_format() returns GraphVizLoggerNodeFormat::SimpleNodes),
    this function specifies how this is to be done.
     **/
    fn get_node_as_gvnode_style(
        &self,
        context_and_param: &Conf::ContextAndParameterization,
        new_node: &Conf::DomainSpecificNode,
        node_name : &str
    ) -> GraphvizNodeStyle;


    /**
     Filtered-out process nodes or process steps are represented by a dedicated Graphviz node.
     This returns the style of the graphviz node that represents the filtration result and
     the color that relates it to its parent node.
     **/
     fn get_filtration_result_as_gvnode_style_and_edge_color(
        &self,
        context_and_param: &Conf::ContextAndParameterization,
        filtration_result: &Conf::FiltrationResult,
        filtration_node_name : &str 
    ) -> (GraphvizNodeStyle,GraphvizColor);

    /**
     In case the process is made of several consecutive phases,
    which we want to highlight in the Graph,
    this method returns the id of the phase to which a given node belongs to.
     **/
    fn get_node_phase_id(
        &self,
        context_and_param: &Conf::ContextAndParameterization,
        new_node: &Conf::DomainSpecificNode
    ) -> Option<u32>;

    /**
     The colors of the background of the cluster in which to draw a specific phase of the process.
    **/
    fn get_phase_color(&self, phase_id : u32) -> GraphvizColor;

    /**
     Specifies whether nodes of the process must be represented in the Graphviz graph
    as GraphViz Clusters or simple Graphviz Nodes
     **/
    fn get_node_format(&self) -> GraphVizLoggerNodeFormat;

}