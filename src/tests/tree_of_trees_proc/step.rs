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




use std::fmt;
use std::fmt::Formatter;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct TreeOfTreesStepKind {
    pub letter : char,
    pub on_the_left : bool
}

impl TreeOfTreesStepKind {
    pub fn new(letter : char,on_the_left : bool) -> Self {
        Self {letter,on_the_left}
    }
}

impl fmt::Display for TreeOfTreesStepKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.on_the_left {
            write!(f, "{}", &format!("←{}", self.letter) )
        } else {
            write!(f, "{}", &format!("{}→", self.letter) )
        }
    }
}