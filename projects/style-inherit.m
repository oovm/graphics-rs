{
    <|
        "type" -> "PointStyle",
        "field" -> "point_style",
        "derive" -> "Debug, Clone, Copy, PartialEq, Default",
        "docs" -> "Represent the available style of a point.",
        "subtype" ->
            {
                <|
                    "field" -> "point_size",
                    "typeInner" -> "f32",
                    "docs" -> "Represent the size of a point",
                    "details" -> "/// 1=1px on canvas.\n///\n/// The shape of the point is always round.",
                    "derive" -> "Debug, Clone, Copy, PartialEq",
                    "typeSuper" -> "PointStyle",
                    "typeOuter" -> "PointSize"
                |>,
                <|
                    "field" -> "point_color",
                    "typeInner" -> "RGBA",
                    "docs" -> "",
                    "details" -> "/// 1=1px on canvas.\n///\n/// The shape of the point is always round.",
                    "derive" -> "Debug, Clone, Copy, PartialEq, Default",
                    "typeSuper" -> "PointStyle",
                    "typeOuter" -> "PointColor"
                |>
            }
    |>,
    <|
        "type" -> "CircleStyle",
        "field" -> "circle_style",
        "derive" -> "Debug, Clone, Copy, PartialEq",
        "docs" -> "",
        "subtype" ->
            {
                <|
                    "field" -> "circle_width",
                    "typeInner" -> "f32",
                    "docs" -> "",
                    "details" -> "/// 1=1px on canvas.\n///\n/// The shape of the point is always round.",
                    "derive" -> "Debug, Clone, Copy, PartialEq",
                    "typeSuper" -> "CircleStyle",
                    "typeOuter" -> "CircleWidth"
                |>,
                <|
                    "field" -> "circle_color",
                    "typeInner" -> "RGBA",
                    "docs" -> "",
                    "details" -> "/// 1=1px on canvas.\n///\n/// The shape of the point is always round.",
                    "derive" -> "Debug, Clone, Copy, PartialEq, Default",
                    "typeSuper" -> "CircleStyle",
                    "typeOuter" -> "CircleColor"
                |>
            }
    |>,
    <|
        "type" -> "DiskStyle",
        "field" -> "disk_style",
        "derive" -> "Debug, Clone, Copy, PartialEq",
        "docs" -> "",
        "subtype" ->
            {
                <|
                    "field" -> "disk_fill_color",
                    "typeInner" -> "RGBA",
                    "docs" -> "",
                    "details" -> "/// 1=1px on canvas.\n///\n/// The shape of the point is always round.",
                    "derive" -> "Debug, Clone, Copy, PartialEq, Default",
                    "typeSuper" -> "DiskStyle",
                    "typeOuter" -> "DiskFillColor"
                |>,
                <|
                    "field" -> "disk_edge_width",
                    "typeInner" -> "f32",
                    "docs" -> "",
                    "details" -> "/// 1=1px on canvas.\n///\n/// The shape of the point is always round.",
                    "derive" -> "Debug, Clone, Copy, PartialEq, Default",
                    "typeSuper" -> "DiskStyle",
                    "typeOuter" -> "DiskEdgeWidth"
                |>,
                <|
                    "field" -> "disk_edge_color",
                    "typeInner" -> "RGBA",
                    "docs" -> "",
                    "details" -> "/// 1=1px on canvas.\n///\n/// The shape of the point is always round.",
                    "derive" -> "Debug, Clone, Copy, PartialEq, Default",
                    "typeSuper" -> "DiskStyle",
                    "typeOuter" -> "DiskEdgeColor"
                |>
            }
    |>,
    <|
        "type" -> "LineStyle",
        "field" -> "line_style",
        "derive" -> "Debug, Clone, Copy, PartialEq",
        "docs" -> "",
        "subtype" ->
            {
                <|
                    "field" -> "line_width",
                    "typeInner" -> "f32",
                    "docs" -> "",
                    "details" -> "/// 1=1px on canvas.\n///\n/// The shape of the point is always round.",
                    "derive" -> "Debug, Clone, Copy, PartialEq",
                    "typeSuper" -> "LineStyle",
                    "typeOuter" -> "LineWidth"
                |>,
                <|
                    "field" -> "line_color",
                    "typeInner" -> "RGBA",
                    "docs" -> "",
                    "details" -> "/// 1=1px on canvas.\n///\n/// The shape of the point is always round.",
                    "derive" -> "Debug, Clone, Copy, PartialEq, Default",
                    "typeSuper" -> "LineStyle",
                    "typeOuter" -> "LineColor"
                |>
            }
    |>
}
