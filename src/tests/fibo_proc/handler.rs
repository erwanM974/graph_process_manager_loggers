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



use graph_process_manager_core::process::handler::AbstractAlgorithmOperationHandler;

use crate::tests::fibo_proc::conf::FiboConfig;
use crate::tests::fibo_proc::node::FiboNodeKind;
use crate::tests::fibo_proc::step::FiboStepKind;

use super::context::FiboContextAndParameterization;
use super::state::FiboPersistentState;


pub struct FiboProcessHandler {}

impl AbstractAlgorithmOperationHandler<FiboConfig> for FiboProcessHandler {

    fn process_new_step(
        _context_and_param : &FiboContextAndParameterization,
        _global_state : &mut FiboPersistentState,
        parent_node : &FiboNodeKind,
        step_to_process : &FiboStepKind
    ) -> FiboNodeKind {
        match &step_to_process {
            FiboStepKind::Next => {
                let new_current = parent_node.next;
                let new_next = parent_node.current + parent_node.next;
                FiboNodeKind::new(new_current,new_next)
            }
        }
    }

    fn collect_next_steps(
        _context_and_param : &FiboContextAndParameterization,
        _global_state : &FiboPersistentState,
        _parent_node : &FiboNodeKind
    ) -> Vec<FiboStepKind> {
        vec![FiboStepKind::Next]
    }

}