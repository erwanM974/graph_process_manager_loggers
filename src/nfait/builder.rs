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



use autour_core::traits::letter::AutLetter;

use graph_process_manager_core::process::config::AbstractProcessConfiguration;



pub trait NFAITProcessBuilder<Conf : AbstractProcessConfiguration, Letter : AutLetter> {

    fn step_into_letter(
        &mut self,
        context_and_param: &Conf::ContextAndParameterization,
        step : &Conf::DomainSpecificStep
    ) -> Option<Letter>;

    fn is_node_final(
        &self,
        context_and_param: &Conf::ContextAndParameterization,
        node : &Conf::DomainSpecificNode
    ) -> bool;

}