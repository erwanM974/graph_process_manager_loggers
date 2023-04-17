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



use std::fs;
use std::path::PathBuf;

use graph_process_manager_core::manager::logger::AbstractProcessLogger;
use graph_process_manager_core::manager::config::AbstractProcessConfiguration;
use graph_process_manager_core::queued_steps::queue::strategy::QueueSearchStrategy;
use graph_process_manager_core::handler::filter::AbstractFilter;
use graph_process_manager_core::delegate::priorities::GenericProcessPriorities;

use crate::nodesprint::logger::GenericNodesPrintLogger;


impl<Conf : AbstractProcessConfiguration>
        AbstractProcessLogger<Conf> for GenericNodesPrintLogger<Conf> {

    fn log_initialize(&mut self) {
        // empties nodesprint directory if exists
        if let Err(e) = fs::remove_dir_all(&self.parent_folder) {
            println!("error during logger initialization : {:?} ", e);
        }
        // creates nodesprint directory
        if let Err(e) = fs::create_dir_all(&self.parent_folder) {
            println!("error during logger initialization : {:?} ", e);
        }
    }

    fn log_parameterization(&mut self,
                            _strategy: &QueueSearchStrategy,
                            _filters: &[Box<dyn AbstractFilter<Conf::FilterCriterion, Conf::FilterEliminationKind>>],
                            _priorities: &GenericProcessPriorities<Conf::Priorities>,
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
        if self.printer.should_print_node(context, param, new_node) {
            let file_name = format!("{:}_node{:}.{:}",
                                    self.prefix,
                                    new_state_id,
                                    self.file_extension);
            let path_buf : PathBuf = [&self.parent_folder, &file_name].iter().collect();
            self.printer.print_node(context, param, new_node, path_buf.as_path());
        }
    }

    fn log_new_step(&mut self,
                    _context: &Conf::Context,
                    _param: &Conf::Parameterization,
                    _origin_state_id: u32,
                    _target_state_id: u32,
                    _step: &Conf::StepKind,
                    _target_node : &Conf::NodeKind,
                    _target_depth : u32) {
        // nothing
    }

    fn log_verdict(&mut self,
                   _context: &Conf::Context,
                   _param: &Conf::Parameterization,
                   _parent_state_id: u32,
                   _verdict: &Conf::LocalVerdict) {
        // nothing
    }

    fn log_terminate(&mut self,
                     _global_verdict: &Conf::GlobalVerdict) {
        // nothing
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