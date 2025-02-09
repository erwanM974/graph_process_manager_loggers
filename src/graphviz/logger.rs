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

use crate::graphviz::process_drawer_trait::GraphVizProcessDrawer;
use crate::graphviz::format::GraphVizProcessLoggerLayout;


pub struct GenericGraphVizLogger<Conf : AbstractProcessConfiguration> {
    // ***
    /** the drawer **/
    pub(crate) drawer : Box<dyn GraphVizProcessDrawer<Conf>>,
    // ***
    pub(crate) output_format : GraphVizOutputFormat,
    // ***
    pub(crate) display_legend : bool,
    // ***
    pub(crate) parent_folder : String,
    pub(crate) output_file_name : String,
    // ***
    /** the Graphviz graph that is being built **/
    pub graph : GraphVizDiGraph, 
    // ***
    /**
    A map that keeps track of the phase as part of which a given node has been processed
    A value is absent from that dictionary if the identifier does not yet exists.
    Or if we do not want to highlight phases.
    **/
    pub(crate) nodes_id_to_process_phase_id : HashMap<u32,u32>,
    /** the drawer **/
    pub(crate) process_phases_clusters : HashMap<u32,GraphVizCluster>
}

impl<Conf : AbstractProcessConfiguration> GenericGraphVizLogger<Conf> {
    pub fn new(drawer : Box<dyn GraphVizProcessDrawer<Conf>>,
               output_format: GraphVizOutputFormat,
               layout: GraphVizProcessLoggerLayout,
               display_legend: bool,
               parent_folder: String,
               output_file_name: String) -> Self {
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
        let nodes_id_to_process_phase_id : HashMap<u32,u32> = HashMap::new();
        let process_phases_clusters = HashMap::new();
        // ***
        Self {
            drawer, output_format, display_legend, parent_folder, output_file_name,
            graph, nodes_id_to_process_phase_id, process_phases_clusters
        }
    }
}



