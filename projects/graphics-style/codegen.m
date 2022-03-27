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


(* ::Subsubsection:: *)
(*Content*)


buildHead = "use super::*;";


getDrawXX[item_Association] := TemplateApply["
    /// Get default [<*\"`\"*>`typeOuter`<*\"`\"*>] when missing.
    #[serde(skip_serializing_if = \"Option::is_none\")]
	pub `field`: Option<`typeOuter`>,
",
    item
];
buildDrawXX[data_] := TemplateApply["
/// Get default style when not specified.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StyleContext {`1`}
",
    {
        getDrawXX /@ data // StringJoin
    }
];


getDrawXXInner[item_Association] := TemplateApply["
    /// Get the [<*\"`\"*>`field`<*\"`\"*>] from theme and state.
    pub fn `field`(&self) -> `typeOuter` {
        self.once.`field`.or(self.local.`field`).or(self.theme.`field`).unwrap_or_default()
    }
",
    item
];
buildDrawXXInner[data_] := getDrawXXInner@data;


getDrawXXField[item_Association] := TemplateApply[
	"`field`: Some(self.`field`()),",
    item
];
getDrawXXOuter[item_Association] := TemplateApply["
    /// Get the [<*\"`\"*>`field`<*\"`\"*>] from theme and state.
    pub fn point_style(&self) -> `typeSuper` {
        `typeSuper` { `FIELDS`}
    }
",
    Join[
    item, 
    <|
    "FIELDS"->getDrawXXField/@item["subtype"]
    |>
    ]
];
buildDrawXXOuter[data_] := TemplateApply["
impl StyleResolver {`1` `2`}
",
    {
        getDrawXXInner /@ data // StringJoin,
        getDrawXXOuter @ data
    }
];


text = Flatten@{
    buildHead,
    buildDrawXX @ styleFlatten,
    "impl StyleResolver {",
    buildDrawXXInner /@ styleFlatten,
    buildDrawXXOuter /@ styleGrouped,
    "}"
};
Export["src/resolver/content.rs", StringRiffle[text , "\n\n"], "Text"];


(* ::Subsection:: *)
(*Shapes*)


(* ::Subsubsection::Closed:: *)
(*DrawStyle*)


buildHead = "use super::*;";


getXX[item_Association] := TemplateApply["
/// `docs`
///
`details`
#[derive(`derive`, Serialize, Deserialize)]
#[serde(into = \"`typeInner`\", from = \"`typeInner`\")]
pub struct `typeOuter` {
    /// Actual value for [<*\"`\"*>StyleResolver::`field`<*\"`\"*>]
    pub value: `typeInner`,
}
",
    item
];
buildXX[data_] := getXX @ data;


getXXStyle[item_Association] := TemplateApply["\
    /// `docs`, see more in [<*\"`\"*>`typeOuter`<*\"`\"*>].
	#[serde(skip_serializing_if = \"Option::is_none\")]
    pub `field`: Option<`typeOuter`>,
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
(*From*)


buildHead = "use super::*;";


getAddXX[item_Association] := TemplateApply["\
impl From<`typeInner`> for `typeOuter` {
    fn from(value: `typeInner`) -> Self {
        Self { value }
    }
}

impl Into<`typeInner`> for `typeOuter` {
    fn into(self) -> `typeInner` {
        self.value
    }
}
",
    item
];
buildFromXX[data_] := getAddXX@data;


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


getEq[item_Association] := TemplateApply["\
impl PartialEq<f32> for `typeOuter` {
    fn eq(&self, other: &f32) -> bool {
        self.value.eq(other)
    }
}

impl PartialOrd<f32> for `typeOuter` {
    fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}
",
    item
];
buildEq[data_] := If[data["typeInner"]==="f32",getEq@data,Nothing];


text = Flatten@{
    buildHead,
    buildFromXX /@ styleFlatten,
    buildEq /@ styleFlatten
};
Export["src/traits/convert.rs", StringRiffle[text , "\n\n"], "Text"];


(* ::Subsubsection:: *)
(*AddAssign*)


buildHead = "use super::*;";


getAddXX[item_Association] := TemplateApply["\
impl AddAssign<`typeOuter`> for `typeSuper` {
    fn add_assign(&mut self, rhs: `typeOuter`) {
        self.`field` = Some(rhs);
    }
}

impl AddAssign<&`typeOuter`> for `typeSuper` {
    fn add_assign(&mut self, rhs: &`typeOuter`) {
        self.`field` = Some(rhs.clone());
    }
}

impl AddAssign<`typeOuter`> for StyleContext {
    fn add_assign(&mut self, rhs: `typeOuter`) {
        self.`field` = Some(rhs);
    }
}

impl AddAssign<&`typeOuter`> for StyleContext {
    fn add_assign(&mut self, rhs: &`typeOuter`) {
        self.`field` = Some(rhs.clone());
    }
}
",
    item
];
buildAddXX[data_] := getAddXX@data;


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
    buildAddXX /@ styleFlatten,
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
