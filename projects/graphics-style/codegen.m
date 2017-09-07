(* ::Package:: *)

(* ::Subsection::Closed:: *)
(*Prepare Data*)


SetDirectory@NotebookDirectory[];
styleRaw = reMap /@ Import["../style-inherit.json", "RawJSON"];
reMap[data_Association] := Block[
    {typeOuter, typeSuper, subtype},
    typeOuter = StringJoin[Capitalize /@ StringSplit[#field, "_"]]&;
    typeSuper = data["type"];
    subtype = Join[#, <|"typeSuper" -> typeSuper, "typeOuter" -> typeOuter[#]|>]& /@ data["subtype"];
    Join[data, <|"subtype" -> subtype|>]
];
styleGrouped = reMap /@ styleRaw;
styleFlatten = Flatten[#subtype& /@ styleGrouped];
Export["../style-inherit.m", ResourceFunction["ReadableForm"][styleGrouped], "Text"];


(* ::Subsection:: *)
(*Resolve*)


(* ::Subsubsection::Closed:: *)
(*Content*)


buildHead = "use super::*;";


getDrawXX[item_Association] := TemplateApply["
    /// Get default [<*\"`\"*>`typeOuter`<*\"`\"*>] when missing.
    pub `field`: Option<`typeInner`>,
",
    item
];
buildDrawXX[data_List] := TemplateApply["
/// Get default style when not specified.
#[derive(Debug, Clone, Default)]
pub struct StyleContext {`1`}
",
    {
        getDrawXX /@ data // StringJoin
    }
];


getDrawXXStyle[item_Association] := TemplateApply["
    /// Set the value of [<*\"`\"*>`typeOuter`<*\"`\"*>]
    pub fn `field`(&self) -> `typeInner` {
        self.local.`field`.unwrap_or(self.theme.`field`.unwrap_or(`typeOuter`::default().value).clone())
    }
",
    item
];
buildDrawXXStyle[data_List] := TemplateApply["
impl StyleResolver {`1`}
",
    {
        getDrawXXStyle /@ data // StringJoin
    }
];


drawStyle = Flatten@{
    buildHead,
    buildDrawXX @ styleFlatten,
    buildDrawXXStyle @ styleFlatten
};
Export["src/resolver/content.rs", StringRiffle[drawStyle , "\n\n"], "Text"];


(* ::Subsection:: *)
(*Traits*)


(* ::Subsubsection:: *)
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
    buildAddXX /@ styleGrouped,
    buildAddSelf /@ styleGrouped
};
Export["add_assign.rs", StringRiffle[upcast , "\n\n"], "Text"];


(* ::Subsubsection:: *)
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
    buildDrawXX /@ styleGrouped,
    buildDrawXXStyle /@ styleGrouped
};
Export["draw_style.rs", StringRiffle[drawStyle , "\n\n"], "Text"]
