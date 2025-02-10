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

use crate::tests::tree_proc::node::TreeNodeKind;
use crate::tests::tree_proc::step::TreeStepKind;

use super::conf::TreeConfig;
use super::context::TreeContextAndParameterization;


pub struct TreeProcessHandler {}

impl AbstractAlgorithmOperationHandler<TreeConfig> for TreeProcessHandler {

    fn process_new_step(
        _context_and_param : &TreeContextAndParameterization,
        parent_node : &TreeNodeKind,
        step_to_process : &TreeStepKind
    ) -> TreeNodeKind {
        match &step_to_process {
            TreeStepKind::A => {
                let new_word = format!("{}A",parent_node.word);
                TreeNodeKind::new(new_word)
            },
            TreeStepKind::B => {
                let new_word = format!("{}B",parent_node.word);
                TreeNodeKind::new(new_word)
            }
        }
    }

    fn collect_next_steps(
        _context_and_param : &TreeContextAndParameterization,
        _parent_node : &TreeNodeKind
    ) -> Vec<TreeStepKind> {
        vec![TreeStepKind::A, TreeStepKind::B]
    }

}