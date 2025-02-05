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
use graphviz_dot_builder::colors::GraphvizColor;
use graphviz_dot_builder::edge::edge::GraphVizEdge;
use graphviz_dot_builder::edge::style::{GraphvizEdgeStyleItem, GvArrowHeadSide, GvArrowHeadStyle};
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyle, GraphvizNodeStyleItem, GvNodeShape, GvNodeStyleKind};
use graphviz_dot_builder::traits::DotBuildable;
use graphviz_dot_builder::traits::DotPrintable;


use graph_process_manager_core::manager::logger::AbstractProcessLogger;
use graph_process_manager_core::manager::config::{AbstractProcessConfiguration, AbstractProcessParameterization};
use graph_process_manager_core::queued_steps::queue::strategy::QueueSearchStrategy;
use graph_process_manager_core::handler::filter::AbstractFilter;
use graph_process_manager_core::delegate::priorities::GenericProcessPriorities;
use graph_process_manager_core::manager::verdict::AbstractGlobalVerdict;
use graphviz_dot_builder::item::cluster::GraphVizCluster;
use graphviz_dot_builder::item::item::GraphVizGraphItem;

use crate::graphviz::format::GraphVizLoggerNodeFormat;
use crate::graphviz::logger::GenericGraphVizLogger;


impl<Conf : AbstractProcessConfiguration + 'static> AbstractProcessLogger<Conf> for GenericGraphVizLogger<Conf> {

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn log_initialize(&mut self) {
        // empties temp directory if exists
        let _ = fs::remove_dir_all(self.drawer.get_temp_folder());
        // creates temp directory if not exist
        let _ = fs::create_dir_all(self.drawer.get_temp_folder());
        // creates parent directory if not exist
        if self.parent_folder != *"" {
            let _ = fs::create_dir_all(&self.parent_folder);
        }
    }

    fn log_parameterization(&mut self,
                            strategy: &QueueSearchStrategy,
                            priorities: &GenericProcessPriorities<Conf::Priorities>,
                            filters: &[Box<dyn AbstractFilter<Conf::FilterCriterion, Conf::FilterEliminationKind>>],
                            goal : &Option<Conf::GlobalVerdict>,
                            memoize : bool,
                            parameterization: &Conf::Parameterization) {

        if self.display_legend {
            let mut options_str : Vec<String> = parameterization.get_param_as_strings();
            options_str.push( format!("strategy={}", strategy) );
            options_str.push( format!("priorities={}", priorities) );
            {
                let filters_strs : Vec<String> = filters.iter()
                    .map(|f| f.to_string()).collect();
                let filters_str = format!("filters=[{}]", filters_strs.join(","));
                options_str.push( filters_str );
            }
            match goal {
                None => {
                    options_str.push( "goal=None".to_string() );
                },
                Some(ref target_verdict) => {
                    options_str.push( format!("goal={}", target_verdict) );
                }
            }
            options_str.push( format!("memoize={}", memoize) );
            // ***
            let legend_node_gv_options : GraphvizNodeStyle = vec![
                GraphvizNodeStyleItem::Label( options_str.join("\n") ),
                GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle),
                GraphvizNodeStyleItem::Style(vec![GvNodeStyleKind::Bold,GvNodeStyleKind::Rounded]),
                GraphvizNodeStyleItem::FontSize( 18 )];
            // ***
            let param_node = GraphVizNode::new("param".to_string(),
                                                legend_node_gv_options);
            self.graph.add_node(param_node);
        }
    }

    fn log_filtered(&mut self,
                    _context: &Conf::Context,
                    parent_node_id: u32,
                    new_node_id: u32,
                    elim_kind: &Conf::FilterEliminationKind) {
        let elim_node : GraphVizNode;
        {
            let node_gv_options : GraphvizNodeStyle = vec![
                GraphvizNodeStyleItem::Label( elim_kind.to_string() ),
                GraphvizNodeStyleItem::Color( GraphvizColor::burlywood4 ),
                GraphvizNodeStyleItem::FontColor( GraphvizColor::beige ),
                GraphvizNodeStyleItem::FontSize( 16 ),
                GraphvizNodeStyleItem::FontName( "times-bold".to_string() ),
                GraphvizNodeStyleItem::Shape(GvNodeShape::Pentagon),
                GraphvizNodeStyleItem::Style(vec![GvNodeStyleKind::Filled])];
            // ***
            elim_node = GraphVizNode::new(format!("e{:}", new_node_id),
                                          node_gv_options);
        }
        // ***
        let elim_edge : GraphVizEdge;
        {
            let tran_gv_options = vec![
                GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ),
                GraphvizEdgeStyleItem::Color( GraphvizColor::burlywood4 ) ];
            // ***
            match self.drawer.get_node_format() {
                GraphVizLoggerNodeFormat::AnchoredCluster => {
                    elim_edge = GraphVizEdge::new(self.drawer.get_anchor_id(parent_node_id),
                                                  Some(self.drawer.get_node_id(parent_node_id)),
                                                  elim_node.id.clone(),
                                                  None,
                                                  tran_gv_options);
                },
                GraphVizLoggerNodeFormat::SimpleNode => {
                    elim_edge = GraphVizEdge::new(self.drawer.get_node_id(parent_node_id),
                                                  None,
                                                  elim_node.id.clone(),
                                                  None,
                                                  tran_gv_options);
                }
            }
        }
        if let Some(phase_id) = self.nodes_id_to_process_phase_id.get(&parent_node_id) {
            let cluster = self.process_phases_clusters.get_mut(phase_id).unwrap();
            cluster.add_node(elim_node);
            cluster.add_edge(elim_edge);
        } else {
            self.graph.add_node(elim_node);
            self.graph.add_edge(elim_edge);
        }
    }

    fn log_new_node(&mut self,
                    context: &Conf::Context,
                    param: &Conf::Parameterization,
                    new_node_id: u32,
                    new_node: &Conf::NodeKind) {
        match self.drawer.get_node_format() {
            GraphVizLoggerNodeFormat::AnchoredCluster => {
                let cluster_name = self.drawer.get_node_id(new_node_id);
                let (cluster_style,mut cluster_nodes) = self.drawer.get_node_as_gvcluster_style(
                    context,
                    param,
                    new_node,
                    &cluster_name
                );
                let anchor_node = GraphVizNode::new(
                    self.drawer.get_anchor_id(new_node_id),
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
                if let Some(phase_id) = self.drawer.get_node_phase_id(context, param, new_node) {
                    let phase_cluster_style = vec![GraphvizNodeStyleItem::FillColor(self.drawer.get_phase_color(phase_id))];
                    let mut phase_cluster = GraphVizCluster::new(format!("phase{}",phase_id), phase_cluster_style, vec![], vec![]);
                    phase_cluster.add_cluster(as_gv_cluster);
                    self.process_phases_clusters.insert(phase_id, phase_cluster);
                    self.nodes_id_to_process_phase_id.insert(new_node_id, phase_id);
                } else {
                    self.graph.add_cluster(as_gv_cluster);
                }
            },
            GraphVizLoggerNodeFormat::SimpleNode => {
                let node_name = self.drawer.get_node_id(new_node_id);
                let node_style = self.drawer.get_node_as_gvnode_style(
                    context,
                    param,
                    new_node,
                    &node_name
                );
                let as_gv_node = GraphVizNode::new(
                    node_name,
                    node_style
                );
                // ***
                if let Some(phase_id) = self.drawer.get_node_phase_id(context, param, new_node) {
                    if !self.process_phases_clusters.contains_key(&phase_id) {
                        let phase_cluster_style = vec![GraphvizNodeStyleItem::FillColor(self.drawer.get_phase_color(phase_id))];
                        let new_phase_cluster = GraphVizCluster::new(format!("phase{}",phase_id), phase_cluster_style, vec![], vec![]);
                        self.process_phases_clusters.insert(phase_id, new_phase_cluster);
                    }
                    let phase_cluster = self.process_phases_clusters.get_mut(&phase_id).unwrap();
                    phase_cluster.add_node(as_gv_node);
                    self.nodes_id_to_process_phase_id.insert(new_node_id, phase_id);
                } else {
                    self.graph.add_node(as_gv_node);
                }
            }
        }
    }

    fn log_new_step(&mut self,
                    context: &Conf::Context,
                    param : &Conf::Parameterization,
                    origin_node_id: u32,
                    target_node_id: u32,
                    step: &Conf::StepKind,
                    _target_node : &Conf::NodeKind,
                    _target_depth : u32) {
        let step_name = self.drawer.get_step_id(origin_node_id,target_node_id);
        let step_style = self.drawer.get_step_gvnode_style(context,
                                                           param,
                                                           step,
                                                           &step_name
        );
        let step_gv_node = GraphVizNode::new(
            step_name,
            step_style
        );
        // *** Transition To Step
        let (tran_to_step,tran_to_new) = match self.drawer.get_node_format() {
            GraphVizLoggerNodeFormat::AnchoredCluster => {
                let tran_gv_options = vec![
                    GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) )
                ];
                let tran_to_step = GraphVizEdge::new(self.drawer.get_anchor_id(origin_node_id),
                                                    Some(self.drawer.get_node_id(origin_node_id)),
                                                     step_gv_node.id.clone(),
                                                    None,
                                                     tran_gv_options.clone());
                let tran_to_new = GraphVizEdge::new(step_gv_node.id.clone(),
                                                    None,
                                                    self.drawer.get_anchor_id(target_node_id),
                                                    Some(self.drawer.get_node_id(target_node_id)),
                                                    tran_gv_options);
                (tran_to_step,tran_to_new)
            },
            GraphVizLoggerNodeFormat::SimpleNode => {
                let tran_gv_options = vec![
                    GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) )
                ];
                let tran_to_step = GraphVizEdge::new(self.drawer.get_node_id(origin_node_id),
                                                     None,
                                                     step_gv_node.id.clone(),
                                                     None,
                                                     tran_gv_options.clone());
                let tran_to_new = GraphVizEdge::new(step_gv_node.id.clone(),
                                                    None,
                                                    self.drawer.get_node_id(target_node_id),
                                                    None,
                                                    tran_gv_options);
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

    fn log_verdict_on_static_analysis(&mut self,
                                      context: &Conf::Context,
                                      param : &Conf::Parameterization,
                                      parent_node_id: u32,
                                      verdict: &Conf::LocalVerdict,
                                      proof_data : &Conf::StaticLocalVerdictAnalysisProof) {
        if self.drawer.repr_static_analysis() {
            let static_analysis_name = self.drawer.get_static_analysis_id(parent_node_id);
            let static_analysis_style = self.drawer.get_static_analysis_as_gvnode_style(
                context,
                param,
                verdict,
                proof_data,
                &static_analysis_name
            );
            let analysis_node = GraphVizNode::new(
                static_analysis_name,
                static_analysis_style
            );
            let verdict_color = self.drawer.get_verdict_color(verdict);
            // ***
            let verd_node : GraphVizNode;
            {
                let node_gv_options : GraphvizNodeStyle = vec![
                    GraphvizNodeStyleItem::Label( verdict.to_string() ),
                    GraphvizNodeStyleItem::Color( verdict_color.clone() ),
                    GraphvizNodeStyleItem::FontColor( GraphvizColor::beige ),
                    GraphvizNodeStyleItem::FontSize( 16 ),
                    GraphvizNodeStyleItem::FontName( "times-bold".to_string() ),
                    GraphvizNodeStyleItem::Shape(GvNodeShape::Diamond),
                    GraphvizNodeStyleItem::Style(vec![GvNodeStyleKind::Filled])];
                // ***
                verd_node = GraphVizNode::new(self.drawer.get_verdict_id(parent_node_id),node_gv_options);
            }
            // ***
            let tran_gv_options = vec![GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ),
                                       GraphvizEdgeStyleItem::Color( verdict_color )];
            // ***
            let to_ana_edge : GraphVizEdge = match self.drawer.get_node_format() {
                GraphVizLoggerNodeFormat::AnchoredCluster => {
                    GraphVizEdge::new(self.drawer.get_anchor_id(parent_node_id),
                                      Some(self.drawer.get_node_id(parent_node_id)),
                                      analysis_node.id.clone(),
                                      None,
                                      tran_gv_options.clone())
                },
                GraphVizLoggerNodeFormat::SimpleNode => {
                    GraphVizEdge::new(self.drawer.get_node_id(parent_node_id),
                                      None,
                                      analysis_node.id.clone(),
                                      None,
                                      tran_gv_options.clone())
                }
            };
            let to_verd_edge = GraphVizEdge::new(
                analysis_node.id.clone(),
                None,
                verd_node.id.clone(),
                None,
                tran_gv_options);
            // ***
            if let Some(phase_id) = self.nodes_id_to_process_phase_id.get(&parent_node_id) {
                let cluster = self.process_phases_clusters.get_mut(phase_id).unwrap();
                cluster.add_node(analysis_node);
                cluster.add_node(verd_node);
                cluster.add_edge(to_ana_edge);
                cluster.add_edge(to_verd_edge);
            } else {
                self.graph.add_node(analysis_node);
                self.graph.add_node(verd_node);
                self.graph.add_edge(to_ana_edge);
                self.graph.add_edge(to_verd_edge);
            }
        } else {
            self.log_verdict_on_no_child(context,param,parent_node_id,verdict);
        }
    }

    fn log_verdict_on_no_child(&mut self,
                   _context: &Conf::Context,
                   _param : &Conf::Parameterization,
                   parent_node_id: u32,
                   verdict: &Conf::LocalVerdict) {
        let verdict_color = self.drawer.get_verdict_color(verdict);
        // ***
        let verd_node : GraphVizNode;
        {
            let node_gv_options : GraphvizNodeStyle = vec![
                GraphvizNodeStyleItem::Label( verdict.to_string() ),
                GraphvizNodeStyleItem::Color( verdict_color.clone() ),
                GraphvizNodeStyleItem::FontColor( GraphvizColor::beige ),
                GraphvizNodeStyleItem::FontSize( 16 ),
                GraphvizNodeStyleItem::FontName( "times-bold".to_string() ),
                GraphvizNodeStyleItem::Shape(GvNodeShape::Diamond),
                GraphvizNodeStyleItem::Style(vec![GvNodeStyleKind::Filled])];
            // ***
            verd_node = GraphVizNode::new(format!("v{:}", parent_node_id),node_gv_options);
        }
        // ***
        let verd_edge : GraphVizEdge;
        {
            let tran_gv_options = vec![GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ),
                                       GraphvizEdgeStyleItem::Color( verdict_color )];
            // ***
            match self.drawer.get_node_format() {
                GraphVizLoggerNodeFormat::AnchoredCluster => {
                    verd_edge = GraphVizEdge::new(self.drawer.get_anchor_id(parent_node_id),
                                                  Some(self.drawer.get_node_id(parent_node_id)),
                                                  verd_node.id.clone(),
                                                  None,
                                                  tran_gv_options);
                },
                GraphVizLoggerNodeFormat::SimpleNode => {
                    verd_edge = GraphVizEdge::new(self.drawer.get_node_id(parent_node_id),
                                                  None,
                                                  verd_node.id.clone(),
                                                  None,
                                                  tran_gv_options);
                }
            }
        }
        // ***
        if let Some(phase_id) = self.nodes_id_to_process_phase_id.get(&parent_node_id) {
            let cluster = self.process_phases_clusters.get_mut(phase_id).unwrap();
            cluster.add_node(verd_node);
            cluster.add_edge(verd_edge);
        } else {
            self.graph.add_node(verd_node);
            self.graph.add_edge(verd_edge);
        }
    }

    fn log_terminate(&mut self,
                     global_verdict: &Conf::GlobalVerdict) {
        for (_,cluster) in self.process_phases_clusters.drain() {
            self.graph.add_cluster(cluster);
        }
        if Conf::GlobalVerdict::is_verdict_pertinent_for_process() && self.display_legend {
            let verd_str = format!("verdict={}", global_verdict);
            let legend_node_gv_options : GraphvizNodeStyle = vec![
                GraphvizNodeStyleItem::Label( verd_str ),
                GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle),
                GraphvizNodeStyleItem::Style(vec![GvNodeStyleKind::Bold,GvNodeStyleKind::Rounded]),
                GraphvizNodeStyleItem::FontSize( 18 )];
            // ***
            let verd_node = GraphVizNode::new("verdict".to_string(),
                                               legend_node_gv_options);
            self.graph.add_node(verd_node);
        }
        if let Err(e) = self.graph.print_dot(&[self.parent_folder.clone()],
                                             &self.output_file_name,
                                             &self.output_format) {
            println!("error during logger termination : {:?} ", e);
        }
    }

    fn log_notify_terminal_node_reached(&mut self,
                                        _context: &Conf::Context,
                                        _node_id: u32) {
        // nothing
    }

    fn log_notify_last_child_of_node_processed(&mut self,
                                               _context: &Conf::Context,
                                               _parent_node_id: u32) {
        // nothing
    }
}