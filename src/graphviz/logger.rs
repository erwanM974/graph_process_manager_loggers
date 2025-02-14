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


use std::collections::HashMap;
use graphviz_dot_builder::graph::graph::GraphVizDiGraph;
use graph_process_manager_core::process::config::AbstractProcessConfiguration;
use graphviz_dot_builder::traits::GraphVizOutputFormat;
use graphviz_dot_builder::graph::style::{GraphvizGraphStyleItem,GvGraphRankDir};
use graphviz_dot_builder::item::cluster::GraphVizCluster;

use crate::graphviz::format::GraphVizProcessLoggerLayout;

use super::drawers::all_the_rest_drawer::CustomAllTheRestDrawerForGraphvizLogger;
use super::drawers::legend_writer::ProcessLegendWriter;
use super::drawers::node_drawer::CustomNodeDrawerForGraphvizLogger;
use super::format::GraphVizLoggerNodeFormat;



pub struct GenericGraphVizLoggerConfiguration {
    pub output_format : GraphVizOutputFormat,
    pub display_legend : bool,
    // ***
    pub temp_folder : String,
    // ***
    pub parent_folder : String,
    pub output_file_name : String,
}

impl GenericGraphVizLoggerConfiguration {

    pub fn new(
        output_format : GraphVizOutputFormat,
        display_legend : bool,
        temp_folder : String,
        parent_folder : String,
        output_file_name : String
    ) -> Self {
        Self {
            output_format,
            display_legend,
            temp_folder,
            parent_folder,
            output_file_name
        }
    }

}



pub struct GenericGraphVizLogger<Conf : AbstractProcessConfiguration> {
    // ***
    pub configuration : GenericGraphVizLoggerConfiguration,
    // ***
    pub legend_writer : Box<dyn ProcessLegendWriter<Conf>>,
    pub node_drawers : Vec<Box<dyn CustomNodeDrawerForGraphvizLogger<Conf>>>,
    pub all_the_rest_drawer : Box<dyn CustomAllTheRestDrawerForGraphvizLogger<Conf>>,
    // ***
    /** the Graphviz graph that is being built **/
    pub graph : GraphVizDiGraph, 
    // ***
    /**
    A map that keeps track of the phase as part of which a given node has been processed
    A value is absent from that dictionary if the identifier does not yet exists.
    Or if we do not want to highlight phases.
    **/
    pub(crate) nodes_id_to_process_phase_id : HashMap<u32,usize>,
    /** the drawer **/
    pub(crate) process_phases_clusters : HashMap<usize,GraphVizCluster>
}

impl<Conf : AbstractProcessConfiguration> GenericGraphVizLogger<Conf> {
    pub fn new(
        configuration : GenericGraphVizLoggerConfiguration,
        legend_writer : Box<dyn ProcessLegendWriter<Conf>>,
        node_drawers : Vec<Box<dyn CustomNodeDrawerForGraphvizLogger<Conf>>>,
        all_the_rest_drawer : Box<dyn CustomAllTheRestDrawerForGraphvizLogger<Conf>>,
        layout: GraphVizProcessLoggerLayout
    ) -> Self {
        // ***
        let rankdir : GvGraphRankDir = match layout {
            GraphVizProcessLoggerLayout::Horizontal => {
                GvGraphRankDir::LR
            },
            GraphVizProcessLoggerLayout::Vertical => {
                GvGraphRankDir::TB
            }
        };
        let style = vec![GraphvizGraphStyleItem::Rankdir(rankdir)];
        let graph = GraphVizDiGraph::new(style);
        let nodes_id_to_process_phase_id : HashMap<u32,usize> = HashMap::new();
        let process_phases_clusters = HashMap::new();
        // ***
        Self {
            configuration,
            legend_writer,
            node_drawers,
            all_the_rest_drawer,
            graph, 
            nodes_id_to_process_phase_id, 
            process_phases_clusters
        }
    }


    pub fn get_node_format(&self) -> GraphVizLoggerNodeFormat {
        if self.node_drawers.len() <= 1 {
            GraphVizLoggerNodeFormat::SimpleNode
        } else {
            GraphVizLoggerNodeFormat::AnchoredCluster
        }
    }

}



