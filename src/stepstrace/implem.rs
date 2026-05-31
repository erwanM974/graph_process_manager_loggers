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

use graph_process_manager_core::process::config::AbstractProcessConfiguration;
use graph_process_manager_core::process::manager::GenericProcessManager;

use crate::logger::AbstractProcessLogger;
use crate::stepstrace::logger::GenericStepsTraceLogger;
use crate::stepstrace::object::ObjectToBuildWhenTracingSteps;


impl<Conf: AbstractProcessConfiguration + 'static,
     ObjectToBuild: ObjectToBuildWhenTracingSteps + 'static>
    AbstractProcessLogger<Conf> for GenericStepsTraceLogger<Conf, ObjectToBuild>
{
    fn log_initialize_process(&mut self, _manager: &GenericProcessManager<Conf>) {
        let _ = fs::remove_dir_all(&self.parent_folder);
        let _ = fs::create_dir_all(&self.parent_folder);
    }

    fn log_new_node(
        &mut self,
        context_and_param: &Conf::ContextAndParameterization,
        new_node_id: u32,
        new_node: &Conf::DomainSpecificNode,
    ) {
        if new_node_id == 1 {
            let initial = self.printer.get_initial_object(context_and_param, new_node);
            self.trace_map.insert(1, std::iter::once(initial).collect());
        } else {
            self.trace_map.insert(new_node_id, HashSet::new());
        }
    }

    fn log_new_step(
        &mut self,
        context_and_param: &Conf::ContextAndParameterization,
        origin_node_id: u32,
        step: &Conf::DomainSpecificStep,
        target_node_id: u32,
        target_node: &Conf::DomainSpecificNode,
    ) {
        let parent_objects = self.trace_map.get(&origin_node_id).unwrap();
        let objects_from_step: HashSet<ObjectToBuild> = parent_objects
            .iter()
            .map(|o| self.printer.add_step_to_object(context_and_param, o, step))
            .collect();

        let new_objects: HashSet<ObjectToBuild> = match &mut self.anti_duplication_memoizer {
            None => objects_from_step,
            Some(memo) => objects_from_step.into_iter().filter(|o| !memo.contains(o)).collect(),
        };

        if self.printer.should_print_on_node_reached(context_and_param, target_node) {
            for o in &new_objects {
                self.trace_counter += 1;
                let file_name = format!("{}{}.{}", self.prefix, self.trace_counter, self.file_extension);
                let path_buf: PathBuf = [&self.parent_folder, &file_name].iter().collect();
                self.printer.print_object(context_and_param, o, path_buf.as_path());
                if let Some(memo) = &mut self.anti_duplication_memoizer {
                    memo.insert(o.clone());
                }
            }
        }

        self.trace_map.get_mut(&target_node_id).unwrap().extend(new_objects);
    }

    fn log_all_children_processed(
        &mut self,
        _context_and_param: &Conf::ContextAndParameterization,
        parent_node_id: u32,
    ) {
        self.trace_map.remove(&parent_node_id);
    }
}
