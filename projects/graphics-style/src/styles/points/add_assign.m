(* ::Package:: *)

SetDirectory@NotebookDirectory[];

buildHead[] := "
use crate::*;
";
kMap[f_, a_ -> b_] := StringRiffle[f@Append[#, a]& /@ b, "\n"];


getField[{field_, inner_, outer_ }] := Block[
    {},
    TemplateApply["\
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
    ]
];
buildSuper[pattern_] := TemplateApply[
    "`1`",
    kMap[getField, pattern]
];


getField1[{field_, inner_, outer_}] := TemplateApply["self.`1` = rhs.`1`;", field];
getField2[{field_, inner_, outer_}] := TemplateApply["self.`1` = rhs.`1`.clone();", field];
buildSuper2[pattern_] := TemplateApply["\
impl AddAssign<Self> for `outer` {
    fn add_assign(&mut self, rhs: Self) {`1`}
}

impl AddAssign<&Self> for `outer` {
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
buildStyleContext1[pattern_] := TemplateApply[
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
    kMap[getSetter, #]& /@ pattern
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
buildConvert[key_ -> pattern_] := TemplateApply[
    "`1`",
    kMap[getConvert, #]& /@ pattern
];


patterns = {
    "PointStyle" -> {
        {"point_size", "PointSize"},
        {"point_color", "PointColor"}
    }
};
codegen = StringRiffle[
    Flatten@{
        buildHead[],
        buildSuper /@ patterns,
        buildSuper2 /@ patterns,
        buildStyleContext1 /@ patterns
    },
    "\n\n"
];
Export["add_assign.rs", codegen, "Text"]
