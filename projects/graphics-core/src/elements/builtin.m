(* ::Package:: *)

(* ::Section:: *)
(*Prepare Data*)


SetDirectory@NotebookDirectory[];
styles = Import["../../../shape-inherit.json", "RawJSON"];


(* ::Section:: *)
(*Content*)


buildHead = "use super::*;";


propertyGetter[item_Association] := TemplateApply["\
    /// Get `docs`
    pub fn get_`field`(&self) -> `type` {
        self.`field`
    }

",
    item
];
styleGetter[item_Association] := TemplateApply["\
    /// Getter of [<*\"`\"*>`name`<*\"`\"*>]
    pub fn get_`field`(&self, style: &StyleResolver) -> `type` {
        self.`field`.unwrap_or(style.`meta`()).clone()
    }

    /// Setter of [<*\"`\"*>`name`<*\"`\"*>]
    pub fn set_`field`<T>(&mut self, value: T)
    where
        T: Into<`type`>,
    {
        self.`field` = Some(value.into())
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
buildSetterGetter[data_Association] := TemplateApply["\
impl `3` {
`1`
`2`
}
",
    {
        propertyGetter /@ data["property"] // StringJoin,
        styleGetter /@ data["style"] // StringJoin,
        data["name"]
    }
];


circleStyle = Flatten@{
    buildHead,
    buildSetterGetter[styles["point"]],
    buildSetterGetter[styles["circle"]],
    buildSetterGetter[styles["disk"]]

};
Export["disk/style.rs", StringRiffle[circleStyle , "\n\n"], "Text"];
