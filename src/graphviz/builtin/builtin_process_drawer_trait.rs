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

use graph_process_manager_core::manager::config::AbstractProcessConfiguration;
use graphviz_dot_builder::colors::GraphvizColor;

use crate::graphviz::builtin::node_drawer::CustomNodeDrawerForGraphvizLogger;
use crate::graphviz::builtin::proof_drawer::CustomProofDrawerForGraphvizLogger;
use crate::graphviz::builtin::step_drawer::CustomStepDrawerForGraphvizLogger;

pub trait BuiltinProcessDrawer<Conf : AbstractProcessConfiguration> {

    fn get_node_drawers(&self) -> &Vec<Box<dyn CustomNodeDrawerForGraphvizLogger<Conf>>>;

    fn get_step_drawer(&self) -> &Box<dyn CustomStepDrawerForGraphvizLogger<Conf>>;

    fn get_proof_drawer(&self) -> Option<&Box<dyn CustomProofDrawerForGraphvizLogger<Conf>>>;

    fn get_temp_folder(&self) -> &str;

    fn get_verdict_color(&self, local_verdict: &Conf::LocalVerdict) -> GraphvizColor;

    fn get_node_phase_id(&self,
        context: &Conf::Context,
        param: &Conf::Parameterization,
        new_node: &Conf::NodeKind) -> Option<u32>;
        
    /**
     The colors of the background of the cluster in which to draw a specific phase of the process.
    **/
    fn get_phase_color(&self, phase_id : u32) -> GraphvizColor;
}

