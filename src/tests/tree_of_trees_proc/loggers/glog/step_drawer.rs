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
use ab_glyph::FontRef;
use image::Rgb;
use image_colored_text::text::line::ColoredTextLine;
use image_colored_text::text::paragraph::{ColoredTextParagraph, MultiLineTextAlignment};
use crate::graphviz::builtin::step_drawer::CustomStepDrawerForGraphvizLogger;
use crate::tests::tree_of_trees_proc::conf::TreeOfTreesConfig;
use crate::tests::tree_of_trees_proc::context::{TreeOfTreesContext, TreeOfTreesParameterization};
use crate::tests::tree_of_trees_proc::loggers::glog::common::{DRAWING_GRAPHIC_FONT, SCALE};
use crate::tests::tree_of_trees_proc::loggers::glog::util::new_image_with_colored_text;
use crate::tests::tree_of_trees_proc::step::TreeOfTreesStepKind;


pub const MY_COLOR_WHITE : [u8;3] = [255u8,  255u8,  255u8];
pub const MY_COLOR_BLACK : [u8;3] = [0u8, 0u8, 0u8];
pub const MY_COLOR_RED : [u8;3] = [255u8, 0u8, 0u8];

pub struct TreeOfTreesStepDrawer {
    pub font : FontRef<'static>,
}

impl TreeOfTreesStepDrawer {
    pub fn new() -> Self {
        let font = ab_glyph::FontRef::try_from_slice(DRAWING_GRAPHIC_FONT).unwrap();
        Self {font}
    }
}

impl CustomStepDrawerForGraphvizLogger<TreeOfTreesConfig> for TreeOfTreesStepDrawer {

    fn draw(&self,
            step : &TreeOfTreesStepKind,
            _context: &TreeOfTreesContext,
            _parameterization: &TreeOfTreesParameterization,
            full_path : &Path) {
        let line = if step.on_the_left {
            ColoredTextLine::new(
                vec![
                    (format!("←"), Rgb(MY_COLOR_RED)),
                    (format!("{}", step.letter), Rgb(MY_COLOR_BLACK)),
                ]
            )
        } else {
            ColoredTextLine::new(
                vec![
                    (format!("{}", step.letter), Rgb(MY_COLOR_BLACK)),
                    (format!("→"), Rgb(MY_COLOR_RED)),
                ]
            )
        };
        let para = ColoredTextParagraph::new(
            vec!(line),
            MultiLineTextAlignment::Center,
            None,
            None
        );
        new_image_with_colored_text(
            full_path,
            &para,
            &self.font,
            SCALE
        );
    }

}



