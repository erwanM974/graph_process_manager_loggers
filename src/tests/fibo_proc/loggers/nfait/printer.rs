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



use autour_core::printers::p_chars::CharAsLetterPrinter;
use crate::nfait::builder::NFAITProcessBuilder;
use crate::nfait::logger::NFAITBuilderPrinter;

use crate::tests::fibo_proc::conf::FiboConfig;
use crate::tests::fibo_proc::context::{FiboContext, FiboParameterization};
use crate::tests::fibo_proc::node::FiboNodeKind;
use crate::tests::fibo_proc::step::FiboStepKind;



impl NFAITProcessBuilder<FiboConfig,char> for CharAsLetterPrinter {

    fn step_into_letter(&mut self,
                        _context: &FiboContext,
                        _param: &FiboParameterization,
                        _step: &FiboStepKind) -> Option<char> {
        Some('n')
    }

    fn is_node_final(&self,
                     _context: &FiboContext,
                     _param: &FiboParameterization,
                     _node: &FiboNodeKind) -> bool {
        false
    }
}

impl NFAITBuilderPrinter<FiboConfig, char> for CharAsLetterPrinter {}