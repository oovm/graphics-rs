(* ::Package:: *)

(* ::Subsection::Closed:: *)
(*Prepare Data*)


SetDirectory@NotebookDirectory[];
styleRaw = Import["../meta-data.m"];
styleGrouped = styleRaw["styleGroup"];
styleFlatten = DeleteDuplicatesBy[styleRaw["styleAtom"], #field&];
styleSubtype = Values[Association[#field -> #& /@ styleFlatten][[#subtype]]]&;


(* ::Subsection::Closed:: *)
(*Resolve*)


(* ::Subsubsection::Closed:: *)
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
    /// Get the [<*\"`\"*>`typeOuter`<*\"`\"*>] from theme and state.
    pub fn `field`(&self) -> `typeOuter` {
        self.once.`field`.or(self.local.`field`).or(self.theme.`field`).unwrap_or_default()
    }
",
    item
];
buildDrawXXInner[data_] := {
    "impl StyleResolver {",
    getDrawXXInner /@ data,
    "}"
};


text = Flatten@{
    buildHead,
    buildDrawXX @ styleFlatten,
    buildDrawXXInner @ styleFlatten
};
Export["src/resolver/content.rs", StringRiffle[text , "\n\n"], "Text"];


(* ::Subsubsection::Closed:: *)
(*Resolved*)


buildHead = "use super::*;";


getDrawXX[item_Association] := TemplateApply["
    /// Get the config of [<*\"`\"*>crate::`typeOuter`<*\"`\"*>]
	pub `field`: `typeInner`,
",
    item
];
buildDrawXX[data_] := {
    TemplateApply["
/// Get default style when not specified.
#[derive(`derive`, Serialize, Deserialize)]
pub struct `typeSuper` {
",
        data
    ],
    getDrawXX /@ styleSubtype[data]
    ,
    "}"
};


getDrawXXField[item_Association] := TemplateApply[
    "`field`: style.`field`.unwrap_or(self.`field`()).value`isCopy`,",
    item
];
buildFnResolve[item_Association] := {
    TemplateApply["
    /// Get the [<*\"`\"*>crate::`typeSuper`<*\"`\"*>] from theme and state.
    pub fn resolve_`field`(&self, style: crate::`typeSuper`) -> `typeSuper` {
        `typeSuper` {
",
        item
    ],
    getDrawXXField /@ styleSubtype[item] // StringJoin,
    "}}"
};


text = Flatten@{
    buildHead,
    buildDrawXX /@ styleGrouped,
    "impl StyleResolver {",
    buildFnResolve /@ styleGrouped,
    "}"
};
Export["src/resolver/resolved.rs", StringRiffle[text, "\n\n"], "Text"];


(* ::Subsection::Closed:: *)
(*Shapes*)


(* ::Subsubsection::Closed:: *)
(*Shape*)


buildHead = "use super::*;";


getXX[item_Association] := TemplateApply["
/// `docs`
/// `details`
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


getXXStyle[item_Association] := TemplateApply["
    /// `docs`, see more in [<*\"`\"*>`typeOuter`<*\"`\"*>].
	#[serde(skip_serializing_if = \"Option::is_none\")]
    pub `field`: Option<`typeOuter`>,
",
    item
];
buildXXStyle[data_] := { TemplateApply["
/// `docs`
/// `details`
#[derive(`derive`, Serialize, Deserialize)]
pub struct `typeSuper` {
",
    data
],
    getXXStyle /@ styleSubtype@data,
    "}"

};


text = Flatten@{
    buildHead,
    buildXXStyle /@ styleGrouped,
    getXX /@ styleFlatten
};
Export["src/shapes/shape.rs", StringRiffle[text , "\n\n"], "Text"];


(* ::Subsection:: *)
(*Traits*)


(* ::Subsubsection::Closed:: *)
(*From*)


buildHead = "use super::*;";


getDefault[item_Association] := TemplateApply["\
impl Default for `typeOuter` {
    fn default() -> Self {
        Self { value: `default` }
    }
}
",
    item
];
buildDefault[item_] := If[MissingQ@item["default"], "", getDefault@item];


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
        getAddSelf /@ styleSubtype@data // StringJoin,
        getSelfClone /@ styleSubtype@data // StringJoin,
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
buildEq[data_] := If[data["typeInner"] === "f32", getEq@data, Nothing];


text = Flatten@{
    buildHead,
    buildDefault /@ styleFlatten,
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
        getAddSelf /@ styleSubtype@data // StringJoin,
        getSelfClone /@ styleSubtype@data // StringJoin,
        data["typeSuper"]
    }
];


upcast = Flatten@{
    buildHead,
    buildAddXX /@ styleFlatten,
    buildAddSelf /@ styleGrouped
};
Export["src/traits/add_assign.rs", StringRiffle[upcast , "\n\n"], "Text"];


(* ::Subsubsection:: *)
(*DrawStyle*)


buildHead = "use super::*;";


getDrawXX[item_Association] := TemplateApply["\
impl GraphicsStyle for `typeOuter` {
    fn draw_style(&self, state: &mut StyleContext) {
        state.`field` = Some(self.clone());
    }
}

",
    item
];
buildDrawXX[data_] := getDrawXX[data];


getDrawXXStyle[item_Association] := TemplateApply[
    "state.`field` = Some(self.`field`.unwrap_or_default());",
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
        getDrawXXStyle /@ styleSubtype@data // StringJoin,
        data["typeSuper"]
    }
];


drawStyle = Flatten@{
    buildHead,
    buildDrawXX /@ styleFlatten,
    buildDrawXXStyle /@ styleGrouped
};
Export["src/traits/draw_style.rs", StringRiffle[drawStyle , "\n\n"], "Text"]'
