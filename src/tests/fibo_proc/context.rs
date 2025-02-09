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

use graph_process_manager_core::process::{config::AbstractContextAndParameterization, persistent_state::AbstractProcessMutablePersistentState};

use super::{conf::FiboConfig, node::FiboNodeKind, step::FiboStepKind};


pub enum FiboFiltrationResult {
    MaxNumberExceeded
}
pub struct FiboContextAndParameterization {}


impl AbstractContextAndParameterization for FiboContextAndParameterization {
    fn get_process_description(&self) -> &str {
        "Fibonacci"
    }
    fn get_parameters_description(&self) -> Vec<&str> {
        vec![]
    }
}


pub struct FiboPersistentState {}

impl AbstractProcessMutablePersistentState<FiboConfig> for FiboPersistentState {
    fn get_initial_state(
        _context_and_param : &FiboContextAndParameterization
    ) -> Self {
        FiboPersistentState {}
    }

    fn update_on_node_reached(
        &mut self, 
        _context_and_param : &FiboContextAndParameterization,
        _node : &FiboNodeKind
    ) {
        // nothing
    }

    fn update_on_next_steps_collected_reached(
        &mut self, 
        _context_and_param : &FiboContextAndParameterization,
        _node : &FiboNodeKind,
        _steps : &[FiboStepKind]
    ) {
        // nothing
    }

    fn update_on_filtered(
        &mut self,
        _context_and_param : &FiboContextAndParameterization,
        _parent_node : &FiboNodeKind,
        _filtration_result : &<FiboConfig as graph_process_manager_core::process::config::AbstractProcessConfiguration>::FiltrationResult
    ) {
        // nothing
    }

    fn warrants_termination_of_the_process(
        &self, 
        _context_and_param : &FiboContextAndParameterization
    ) -> bool {
        false 
    }
}