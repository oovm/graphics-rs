(* ::Package:: *)

(* ::Section:: *)
(*Prepare Data*)


SetDirectory@NotebookDirectory[];
styles = Import["../../../style-inherit.json", "RawJSON"];
kMap[k_ , v_] := <|"super" -> k, "items" -> (Append[#, "typeSuper" -> k]& /@ v) |>;
styles = KeyValueMap[kMap, styles];


(* ::Section:: *)
(*Content*)


buildHead = "use super::*;";


getDrawXX[item_Association] := TemplateApply["\
    /// Get default [<*\"`\"*>`typeOuter`<*\"`\"*>] when missing.
    pub `field`: Option<`typeInner`>,\
",
    item
];
buildDrawXX[data_List] := TemplateApply["\
/// Get default style when not specified.
#[derive(Debug, Clone, Default)]
pub struct StyleContext {`1`}
",
    {
        getDrawXX /@ data // StringJoin
    }
];


getDrawXXStyle[item_Association] := TemplateApply["\
    /// Set the value of [<*\"`\"*>`typeOuter`<*\"`\"*>]
    pub fn `field`(&self) -> `typeInner` {
        self.local.`field`.unwrap_or(self.theme.`field`.unwrap_or(`typeOuter`::default().value).clone())
    }
",
    item
];
buildDrawXXStyle[data_List] := TemplateApply["\
impl StyleResolver {`1`}
",
    {
        getDrawXXStyle /@ data // StringJoin
    }
];


drawStyle = Flatten@{
    buildHead,
    buildDrawXX@Flatten[#items& /@ styles],
    buildDrawXXStyle@Flatten[#items& /@ styles]
};
Export["content.rs", StringRiffle[drawStyle , "\n\n"], "Text"]
