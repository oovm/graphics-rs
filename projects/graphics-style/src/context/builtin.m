(* ::Package:: *)

SetDirectory@NotebookDirectory[];


TemplateApply["
use super::*;

#[derive(Debug, Clone)]
pub(crate) struct StyleContext {
    pub point_size: Option<PointSize>,
}

impl StyleResolver {
    /// Set the value of [`PointSize`]
    pub fn point_size(&self) -> PointSize {
        self.local.point_size.unwrap_or(self.theme.point_size.unwrap_or(PointSize::default()))
    }
}

impl GraphicsStyle for PointSize {
    fn set_local_style(&self, context: &mut StyleResolver) {
        context.local.point_size = Some(self.clone());
    }
}
"]


getField[{field_,type_}]:=TemplateApply["pub `field`: Option<`type`>,", <|"field"->field, "type"->type|>]

buildField[pattern_]:=TemplateApply["
#[derive(Debug, Clone)]
pub(crate) struct StyleContext {\n`1`\n}",
StringRiffle[getField/@pattern,"\n"]
]


getGetter[{field_,type_}]:=TemplateApply["
    /// Set the value of [`e``type``e`]
    pub fn `field`(&self) -> `type` {
        self.local.`field`.unwrap_or(self.theme.`field`.unwrap_or(`type`::default()))
    }", <|"field"->field, "type"->type, "e" ->"`"|>]
buildGetter[pattern_]:=TemplateApply["impl StyleResolver {\n`1`\n}",
StringRiffle[getGetter/@pattern,"\n"]
]

getSetter[{field_,type_}]:=TemplateApply["impl GraphicsStyle for `type` {
    fn set_local_style(&self, context: &mut StyleResolver) {
        context.local.`field` = Some(self.clone());
    }
}", <|"field"->field, "type"->type|>]


    buildField[{{"point_size", "PointSize"},{"point_size", "PointSize"},{"point_size", "PointSize"}}]
buildGetter[{{"point_size", "PointSize"},{"point_size", "PointSize"},{"point_size", "PointSize"}}]
StringRiffle[getSetter/@{{"point_size", "PointSize"},{"point_size", "PointSize"},{"point_size", "PointSize"}},"\n"]



