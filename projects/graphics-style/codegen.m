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


(* ::Subsection::Closed:: *)
(*Resolve*)


(* ::Subsubsection:: *)
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
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
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
(*Shapes*)


(* ::Subsubsection:: *)
(*DrawStyle*)


buildHead = "use super::*;";


getXX[item_Association] := TemplateApply["
/// `docs`
///
`details`
#[derive(`derive`, Serialize, Deserialize)]
pub struct `typeOuter` {
    /// Actual value for [<*\"`\"*>StyleResolver::`field`<*\"`\"*>]
    pub value: `typeInner`,
}
",
    item
];
buildXX[data_] := getXX[data];


getXXStyle[item_Association] := TemplateApply["\
    /// `docs`, see more in [<*\"`\"*>`typeOuter`<*\"`\"*>].
    pub `field`: Option<`typeInner`>,
",
    item
];
buildXXStyle[data_] := TemplateApply["
/// `docs`
#[derive(`derive`, Serialize, Deserialize)]
pub struct `type` {`record`}

`subtypes`
",
    Join[
    data,
    <|
    "subtypes" -> StringJoin[getXX /@ data["subtype"]],
        "record"->StringJoin[getXXStyle /@ data["subtype"]]
    |>]
];


drawStyle = Flatten@{
        buildXXStyle /@ styleGrouped
};
Export["src/shapes/shape.rs", StringRiffle[drawStyle , "\n\n"], "Text"]


(* ::Subsection:: *)
(*Traits*)


(* ::Subsubsection::Closed:: *)
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
buildAddXX[data_List] := TemplateApply[
    "`1`",
    {
        getAddXX /@ data // StringJoin
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
        getAddSelf /@ data["subtype"] // StringJoin,
        getSelfClone /@ data["subtype"] // StringJoin,
        data["type"]
    }
];


upcast = Flatten@{
    buildHead,
    buildAddXX @ styleFlatten,
    buildAddSelf /@ styleGrouped
};
Export["src/traits/add_assign.rs", StringRiffle[upcast , "\n\n"], "Text"];


(* ::Subsubsection::Closed:: *)
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
buildDrawXX[data_] := getDrawXX[data];


getDrawXXStyle[item_Association] := TemplateApply["\
state.`field` = Some(self.`field`.unwrap_or(`typeOuter`::default().value).clone());\
",
    item
];
buildDrawXXStyle[data_] := TemplateApply["\
impl GraphicsStyle for `2` {
    fn draw_style(&self, state: &mut StyleContext) {
`1`
    }
}
",
    {
        getDrawXXStyle /@ data["subtype"] // StringJoin,
        data["type"]
    }
];


drawStyle = Flatten@{
    buildHead,
    buildDrawXX /@ styleFlatten,
    buildDrawXXStyle /@ styleGrouped
};
Export["src/traits/draw_style.rs", StringRiffle[drawStyle , "\n\n"], "Text"]'
