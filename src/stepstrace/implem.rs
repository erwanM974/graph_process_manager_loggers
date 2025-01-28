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


impl<Conf : AbstractProcessConfiguration + 'static,
    ObjectToBuild : ObjectToBuildWhenTracingSteps + 'static>
        AbstractProcessLogger<Conf> for GenericStepsTraceLogger<Conf,ObjectToBuild> {

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn log_initialize(&mut self) {
        // empties tracegen directory if exists
        let _ = fs::remove_dir_all(&self.parent_folder);
        // creates tracegen directory
        let _ = fs::create_dir_all(&self.parent_folder);
    }

    fn log_parameterization(&mut self,
                            _strategy: &QueueSearchStrategy,
                            _priorities: &GenericProcessPriorities<Conf::Priorities>,
                            _filters: &[Box<dyn AbstractFilter<Conf::FilterCriterion, Conf::FilterEliminationKind>>],
                            _goal : &Option<Conf::GlobalVerdict>,
                            _memoize : bool,
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
            // initializes the objects storage in 'self.trace_map'
            // with the initial object
            self.trace_map.insert(1,
                                  hashset!{self.printer.get_initial_object(context,param,new_node)});
        } else {
            // creates an entry for the new node
            // it will be completed when log_new_step is called
            self.trace_map.insert(new_state_id,
                                  hashset!{});
            // removal of entries is handled in 'log_notify_last_child_of_node_processed'
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

        // collect objects already build on the origin node
        let parent_node_objects = self.trace_map.get(&origin_state_id).unwrap();
        // add the steps to all of these objects to obtain the newly reached objects
        let objects_from_step : HashSet<ObjectToBuild> = parent_node_objects.iter()
            .map(|o|self.printer.add_step_to_object(context,param,o, step))
            .collect();

        // filters out objects that have already been printed if memoizer is ON
        let new_objects : HashSet<ObjectToBuild> =
            match &mut self.anti_duplication_memoizer {
                None => {
                    objects_from_step
                },
                Some(memo) => {
                    objects_from_step.into_iter().filter(|o| !memo.contains(o)).collect()
                }
            };

        if self.printer.should_print_on_node_reached(context, param, target_node, target_depth) {
            for o in &new_objects {
                self.trace_counter += 1;
                let file_name = format!("{:}{:}.{:}",self.prefix,self.trace_counter,self.file_extension);
                let path_buf : PathBuf = [&self.parent_folder, &file_name].iter().collect();
                self.printer.print_object(context, param, o, path_buf.as_path());
                if let Some(memo) = &mut self.anti_duplication_memoizer {
                    memo.insert(o.clone());
                }
            }
        }

        // updates objects built on the target node
        let objs_on_node = self.trace_map.get_mut(&target_state_id).unwrap();
        objs_on_node.extend(new_objects);
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