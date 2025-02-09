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

use graph_process_manager_core::process::config::AbstractProcessConfiguration;
use graphviz_dot_builder::colors::GraphvizColor;

use crate::graphviz::builtin::node_drawer::CustomNodeDrawerForGraphvizLogger;
use crate::graphviz::builtin::filtration_drawer::FiltrationDrawerForGraphvizLogger;
use crate::graphviz::builtin::step_drawer::CustomStepDrawerForGraphvizLogger;

pub trait BuiltinProcessDrawer<Conf : AbstractProcessConfiguration> {

    fn get_node_drawers(&self) -> &Vec<Box<dyn CustomNodeDrawerForGraphvizLogger<Conf>>>;

    fn get_step_drawer(&self) -> &dyn CustomStepDrawerForGraphvizLogger<Conf>;

    fn get_filter_drawer(&self) -> &dyn FiltrationDrawerForGraphvizLogger<Conf>;

    fn get_final_global_state_description_for_legend(
        &self, 
        context_and_param: &Conf::ContextAndParameterization,
        final_state : &Conf::MutablePersistentState
    ) -> Vec<String>;

    fn get_temp_folder(&self) -> &str;

    fn get_node_phase_id(
        &self,
        context_and_param : &Conf::ContextAndParameterization,
        new_node: &Conf::DomainSpecificNode
    ) -> Option<u32>;
        
    /**
     The colors of the background of the cluster in which to draw a specific phase of the process.
    **/
    fn get_phase_color(&self, phase_id : u32) -> GraphvizColor;
}

