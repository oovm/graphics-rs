(* ::Package:: *)

(* ::Section:: *)
(*Prepare Data*)


SetDirectory@NotebookDirectory[];
color = Import["color.m"];


(* ::Section:: *)
(*Content*)


buildHead = "use super::*;";


buildColor = TemplateApply["
/// <*\"`\"*>`hex` = rgb(`r` `g` `b`)<*\"`\"*> 
/// 
/// <div style=\"display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: `hex`;\"></div>
pub const `name`: Self = Self::new(`r`, `g`, `b`, `a`);
",
    <|
    "name" -> ToUpperCase@#Name,
            "hex" -> #Hex,
    "r" -> #RGB[[1]] ,
    "g" -> #RGB[[2]],
        "b" -> #RGB[[3]],
        "a" -> 255
        
    |>
]&;


draw = Flatten@{
    buildHead,
    "// noinspection SpellCheckingInspection",
    "impl RGBA {",
    buildColor /@ color,
       "}"
};
Export["builtin.rs", StringRiffle[draw,"\n\n"], "Text"]
