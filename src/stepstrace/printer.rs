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



use std::path::Path;
use graph_process_manager_core::process::config::AbstractProcessConfiguration;

use crate::stepstrace::object::ObjectToBuildWhenTracingSteps;


pub trait StepsTraceProcessPrinter<Conf : AbstractProcessConfiguration, ObjectToBuild : ObjectToBuildWhenTracingSteps> {

    fn get_initial_object(
        &self,
        context_and_param: &Conf::ContextAndParameterization,
        node: &Conf::DomainSpecificNode
    ) -> ObjectToBuild;

    fn add_step_to_object(
        &self,
        context_and_param: &Conf::ContextAndParameterization,
        object : &ObjectToBuild,
        step : &Conf::DomainSpecificStep
    ) -> ObjectToBuild;

    fn should_print_on_node_reached(
        &self,
        context_and_param: &Conf::ContextAndParameterization,
        node: &Conf::DomainSpecificNode
    ) -> bool;

    fn print_object(
        &self,
        context_and_param: &Conf::ContextAndParameterization,
        object : &ObjectToBuild,
        path : &Path
    );
}