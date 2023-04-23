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


use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;


use graph_process_manager_core::manager::logger::AbstractProcessLogger;
use graph_process_manager_core::manager::config::AbstractProcessConfiguration;
use graph_process_manager_core::queued_steps::queue::strategy::QueueSearchStrategy;
use graph_process_manager_core::handler::filter::AbstractFilter;
use graph_process_manager_core::delegate::priorities::GenericProcessPriorities;

use crate::stepstrace::logger::GenericStepsTraceLogger;
use crate::stepstrace::object::ObjectToBuildWhenTracingSteps;


impl<Conf : AbstractProcessConfiguration, ObjectToBuild : ObjectToBuildWhenTracingSteps>
        AbstractProcessLogger<Conf> for GenericStepsTraceLogger<Conf,ObjectToBuild> {

    fn log_initialize(&mut self) {
        // empties tracegen directory if exists
        if let Err(e) = fs::remove_dir_all(&self.parent_folder) {
            println!("error during logger initialization : {:?} ", e);
        }
        // creates tracegen directory
        if let Err(e) = fs::create_dir_all(&self.parent_folder) {
            println!("error during logger initialization : {:?} ", e);
        }
    }

    fn log_parameterization(&mut self,
                            _strategy: &QueueSearchStrategy,
                            _priorities: &GenericProcessPriorities<Conf::Priorities>,
                            _filters: &[Box<dyn AbstractFilter<Conf::FilterCriterion, Conf::FilterEliminationKind>>],
                            _goal : &Option<Conf::GlobalVerdict>,
                            _use_memoization : bool,
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
        if new_state_id == 1 {
            self.trace_map.insert(1,
                                  hashset!{self.printer.get_initial_object(context,param,new_node)});
        }
    }

    fn log_new_step(&mut self,
                    context: &Conf::Context,
                    param: &Conf::Parameterization,
                    origin_state_id: u32,
                    target_state_id: u32,
                    step: &Conf::StepKind,
                    target_node : &Conf::NodeKind,
                    target_depth : u32) {

        let parent_node_objects = self.trace_map.get(&origin_state_id).unwrap();
        let objects_from_step : HashSet<ObjectToBuild> = parent_node_objects.iter()
            .map(|o|self.printer.add_step_to_object(context,param,o, step))
            .collect();

        let new_objects : HashSet<ObjectToBuild> =
            if let Some(already_reached_objs) = self.trace_map.get_mut(&target_state_id) {
            objects_from_step.into_iter().filter(|o| !already_reached_objs.contains(o)).collect()
        } else {
            self.trace_map.insert(target_state_id, hashset!{});
            objects_from_step
        };

        if self.printer.should_print_on_node_reached(context, param, target_node, target_depth) {
            let mut obj_id = 1;
            for o in &new_objects {
                let file_name = format!("{:}_node{:}_from{:}_o{:}.{:}",
                                        self.prefix,
                                        target_state_id,
                                        origin_state_id,
                                        obj_id,
                                        self.file_extension);
                let path_buf : PathBuf = [&self.parent_folder, &file_name].iter().collect();
                self.printer.print_object(context, param, o, path_buf.as_path());
                obj_id += 1;
            }
        }

        if let Some(objs_on_node) = self.trace_map.get_mut(&target_state_id) {
            objs_on_node.extend(new_objects);
        }
    }

    fn log_verdict_on_no_child(&mut self,
                               _context: &Conf::Context,
                               _param: &Conf::Parameterization,
                               _parent_state_id: u32,
                               _verdict: &Conf::LocalVerdict) {
        // nothing
    }

    fn log_verdict_on_static_analysis(&mut self,
                                      _context: &Conf::Context,
                                      _param: &Conf::Parameterization,
                                      _parent_state_id: u32,
                                      _verdict: &Conf::LocalVerdict,
                                      _proof : &Conf::StaticLocalVerdictAnalysisProof) {
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
                                               parent_node_id: u32) {
        self.trace_map.remove(&parent_node_id);
    }
}