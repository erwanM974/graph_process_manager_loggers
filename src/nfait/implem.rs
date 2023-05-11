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

use std::any::Any;
use autour_core::traits::letter::AutLetter;
use autour_core::traits::repr::AutGraphvizDrawable;

use graph_process_manager_core::manager::logger::AbstractProcessLogger;
use graph_process_manager_core::manager::config::AbstractProcessConfiguration;
use graph_process_manager_core::queued_steps::queue::strategy::QueueSearchStrategy;
use graph_process_manager_core::handler::filter::AbstractFilter;
use graph_process_manager_core::delegate::priorities::GenericProcessPriorities;
use graphviz_dot_builder::traits::DotPrintable;

use crate::nfait::logger::{GenericNFAITLogger, NFAITBuilderPrinter};


impl<Conf, Letter,BP>
        AbstractProcessLogger<Conf> for GenericNFAITLogger<Conf,Letter,BP>

    where
        Conf : AbstractProcessConfiguration + 'static,
        Letter : AutLetter + 'static,
        BP : NFAITBuilderPrinter<Conf, Letter> + 'static
            {


    fn as_any(&self) -> &dyn Any {
        self
    }

    fn log_initialize(&mut self) {
        // nothing
    }

    fn log_parameterization(&mut self,
                            _strategy: &QueueSearchStrategy,
                            _priorities: &GenericProcessPriorities<Conf::Priorities>,
                            _filters: &[Box<dyn AbstractFilter<Conf::FilterCriterion, Conf::FilterEliminationKind>>],
                            _goal : &Option<Conf::GlobalVerdict>,
                            _memoize : bool,
                            _parameterization: &Conf::Parameterization) {
        // nothing
    }

    fn log_filtered(&mut self,
                    _context: &Conf::Context,
                    _parent_state_id: u32,
                    _new_state_id: u32,
                    _elim_kind: &Conf::FilterEliminationKind) {
        // nothing
    }

    fn log_new_node(&mut self,
                    context: &Conf::Context,
                    param: &Conf::Parameterization,
                    new_state_id: u32,
                    new_node: &Conf::NodeKind) {
        // minus 1 because node id in graph_process_manager_core starts at 1
        let node_id = (new_state_id - 1) as usize;
        self.nodes_ids.insert(node_id);
        if self.builder_printer.is_node_final(context,param,new_node) {
            self.finals.insert(node_id);
        }
    }

    fn log_new_step(&mut self,
                    context: &Conf::Context,
                    param: &Conf::Parameterization,
                    origin_state_id: u32,
                    target_state_id: u32,
                    step: &Conf::StepKind,
                    _target_node : &Conf::NodeKind,
                    _target_depth : u32) {
        let orig_stid_usize = (origin_state_id -1) as usize;
        let targ_stid_usize = (target_state_id -1) as usize;
        match self.builder_printer.step_into_letter(context,param,step) {
            None => {
                match self.epsilon_trans.get_mut(&orig_stid_usize) {
                    None => {
                        self.epsilon_trans.insert(orig_stid_usize,
                                                  hashset! {targ_stid_usize});
                    },
                    Some(ep_targets) => {
                        ep_targets.insert(targ_stid_usize);
                    }
                }
            },
            Some(letter) => {
                self.alphabet.insert(letter);
                match self.transitions.get_mut(&orig_stid_usize) {
                    None => {
                        self.transitions.insert(orig_stid_usize,
                                                hashmap! {letter => hashset!{targ_stid_usize}});
                    },
                    Some(outgoing) => {
                        match outgoing.get_mut(&letter) {
                            None => {
                                outgoing.insert(letter,hashset!{targ_stid_usize});
                            },
                            Some(letter_targets) => {
                                letter_targets.insert(targ_stid_usize);
                            }
                        }
                    }
                }
            }
        }
    }

    fn log_verdict_on_no_child(&mut self,
                               _context: &Conf::Context,
                               _param: &Conf::Parameterization,
                               _parent_state_id: u32,
                               _verdict: &Conf::LocalVerdict) {
        // nothing
    }

    fn log_verdict_on_static_analysis(&mut self,
                                      _context: &Conf::Context,
                                      _param: &Conf::Parameterization,
                                      _parent_state_id: u32,
                                      _verdict: &Conf::LocalVerdict,
                                      _proof : &Conf::StaticLocalVerdictAnalysisProof) {
        // nothing
    }

    fn log_terminate(&mut self,
                     _global_verdict: &Conf::GlobalVerdict) {
        let got_nfait = self.get_nfait();
        match &self.draw {
            None => {},
            Some((access,format)) => {
                let graph = got_nfait.to_dot(*access,
                                             &hashset!{},
                                             &self.builder_printer);
                graph.print_dot(&[self.parent_folder.clone()],
                                &self.name,
                                format);
            }
        }
    }

    fn log_notify_terminal_node_reached(&mut self,
                                        _context: &Conf::Context,
                                        _node_id: u32) {
        // nothing
    }

    fn log_notify_last_child_of_node_processed(&mut self,
                                               _context: &Conf::Context,
                                               _parent_node_id: u32) {
        // nothing
    }
}