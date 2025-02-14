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


use std::any::Any;
use std::fs;

use graphviz_dot_builder::edge::edge::GraphVizEdge;
use graphviz_dot_builder::edge::style::{GraphvizEdgeStyleItem, GvArrowHeadSide, GvArrowHeadStyle};
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyleItem, GvNodeStyleKind};
use graphviz_dot_builder::traits::DotBuildable;
use graphviz_dot_builder::traits::DotPrintable;


use graph_process_manager_core::queue::priorities::GenericProcessPriorities;
use graph_process_manager_core::queue::strategy::QueueSearchStrategy;
use graph_process_manager_core::process::logger::AbstractProcessLogger;
use graph_process_manager_core::process::config::AbstractProcessConfiguration;
use graph_process_manager_core::process::filter::GenericFiltersManager;
use graphviz_dot_builder::item::cluster::GraphVizCluster;
use graphviz_dot_builder::item::item::GraphVizGraphItem;

use crate::graphviz::format::GraphVizLoggerNodeFormat;
use crate::graphviz::logger::GenericGraphVizLogger;

use crate::graphviz::util::*;

use super::drawers::all_the_rest_drawer::{get_filtration_result_as_gvnode_style_and_edge_color, get_step_gvnode_style_and_edge_color};
use super::drawers::node_drawer::*;


