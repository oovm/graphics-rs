(* ::Package:: *)

(* ::Section:: *)
(*Prepare Data*)


SetDirectory@NotebookDirectory[];
styles = Import["../../../style-inherit.json", "RawJSON"];
kMap[k_ , v_] := <|"super" -> k, "items" -> (Append[#, "typeSuper" -> k]& /@ v) |>;
styles = KeyValueMap[kMap, styles];


(* ::Section:: *)
(*AddAssign*)


buildHead = "use super::*;";


getAddXX[item_Association] := TemplateApply["\
impl AddAssign<`typeOuter`> for `typeSuper` {
    fn add_assign(&mut self, rhs: `typeOuter`) {
        self.`field` = Some(rhs.value);
    }
}

impl AddAssign<&`typeOuter`> for `typeSuper` {
    fn add_assign(&mut self, rhs: &`typeOuter`) {
        self.`field` = Some(rhs.value.clone());
    }
}

impl AddAssign<`typeOuter`> for StyleContext {
    fn add_assign(&mut self, rhs: `typeOuter`) {
        self.`field` = Some(rhs.value);
    }
}

impl AddAssign<&`typeOuter`> for StyleContext {
    fn add_assign(&mut self, rhs: &`typeOuter`) {
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
    fn add_assign(&mut self, rhs: &Self) {`2`}
}",
    {
        getAddSelf /@ data["items"] // StringJoin,
        getSelfClone /@ data["items"] // StringJoin,
        data["super"]
    }
];


upcast = Flatten@{
    buildHead,
    buildAddXX /@ styles,
    buildAddSelf /@ styles
};
Export["add_assign.rs", StringRiffle[upcast , "\n\n"], "Text"];


(* ::Section:: *)
(*AddAssign*)


buildHead = "use super::*;";


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


upcast = Flatten@{
    buildHead,
    buildFromXX /@ styles
};
Export["from.rs", StringRiffle[upcast , "\n\n"], "Text"];


(* ::Section:: *)
(*DrawStyle*)


buildHead = "use super::*;";


getDrawXX[item_Association] := TemplateApply["\
impl GraphicsStyle for `typeOuter` {
    fn draw_style(&self, state: &mut StyleContext) {
        state.`field` = Some(self.value.clone());
    }
}

",
    item
];
buildDrawXX[data_Association] := TemplateApply[
    "`1`",
    {
        getDrawXX /@ data["items"] // StringJoin
    }
];


getDrawXXStyle[item_Association] := TemplateApply["\
state.`field` = Some(self.`field`.unwrap_or(`typeOuter`::default().value).clone());\
",
    item
];
buildDrawXXStyle[data_Association] := TemplateApply["\
impl GraphicsStyle for `2` {
    fn draw_style(&self, state: &mut StyleContext) {
`1`
    }
}
",
    {
        getDrawXXStyle /@ data["items"] // StringJoin,
        data["items"][[1]]["typeSuper"]
    }
];


drawStyle = Flatten@{
    buildHead,
    buildDrawXX /@ styles,
    buildDrawXXStyle /@ styles
};
Export["draw_style.rs", StringRiffle[drawStyle , "\n\n"], "Text"]
