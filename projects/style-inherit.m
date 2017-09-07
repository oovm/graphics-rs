{
    <|
        "type" -> "PointStyle",
        "subtype" ->
            {
                <|
                    "field" -> "point_size",
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
                    "field" -> "fill_color",
                    "typeInner" -> "RGBA",
                    "typeSuper" -> "DiskStyle",
                    "typeOuter" -> "FillColor"
                |>,
                <|
                    "field" -> "edge_width",
                    "typeInner" -> "f32",
                    "typeSuper" -> "DiskStyle",
                    "typeOuter" -> "EdgeWidth"
                |>,
                <|
                    "field" -> "edge_color",
                    "typeInner" -> "RGBA",
                    "typeSuper" -> "DiskStyle",
                    "typeOuter" -> "EdgeColor"
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
