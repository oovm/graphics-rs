(* ::Package:: *)

(* ::Section:: *)
(*Prepare Data*)


SetDirectory@NotebookDirectory[];
color = Import["color.m"];


(* ::Section:: *)
(*Content*)


buildHead = "use super::*;";


buildColor = TemplateApply["
/// <*\"`\"*>`hex` = rgb(`r8` `g8` `b8`)<*\"`\"*> 
/// 
/// <div style=\"display: inline-block;width: 16rem;height: 1rem;border: 1px solid black;background: `hex`;\"></div>
pub const `name`: Self = Self::new(`r`, `g`, `b`, `a`);
",
    <|
    "name" -> ToUpperCase@#Name,
            "hex" -> #Hex,
    "r8" -> #RGB[[1]] ,
    "g8" -> #RGB[[2]],
        "b8" -> #RGB[[3]],
    "r" -> #RGB[[1]] / 255.0,
    "g" -> #RGB[[2]]/ 255.0,
        "b" -> #RGB[[3]]/ 255.0,
        "a" -> 1.0
        
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
