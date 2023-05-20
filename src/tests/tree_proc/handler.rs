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



use graph_process_manager_core::delegate::node::GenericNode;
use graph_process_manager_core::handler::handler::AbstractProcessHandler;
use graph_process_manager_core::queued_steps::step::GenericStep;
use crate::tests::tree_proc::conf::{TreeConfig, TreeStaticProof};
use crate::tests::tree_proc::context::{TreeContext, TreeParameterization};
use crate::tests::tree_proc::filter::filter::TreeFilterCriterion;
use crate::tests::tree_proc::node::TreeNodeKind;
use crate::tests::tree_proc::step::TreeStepKind;
use crate::tests::tree_proc::verdict::local::TreeLocalVerdict;


pub struct TreeProcessHandler {}

impl AbstractProcessHandler<TreeConfig> for TreeProcessHandler {

    fn process_new_step(_context: &TreeContext,
                        _param : &TreeParameterization,
                        parent_node: &GenericNode<TreeNodeKind>,
                        step_to_process: &GenericStep<TreeStepKind>,
                        _new_node_id: u32,
                        _node_counter: u32) -> TreeNodeKind {
        match &step_to_process.kind {
            TreeStepKind::A => {
                let new_word = format!("{}A",parent_node.kind.word);
                TreeNodeKind::new(new_word)
            },
            TreeStepKind::B => {
                let new_word = format!("{}B",parent_node.kind.word);
                TreeNodeKind::new(new_word)
            }
        }
    }

    fn get_criterion(_context: &TreeContext,
                     _param : &TreeParameterization,
                     _parent_node: &GenericNode<TreeNodeKind>,
                     _step_to_process: &GenericStep<TreeStepKind>,
                     _new_node_id: u32,
                     _node_counter: u32) -> TreeFilterCriterion {
        TreeFilterCriterion{}
    }

    fn collect_next_steps(_context: &TreeContext,
                          _param : &TreeParameterization,
                          _parent_node_kind: &TreeNodeKind)
                -> Vec<TreeStepKind> {
        vec![TreeStepKind::A, TreeStepKind::B]
    }

    fn get_local_verdict_when_no_child(_context: &TreeContext,
                                       _param: &TreeParameterization,
                                       _node_kind: &TreeNodeKind) -> TreeLocalVerdict {
        TreeLocalVerdict{}
    }

    fn get_local_verdict_from_static_analysis(_context: &TreeContext,
                                              _param: &TreeParameterization,
                                              _node_kind: &mut TreeNodeKind) -> Option<(TreeLocalVerdict, TreeStaticProof)> {
        None
    }

    fn pursue_process_after_static_verdict(_context: &TreeContext,
                                           _param: &TreeParameterization,
                                           _loc_verd: &TreeLocalVerdict) -> bool {
        true
    }
}