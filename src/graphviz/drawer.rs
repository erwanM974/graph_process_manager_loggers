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



use graphviz_dot_builder::item::cluster::GraphVizCluster;
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::colors::GraphvizColor;

use graph_process_manager_core::manager::config::AbstractProcessConfiguration;
use crate::graphviz::format::GraphVizLoggerNodeFormat;

pub trait GraphVizProcessDrawer<Conf : AbstractProcessConfiguration> {

    fn repr_static_analysis(&self) -> bool;

    fn get_temp_folder(&self) -> &str;

    fn get_verdict_color(&self,
                         local_verdict : &Conf::LocalVerdict) -> GraphvizColor;

    fn make_static_analysis_as_gvcluster(&self,
                                         context: &Conf::Context,
                                         param: &Conf::Parameterization,
                                         parent_state_id: u32,
                                         verdict: &Conf::LocalVerdict,
                                         data_proof : &Conf::StaticLocalVerdictAnalysisProof) -> GraphVizCluster;

    fn make_step_gvnode(&self,
                        context: &Conf::Context,
                        param: &Conf::Parameterization,
                        origin_state_id: u32,
                        target_state_id: u32,
                        step: &Conf::StepKind) -> GraphVizNode;

    fn make_node_gvitem_as_gvcluster(&self,
                                     context: &Conf::Context,
                                     param: &Conf::Parameterization,
                                   new_state_id: u32,
                                   new_node: &Conf::NodeKind) -> GraphVizCluster;

    fn make_node_gvitem_as_gvnode(&self,
                                  context: &Conf::Context,
                                   param: &Conf::Parameterization,
                                   new_state_id: u32,
                                   new_node: &Conf::NodeKind) -> GraphVizNode;

    fn get_node_format(&self) -> &GraphVizLoggerNodeFormat;

    fn get_anchor_id(&self, id : u32) -> String; // format!("a{:}", id)

    fn get_node_id(&self, id : u32) -> String; // format!("n{:}", id)

    fn get_verdict_id(&self, id : u32) -> String; // format!("v{:}", id)

    fn get_static_analysis_ids(&self, id : u32) -> (String,String); // (format!("stat{:}", id),format!("stat_anchor{:}", id))

}