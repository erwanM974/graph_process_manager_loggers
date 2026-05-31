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

use std::fs;
use std::path::PathBuf;

use graphviz_dot_builder::edge::edge::GraphVizEdge;
use graphviz_dot_builder::edge::style::{GraphvizEdgeStyleItem, GvArrowHeadSide, GvArrowHeadStyle};
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyleItem, GvNodeStyleKind};
use graphviz_dot_builder::item::item::GraphVizGraphItem;
use graphviz_dot_builder::item::cluster::GraphVizCluster;
use graphviz_dot_builder::traits::{DotBuildable, DotPrintable};

use graph_process_manager_core::process::config::AbstractProcessConfiguration;
use graph_process_manager_core::process::manager::GenericProcessManager;

use crate::graphviz::format::GraphVizLoggerNodeFormat;
use crate::graphviz::logger::GenericGraphVizLogger;
use crate::graphviz::util::*;
use crate::logger::AbstractProcessLogger;


impl<Conf: AbstractProcessConfiguration + 'static> AbstractProcessLogger<Conf>
    for GenericGraphVizLogger<Conf>
{
    fn log_initialize_process(&mut self, manager: &GenericProcessManager<Conf>) {
        let _ = fs::remove_dir_all(&self.configuration.temp_folder);
        let _ = fs::create_dir_all(&self.configuration.temp_folder);
        if !self.configuration.parent_folder.is_empty() {
            let _ = fs::create_dir_all(&self.configuration.parent_folder);
        }
        if let Some(legend) = &self.legend_writer {
            let legend_node = legend.get_legend_node(
                &manager.context_and_param,
                manager.get_strategy(),
                manager.get_priorities(),
                manager.get_filters_manager(),
                manager.is_memoized(),
            );
            self.graph.add_node(legend_node);
        }
    }

    fn log_new_node(
        &mut self,
        ctx: &Conf::ContextAndParameterization,
        new_node_id: u32,
        new_node: &Conf::DomainSpecificNode,
    ) {
        let as_gv_item = match self.node_format() {
            GraphVizLoggerNodeFormat::AnchoredCluster => {
                let cluster_name = get_node_id("", new_node_id);
                let cluster_style = vec![
                    GraphvizNodeStyleItem::FillColor(graphviz_dot_builder::colors::GraphvizColor::lightgrey),
                    GraphvizNodeStyleItem::Label("".to_string()),
                ];
                let mut sub_nodes: Vec<Box<GraphVizGraphItem>> = vec![];
                for view_index in 0..self.drawer.node_view_count() {
                    let sub_name = format!("{}_drawn{}", cluster_name, view_index);
                    let img_path: PathBuf =
                        [&self.configuration.temp_folder, &format!("{}.png", sub_name)]
                            .iter()
                            .collect();
                    let style = self
                        .drawer
                        .draw_node_view(ctx, new_node, view_index, &img_path)
                        .to_graphviz_node_styte(&img_path);
                    sub_nodes.push(Box::new(GraphVizGraphItem::Node(GraphVizNode::new(sub_name, style))));
                }
                let anchor = GraphVizNode::new(
                    get_anchor_id("", new_node_id),
                    vec![
                        GraphvizNodeStyleItem::Label("".to_string()),
                        GraphvizNodeStyleItem::Style(vec![GvNodeStyleKind::Invis]),
                        GraphvizNodeStyleItem::Peripheries(0),
                        GraphvizNodeStyleItem::Height(0),
                        GraphvizNodeStyleItem::Width(0),
                    ],
                );
                sub_nodes.insert(sub_nodes.len() / 2, Box::new(GraphVizGraphItem::Node(anchor)));
                GraphVizGraphItem::Cluster(GraphVizCluster::new(
                    cluster_name,
                    cluster_style,
                    sub_nodes,
                    vec![],
                ))
            }
            GraphVizLoggerNodeFormat::SimpleNode => {
                let node_name = get_node_id("", new_node_id);
                let img_path: PathBuf =
                    [&self.configuration.temp_folder, &format!("{}.png", node_name)]
                        .iter()
                        .collect();
                let style = self
                    .drawer
                    .draw_node_view(ctx, new_node, 0, &img_path)
                    .to_graphviz_node_styte(&img_path);
                GraphVizGraphItem::Node(GraphVizNode::new(node_name, style))
            }
        };

        if let Some(phase_id) = self.drawer.node_phase(ctx, new_node) {
            if let std::collections::hash_map::Entry::Vacant(e) =
                self.process_phases_clusters.entry(phase_id)
            {
                let phase_style =
                    vec![GraphvizNodeStyleItem::FillColor(self.drawer.phase_color(phase_id))];
                e.insert(GraphVizCluster::new(
                    format!("phase{}", phase_id),
                    phase_style,
                    vec![],
                    vec![],
                ));
            }
            self.process_phases_clusters
                .get_mut(&phase_id)
                .unwrap()
                .items
                .push(Box::new(as_gv_item));
            self.nodes_id_to_process_phase_id.insert(new_node_id, phase_id);
        } else {
            match as_gv_item {
                GraphVizGraphItem::Node(n)    => self.graph.add_node(n),
                GraphVizGraphItem::Cluster(c) => self.graph.add_cluster(c),
            }
        }
    }

    fn log_new_step(
        &mut self,
        ctx: &Conf::ContextAndParameterization,
        origin_node_id: u32,
        step: &Conf::DomainSpecificStep,
        target_node_id: u32,
        _target_node: &Conf::DomainSpecificNode,
    ) {
        let step_name = get_step_id("", origin_node_id, target_node_id);
        let img_path: PathBuf =
            [&self.configuration.temp_folder, &format!("{}.png", step_name)]
                .iter()
                .collect();
        let step_style = self.drawer.draw_step(ctx, step, &img_path).to_graphviz_node_styte(&img_path);
        let edge_color = self.drawer.step_edge_color(ctx, step);

        let step_node = GraphVizNode::new(step_name, step_style);
        let edge_opts = vec![
            GraphvizEdgeStyleItem::Head(GvArrowHeadStyle::Vee(GvArrowHeadSide::Both)),
            GraphvizEdgeStyleItem::Color(edge_color),
        ];
        let (tran_to_step, tran_to_new) = match self.node_format() {
            GraphVizLoggerNodeFormat::AnchoredCluster => (
                GraphVizEdge::new(
                    get_anchor_id("", origin_node_id),
                    Some(get_node_id("", origin_node_id)),
                    step_node.id.clone(),
                    None,
                    edge_opts.clone(),
                ),
                GraphVizEdge::new(
                    step_node.id.clone(),
                    None,
                    get_anchor_id("", target_node_id),
                    Some(get_node_id("", target_node_id)),
                    edge_opts,
                ),
            ),
            GraphVizLoggerNodeFormat::SimpleNode => (
                GraphVizEdge::new(
                    get_node_id("", origin_node_id),
                    None,
                    step_node.id.clone(),
                    None,
                    edge_opts.clone(),
                ),
                GraphVizEdge::new(
                    step_node.id.clone(),
                    None,
                    get_node_id("", target_node_id),
                    None,
                    edge_opts,
                ),
            ),
        };

        match (
            self.nodes_id_to_process_phase_id.get(&origin_node_id),
            self.nodes_id_to_process_phase_id.get(&target_node_id),
        ) {
            (Some(orig_phase), Some(targ_phase)) if orig_phase == targ_phase => {
                let cluster = self.process_phases_clusters.get_mut(orig_phase).unwrap();
                cluster.add_node(step_node);
                cluster.add_edge(tran_to_step);
                cluster.add_edge(tran_to_new);
            }
            _ => {
                self.graph.add_node(step_node);
                self.graph.add_edge(tran_to_step);
                self.graph.add_edge(tran_to_new);
            }
        }
    }

    fn log_filtered(
        &mut self,
        ctx: &Conf::ContextAndParameterization,
        parent_node_id: u32,
        filtration_result: &Conf::FiltrationResult,
    ) {
        self.filtration_counter += 1;
        let filter_name = get_filtration_id("", self.filtration_counter);
        let img_path: PathBuf =
            [&self.configuration.temp_folder, &format!("{}.png", filter_name)]
                .iter()
                .collect();
        let filter_style = self
            .drawer
            .draw_filter(ctx, filtration_result, &img_path)
            .to_graphviz_node_styte(&img_path);
        let edge_color = self.drawer.filter_edge_color(ctx, filtration_result);

        let filter_node = GraphVizNode::new(filter_name, filter_style);
        let edge_opts = vec![
            GraphvizEdgeStyleItem::Head(GvArrowHeadStyle::Vee(GvArrowHeadSide::Both)),
            GraphvizEdgeStyleItem::Color(edge_color),
        ];
        let elim_edge = match self.node_format() {
            GraphVizLoggerNodeFormat::AnchoredCluster => GraphVizEdge::new(
                get_anchor_id("", parent_node_id),
                Some(get_node_id("", parent_node_id)),
                filter_node.id.clone(),
                None,
                edge_opts,
            ),
            GraphVizLoggerNodeFormat::SimpleNode => GraphVizEdge::new(
                get_node_id("", parent_node_id),
                None,
                filter_node.id.clone(),
                None,
                edge_opts,
            ),
        };

        if let Some(phase_id) = self.nodes_id_to_process_phase_id.get(&parent_node_id) {
            let cluster = self.process_phases_clusters.get_mut(phase_id).unwrap();
            cluster.add_node(filter_node);
            cluster.add_edge(elim_edge);
        } else {
            self.graph.add_node(filter_node);
            self.graph.add_edge(elim_edge);
        }
    }

    fn log_terminate_process(&mut self, manager: &GenericProcessManager<Conf>) {
        for (_, cluster) in self.process_phases_clusters.drain() {
            self.graph.add_cluster(cluster);
        }
        if let Some(legend) = &self.legend_writer {
            self.graph.add_node(
                legend.get_verdict_node(&manager.context_and_param, &manager.global_state),
            );
        }
        if let Err(e) = self.graph.print_dot(
            &[self.configuration.parent_folder.clone()],
            &self.configuration.output_file_name,
            &self.configuration.output_format,
        ) {
            println!("error during logger termination : {:?}", e);
        }
    }
}
