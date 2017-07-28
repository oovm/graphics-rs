(* ::Package:: *)

SetDirectory@NotebookDirectory[];

buildHead[] := "\
use super::*;
";
kMap[f_, a_ -> b_] := StringRiffle[f@Append[#, a]& /@ b, "\n"];


getField[{field_, inner_, outer_ }] := TemplateApply["\
impl AddAssign<`inner`> for `outer` {
    fn add_assign(&mut self, rhs: `inner`) {
        self.`field` = Some(rhs.value);
    }
}

impl AddAssign<&`inner`> for `outer` {
    fn add_assign(&mut self, rhs: `inner`) {
        self.`field` = Some(rhs.value.clone());
    }
}
",
    <|"field" -> field, "outer" -> outer, "inner" -> inner|>
];
buildXXStyle[pattern_] := TemplateApply[
    "`1`",
    kMap[getField, pattern]
];


getField1[{field_, inner_, outer_}] := TemplateApply["self.`1` = rhs.`1`;", field];
getField2[{field_, inner_, outer_}] := TemplateApply["self.`1` = rhs.`1`.clone();", field];
buildSelf[pattern_] := TemplateApply["\
impl AddAssign<Self> for `3` {
    fn add_assign(&mut self, rhs: Self) {`1`}
}

impl AddAssign<&Self> for `3` {
    fn add_assign(&mut self, rhs: Self) {`2`}
}",
    {
        kMap[getField1, pattern] ,
        kMap[getField2, pattern],
        First@pattern
    }
];


getField[{field_, inner_, outer_}] := TemplateApply["\
impl AddAssign<`inner`> for StyleContext {
    fn add_assign(&mut self, rhs: `inner`) {
        self.`field` = Some(rhs.value);
    }
}

impl AddAssign<&`inner`> for StyleContext {
    fn add_assign(&mut self, rhs: `inner`) {
        self.`field` = Some(rhs.value);
    }
}
",
    <|"field" -> field, "outer" -> outer, "inner" -> inner, "e" -> "`"|>
];
buildXX2Context[pattern_] := TemplateApply[
    "`1`",
    kMap[getField, pattern]
];


getSetter[{field_, outer_, inner_}] := TemplateApply["
impl AddAssign<PointSize> for StyleContext {
    fn add_assign(&mut self, rhs: PointSize) {
        self.point_size = Some(rhs.value);
    }
}
",
    <|"field" -> field, "outer" -> outer|>
];
buildSetter[pattern_] := TemplateApply["\
impl AddAssign<PointSize> for StyleContext {
    fn add_assign(&mut self, rhs: PointSize) {
        self.point_size = Some(rhs.value);
    }
}
",
    kMap[getSetter, pattern]
];


getField[{field_, inner_, outer_}] := TemplateApply["\
impl From<`inner`> for GraphicsStyle {
    fn from(s: `inner`) -> Self {
        Self::`inner`(s.value)
    }
}
",
    <|"field" -> field, "inner" -> inner|>
];
buildFromXX[pattern_] := TemplateApply[
    "`1`",
    kMap[getField, pattern]
];


patterns = {
    "PointStyle" -> {
        {"point_size", "PointSize"},
        {"point_color", "PointColor"}
    },
    "LineStyle" -> {
        {"line_width", "LineWidth"},
        {"line_color", "LineColor"}
    }
};
codegen = StringRiffle[
    Flatten@{
        buildHead[],
        buildXXStyle /@ patterns,
        buildSelf /@ patterns,
        buildXX2Context /@ patterns,
        buildFromXX /@ patterns
    },
    "\n\n"
];
Export["upcast.rs", codegen, "Text"]
