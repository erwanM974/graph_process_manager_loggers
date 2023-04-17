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
use crate::tests::fibo_proc::conf::FiboConfig;
use crate::tests::fibo_proc::context::{FiboContext, FiboParameterization};
use crate::tests::fibo_proc::filter::filter::FiboFilterCriterion;
use crate::tests::fibo_proc::node::FiboNodeKind;
use crate::tests::fibo_proc::step::FiboStepKind;
use crate::tests::fibo_proc::verdict::local::FiboLocalVerdict;


pub struct FiboProcessHandler {}

impl AbstractProcessHandler<FiboConfig> for FiboProcessHandler {

    fn process_new_step(_context: &FiboContext,
                        _param : &FiboParameterization,
                        parent_node: &GenericNode<FiboNodeKind>,
                        step_to_process: &GenericStep<FiboStepKind>,
                        _new_node_id: u32,
                        _node_counter: u32) -> FiboNodeKind {
        match &step_to_process.kind {
            FiboStepKind::Next => {
                let new_current = parent_node.kind.next;
                let new_next = parent_node.kind.current + parent_node.kind.next;
                FiboNodeKind::new(new_current,new_next)
            }
        }
    }

    fn get_criterion(_context: &FiboContext,
                     _param : &FiboParameterization,
                     _parent_node: &GenericNode<FiboNodeKind>,
                     _step_to_process: &GenericStep<FiboStepKind>,
                     _new_node_id: u32,
                     _node_counter: u32) -> FiboFilterCriterion {
        FiboFilterCriterion{}
    }

    fn collect_next_steps(_context: &FiboContext,
                          _param : &FiboParameterization,
                          parent_node_id: u32,
                          _parent_node_kind: &FiboNodeKind)
                -> (u32, Vec<GenericStep<FiboStepKind>>) {
        let to_enqueue = vec![GenericStep::new(parent_node_id, 0, FiboStepKind::Next)];
        return (1,to_enqueue);
    }

    fn get_local_verdict_when_no_child(_context: &FiboContext,
                                       _param: &FiboParameterization,
                                       _node_kind: &FiboNodeKind) -> FiboLocalVerdict {
        FiboLocalVerdict{}
    }

    fn get_local_verdict_from_static_analysis(_context: &FiboContext,
                                              _param: &FiboParameterization,
                                              _node_kind: &FiboNodeKind) -> FiboLocalVerdict {
        FiboLocalVerdict{}
    }

    fn pursue_process_after_static_verdict(_context: &FiboContext,
                                           _param: &FiboParameterization,
                                           _loc_verd: &FiboLocalVerdict) -> bool {
        true
    }
}