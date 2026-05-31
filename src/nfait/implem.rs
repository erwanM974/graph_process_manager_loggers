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

use std::collections::{HashMap, HashSet};

use autour_core::traits::letter::AutLetter;
use autour_core::traits::repr::AutGraphvizDrawable;

use graph_process_manager_core::process::config::AbstractProcessConfiguration;
use graph_process_manager_core::process::manager::GenericProcessManager;
use graphviz_dot_builder::traits::DotPrintable;

use crate::logger::AbstractProcessLogger;
use crate::nfait::logger::{GenericNFAITLogger, NFAITBuilderPrinter};


impl<Conf, Letter, BP> AbstractProcessLogger<Conf> for GenericNFAITLogger<Conf, Letter, BP>
where
    Conf: AbstractProcessConfiguration + 'static,
    Letter: AutLetter + 'static,
    BP: NFAITBuilderPrinter<Conf, Letter> + 'static,
{
    fn log_new_node(
        &mut self,
        context_and_param: &Conf::ContextAndParameterization,
        new_node_id: u32,
        new_node: &Conf::DomainSpecificNode,
    ) {
        let nfa_state_id = self.next_nfa_state_id;
        self.next_nfa_state_id += 1;
        self.explo_node_id_to_nfa_state_id_map.insert(new_node_id, nfa_state_id);
        if self.builder_printer.is_node_final(context_and_param, new_node) {
            self.finals.insert(nfa_state_id);
        }
    }

    fn log_new_step(
        &mut self,
        context_and_param: &Conf::ContextAndParameterization,
        origin_node_id: u32,
        step: &Conf::DomainSpecificStep,
        target_node_id: u32,
        _target_node: &Conf::DomainSpecificNode,
    ) {
        let nfa_orig = *self.explo_node_id_to_nfa_state_id_map.get(&origin_node_id).unwrap();
        let nfa_targ = *self.explo_node_id_to_nfa_state_id_map.get(&target_node_id).unwrap();
        match self.builder_printer.step_into_letter(context_and_param, step) {
            None => {
                self.epsilon_trans.entry(nfa_orig).or_insert_with(HashSet::new).insert(nfa_targ);
            }
            Some(letter) => {
                self.alphabet.insert(letter);
                self.transitions
                    .entry(nfa_orig)
                    .or_insert_with(HashMap::new)
                    .entry(letter)
                    .or_insert_with(HashSet::new)
                    .insert(nfa_targ);
            }
        }
    }

    fn log_terminate_process(&mut self, _manager: &GenericProcessManager<Conf>) {
        let got_nfait = self.get_nfait();
        if let Some((access, format)) = &self.draw {
            let graph = got_nfait.to_dot(*access, &HashSet::new(), &self.builder_printer);
            let _ = graph.print_dot(&[self.parent_folder.clone()], &self.name, format);
        }
    }
}
