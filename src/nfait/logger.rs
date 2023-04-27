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
use itertools::Itertools;

use autour_core::nfait::nfait::AutNFAIT;
use autour_core::traits::letter::AutLetter;
use autour_core::traits::repr::AbstractLanguagePrinter;
use graph_process_manager_core::manager::config::AbstractProcessConfiguration;
use graphviz_dot_builder::traits::GraphVizOutputFormat;

use crate::nfait::printer::NFAITProcessPrinter;


pub struct GenericNFAITLogger<Conf,Letter,LPrinter>
    where
        Conf : AbstractProcessConfiguration,
        Letter : AutLetter,
        LPrinter : AbstractLanguagePrinter<Letter>
        {
    // ***
    phantom : std::marker::PhantomData<LPrinter>,
    // ***
    pub(crate) printer : Box<dyn NFAITProcessPrinter<Conf, Letter>>,
    // ***
    pub(crate) name : String,   // name of the generated files (.aaf et soit .svg soit .png)
    pub(crate) draw : Option<GraphVizOutputFormat>, // whether or not to draw the NFAIT at the end and if so in which format
    pub(crate) parent_folder : String,
    // ***

    // depending on the process (filtered nodes, memoization etc,
    // the set of nodes may not be a contiguous 0..n
    pub(crate) nodes_ids : HashSet<usize>,
    pub(crate) alphabet : HashSet<Letter>,
    // below : attributes of the NFAIT being build
    // there is a single initial : the process start state
    pub(crate) finals: HashSet<usize>,
    pub(crate) transitions: HashMap<usize,HashMap<Letter, HashSet<usize>>>,
    pub(crate) epsilon_trans : HashMap<usize,HashSet<usize>>
}

impl<Conf, Letter, LPrinter> GenericNFAITLogger<Conf, Letter, LPrinter> where
    Conf: AbstractProcessConfiguration,
    Letter: AutLetter,
    LPrinter: AbstractLanguagePrinter<Letter> {

    pub fn new(printer: Box<dyn NFAITProcessPrinter<Conf, Letter>>,
               name: String,
               draw: Option<GraphVizOutputFormat>,
               parent_folder: String) -> Self {
        Self {
            phantom : std::marker::PhantomData,
            printer,
            name,
            draw,
            parent_folder,
            nodes_ids: hashset!{},
            alphabet: hashset!{},
            finals: hashset!{},
            transitions: hashmap!{},
            epsilon_trans: hashmap!{} }
    }

    pub fn get_nfait(&self) -> AutNFAIT<Letter> {
        let nodes_ids_sorted : Vec<usize> = self.nodes_ids.iter().cloned().sorted().collect();
        let node_of_max_id = nodes_ids_sorted.last().unwrap();
        let mut transitions : Vec<HashMap<Letter, HashSet<usize>>> = vec![];
        let mut epsilon_trans : Vec<HashSet<usize>> = vec![];
        for i in 0..=*node_of_max_id {
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









