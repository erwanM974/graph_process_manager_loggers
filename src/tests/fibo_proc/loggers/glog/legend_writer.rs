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

use graph_process_manager_core::process::filter::{AbstractNodePostFilter, AbstractNodePreFilter, AbstractStepFilter};

use crate::{graphviz::drawers::legend_writer::ProcessLegendWriter, tests::fibo_proc::{conf::FiboConfig, context::FiboContextAndParameterization, filter::FiboFilter, priorities::FiboPriorities, state::FiboPersistentState}};




pub struct FiboLegendWriter {}


impl ProcessLegendWriter<FiboConfig> for FiboLegendWriter {
    fn get_process_description(&self) -> String {
        "fibo".to_string()
    }

    fn get_parameters_description(
        &self, 
        _context_and_param : &FiboContextAndParameterization
    ) -> Vec<Vec<String>> {
        vec![]
    }

    fn get_priorities_description(
        &self, 
        _priorities : &FiboPriorities
    ) -> Vec<Vec<String>> {
        vec![]
    }

    fn get_step_filter_description(
        &self, 
        _filter : &dyn AbstractStepFilter<FiboConfig>
    ) -> Option<Vec<String>> {
        None
    }

    fn get_node_pre_filter_description(&self, filter : &dyn AbstractNodePreFilter<FiboConfig>) -> Option<Vec<String>> {
        match filter.as_any().downcast_ref::<FiboFilter>() {
            Some(_) => {
                Some(vec!["FiboMaxNumFilter".to_string()])
            }
            None => {
                None
            }
        }
    }

    fn get_node_post_filter_description(&self, _filter : &dyn AbstractNodePostFilter<FiboConfig>) -> Option<Vec<String>> {
        None
    }

    fn get_final_global_state_description_for_legend(
        &self, 
        _context_and_param : &FiboContextAndParameterization,
        _final_state : &FiboPersistentState
    ) -> Vec<String> {
        vec![]
    }
}
