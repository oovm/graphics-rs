(* ::Package:: *)

(* ::Section:: *)
(*Prepare Data*)


SetDirectory@NotebookDirectory[];
styles = Import["../../../shape-inherit.json", "RawJSON"];


(* ::Section:: *)
(*Content*)


buildHead = "use super::*;";


propertyGetter[item_Association] := TemplateApply["\
    /// `getter`
    pub fn get_`field`(&self) -> `type` {
        self.`field`
    }
",
    item
];
styleGetter[item_Association] := TemplateApply["\
    /// Getter of [<*\"`\"*>`name`<*\"`\"*>]
    pub fn get_`field`(&self, style: &StyleResolver) -> `type` {
        self.size.unwrap_or(style.`meta`()).clone()
    }
    /// Setter of [<*\"`\"*>`name`<*\"`\"*>]
    pub fn set_`field`<T>(&mut self, value: T)
    where
        T: Into<f32>,
    {
        self.size = Some(value.into())
    }
    /// Builder of [<*\"`\"*>`name`<*\"`\"*>]
    pub fn with_`field`<T>(mut self, value: T) -> Self
    where
        T: Into<`type`>,
    {
        self.set_`field`(value);
        self
    }
",
    item
];
buildSetterGetter[data_List] := TemplateApply["\
impl `3` {
`1`
`2`
}
",
    {
        propertyGetter /@ data // StringJoin,
        styleGetter /@ data // StringJoin,
        data[[1]]["name"]
    }
];


circleStyle = Flatten@{
    buildHead,
    buildSetterGetter[styles["point"]],
    buildSetterGetter[styles["circle"]]
};
Export["circle/style.rs", StringRiffle[circleStyle , "\n\n"], "Text"]
