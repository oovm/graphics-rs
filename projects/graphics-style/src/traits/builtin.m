(* ::Package:: *)

SetDirectory@NotebookDirectory[];
styles = Import["../../../style-inherit.json", "RawJSON"];
kMap[k_ , v_] := Append[v, "typeSuper" -> k];
styles = KeyValueMap[kMap, styles];


buildHead[] := "\
use super::*;
";


getField[item_Association] := TemplateApply["\
impl AddAssign<`typeOuter`> for `typeSuper` {
    fn add_assign(&mut self, rhs: `typeOuter`) {
        self.`field` = Some(rhs.value);
    }
}

impl AddAssign<&`typeOuter`> for `typeSuper` {
    fn add_assign(&mut self, rhs: `typeOuter`) {
        self.`field` = Some(rhs.value.clone());
    }
}
",
    item
];
buildXXStyle[data_List] := getField /@ Flatten@data;


getField1[item_Association] := TemplateApply["self.`field` = rhs.`field`;", item];
getField2[item_Association] := TemplateApply["self.`field` = rhs.`field`.clone();", item];
buildSelf[data_Association] := TemplateApply["\
impl AddAssign<Self> for `3` {
    fn add_assign(&mut self, rhs: Self) {`1`}
}

impl AddAssign<&Self> for `3` {
    fn add_assign(&mut self, rhs: Self) {`2`}
}",
    {
        getField1 /@ data // StringJoin,
        getField2 /@ data // StringJoin,
        data["super"]
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
buildXXToContext[pattern_] := TemplateApply[
    "`1`",
    kMap[getField, pattern]
];


getField1[{field_, inner_, outer_}] := TemplateApply["\
impl From<`inner`> for GraphicsStyle {
    fn from(s: `inner`) -> Self {
        Self::`inner`(s.value)
    }
}
",
    <|"field" -> field, "inner" -> inner|>
];
getField2[{field_, inner_, outer_}] := TemplateApply["\
impl From<`outer`> for GraphicsStyle {
    fn from(s: `outer`) -> Self {
        Self::`outer`(s.value)
    }
}
",
    <|"field" -> field, "outer" -> outer|>
];
buildFromXX[pattern_] := TemplateApply[
    "`1``2`",
    {
        kMap[getField1, pattern],
        kMap[getField2, pattern]
    }
];



codegen = StringRiffle[
    Flatten@{
        buildHead[],
        buildXXStyle[styles],
        buildSelf[styles],
        buildXXToContext[styles],
        buildFromXX[styles]
    },
    "\n\n"
];
Export["upcast.rs", codegen, "Text"]