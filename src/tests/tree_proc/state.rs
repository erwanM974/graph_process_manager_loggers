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

use graph_process_manager_core::process::persistent_state::AbstractProcessMutablePersistentState;

use super::{conf::TreeConfig, context::TreeContextAndParameterization, filtration::TreeFiltrationResult, node::TreeNodeKind, step::TreeStepKind};


pub struct TreePersistentState {
    pub node_count : u32
}

impl AbstractProcessMutablePersistentState<TreeConfig> for TreePersistentState {
    fn get_initial_state(
        _context_and_param : &TreeContextAndParameterization
    ) -> Self {
        TreePersistentState {node_count : 0}
    }

    fn update_on_node_reached(
        &mut self, 
        _context_and_param : &TreeContextAndParameterization,
        _node : &TreeNodeKind
    ) {
        self.node_count += 1;
    }

    fn update_on_next_steps_collected_reached(
        &mut self, 
        _context_and_param : &TreeContextAndParameterization,
        _node : &TreeNodeKind,
        _steps : &[TreeStepKind]
    ) {
        // nothing
    }

    fn update_on_filtered(
        &mut self,
        _context_and_param : &TreeContextAndParameterization,
        _parent_node : &TreeNodeKind,
        _filtration_result : &TreeFiltrationResult
    ) {
        // nothing
    }

    fn warrants_termination_of_the_process(
        &self, 
        _context_and_param : &TreeContextAndParameterization
    ) -> bool {
        false 
    }
}