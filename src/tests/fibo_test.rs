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
use graph_process_manager_core::process::filter::GenericFiltersManager;
use graph_process_manager_core::process::manager::GenericProcessManager;
use graph_process_manager_core::queue::priorities::GenericProcessPriorities;
use graph_process_manager_core::queue::strategy::QueueSearchStrategy;
use graphviz_dot_builder::traits::GraphVizOutputFormat;

use crate::graphviz::format::GraphVizProcessLoggerLayout;
use crate::graphviz::logger::{GenericGraphVizLogger, GenericGraphVizLoggerConfiguration};
use crate::nfait::logger::GenericNFAITLogger;
use crate::nodesprint::logger::GenericNodesPrintLogger;
use crate::stepstrace::logger::GenericStepsTraceLogger;
use crate::tests::fibo_proc::conf::FiboConfig;
use crate::tests::fibo_proc::loggers::nlog::printer::FiboProcessNodePrinter;
use crate::tests::fibo_proc::loggers::slog::object::FiboStepsTrace;
use crate::tests::fibo_proc::loggers::slog::printer::FiboProcessStepPrinter;
use crate::tests::fibo_proc::node::FiboNodeKind;
use crate::tests::fibo_proc::priorities::FiboPriorities;

use super::fibo_proc::context::FiboContextAndParameterization;
use super::fibo_proc::filter::FiboFilter;
use super::fibo_proc::loggers::glog::all_the_rest_drawer::FiboAllTheRestDrawer;
use super::fibo_proc::loggers::glog::legend_writer::FiboLegendWriter;
use super::fibo_proc::loggers::glog::node_drawer::FiboNodeDrawer;

#[test]
fn process_fibo() {

    let graphviz_logger : GenericGraphVizLogger<FiboConfig> = {
        let gv_conf = GenericGraphVizLoggerConfiguration::new(
            GraphVizOutputFormat::svg, 
            true, 
            "fibo_temp".to_string(), 
            "fibo".to_string(), 
            "fibo_gv".to_string()
        );
        GenericGraphVizLogger::new(
            gv_conf,
            Box::new(FiboLegendWriter{}),
            vec![Box::new(FiboNodeDrawer{})],
            Box::new(FiboAllTheRestDrawer{}),
            GraphVizProcessLoggerLayout::Vertical
        )
    };

    let node_printer = FiboProcessNodePrinter{};
    let node_logger : GenericNodesPrintLogger<FiboConfig> = GenericNodesPrintLogger::new(
        Box::new(node_printer),
        "fib_number".to_string(),
        "txt".to_string(),
        "fibo".to_string()
    );

    let steps_printer = FiboProcessStepPrinter{};
    let steps_logger : GenericStepsTraceLogger<FiboConfig,FiboStepsTrace> = GenericStepsTraceLogger::new(
        Box::new(steps_printer),
        true,
        "fib_trace".to_string(),
        "txt".to_string(),
        "fibo".to_string()
    );

    let nfait_printer = CharAsLetterPrinter{};
    let nfait_logger : GenericNFAITLogger<FiboConfig,char,CharAsLetterPrinter> = GenericNFAITLogger::new(
        nfait_printer,
        "fib_nfait".to_string(),
        Some((true,GraphVizOutputFormat::svg)),
        "fibo".to_string()
    );

    let init_node = FiboNodeKind::new(0,1);

    let process_ctx = FiboContextAndParameterization{};
    let priorities : GenericProcessPriorities<FiboPriorities> = GenericProcessPriorities::new(FiboPriorities{},false);
    let filters_manager = GenericFiltersManager::new(
        vec![Box::new(FiboFilter::MaxNum(10))], 
        vec![], 
        vec![]
    );
    let mut manager : GenericProcessManager<FiboConfig> = GenericProcessManager::new(
        process_ctx,
        QueueSearchStrategy::DFS,
        priorities,
        filters_manager,
        vec![Box::new(graphviz_logger),
            Box::new(node_logger),
            Box::new(steps_logger),
            Box::new(nfait_logger)],
        false
    );

    manager.start_process(init_node);
}