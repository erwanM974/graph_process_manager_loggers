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

use graphviz_dot_builder::{colors::GraphvizColor, item::node::style::GvNodeShape};

use crate::{graphviz::{drawers::node_drawer::CustomNodeDrawerForGraphvizLogger, item::{BuiltinGraphvizLoggerDefaultGvItemStyle, BuiltinGraphvizLoggerItemStyle}}, tests::fibo_proc::{conf::FiboConfig, context::FiboContextAndParameterization, node::FiboNodeKind}};




pub struct FiboNodeDrawer {}


impl CustomNodeDrawerForGraphvizLogger<FiboConfig> for FiboNodeDrawer {

    fn get_node_node_inner_style_and_draw_if_needed(
        &self,
        _context_and_param : &FiboContextAndParameterization,
        node : &FiboNodeKind,
        _full_path : &Path
    ) -> BuiltinGraphvizLoggerItemStyle {
        BuiltinGraphvizLoggerItemStyle::Default(
            BuiltinGraphvizLoggerDefaultGvItemStyle::new(
            GvNodeShape::Rectangle,
            node.current.to_string(),
            12,
            None,
            GraphvizColor::black,
            GraphvizColor::black,
            GraphvizColor::white
            )
        )
    }

}




