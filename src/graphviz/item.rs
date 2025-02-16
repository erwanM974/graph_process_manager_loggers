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
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyle, GraphvizNodeStyleItem, GvNodeShape, GvNodeStyleKind};





pub struct BuiltinGraphvizLoggerDefaultGvItemStyle {
    pub shape : GvNodeShape,
    pub label : String,
    pub font_size : u32,
    pub font_name : Option<&'static str>,
    pub font_color : GraphvizColor,
    pub border_color : GraphvizColor,
    pub fill_color : GraphvizColor
}

impl BuiltinGraphvizLoggerDefaultGvItemStyle {

    pub fn new(
        shape : GvNodeShape,
        label : String, 
        font_size : u32, 
        font_name : Option<&'static str>, 
        font_color : GraphvizColor, 
        border_color : GraphvizColor, 
        fill_color :GraphvizColor 
    ) -> Self {
        Self {
            shape, 
            label, 
            font_size, 
            font_name, 
            font_color, 
            border_color, 
            fill_color 
        }
    }

    pub fn to_gv_style(self) -> GraphvizNodeStyle {
        let mut style = vec![
            GraphvizNodeStyleItem::Shape( self.shape ),
            GraphvizNodeStyleItem::Label( self.label ),
            GraphvizNodeStyleItem::FontSize( self.font_size ),
            GraphvizNodeStyleItem::FontColor( self.font_color ),
            GraphvizNodeStyleItem::Color( self.border_color ),
            GraphvizNodeStyleItem::FillColor( self.fill_color ),
            GraphvizNodeStyleItem::Style(vec![GvNodeStyleKind::Filled])
        ];
        if let Some(font_name) = self.font_name {
            style.push(GraphvizNodeStyleItem::FontName( font_name.to_string() ))
        }
        style
    }

}


 pub enum BuiltinGraphvizLoggerItemStyle {
    Default(BuiltinGraphvizLoggerDefaultGvItemStyle),
    CustomImage
}


impl BuiltinGraphvizLoggerItemStyle {

    pub fn to_graphviz_node_styte(self, image_file_path : &Path) -> GraphvizNodeStyle {
        match self {
            BuiltinGraphvizLoggerItemStyle::CustomImage => {
                vec![
                    GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle),
                    GraphvizNodeStyleItem::FillColor(GraphvizColor::white),
                    GraphvizNodeStyleItem::Label("".to_string()),
                    GraphvizNodeStyleItem::Image(
                        image_file_path
                            .to_str()
                            .unwrap()
                            .to_string(),
                    )
                ]
            },
            BuiltinGraphvizLoggerItemStyle::Default(item_style) => {
                item_style.to_gv_style()
            }
        }

    }

}