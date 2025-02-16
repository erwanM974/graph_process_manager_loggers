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

use graphviz_dot_builder::colors::GraphvizColor;
use graphviz_dot_builder::item::node::style::GvNodeShape;


use crate::graphviz::drawers::all_the_rest_drawer::CustomAllTheRestDrawerForGraphvizLogger;
use crate::graphviz::item::{BuiltinGraphvizLoggerDefaultGvItemStyle, BuiltinGraphvizLoggerItemStyle};
use crate::tests::fibo_proc::conf::FiboConfig;
use crate::tests::fibo_proc::context::FiboContextAndParameterization;
use crate::tests::fibo_proc::filtration::FiboFiltrationResult;
use crate::tests::fibo_proc::node::FiboNodeKind;
use crate::tests::fibo_proc::step::FiboStepKind;



pub struct FiboAllTheRestDrawer {}


impl CustomAllTheRestDrawerForGraphvizLogger<FiboConfig> for FiboAllTheRestDrawer {

    fn get_step_node_inner_style_and_draw_if_needed(
        &self,
        _context_and_param: &FiboContextAndParameterization,
        step : &FiboStepKind,
        _full_path : &Path
    ) -> BuiltinGraphvizLoggerItemStyle {
        match step {
            FiboStepKind::Next => {
                BuiltinGraphvizLoggerItemStyle::Default(
                    BuiltinGraphvizLoggerDefaultGvItemStyle::new(
                    GvNodeShape::Ellipse,
                    "next".to_string(),
                    12,
                    None,
                    GraphvizColor::black,
                    GraphvizColor::black,
                    GraphvizColor::white
                    )
                )
            }
        }
    }
    
    fn get_step_edge_color(
        &self,
        _context_and_param: &FiboContextAndParameterization,
        _step : &FiboStepKind,
    ) -> GraphvizColor {
        GraphvizColor::black
    }
    
    fn get_filter_node_inner_style_and_draw_if_needed(
        &self,
        _context_and_param: &FiboContextAndParameterization,
        filtration_result: &FiboFiltrationResult,
        _image_file_path : &Path
    ) -> BuiltinGraphvizLoggerItemStyle {
        match filtration_result {
            FiboFiltrationResult::MaxNumberExceeded => {
                BuiltinGraphvizLoggerItemStyle::Default(
                    BuiltinGraphvizLoggerDefaultGvItemStyle::new(
                    GvNodeShape::Rectangle,
                    "MaxNumberExceeded".to_string(),
                    18,
                    None,
                    GraphvizColor::white,
                    GraphvizColor::burlywood4,
                    GraphvizColor::burlywood4
                    )
                )
            }
        }
    }
    
    fn get_filter_edge_color(
        &self,
        _context_and_param: &FiboContextAndParameterization,
        _filtration_result: &FiboFiltrationResult,
    ) -> graphviz_dot_builder::colors::GraphvizColor {
        GraphvizColor::burlywood4
    }
    
    fn get_node_phase_id(
        &self,
        _context_and_param: &FiboContextAndParameterization,
        _new_node: &FiboNodeKind
    ) -> Option<usize> {
        None
    }
    
    fn get_phase_color(
        &self, 
        _phase_id : usize
    ) -> GraphvizColor {
        GraphvizColor::white
    }

}



