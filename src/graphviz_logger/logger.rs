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
use graphviz_dot_builder::graph::graph::GraphVizDiGraph;
use graph_process_manager_core::manager::config::AbstractConfiguration;
use graphviz_dot_builder::traits::GraphVizOutputFormat;
use graphviz_dot_builder::graph::style::{GraphvizGraphStyleItem,GvGraphRankDir};

use crate::graphviz_logger::drawer::GraphicProcessDrawer;
use crate::graphviz_logger::format::{GraphicProcessLoggerLayout};


pub struct GenericGraphVizLogger<Conf : AbstractConfiguration> {
    pub process_drawer : Box<dyn GraphicProcessDrawer<Conf>>,
    // ***
    pub output_format : GraphVizOutputFormat,
    pub layout : GraphicProcessLoggerLayout,
    // ***
    pub display_legend : bool,
    // ***
    pub temp_folder : String,
    // ***
    pub(crate) parent_folder : String,
    pub(crate) output_file_name : String,
    // ***
    pub graph : GraphVizDiGraph
}

impl<Conf: AbstractConfiguration> GenericGraphVizLogger<Conf> {
    pub fn new(process_drawer: Box<dyn GraphicProcessDrawer<Conf>>,
               output_format: GraphVizOutputFormat,
               layout: GraphicProcessLoggerLayout,
               display_legend: bool,
               temp_folder: String,
               parent_folder: String,
               output_file_name: String) -> Self {
        // ***
        let rankdir : GvGraphRankDir = match layout {
            GraphicProcessLoggerLayout::Horizontal => {
                GvGraphRankDir::LR
            },
            GraphicProcessLoggerLayout::Vertical => {
                GvGraphRankDir::TB
            }
        };
        let style = vec![GraphvizGraphStyleItem::Rankdir(rankdir)];
        let graph = GraphVizDiGraph::new(style);
        // ***

        // empties temp directory if exists
        match fs::remove_dir_all(&temp_folder) {
            Ok(_) => {
                // do nothing
            },
            Err(_) => {
                // do nothing
            }
        }
        // creates temp directory if not exist
        fs::create_dir_all(&temp_folder).unwrap();
        // creates parent directory if not exist
        if parent_folder != *"" {
            fs::create_dir_all(&parent_folder).unwrap();
        }

        // ***
        Self { process_drawer, output_format, layout, display_legend, temp_folder, parent_folder, output_file_name, graph }
    }
}


