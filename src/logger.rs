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

use std::collections::HashMap;
use std::sync::Arc;

use graph_process_manager_core::process::config::AbstractProcessConfiguration;
use graph_process_manager_core::process::event::ExplorationEvent;
use graph_process_manager_core::process::manager::GenericProcessManager;


pub trait AbstractProcessLogger<Conf: AbstractProcessConfiguration> {

    fn log_initialize_process(&mut self, _manager: &GenericProcessManager<Conf>) {}

    fn log_new_node(
        &mut self,
        _context_and_param: &Conf::ContextAndParameterization,
        _new_node_id: u32,
        _new_node: &Conf::DomainSpecificNode,
    ) {}

    /**
     * target_node is looked up from previously received NewNode events,
     * so it is always available even when memoization is active and the target
     * was discovered on a different path.
     **/
    fn log_new_step(
        &mut self,
        _context_and_param: &Conf::ContextAndParameterization,
        _origin_node_id: u32,
        _step: &Conf::DomainSpecificStep,
        _target_node_id: u32,
        _target_node: &Conf::DomainSpecificNode,
    ) {}

    fn log_all_children_processed(
        &mut self,
        _context_and_param: &Conf::ContextAndParameterization,
        _parent_node_id: u32,
    ) {}

    fn log_notify_node_without_children(
        &mut self,
        _context_and_param: &Conf::ContextAndParameterization,
        _node_id: u32,
    ) {}

    fn log_filtered(
        &mut self,
        _context_and_param: &Conf::ContextAndParameterization,
        _parent_node_id: u32,
        _filtration_result: &Conf::FiltrationResult,
    ) {}

    fn log_terminate_process(&mut self, _manager: &GenericProcessManager<Conf>) {}
}


/**
 * Drives a set of loggers over a complete process exploration.
 *
 * Calls log_initialize on each logger, then iterates the manager and dispatches
 * each ExplorationEvent to the appropriate logger method, then calls
 * log_terminate_process when the iterator is exhausted.
 *
 * A local id→node registry is maintained so that log_new_step can always supply
 * target_node, even for back-edges to already-memoized nodes.
 **/
pub fn drive_loggers<Conf: 'static + AbstractProcessConfiguration>(
    manager: &mut GenericProcessManager<Conf>,
    loggers: &mut [Box<dyn AbstractProcessLogger<Conf>>],
) {
    for logger in loggers.iter_mut() {
        logger.log_initialize_process(manager);
    }

    let mut node_registry: HashMap<u32, Arc<Conf::DomainSpecificNode>> = HashMap::new();

    while let Some(event) = manager.next() {
        let ctx = &manager.context_and_param;
        match &event {
            ExplorationEvent::NewNode { id, node } => {
                node_registry.insert(*id, Arc::clone(node));
                for logger in loggers.iter_mut() {
                    logger.log_new_node(ctx, *id, node.as_ref());
                }
            }
            ExplorationEvent::NewStep { origin_node_id, step, target_node_id } => {
                if let Some(target_node) = node_registry.get(target_node_id) {
                    let target = Arc::clone(target_node);
                    for logger in loggers.iter_mut() {
                        logger.log_new_step(ctx, *origin_node_id, step, *target_node_id, &target);
                    }
                }
            }
            ExplorationEvent::AllChildrenProcessed { parent_node_id } => {
                for logger in loggers.iter_mut() {
                    logger.log_all_children_processed(ctx, *parent_node_id);
                }
            }
            ExplorationEvent::NodeWithoutChildren { node_id } => {
                for logger in loggers.iter_mut() {
                    logger.log_notify_node_without_children(ctx, *node_id);
                }
            }
            ExplorationEvent::Filtered { parent_node_id, filtration_result } => {
                for logger in loggers.iter_mut() {
                    logger.log_filtered(ctx, *parent_node_id, filtration_result);
                }
            }
        }
    }

    for logger in loggers.iter_mut() {
        logger.log_terminate_process(manager);
    }
}
