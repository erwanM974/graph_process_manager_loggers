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
use std::fs;
use std::path::PathBuf;

use graph_process_manager_core::process::filter::GenericFiltersManager;
use graph_process_manager_core::process::logger::AbstractProcessLogger;
use graph_process_manager_core::process::config::AbstractProcessConfiguration;
use graph_process_manager_core::queue::priorities::GenericProcessPriorities;
use graph_process_manager_core::queue::strategy::QueueSearchStrategy;

use crate::nodesprint::logger::GenericNodesPrintLogger;


impl<Conf : AbstractProcessConfiguration+ 'static>
        AbstractProcessLogger<Conf> for GenericNodesPrintLogger<Conf> {

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
        _use_memoization : bool
    ) {
        // empties nodesprint directory if exists
        let _ = fs::remove_dir_all(&self.parent_folder);
        // creates nodesprint directory
        let _ = fs::create_dir_all(&self.parent_folder);
    }

    fn log_new_node(
        &mut self,
        context_and_param : &Conf::ContextAndParameterization,
        new_node_id : u32,
        new_node : &Conf::DomainSpecificNode
    ) {
        if self.printer.should_print_node(context_and_param, new_node) {
            let file_name = format!("{:}_node{:}.{:}",
                                    self.prefix,
                                    new_node_id,
                                    self.file_extension);
            let path_buf : PathBuf = [&self.parent_folder, &file_name].iter().collect();
            self.printer.print_node(context_and_param, new_node, path_buf.as_path());
        }
    }

    fn log_new_step(
        &mut self,
        _context_and_param : &Conf::ContextAndParameterization,
        _origin_node_id : u32,
        _step : &Conf::DomainSpecificStep,
        _target_node_id : u32,
        _target_node : &Conf::DomainSpecificNode
    ) {
        // nothing
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
        // nothing
    }

}