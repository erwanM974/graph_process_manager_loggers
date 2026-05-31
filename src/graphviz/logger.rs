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

use graph_process_manager_core::process::config::AbstractProcessConfiguration;
use graphviz_dot_builder::graph::graph::GraphVizDiGraph;
use graphviz_dot_builder::graph::style::{GraphvizGraphStyleItem, GvGraphRankDir};
use graphviz_dot_builder::item::cluster::GraphVizCluster;
use graphviz_dot_builder::traits::GraphVizOutputFormat;

use crate::graphviz::format::{GraphVizLoggerNodeFormat, GraphVizProcessLoggerLayout};

use super::drawers::drawer::GraphVizProcessDrawer;
use super::drawers::legend_writer::ProcessLegendWriter;


pub struct GenericGraphVizLoggerConfiguration {
    pub output_format: GraphVizOutputFormat,
    pub temp_folder: String,
    pub parent_folder: String,
    pub output_file_name: String,
}

impl GenericGraphVizLoggerConfiguration {
    pub fn new(
        output_format: GraphVizOutputFormat,
        temp_folder: String,
        parent_folder: String,
        output_file_name: String,
    ) -> Self {
        Self { output_format, temp_folder, parent_folder, output_file_name }
    }
}


pub struct GenericGraphVizLogger<Conf: AbstractProcessConfiguration> {
    pub configuration: GenericGraphVizLoggerConfiguration,
    pub drawer: Box<dyn GraphVizProcessDrawer<Conf>>,
    /// When `Some`, a legend node and a verdict node are added to the graph.
    pub legend_writer: Option<Box<dyn ProcessLegendWriter<Conf>>>,
    // internal graph being built
    pub graph: GraphVizDiGraph,
    pub(crate) nodes_id_to_process_phase_id: HashMap<u32, usize>,
    pub(crate) process_phases_clusters: HashMap<usize, GraphVizCluster>,
    pub(crate) filtration_counter: u32,
}

impl<Conf: AbstractProcessConfiguration> GenericGraphVizLogger<Conf> {
    pub fn new(
        configuration: GenericGraphVizLoggerConfiguration,
        drawer: Box<dyn GraphVizProcessDrawer<Conf>>,
        legend_writer: Option<Box<dyn ProcessLegendWriter<Conf>>>,
        layout: GraphVizProcessLoggerLayout,
    ) -> Self {
        let rankdir = match layout {
            GraphVizProcessLoggerLayout::Horizontal => GvGraphRankDir::LR,
            GraphVizProcessLoggerLayout::Vertical   => GvGraphRankDir::TB,
        };
        Self {
            configuration,
            drawer,
            legend_writer,
            graph: GraphVizDiGraph::new(vec![GraphvizGraphStyleItem::Rankdir(rankdir)]),
            nodes_id_to_process_phase_id: HashMap::new(),
            process_phases_clusters: HashMap::new(),
            filtration_counter: 0,
        }
    }

    pub(crate) fn node_format(&self) -> GraphVizLoggerNodeFormat {
        if self.drawer.node_view_count() <= 1 {
            GraphVizLoggerNodeFormat::SimpleNode
        } else {
            GraphVizLoggerNodeFormat::AnchoredCluster
        }
    }
}
