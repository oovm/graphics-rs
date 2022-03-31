(* ::Package:: *)

SetDirectory@NotebookDirectory[];
<<ImportX`;
styleAtom = Flatten[{
    Import["style-atom-2d.yaml","YAML"],
    Import["style-atom-3d.yaml","YAML"]
}];
styleGroup = Flatten[{
    Import["style-group-2d.yaml","YAML"]
}];


isCopyType = MemberQ[{"f32", "RGBA"}, #]&;
CamelCase = StringJoin[Capitalize /@ StringSplit[#, "_"]]&;
commentLines[text_] := "\n" <> StringRiffle[StringJoin[{"/// ", #}]& /@ StringSplit[text, "\n"], "\n"];


atomAddition[data_] := Association@SortBy[Normal@Join[data, atomAdditionData@data], First];
atomAdditionData := Block[
    {
        derive = {"Debug", "Clone", "PartialEq"},
        isCopy = isCopyType[#typeInner]
    },
    If[MissingQ@#["default"], AppendTo[derive, "Default"]];
    If[isCopy, AppendTo[derive, "Copy"]];
    <|
        "details" -> If[MissingQ@#["details"], "", commentLines[#["details"]]],
        "typeOuter" -> If[MissingQ@#["typeOuter"], CamelCase[#field], #["typeOuter"]],
        "isCopy" -> If[isCopy, "", ".clone()"],
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
Export["../../projects/meta-data.wl", metaData, "Text"]
