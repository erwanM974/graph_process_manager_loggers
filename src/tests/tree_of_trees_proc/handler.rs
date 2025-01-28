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
use crate::tests::tree_of_trees_proc::conf::{TreeOfTreesConfig, TreeOfTreesStaticProof};
use crate::tests::tree_of_trees_proc::context::{TreeOfTreesContext, TreeOfTreesParameterization};
use crate::tests::tree_of_trees_proc::filter::filter::TreeOfTreesFilterCriterion;
use crate::tests::tree_of_trees_proc::node::TreeOfTreesNodeKind;
use crate::tests::tree_of_trees_proc::step::TreeOfTreesStepKind;
use crate::tests::tree_of_trees_proc::verdict::local::TreeOfTreesLocalVerdict;


pub struct TreeOfTreesProcessHandler {}

impl AbstractProcessHandler<TreeOfTreesConfig> for TreeOfTreesProcessHandler {

    fn process_new_step(_context: &TreeOfTreesContext,
                        _param : &TreeOfTreesParameterization,
                        parent_node: &GenericNode<TreeOfTreesNodeKind>,
                        step_to_process: &GenericStep<TreeOfTreesStepKind>,
                        _new_node_id: u32,
                        _node_counter: u32) -> TreeOfTreesNodeKind {
        if step_to_process.kind.on_the_left {
            parent_node.kind.add_to_the_left(step_to_process.kind.letter)
        } else {
            parent_node.kind.add_to_the_right(step_to_process.kind.letter)
        }
    }

    fn get_criterion(_context: &TreeOfTreesContext,
                     _param : &TreeOfTreesParameterization,
                     _parent_node: &GenericNode<TreeOfTreesNodeKind>,
                     _step_to_process: &GenericStep<TreeOfTreesStepKind>,
                     _new_node_id: u32,
                     _node_counter: u32) -> TreeOfTreesFilterCriterion {
        TreeOfTreesFilterCriterion{}
    }

    fn collect_next_steps(_context: &TreeOfTreesContext,
                          _param : &TreeOfTreesParameterization,
                          _parent_node_kind: &TreeOfTreesNodeKind)
                -> Vec<TreeOfTreesStepKind> {
        vec![
            TreeOfTreesStepKind::new('A',true),
            TreeOfTreesStepKind::new('A',false),
            TreeOfTreesStepKind::new('B',true),
            TreeOfTreesStepKind::new('B',false),
        ]
    }

    fn get_local_verdict_when_no_child(_context: &TreeOfTreesContext,
                                       _param: &TreeOfTreesParameterization,
                                       _node_kind: &TreeOfTreesNodeKind) -> TreeOfTreesLocalVerdict {
        TreeOfTreesLocalVerdict{}
    }

    fn get_local_verdict_from_static_analysis(_context: &TreeOfTreesContext,
                                              _param: &TreeOfTreesParameterization,
                                              _node_kind: &mut TreeOfTreesNodeKind) -> Option<(TreeOfTreesLocalVerdict, TreeOfTreesStaticProof)> {
        None
    }

    fn pursue_process_after_static_verdict(_context: &TreeOfTreesContext,
                                           _param: &TreeOfTreesParameterization,
                                           _loc_verd: &TreeOfTreesLocalVerdict) -> bool {
        true
    }
}