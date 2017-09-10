{
    <|
        "type" -> "PointStyle",
        "derive" -> "Debug, Clone, Copy, PartialEq, Default",
        "docs" -> "Represent the available style of a point.",
        "subtype" ->
            {
                <|
                    "field" -> "point_size",
                    "docs" -> "Represent the size of a point",
                    "details" -> "/// 1=1px on canvas.\n///\n/// The shape of the point is always round.",
                    "derive" -> "Debug, Clone, Copy, PartialEq",
                    "typeInner" -> "f32",
                    "typeSuper" -> "PointStyle",
                    "typeOuter" -> "PointSize"
                |>,
                <|
                    "field" -> "point_color",
                    "typeInner" -> "RGBA",
                    "typeSuper" -> "PointStyle",
                    "typeOuter" -> "PointColor"
                |>
            }
    |>,
    <|
        "type" -> "CircleStyle",
        "subtype" ->
            {
                <|
                    "field" -> "circle_width",
                    "typeInner" -> "f32",
                    "typeSuper" -> "CircleStyle",
                    "typeOuter" -> "CircleWidth"
                |>,
                <|
                    "field" -> "circle_color",
                    "typeInner" -> "RGBA",
                    "typeSuper" -> "CircleStyle",
                    "typeOuter" -> "CircleColor"
                |>
            }
    |>,
    <|
        "type" -> "DiskStyle",
        "subtype" ->
            {
                <|
                    "field" -> "disk_fill_color",
                    "typeInner" -> "RGBA",
                    "typeSuper" -> "DiskStyle",
                    "typeOuter" -> "DiskFillColor"
                |>,
                <|
                    "field" -> "disk_edge_width",
                    "typeInner" -> "f32",
                    "typeSuper" -> "DiskStyle",
                    "typeOuter" -> "DiskEdgeWidth"
                |>,
                <|
                    "field" -> "disk_edge_color",
                    "typeInner" -> "RGBA",
                    "typeSuper" -> "DiskStyle",
                    "typeOuter" -> "DiskEdgeColor"
                |>
            }
    |>,
    <|
        "type" -> "LineStyle",
        "subtype" ->
            {
                <|
                    "field" -> "line_width",
                    "typeInner" -> "f32",
                    "typeSuper" -> "LineStyle",
                    "typeOuter" -> "LineWidth"
                |>,
                <|
                    "field" -> "line_color",
                    "typeInner" -> "RGBA",
                    "typeSuper" -> "LineStyle",
                    "typeOuter" -> "LineColor"
                |>
            }
    |>
}