impl<Conf : AbstractProcessConfiguration + 'static> AbstractProcessLogger<Conf> for GenericGraphVizLogger<Conf> {

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn log_initialize(
        &mut self,
        context_and_param : &Conf::ContextAndParameterization,
        strategy: &QueueSearchStrategy,
        priorities: &GenericProcessPriorities<Conf::Priorities>,
        filters_manager : &GenericFiltersManager<Conf>,
        _initial_global_state : &Conf::MutablePersistentState,
        use_memoization : bool,
    ) {
        // empties temp directory if exists
        let _ = fs::remove_dir_all(&self.configuration.temp_folder);
        // creates temp directory if not exist
        let _ = fs::create_dir_all(&self.configuration.temp_folder);
        // creates parent directory if not exist
        if self.configuration.parent_folder != *"" {
            let _ = fs::create_dir_all(&self.configuration.parent_folder);
        }
        // creates legend
        if self.configuration.display_legend {
            let legend_node = self.legend_writer.get_legend_node(
                context_and_param, 
                strategy, 
                priorities, 
                filters_manager, 
                use_memoization
            );
            self.graph.add_node(legend_node);
        }
    }




    fn log_new_node(
        &mut self,
        context_and_param : &Conf::ContextAndParameterization,
        new_node_id : u32,
        new_node : &Conf::DomainSpecificNode
    ) {
        let as_gv_item = match self.get_node_format() {
            GraphVizLoggerNodeFormat::AnchoredCluster => {
                let cluster_name = get_node_id("", new_node_id);
                let (cluster_style,mut cluster_nodes) = get_node_as_gvcluster_style(
                    &self.node_drawers,
                    &self.configuration.temp_folder,
                    context_and_param,
                    new_node,
                    &cluster_name
                );
                let anchor_node = GraphVizNode::new(
                    get_anchor_id("",new_node_id),
                      vec![GraphvizNodeStyleItem::Label("".to_string()),
                           GraphvizNodeStyleItem::Style(vec![GvNodeStyleKind::Invis]),
                           GraphvizNodeStyleItem::Peripheries(0),
                           GraphvizNodeStyleItem::Height(0),GraphvizNodeStyleItem::Width(0)
                      ]
                );
                cluster_nodes.insert(
                    cluster_nodes.len()/2,
                    Box::new(GraphVizGraphItem::Node(anchor_node))
                );
                // ***
                let as_gv_cluster = GraphVizCluster::new(
                    cluster_name,
                    cluster_style,
                    cluster_nodes,
                    vec![]
                );
                // ***
                GraphVizGraphItem::Cluster(as_gv_cluster)
            },
            GraphVizLoggerNodeFormat::SimpleNode => {
                let node_name = get_node_id("",new_node_id);
                let node_style = get_node_as_gvnode_style(
                    &self.node_drawers,
                    &self.configuration.temp_folder,
                    context_and_param,
                    new_node,
                    &node_name
                );
                let as_gv_node = GraphVizNode::new(
                    node_name,
                    node_style
                );
                // ***
                GraphVizGraphItem::Node(as_gv_node)
            }
        };

        if let Some(phase_id) = self.all_the_rest_drawer.get_node_phase_id(context_and_param, new_node) {
            if let std::collections::hash_map::Entry::Vacant(e) = self.process_phases_clusters.entry(
                phase_id
            ) {
                let phase_cluster_style = vec![
                    GraphvizNodeStyleItem::FillColor(self.all_the_rest_drawer.get_phase_color(phase_id))
                ];
                let new_phase_cluster = GraphVizCluster::new(
                    format!("phase{}",phase_id), 
                    phase_cluster_style, 
                    vec![], 
                    vec![]
                );
                e.insert(new_phase_cluster);
            }
            let phase_cluster = self.process_phases_clusters.get_mut(&phase_id).unwrap();
            phase_cluster.items.push(Box::new(as_gv_item));
            self.nodes_id_to_process_phase_id.insert(new_node_id, phase_id);
        } else {
            match as_gv_item {
                GraphVizGraphItem::Node(graph_viz_node) => {
                self.graph.add_node(graph_viz_node);
                },
                GraphVizGraphItem::Cluster(graph_viz_cluster) => {
                    self.graph.add_cluster(graph_viz_cluster);
                }
            }
        }
    }

    fn log_new_step(
        &mut self,
        context_and_param : &Conf::ContextAndParameterization,
        origin_node_id: u32,
        step: &Conf::DomainSpecificStep,
        target_node_id: u32,
        _target_node : &Conf::DomainSpecificNode
    ) {
        let step_name = get_step_id("",origin_node_id,target_node_id);
        let (step_style,edge_color) = get_step_gvnode_style_and_edge_color(
            &*self.all_the_rest_drawer,
            &self.configuration.temp_folder,
            context_and_param,
            step,
            &step_name
        );
        let step_gv_node = GraphVizNode::new(
            step_name,
            step_style
        );
        // *** Transition To Step
        let (tran_to_step,tran_to_new) = match self.get_node_format() {
            GraphVizLoggerNodeFormat::AnchoredCluster => {
                let tran_gv_options = vec![
                    GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ),
                    GraphvizEdgeStyleItem::Color(edge_color)
                ];
                let tran_to_step = GraphVizEdge::new(
                    get_anchor_id("",origin_node_id),
                    Some(get_node_id("",origin_node_id)),
                    step_gv_node.id.clone(),
                    None,
                    tran_gv_options.clone()
                );
                let tran_to_new = GraphVizEdge::new(
                    step_gv_node.id.clone(),
                    None,
                    get_anchor_id("",target_node_id),
                    Some(get_node_id("",target_node_id)),
                    tran_gv_options
                );
                (tran_to_step,tran_to_new)
            },
            GraphVizLoggerNodeFormat::SimpleNode => {
                let tran_gv_options = vec![
                    GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ),
                    GraphvizEdgeStyleItem::Color(edge_color)
                ];
                let tran_to_step = GraphVizEdge::new(
                    get_node_id("",origin_node_id),
                    None,
                    step_gv_node.id.clone(),
                    None,
                    tran_gv_options.clone()
                );
                let tran_to_new = GraphVizEdge::new(
                    step_gv_node.id.clone(),
                    None,
                    get_node_id("",target_node_id),
                    None,
                    tran_gv_options
                );
                (tran_to_step,tran_to_new)
            }
        };
        // ***
        match (
            self.nodes_id_to_process_phase_id.get(&origin_node_id),
            self.nodes_id_to_process_phase_id.get(&target_node_id)
        ) {
            (Some(origin_phase_id),Some(target_phase_id)) => {
                if origin_phase_id == target_phase_id {
                    let cluster = self.process_phases_clusters.get_mut(origin_phase_id).unwrap();
                    cluster.add_node(step_gv_node);
                    cluster.add_edge(tran_to_step);
                    cluster.add_edge(tran_to_new);
                } else {
                    self.graph.add_node(step_gv_node);
                    self.graph.add_edge(tran_to_step);
                    self.graph.add_edge(tran_to_new);
                }
            },
            (_,_) => {
                self.graph.add_node(step_gv_node);
                self.graph.add_edge(tran_to_step);
                self.graph.add_edge(tran_to_new);
            }
        }

    }


    fn log_filtered(
        &mut self,
        context_and_param : &Conf::ContextAndParameterization,
        parent_node_id : u32,
        filtration_result_id : u32,
        filtration_result : &Conf::FiltrationResult
    ) {
        let filtration_node_name = get_filtration_id("", filtration_result_id);
        let (filter_node_style, edge_color) = get_filtration_result_as_gvnode_style_and_edge_color(
            &*self.all_the_rest_drawer,
            &self.configuration.temp_folder,
            context_and_param,
            filtration_result, 
            &filtration_node_name
        );
        let filtration_node = GraphVizNode::new(filtration_node_name,filter_node_style);
        // ***
        let elim_edge =
        {
            let tran_gv_options = vec![
                GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ),
                GraphvizEdgeStyleItem::Color( edge_color ) ];
            // ***
            match self.get_node_format() {
                GraphVizLoggerNodeFormat::AnchoredCluster => {
                    GraphVizEdge::new(
                        get_anchor_id("",parent_node_id),
                        Some(get_node_id("",parent_node_id)),
                        filtration_node.id.clone(),
                        None,
                        tran_gv_options
                    )
                },
                GraphVizLoggerNodeFormat::SimpleNode => {
                    GraphVizEdge::new(
                        get_node_id("",parent_node_id),
                        None,
                        filtration_node.id.clone(),
                        None,
                        tran_gv_options
                    )
                }
            }
        };
        if let Some(phase_id) = self.nodes_id_to_process_phase_id.get(&parent_node_id) {
            let cluster = self.process_phases_clusters.get_mut(phase_id).unwrap();
            cluster.add_node(filtration_node);
            cluster.add_edge(elim_edge);
        } else {
            self.graph.add_node(filtration_node);
            self.graph.add_edge(elim_edge);
        }
    }


    fn log_terminate_process(
        &mut self,
        context_and_param : &Conf::ContextAndParameterization,
        final_global_state : &Conf::MutablePersistentState
    ) {
        for (_,cluster) in self.process_phases_clusters.drain() {
            self.graph.add_cluster(cluster);
        }
        if self.configuration.display_legend {
            self.graph.add_node(self.legend_writer.get_verdict_node(context_and_param, final_global_state));
        }
        if let Err(e) = self.graph.print_dot(
            &[self.configuration.parent_folder.clone()],
            &self.configuration.output_file_name,
            &self.configuration.output_format
        ) {
            println!("error during logger termination : {:?} ", e);
        }
    }

    fn log_notify_node_without_children(
        &mut self,
        _context_and_param : &Conf::ContextAndParameterization,
        _node_id: u32
    ) {
        // nothing
    }

    fn log_notify_last_child_step_of_node_processed(
        &mut self,
        _context_and_param : &Conf::ContextAndParameterization,
        _parent_node_id: u32
    ) {
        // nothing
    }
}