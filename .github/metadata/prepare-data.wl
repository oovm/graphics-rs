(* ::Package:: *)

SetDirectory@NotebookDirectory[];
RunProcess[{"wex", "style-atom-2d.yaml", "-c"}];
RunProcess[{"wex", "style-atom-3d.yaml", "-c"}];
RunProcess[{"wex", "style-group-2d.yaml", "-c"}];
RunProcess[{"wex", "style-group-2d.yaml", "-c"}];
styleAtom=Flatten[{
Import["style-atom-2d.yaml.mx"]
}];
styleGroup=Flatten[{
Import["style-group-2d.yaml.mx"]
}];


isCopyType=MemberQ[{"f32","RGBA"},#]&;
CamelCase = StringJoin[Capitalize /@ StringSplit[#, "_"]]&;
reGroupAtom=Join[
#,
<|
"details"->If[MissingQ@#["details"],"",#details],
"typeOuter"->If[MissingQ@#["typeOuter"],CamelCase[#field],#["typeOuter"]],
"isCopy"->isCopyType[#typeInner],
"derive"->If[MissingQ@#["default"],StringJoin[#derive,", Default"]]
|>
]&;
reGroupAtom/@styleAtom
