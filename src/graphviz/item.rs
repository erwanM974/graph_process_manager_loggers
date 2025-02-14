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





/** 
 * Draw the custom item as a Graghviz node with:
 * - the following style :
 *   + a specific shape
 *   + a label
 *   + a fill color
 *   + a font size
 *   + a font name
 *   + a font color
 * - or:
 *   + as a rectangle
 *   + with "" (empty string) as label
 *   + containing an image
 * **/
 pub enum BuiltinGraphvizLoggerItemStyle {
    ShapeAndLabel(GvNodeShape,String,GraphvizColor,u32,&'static str,GraphvizColor),
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
            BuiltinGraphvizLoggerItemStyle::ShapeAndLabel(
                shape, 
                label,
                fill_color,
                font_size,
                font_name,
                font_color) => {
                    vec![
                        GraphvizNodeStyleItem::Shape( shape ),
                        GraphvizNodeStyleItem::Label( label ),
                        GraphvizNodeStyleItem::Color( fill_color ),
                        GraphvizNodeStyleItem::FontSize( font_size ),
                        GraphvizNodeStyleItem::FontName( font_name.to_string() ),
                        GraphvizNodeStyleItem::FontColor( font_color ),
                        GraphvizNodeStyleItem::Style(vec![GvNodeStyleKind::Filled])
                    ]
            }
        }

    }

}