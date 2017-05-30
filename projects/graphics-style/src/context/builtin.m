(* ::Package:: *)

SetDirectory@NotebookDirectory[];

buildHead[] := "
use crate::*;

/// Resolve missing style
#[derive(Debug, Clone)]
pub struct StyleResolver {
    theme: StyleContext,
    local: StyleContext,
}
";


getGraphicsStyle[{field_, type_}] := TemplateApply["\
    /// Set the missing style of `type`
    `type`(`type`),",
    <|"field" -> field, "type" -> type|>
];
buildGraphicsStyle[pattern_] := TemplateApply["\
/// All available styles.
#[derive(Debug, Clone)]
pub enum GraphicsStyle {
`1`
}",
    StringRiffle[getGraphicsStyle /@ pattern, "\n"]
];


getStyleContext[{field_, type_}] := TemplateApply["    pub `field`: Option<`type`>,", <|"field" -> field, "type" -> type|>];
buildStyleContext[pattern_] := TemplateApply["\
#[derive(Debug, Clone)]
pub(crate) struct StyleContext {
`1`
}",
    StringRiffle[getStyleContext /@ pattern, "\n"]
];


getGetter[{field_, type_}] := TemplateApply["\
    /// Set the value of [`e``type``e`]
    pub fn `field`(&self) -> `type` {
        self.local.`field`.unwrap_or(self.theme.`field`.unwrap_or(`type`::default()))
    }"
    ,
    <|"field" -> field, "type" -> type, "e" -> "`"|>
];
buildGetter[pattern_] := TemplateApply[
    "impl StyleResolver {\n`1`\n}",
    StringRiffle[getGetter /@ pattern, "\n"]
];


getSetter[{field_, type_}] := TemplateApply["            GraphicsStyle::`type`(s) => self.local.`field` = Some(s.clone()),",
    <|"field" -> field, "type" -> type|>
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


getConvert[{field_, type_}] := TemplateApply["\
impl From<`type`> for GraphicsStyle {
    fn from(s: `type`) -> Self {
        Self::`type`(s)
    }
}",
    <|"field" -> field, "type" -> type|>
];
buildConvert[pattern_] := TemplateApply[
    "`1`",
    StringRiffle[getConvert /@ pattern, "\n"]
];


patterns = {
    {"point_size", "PointSize"},
    {"point_color", "PointColor"},
    {"line_color", "LineColor"},
    {"line_width", "LineWidth"}
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
Export["mod.rs", codegen, "Text"]
