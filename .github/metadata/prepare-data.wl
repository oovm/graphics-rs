(* ::Package:: *)

SetDirectory@NotebookDirectory[];
RunProcess[{"wex", "style-atom-2d.yaml", "-c"}];
RunProcess[{"wex", "style-atom-3d.yaml", "-c"}];
RunProcess[{"wex", "style-group-2d.yaml", "-c"}];
RunProcess[{"wex", "style-group-2d.yaml", "-c"}];
styleAtom = Flatten[{
    Import["style-atom-2d.yaml.mx"],
    Import["style-atom-3d.yaml.mx"]
}];
styleGroup = Flatten[{
    Import["style-group-2d.yaml.mx"]
}];


isCopyType = MemberQ[{"f32", "RGBA"}, #]&;
CamelCase = StringJoin[Capitalize /@ StringSplit[#, "_"]]&;
commentLines[text_] := StringRiffle[StringJoin[{"/// ", #}]& /@ StringSplit[text, "\n"], "\n"];


atomAddition[data_] := Association@SortBy[Normal@Join[data, atomAdditionData@data], First];
atomAdditionData := Block[
    {derive = {"Debug", "Clone", "PartialEq"}},
    If[MissingQ@#["default"], AppendTo[derive, "Default"]];
    <|
        "details" -> If[MissingQ@#["details"], "", commentLines[#["details"]]],
        "typeOuter" -> If[MissingQ@#["typeOuter"], CamelCase[#field], #["typeOuter"]],
        "isCopy" -> isCopyType[#typeInner],
        "derive" -> StringRiffle[DeleteDuplicates@Sort@derive, ", "]
    |>
]&;


groupAddition[data_] := Association@SortBy[Normal@Join[data, groupAdditionData@data], First];
groupAdditionData := Block[
    {derive = StringSplit[#derive, ", "]},
    AppendTo[derive, "Debug"];
    AppendTo[derive, "Clone"];
    AppendTo[derive, "PartialEq"];
    <|
        "details" -> If[MissingQ@#["details"], "", commentLines[#["details"]]],
        "typeSuper" -> If[MissingQ@#["typeSuper"], CamelCase[#field], #["typeSuper"]],
        "derive" -> StringRiffle[DeleteDuplicates@Sort@derive, ", "]
    |>
]&;


metaData = ResourceFunction["ReadableForm"]@<|
    "styleGroup" -> SortBy[groupAddition /@ styleGroup, #field&],
    "styleAtom" -> SortBy[atomAddition /@ styleAtom, #field&]
|>;
Export["../../projects/meta-data.m", metaData, "Text"]
