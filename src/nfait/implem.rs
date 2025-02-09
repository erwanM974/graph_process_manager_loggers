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

use graph_process_manager_core::process::{filter::GenericFiltersManager, logger::AbstractProcessLogger};
use graph_process_manager_core::process::config::AbstractProcessConfiguration;
use graph_process_manager_core::queue::priorities::GenericProcessPriorities;
use graph_process_manager_core::queue::strategy::QueueSearchStrategy;
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

    fn log_initialize(
        &mut self,
        _context_and_param : &Conf::ContextAndParameterization,
        _strategy : &QueueSearchStrategy,
        _priorities : &GenericProcessPriorities<Conf::Priorities>,
        _filters_manager : &GenericFiltersManager<Conf>,
        _initial_global_state : &Conf::MutablePersistentState,
        _use_memoization : bool,
    ) {
        // nothing
    }

    fn log_new_node(
        &mut self,
        context_and_param : &Conf::ContextAndParameterization,
        new_node_id : u32,
        new_node : &Conf::DomainSpecificNode
    ) {
        let nfa_state_id = self.next_nfa_state_id;
        self.next_nfa_state_id += 1;
        self.explo_node_id_to_nfa_state_id_map.insert(new_node_id,nfa_state_id);
        // ***
        if self.builder_printer.is_node_final(context_and_param,new_node) {
            self.finals.insert(nfa_state_id);
        }
    }

    fn log_new_step(
        &mut self,
        context_and_param : &Conf::ContextAndParameterization,
        origin_node_id : u32,
        step : &Conf::DomainSpecificStep,
        target_node_id : u32,
        _target_node : &Conf::DomainSpecificNode
    ) {
        let nfa_orig_state_id = *self.explo_node_id_to_nfa_state_id_map.get(&origin_node_id).unwrap();
        let nfa_targ_state_id = *self.explo_node_id_to_nfa_state_id_map.get(&target_node_id).unwrap();
        match self.builder_printer.step_into_letter(context_and_param,step) {
            None => {
                match self.epsilon_trans.get_mut(&nfa_orig_state_id) {
                    None => {
                        self.epsilon_trans.insert(nfa_orig_state_id,
                                                  hashset! {nfa_targ_state_id});
                    },
                    Some(ep_targets) => {
                        ep_targets.insert(nfa_targ_state_id);
                    }
                }
            },
            Some(letter) => {
                self.alphabet.insert(letter);
                match self.transitions.get_mut(&nfa_orig_state_id) {
                    None => {
                        self.transitions.insert(nfa_orig_state_id,
                                                hashmap! {letter => hashset!{nfa_targ_state_id}});
                    },
                    Some(outgoing) => {
                        match outgoing.get_mut(&letter) {
                            None => {
                                outgoing.insert(letter,hashset!{nfa_targ_state_id});
                            },
                            Some(letter_targets) => {
                                letter_targets.insert(nfa_targ_state_id);
                            }
                        }
                    }
                }
            }
        }
    }

    fn log_notify_last_child_step_of_node_processed(
        &mut self,
        _context_and_param : &Conf::ContextAndParameterization,
        _parent_node_id : u32
    ) {
        // nothing
    }

    fn log_notify_node_without_children(
        &mut self,
        _context_and_param : &Conf::ContextAndParameterization,
        _node_id : u32
    ) {
        // nothing
    }

    fn log_filtered(
        &mut self,
        _context_and_param : &Conf::ContextAndParameterization,
        _parent_node_id : u32,
        _filtration_result_id : u32,
        _filtration_result : &Conf::FiltrationResult
    ) {
        // nothing
    }

    fn log_terminate_process(
        &mut self,
        _context_and_param : &Conf::ContextAndParameterization,
        _global_state : &Conf::MutablePersistentState
    ) {
        let got_nfait = self.get_nfait();
        match &self.draw {
            None => {},
            Some((access,format)) => {
                let graph = got_nfait.to_dot(*access,
                                             &hashset!{},
                                             &self.builder_printer);
                let _ = graph.print_dot(&[self.parent_folder.clone()],
                                &self.name,
                                format);
            }
        }
    }

}