(* ::Package:: *)

SetDirectory@NotebookDirectory[];

buildHead[] := "
use crate::*;
";


getGraphicsStyle[{field_, outer_, inner_}] := Block[
    {},
    TemplateApply["\
    /// Set the missing style of `outer`
    `outer`(`inner`),",
        <|"outer" -> outer, "inner" -> inner|>
    ]
];
buildGraphicsStyle[pattern_] := TemplateApply["\
/// All available styles.
#[derive(Debug, Clone)]
pub enum GraphicsStyle {
`1`
}",
    StringRiffle[getGraphicsStyle /@ pattern, "\n"]
];


getStyleContext[{field_, outer_, inner_}] := Block[
    {},
    TemplateApply["    pub `field`: Option<`inner`>,", <|"field" -> field, "inner" -> inner|>]
];
buildStyleContext[pattern_] := TemplateApply["\
#[derive(Debug, Clone, Default)]
pub(crate) struct StyleContext {
`1`
}",
    StringRiffle[getStyleContext /@ pattern, "\n"]
];


getGetter[{field_, outer_, inner_}] := Block[
    {},
    TemplateApply["\
    /// Set the value of [`e``outer``e`]
    pub fn `field`(&self) -> `inner` {
        self.local.`field`.unwrap_or(self.theme.`field`.unwrap_or(`outer`::default().value))
    }"
        ,
        <|"field" -> field, "outer" -> outer, "inner" -> inner, "e" -> "`"|>
    ]
];
buildGetter[pattern_] := TemplateApply[
    "impl StyleResolver {\n`1`\n}",
    StringRiffle[getGetter /@ pattern, "\n"]
];


getSetter[{field_, outer_, inner_}] := TemplateApply["            GraphicsStyle::`outer`(s) => self.local.`field` = Some(s.clone()),",
    <|"field" -> field, "outer" -> outer|>
];
buildSetter[pattern_] := TemplateApply["\
impl StyleResolver {
    /// Set the style of the given element.
    pub fn set_local_style<T>(&mut self, style: T)
    where
        T: Into<GraphicsStyle>,
    {
        match style.into() {
`1`
        }
    }
}",
    StringRiffle[getSetter /@ pattern, "\n"]
];


getConvert[{field_, outer_, inner_}] := Block[
    {},
    TemplateApply["\
impl From<`type`> for GraphicsStyle {
    fn from(s: `type`) -> Self {
        Self::`type`(s.value)
    }
}",
        <|"field" -> field, "type" -> outer|>
    ]
];
buildConvert[pattern_] := TemplateApply[
    "`1`",
    StringRiffle[getConvert /@ pattern, "\n"]
];


patterns = {
    {"point_size", "PointSize", "f32"},
    {"point_color", "PointColor", "RGBA"},
    {"line_color", "LineColor", "RGBA"},
    {"line_width", "LineWidth", "f32"}
};
codegen = StringRiffle[
    Flatten@{
        buildHead[],
        buildGraphicsStyle[patterns],
        buildStyleContext[patterns],
        buildGetter[patterns],
        buildSetter[patterns],
        buildConvert[patterns]
    },
    "\n\n"
];
Export["resolver.rs", codegen, "Text"]
