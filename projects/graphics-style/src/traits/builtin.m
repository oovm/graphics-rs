(* ::Package:: *)

SetDirectory@NotebookDirectory[];
styles = Import["../../../style-inherit.json", "RawJSON"];
kMap[k_ , v_] := <|"super" -> k, "items" -> (Append[#, "typeSuper" -> k]& /@ v) |>;
styles = KeyValueMap[kMap, styles];


buildHead[] := "\
use super::*;
";


getAddXX[item_Association] := TemplateApply["\
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

impl AddAssign<`typeOuter`> for StyleContext {
    fn add_assign(&mut self, rhs: `inner`) {
        self.`field` = Some(rhs.value);
    }
}

impl AddAssign<&`typeOuter`> for StyleContext {
    fn add_assign(&mut self, rhs: `typeOuter`) {
        self.`field` = Some(rhs.value);
    }
}
",
    item
];
buildAddXX[data_Association] := TemplateApply[
    "`1`",
    {
        getAddXX /@ data["items"] // StringJoin
    }
];


getAddSelf[item_Association] := TemplateApply["self.`field` = rhs.`field`;", item];
getSelfClone[item_Association] := TemplateApply["self.`field` = rhs.`field`.clone();", item];
buildAddSelf[data_Association] := TemplateApply["\
impl AddAssign<Self> for `3` {
    fn add_assign(&mut self, rhs: Self) {`1`}
}

impl AddAssign<&Self> for `3` {
    fn add_assign(&mut self, rhs: Self) {`2`}
}",
    {
        getAddSelf /@ data["items"] // StringJoin,
        getSelfClone /@ data["items"] // StringJoin,
        data["super"]
    }
];

getFromXX[item_Association] := TemplateApply["\
impl From<`typeOuter`> for GraphicsStyle {
    fn from(s: `typeOuter`) -> Self {
        Self::`typeOuter`(s.value)
    }
}

impl From<`typeSuper`> for GraphicsStyle {
    fn from(s: `typeSuper`) -> Self {
        Self::`typeSuper`(s.value)
    }
}
",
    item
];
buildFromXX[data_Association] := TemplateApply[
    "`1`",
    {
        getFromXX /@ data["items"] // StringJoin
    }
];



codegen = StringRiffle[
    Flatten@{
        buildHead[],
        buildAddXX /@ styles,
        buildAddSelf /@ styles,
        buildFromXX /@ styles
    },
    "\n\n"
];
Export["upcast.rs", codegen, "Text"]
