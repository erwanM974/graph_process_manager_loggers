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
use graphviz_dot_builder::edge::edge::GraphVizEdge;
use graphviz_dot_builder::edge::style::{GraphvizEdgeStyleItem, GvArrowHeadSide, GvArrowHeadStyle};
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyle, GraphvizNodeStyleItem, GvNodeShape, GvNodeStyleKind};
use graphviz_dot_builder::traits::DotBuildable;
use graphviz_dot_builder::traits::DotPrintable;


use graph_process_manager_core::manager::logger::AbstractProcessLogger;
use graph_process_manager_core::manager::config::AbstractConfiguration;
use graph_process_manager_core::queued_steps::queue::strategy::QueueSearchStrategy;
use graph_process_manager_core::handler::filter::AbstractFilter;
use graph_process_manager_core::delegate::priorities::GenericProcessPriorities;
use graph_process_manager_core::manager::verdict::AbstractGlobalVerdict;
use graph_process_manager_core::manager::config::AbstractProcessParameterization;

use crate::graphviz_logger::format::GraphicLoggerNodeFormat;
use crate::graphviz_logger::logger::GenericGraphVizLogger;


impl<Config : AbstractConfiguration> AbstractProcessLogger<Config> for GenericGraphVizLogger<Config> {

    fn log_parameterization(&mut self,
                            strategy: &QueueSearchStrategy,
                            filters: &[Box<dyn AbstractFilter<Config::FilterCriterion, Config::FilterEliminationKind>>],
                            priorities: &GenericProcessPriorities<Config::Priorities>,
                            parameterization: &Config::ProcessParameterization) {

        if self.display_legend {
            let mut options_str : Vec<String> = parameterization.get_param_as_strings();
            options_str.push( format!("strategy={}", strategy.to_string()) );
            options_str.push( format!("priorities={}", priorities.to_string()) );
            {
                let filters_strs : Vec<String> = filters.iter()
                    .map(|f| f.to_string()).collect();
                let filters_str = format!("filters=[{}]", filters_strs.join(","));
                options_str.push( filters_str );
            }
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
                    context: &Config::ProcessContext,
                    parent_state_id: u32,
                    new_state_id: u32,
                    elim_kind: &Config::FilterEliminationKind) {
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
            elim_node = GraphVizNode::new(format!("e{:}", new_state_id),
                                          node_gv_options);
        }
        // ***
        let elim_edge : GraphVizEdge;
        {
            let tran_gv_options = vec![
                GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ),
                GraphvizEdgeStyleItem::Color( GraphvizColor::burlywood4 ) ];
            // ***
            elim_edge = GraphVizEdge::new(format!("a{:}", parent_state_id),
                                          Some(format!("n{}",parent_state_id)),
                                          elim_node.id.clone(),
                                          None,
                                          tran_gv_options);
        }
        self.graph.add_node(elim_node);
        self.graph.add_edge(elim_edge);
    }

    fn log_new_node(&mut self,
                    context: &Config::ProcessContext,
                    parameterization: &Config::ProcessParameterization,
                    new_state_id: u32,
                    new_node: &Config::NodeKind) {
        match self.process_drawer.get_node_format() {
            GraphicLoggerNodeFormat::AnchoredCluster => {
                let as_cluster = self.process_drawer.make_node_gvitem_as_gvcluster(context,
                                                                                   parameterization,
                                                                                   new_state_id,
                                                                                   new_node);
                self.graph.add_cluster(as_cluster);
            },
            GraphicLoggerNodeFormat::SimpleNode => {
                let as_node = self.process_drawer.make_node_gvitem_as_gvnode(context,
                                                                                   parameterization,
                                                                                   new_state_id,
                                                                                   new_node);
                self.graph.add_node(as_node);
            }
        }
    }

    fn log_new_transition(&mut self,
                          context: &Config::ProcessContext,
                          origin_state_id: u32,
                          target_state_id: u32,
                          step: &Config::StepKind) {
        let step_gv_node = self.process_drawer.make_step_gvnode(context,
                                                                origin_state_id,
                                                                target_state_id,
                                                                step);
        // *** Transition To Step
        match self.process_drawer.get_node_format() {
            GraphicLoggerNodeFormat::AnchoredCluster => {
                let tran_gv_options = vec![ GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) ];
                let tran_to_step = GraphVizEdge::new(self.process_drawer.get_anchor_id(origin_state_id),
                                                    Some(self.process_drawer.get_node_id(origin_state_id)),
                                                     step_gv_node.id.clone(),
                                                    None,
                                                     tran_gv_options.clone());
                let tran_to_new = GraphVizEdge::new(step_gv_node.id.clone(),
                                                    None,
                                                    self.process_drawer.get_anchor_id(target_state_id),
                                                    Some(self.process_drawer.get_node_id(target_state_id)),
                                                    tran_gv_options);
                self.graph.add_node(step_gv_node);
                self.graph.add_edge(tran_to_step);
                self.graph.add_edge(tran_to_new);
            },
            GraphicLoggerNodeFormat::SimpleNode => {
                let tran_gv_options = vec![ GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) ];
                let tran_to_step = GraphVizEdge::new(self.process_drawer.get_node_id(origin_state_id),
                                                     None,
                                                     step_gv_node.id.clone(),
                                                     None,
                                                     tran_gv_options.clone());
                let tran_to_new = GraphVizEdge::new(step_gv_node.id.clone(),
                                                    None,
                                                    self.process_drawer.get_node_id(target_state_id),
                                                    None,
                                                    tran_gv_options);
                self.graph.add_node(step_gv_node);
                self.graph.add_edge(tran_to_step);
                self.graph.add_edge(tran_to_new);
            }
        }
    }

    fn log_verdict(&mut self,
                   context: &Config::ProcessContext,
                   parent_state_id: u32,
                   verdict: &Config::LocalVerdict) {
        let verdict_color = self.process_drawer.get_verdict_color(verdict);
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
            verd_node = GraphVizNode::new(format!("v{:}", parent_state_id),node_gv_options);
        }
        // ***
        let verd_edge : GraphVizEdge;
        {
            let tran_gv_options = vec![GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ),
                                       GraphvizEdgeStyleItem::Color( verdict_color )];
            // ***
            match self.process_drawer.get_node_format() {
                GraphicLoggerNodeFormat::AnchoredCluster => {
                    verd_edge = GraphVizEdge::new(self.process_drawer.get_anchor_id(parent_state_id),
                                                  Some(self.process_drawer.get_node_id(parent_state_id)),
                                                  verd_node.id.clone(),
                                                  None,
                                                  tran_gv_options);
                },
                GraphicLoggerNodeFormat::SimpleNode => {
                    verd_edge = GraphVizEdge::new(self.process_drawer.get_node_id(parent_state_id),
                                                  None,
                                                  verd_node.id.clone(),
                                                  None,
                                                  tran_gv_options);
                }
            }
        }
        self.graph.add_node(verd_node);
        self.graph.add_edge(verd_edge);
    }

    fn log_terminate(&mut self,
                     global_verdict: &Config::GlobalVerdict) {
        if Config::GlobalVerdict::is_verdict_pertinent_for_process() && self.display_legend {
            let verd_str = format!("verdict={}", global_verdict.to_string());
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
        self.graph.print_dot(&[self.parent_folder.clone()],&self.output_file_name,&self.output_format);
    }

    fn log_notify_terminal_node_reached(&mut self,
                                        context: &Config::ProcessContext,
                                        node_id: u32) {
        // nothing
    }

    fn log_notify_last_child_of_node_processed(&mut self,
                                               context: &Config::ProcessContext,
                                               parent_node_id: u32) {
        // nothing
    }
}