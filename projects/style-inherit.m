{
    <|
        "field" -> "point_style",
        "derive" -> "Debug, Clone, Copy, PartialEq, Default",
        "docs" -> "Represent the available style of a point.",
        "subtype" ->
            {
                <|
                    "field" -> "point_size",
                    "typeInner" -> "f32",
                    "docs" -> "Represent the size of a point, default size is 1.0",
                    "details" -> "The shape of the point is always round.",
                    "derive" -> "Debug, Clone, Copy, PartialEq",
                    "typeSuper" -> "PointStyle",
                    "typeOuter" -> "PointSize"
                |>,
                <|
                    "field" -> "point_color",
                    "typeInner" -> "RGBA",
                    "docs" -> "Represent the color of a point, default color is black",
                    "details" -> "",
                    "derive" -> "Debug, Clone, Copy, PartialEq, Default",
                    "typeSuper" -> "PointStyle",
                    "typeOuter" -> "PointColor"
                |>
            },
        "type" -> "PointStyle"
    |>,
    <|
        "field" -> "point3D_style",
        "derive" -> "Debug, Clone, Copy, PartialEq, Default",
        "docs" -> "Represent the available style of a point.",
        "subtype" ->
            {
                <|
                    "field" -> "point_size",
                    "typeInner" -> "f32",
                    "docs" -> "Represent the size of a point, default size is 1.0",
                    "details" -> "The shape of the point is always round.",
                    "derive" -> "Debug, Clone, Copy, PartialEq",
                    "typeSuper" -> "Point3DStyle",
                    "typeOuter" -> "PointSize"
                |>,
                <|
                    "field" -> "point_color",
                    "typeInner" -> "RGBA",
                    "docs" -> "Represent the color of a point, default color is black",
                    "details" -> "",
                    "derive" -> "Debug, Clone, Copy, PartialEq, Default",
                    "typeSuper" -> "Point3DStyle",
                    "typeOuter" -> "PointColor"
                |>
            },
        "type" -> "Point3DStyle"
    |>,
    <|
        "field" -> "circle_style",
        "derive" -> "Debug, Clone, Copy, PartialEq",
        "docs" -> "Represent the available style of a circle.",
        "subtype" ->
            {
                <|
                    "field" -> "circle_width",
                    "typeInner" -> "f32",
                    "docs" -> "Represent the width of a circle, default width is 1.0",
                    "details" -> "",
                    "derive" -> "Debug, Clone, Copy, PartialEq",
                    "typeSuper" -> "CircleStyle",
                    "typeOuter" -> "CircleWidth"
                |>,
                <|
                    "field" -> "circle_color",
                    "typeInner" -> "RGBA",
                    "docs" -> "Represent the color of a point, default color is black",
                    "details" -> "",
                    "derive" -> "Debug, Clone, Copy, PartialEq, Default",
                    "typeSuper" -> "CircleStyle",
                    "typeOuter" -> "CircleColor"
                |>
            },
        "type" -> "CircleStyle"
    |>,
    <|
        "field" -> "disk_style",
        "derive" -> "Debug, Clone, Copy, PartialEq",
        "docs" -> "Represent the available style of a disk.",
        "subtype" ->
            {
                <|
                    "field" -> "disk_fill_color",
                    "typeInner" -> "RGBA",
                    "docs" -> "Represent the color of a disk, default color is black",
                    "details" -> "",
                    "derive" -> "Debug, Clone, Copy, PartialEq, Default",
                    "typeSuper" -> "DiskStyle",
                    "typeOuter" -> "DiskFillColor"
                |>,
                <|
                    "field" -> "disk_edge_width",
                    "typeInner" -> "f32",
                    "docs" -> "Represent the edge width of a disk, default width is 1.0",
                    "details" -> "",
                    "derive" -> "Debug, Clone, Copy, PartialEq, Default",
                    "typeSuper" -> "DiskStyle",
                    "typeOuter" -> "DiskEdgeWidth"
                |>,
                <|
                    "field" -> "disk_edge_color",
                    "typeInner" -> "RGBA",
                    "docs" -> "Represent the edge color of a disk, default is transparent",
                    "details" -> "",
                    "derive" -> "Debug, Clone, Copy, PartialEq, Default",
                    "typeSuper" -> "DiskStyle",
                    "typeOuter" -> "DiskEdgeColor"
                |>
            },
        "type" -> "DiskStyle"
    |>,
    <|
        "field" -> "line_style",
        "derive" -> "Debug, Clone, Copy, PartialEq",
        "docs" -> "Represent the available style of a line.",
        "subtype" ->
            {
                <|
                    "field" -> "line_width",
                    "typeInner" -> "f32",
                    "docs" -> "Represent the with of a line, default width is 1.0",
                    "details" -> "",
                    "derive" -> "Debug, Clone, Copy, PartialEq",
                    "typeSuper" -> "LineStyle",
                    "typeOuter" -> "LineWidth"
                |>,
                <|
                    "field" -> "line_color",
                    "typeInner" -> "RGBA",
                    "docs" -> "Represent the color of a line, default color is black",
                    "details" -> "",
                    "derive" -> "Debug, Clone, Copy, PartialEq, Default",
                    "typeSuper" -> "LineStyle",
                    "typeOuter" -> "LineColor"
                |>
            },
        "type" -> "LineStyle"
    |>,
    <|
        "field" -> "triangle_style",
        "derive" -> "Debug, Clone, Copy, PartialEq",
        "docs" -> "Represent the available style of a triangle.",
        "subtype" ->
            {
                <|
                    "field" -> "triangle_edge_width",
                    "typeInner" -> "f32",
                    "docs" -> "Represent the with of a line, default width is 1.0",
                    "details" -> "",
                    "derive" -> "Debug, Clone, Copy, PartialEq",
                    "typeSuper" -> "TriangleStyle",
                    "typeOuter" -> "TriangleEdgeWidth"
                |>,
                <|
                    "field" -> "triangle_fill_color",
                    "typeInner" -> "RGBA",
                    "docs" -> "Represent the color of a line, default color is black",
                    "details" -> "",
                    "derive" -> "Debug, Clone, Copy, PartialEq, Default",
                    "typeSuper" -> "TriangleStyle",
                    "typeOuter" -> "TriangleFillColor"
                |>
            },
        "type" -> "TriangleStyle"
    |>,
    <|
        "field" -> "square_style",
        "derive" -> "Debug, Clone, Copy, PartialEq",
        "docs" -> "Represent the available style of a triangle.",
        "subtype" ->
            {
                <|
                    "field" -> "square_edge_width",
                    "typeInner" -> "f32",
                    "docs" -> "Represent the with of a line, default width is 1.0",
                    "details" -> "",
                    "derive" -> "Debug, Clone, Copy, PartialEq",
                    "typeSuper" -> "SquareStyle",
                    "typeOuter" -> "SquareEdgeWidth"
                |>,
                <|
                    "field" -> "square_fill_color",
                    "typeInner" -> "RGBA",
                    "docs" -> "Represent the color of a line, default color is black",
                    "details" -> "",
                    "derive" -> "Debug, Clone, Copy, PartialEq, Default",
                    "typeSuper" -> "SquareStyle",
                    "typeOuter" -> "SquareFillColor"
                |>
            },
        "type" -> "SquareStyle"
    |>,
    <|
        "field" -> "rectangle_style",
        "derive" -> "Debug, Clone, Copy, PartialEq",
        "docs" -> "Represent the available style of a triangle.",
        "subtype" ->
            {
                <|
                    "field" -> "square_edge_width",
                    "typeInner" -> "f32",
                    "docs" -> "Represent the with of a line, default width is 1.0",
                    "details" -> "",
                    "derive" -> "Debug, Clone, Copy, PartialEq",
                    "typeSuper" -> "RectangleStyle",
                    "typeOuter" -> "SquareEdgeWidth"
                |>,
                <|
                    "field" -> "square_fill_color",
                    "typeInner" -> "RGBA",
                    "docs" -> "Represent the color of a line, default color is black",
                    "details" -> "",
                    "derive" -> "Debug, Clone, Copy, PartialEq, Default",
                    "typeSuper" -> "RectangleStyle",
                    "typeOuter" -> "SquareFillColor"
                |>
            },
        "type" -> "RectangleStyle"
    |>
}
