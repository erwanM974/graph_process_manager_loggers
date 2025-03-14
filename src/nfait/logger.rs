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


use std::collections::{BTreeMap, HashMap, HashSet};

use autour_core::nfait::nfait::AutNFAIT;
use autour_core::traits::letter::AutLetter;
use autour_core::traits::repr::AbstractLanguagePrinter;
use graph_process_manager_core::process::config::AbstractProcessConfiguration;
use graphviz_dot_builder::traits::GraphVizOutputFormat;
use maplit::{btreemap, hashmap, hashset};

use crate::nfait::builder::NFAITProcessBuilder;


pub trait NFAITBuilderPrinter<Conf : AbstractProcessConfiguration,Letter : AutLetter> :
    NFAITProcessBuilder<Conf, Letter> + AbstractLanguagePrinter<Letter> {}

pub struct GenericNFAITLogger<Conf,Letter,BP>
    where
        Conf : AbstractProcessConfiguration,
        Letter : AutLetter,
        BP : NFAITBuilderPrinter<Conf, Letter>
        {
    // ***
    phantom : std::marker::PhantomData<Conf>,
    pub builder_printer : BP,
    // ***
    pub(crate) name : String,   // name of the generated files (.aaf et soit .svg soit .png)
    pub(crate) draw : Option<(bool,GraphVizOutputFormat)>, // whether or not to draw the NFAIT at the end and if so:
                                            // - in which format
                                            // - and if we should colorize nodes given accessibility
    // ***
    pub(crate) parent_folder : String,
    // ***

    // depending on the process (filtered nodes, memoization etc,
    // the set of nodes may not be a contiguous 0..n
    pub(crate) explo_node_id_to_nfa_state_id_map : BTreeMap<u32,usize>,
    pub(crate) next_nfa_state_id : usize,

    pub(crate) alphabet : HashSet<Letter>,
    // below : attributes of the NFAIT being build
    // there is a single initial : the process start state
    pub(crate) finals: HashSet<usize>,
    pub(crate) transitions: HashMap<usize,HashMap<Letter, HashSet<usize>>>,
    pub(crate) epsilon_trans : HashMap<usize,HashSet<usize>>
}

impl<Conf, Letter,BP> GenericNFAITLogger<Conf, Letter,BP> where
    Conf: AbstractProcessConfiguration,
    Letter: AutLetter,
    BP : NFAITBuilderPrinter<Conf, Letter> {

    pub fn new(builder_printer: BP,
               name: String,
               draw: Option<(bool,GraphVizOutputFormat)>,
               parent_folder: String) -> Self {
        Self {
            phantom : std::marker::PhantomData,
            builder_printer,
            name,
            draw,
            parent_folder,
            explo_node_id_to_nfa_state_id_map: btreemap!{},
            next_nfa_state_id:0,
            alphabet: hashset!{},
            finals: hashset!{},
            transitions: hashmap!{},
            epsilon_trans: hashmap!{} }
    }

    pub fn get_nfait(&self) -> AutNFAIT<Letter> {
        let mut transitions : Vec<HashMap<Letter, HashSet<usize>>> = vec![];
        let mut epsilon_trans : Vec<HashSet<usize>> = vec![];
        for i in 0..self.next_nfa_state_id {
            if let Some(outgoing) = self.transitions.get(&i) {
                transitions.push(outgoing.clone());
            } else {
                transitions.push(hashmap! {});
            }
            if let Some(outgoing) = self.epsilon_trans.get(&i) {
                epsilon_trans.push(outgoing.clone());
            } else {
                epsilon_trans.push(hashset! {});
            }
        }
        AutNFAIT::from_raw(self.alphabet.clone(),
                           hashset!{0},
                           self.finals.clone(),
                           transitions,
                           epsilon_trans).unwrap()
    }
}









