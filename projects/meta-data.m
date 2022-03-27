<|
    "styleGroup" ->
        {
            <|
                "derive" -> "Clone, Copy, Debug, PartialEq",
                "details" -> "",
                "docs" -> "Represent the available style of a background.",
                "field" -> "background_style",
                "subtype" -> { "background_color" },
                "typeSuper" -> "BackgroundStyle"
            |>,
            <|
                "derive" -> "Clone, Copy, Debug, PartialEq",
                "details" -> "/// The circle is defined by its center and its radius.",
                "docs" -> "Represent the available style of a circle.",
                "field" -> "circle_style",
                "subtype" -> { "circle_width", "circle_color" },
                "typeSuper" -> "CircleStyle"
            |>,
            <|
                "derive" -> "Clone, Copy, Debug, PartialEq",
                "details" -> "",
                "docs" -> "Represent the available style of a disk.",
                "field" -> "disk_style",
                "subtype" ->
                    { "disk_fill_color", "disk_edge_width", "disk_edge_color" },
                "typeSuper" -> "DiskStyle"
            |>,
            <|
                "derive" -> "Clone, Copy, Debug, PartialEq",
                "details" -> "",
                "docs" -> "Represent the available style of a style.",
                "field" -> "edge_color",
                "subtype" ->
                    {
                        "disk_edge_color",
                        "triangle_edge_color",
                        "square_edge_color",
                        "rectangle_edge_color",
                        "polygon_edge_color"
                    },
                "typeSuper" -> "EdgeColor"
            |>,
            <|
                "derive" -> "Clone, Copy, Debug, PartialEq",
                "details" -> "",
                "docs" -> "Represent the available style of a style.",
                "field" -> "edge_style",
                "subtype" -> { "disk_edge_width", "triangle_edge_color" },
                "typeSuper" -> "EdgeStyle"
            |>,
            <|
                "derive" -> "Clone, Copy, Debug, PartialEq",
                "details" -> "",
                "docs" -> "Represent the available style of a style.",
                "field" -> "edge_width",
                "subtype" ->
                    {
                        "disk_edge_width",
                        "triangle_edge_width",
                        "square_edge_width",
                        "rectangle_edge_width",
                        "polygon_edge_width"
                    },
                "typeSuper" -> "EdgeWidth"
            |>,
            <|
                "derive" -> "Clone, Copy, Debug, PartialEq",
                "details" -> "",
                "docs" -> "Represent the available style of a style.",
                "field" -> "fill_color",
                "subtype" ->
                    {
                        "disk_fill_color",
                        "triangle_fill_color",
                        "square_fill_color",
                        "rectangle_fill_color",
                        "polygon_fill_color"
                    },
                "typeSuper" -> "FillColor"
            |>,
            <|
                "derive" -> "Clone, Copy, Debug, PartialEq",
                "details" -> "",
                "docs" -> "Represent the available style of a line.",
                "field" -> "line_style",
                "subtype" -> { "line_width", "line_color" },
                "typeSuper" -> "LineStyle"
            |>,
            <|
                "derive" -> "Clone, Copy, Debug, PartialEq",
                "details" -> "",
                "docs" -> "Represent the available style of a point.",
                "field" -> "point_style",
                "subtype" -> { "point_size", "point_color" },
                "typeSuper" -> "PointStyle"
            |>,
            <|
                "derive" -> "Clone, Copy, Debug, PartialEq",
                "details" -> "",
                "docs" -> "Represent the available style of a polygon.",
                "field" -> "polygon_style",
                "subtype" ->
                    {
                        "polygon_fill_color",
                        "polygon_edge_width",
                        "polygon_edge_color"
                    },
                "typeSuper" -> "PolygonStyle"
            |>,
            <|
                "derive" -> "Clone, Copy, Debug, PartialEq",
                "details" -> "/// The polyline is a collection of points.",
                "docs" -> "Represent the available style of a line.",
                "field" -> "polyline_style",
                "subtype" -> { "polyline_width", "polyline_color" },
                "typeSuper" -> "PolylineStyle"
            |>,
            <|
                "derive" -> "Clone, Copy, Debug, PartialEq",
                "details" -> "",
                "docs" -> "Represent the available style of a rectangle.",
                "field" -> "rectangle_style",
                "subtype" ->
                    {
                        "rectangle_fill_color",
                        "rectangle_edge_width",
                        "rectangle_edge_color"
                    },
                "typeSuper" -> "RectangleStyle"
            |>,
            <|
                "derive" -> "Clone, Copy, Debug, PartialEq",
                "details" -> "",
                "docs" -> "Represent the available style of a triangle.",
                "field" -> "square_style",
                "subtype" ->
                    {
                        "square_fill_color",
                        "square_edge_width",
                        "square_edge_color"
                    },
                "typeSuper" -> "SquareStyle"
            |>,
            <|
                "derive" -> "Clone, Copy, Debug, PartialEq",
                "details" -> "",
                "docs" -> "Represent the available style of a text.",
                "field" -> "text_style",
                "subtype" -> { "text_color", "text_size", "text_font" },
                "typeSuper" -> "TextStyle"
            |>,
            <|
                "derive" -> "Clone, Copy, Debug, PartialEq",
                "details" -> "",
                "docs" -> "Represent the available style of a triangle.",
                "field" -> "triangle_style",
                "subtype" ->
                    {
                        "triangle_fill_color",
                        "triangle_edge_width",
                        "triangle_edge_color"
                    },
                "typeSuper" -> "TriangleStyle"
            |>
        },
    "styleAtom" ->
        {
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the color of a line, default color is black",
                "field" -> "background_color",
                "isCopy" -> True,
                "typeInner" -> "RGBA",
                "typeOuter" -> "BackgroundColor"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the color of a point, default color is black",
                "field" -> "circle_color",
                "isCopy" -> True,
                "typeInner" -> "RGBA",
                "typeOuter" -> "CircleColor"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the color of a point, default color is black",
                "field" -> "circle_color",
                "isCopy" -> True,
                "typeInner" -> "RGBA",
                "typeOuter" -> "CircleColor"
            |>,
            <|
                "default" -> "1.0",
                "derive" -> "Clone, Debug, PartialEq",
                "details" -> "",
                "docs" -> "Represent the width of a circle, default width is 1.0",
                "field" -> "circle_width",
                "isCopy" -> True,
                "typeInner" -> "f32",
                "typeOuter" -> "CircleWidth"
            |>,
            <|
                "default" -> "1.0",
                "derive" -> "Clone, Debug, PartialEq",
                "details" -> "",
                "docs" -> "Represent the width of a circle, default width is 1.0",
                "field" -> "circle_width",
                "isCopy" -> True,
                "typeInner" -> "f32",
                "typeOuter" -> "CircleWidth"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the edge color of a disk, default is transparent",
                "field" -> "disk_edge_color",
                "isCopy" -> True,
                "typeInner" -> "RGBA",
                "typeOuter" -> "DiskEdgeColor"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the edge color of a disk, default is transparent",
                "field" -> "disk_edge_color",
                "isCopy" -> True,
                "typeInner" -> "RGBA",
                "typeOuter" -> "DiskEdgeColor"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the edge width of a disk, default width is 1.0",
                "field" -> "disk_edge_width",
                "isCopy" -> True,
                "typeInner" -> "f32",
                "typeOuter" -> "DiskEdgeWidth"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the edge width of a disk, default width is 1.0",
                "field" -> "disk_edge_width",
                "isCopy" -> True,
                "typeInner" -> "f32",
                "typeOuter" -> "DiskEdgeWidth"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the color of a disk, default color is black",
                "field" -> "disk_fill_color",
                "isCopy" -> True,
                "typeInner" -> "RGBA",
                "typeOuter" -> "DiskFillColor"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the color of a disk, default color is black",
                "field" -> "disk_fill_color",
                "isCopy" -> True,
                "typeInner" -> "RGBA",
                "typeOuter" -> "DiskFillColor"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the color of a line, default color is black",
                "field" -> "line_color",
                "isCopy" -> True,
                "typeInner" -> "RGBA",
                "typeOuter" -> "LineColor"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the color of a line, default color is black",
                "field" -> "line_color",
                "isCopy" -> True,
                "typeInner" -> "RGBA",
                "typeOuter" -> "LineColor"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the with of a line, default width is 1.0",
                "field" -> "line_width",
                "isCopy" -> True,
                "typeInner" -> "f32",
                "typeOuter" -> "LineWidth"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the with of a line, default width is 1.0",
                "field" -> "line_width",
                "isCopy" -> True,
                "typeInner" -> "f32",
                "typeOuter" -> "LineWidth"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the color of a point, default color is black",
                "field" -> "point_color",
                "isCopy" -> True,
                "typeInner" -> "RGBA",
                "typeOuter" -> "PointColor"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the color of a point, default color is black",
                "field" -> "point_color",
                "isCopy" -> True,
                "typeInner" -> "RGBA",
                "typeOuter" -> "PointColor"
            |>,
            <|
                "default" -> "1.0",
                "derive" -> "Clone, Debug, PartialEq",
                "details" -> "/// The shape of the point is always round.",
                "docs" -> "Represent the size of a point, default size is 1.0",
                "field" -> "point_size",
                "isCopy" -> True,
                "typeInner" -> "f32",
                "typeOuter" -> "PointSize"
            |>,
            <|
                "default" -> "1.0",
                "derive" -> "Clone, Debug, PartialEq",
                "details" -> "/// The shape of the point is always round.",
                "docs" -> "Represent the size of a point, default size is 1.0",
                "field" -> "point_size",
                "isCopy" -> True,
                "typeInner" -> "f32",
                "typeOuter" -> "PointSize"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the color of a line, default color is black",
                "field" -> "polygon_edge_color",
                "isCopy" -> True,
                "typeInner" -> "RGBA",
                "typeOuter" -> "PolygonEdgeColor"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the color of a line, default color is black",
                "field" -> "polygon_edge_color",
                "isCopy" -> True,
                "typeInner" -> "RGBA",
                "typeOuter" -> "PolygonEdgeColor"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the with of a line, default width is 1.0",
                "field" -> "polygon_edge_width",
                "isCopy" -> True,
                "typeInner" -> "f32",
                "typeOuter" -> "PolygonEdgeWidth"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the with of a line, default width is 1.0",
                "field" -> "polygon_edge_width",
                "isCopy" -> True,
                "typeInner" -> "f32",
                "typeOuter" -> "PolygonEdgeWidth"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the color of a line, default color is black",
                "field" -> "polygon_fill_color",
                "isCopy" -> True,
                "typeInner" -> "RGBA",
                "typeOuter" -> "PolygonFillColor"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the color of a line, default color is black",
                "field" -> "polygon_fill_color",
                "isCopy" -> True,
                "typeInner" -> "RGBA",
                "typeOuter" -> "PolygonFillColor"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the color of a line, default color is black",
                "field" -> "polyline_color",
                "isCopy" -> True,
                "typeInner" -> "RGBA",
                "typeOuter" -> "PolylineColor"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the with of a line, default width is 1.0",
                "field" -> "polyline_width",
                "isCopy" -> True,
                "typeInner" -> "f32",
                "typeOuter" -> "PolylineWidth"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the color of a line, default color is black",
                "field" -> "rectangle_edge_color",
                "isCopy" -> True,
                "typeInner" -> "RGBA",
                "typeOuter" -> "RectangleEdgeColor"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the color of a line, default color is black",
                "field" -> "rectangle_edge_color",
                "isCopy" -> True,
                "typeInner" -> "RGBA",
                "typeOuter" -> "RectangleEdgeColor"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the with of a line, default width is 1.0",
                "field" -> "rectangle_edge_width",
                "isCopy" -> True,
                "typeInner" -> "f32",
                "typeOuter" -> "RectangleEdgeWidth"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the with of a line, default width is 1.0",
                "field" -> "rectangle_edge_width",
                "isCopy" -> True,
                "typeInner" -> "f32",
                "typeOuter" -> "RectangleEdgeWidth"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the color of a line, default color is black",
                "field" -> "rectangle_fill_color",
                "isCopy" -> True,
                "typeInner" -> "RGBA",
                "typeOuter" -> "RectangleFillColor"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the color of a line, default color is black",
                "field" -> "rectangle_fill_color",
                "isCopy" -> True,
                "typeInner" -> "RGBA",
                "typeOuter" -> "RectangleFillColor"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the with of a line, default width is 1.0",
                "field" -> "square_edge_color",
                "isCopy" -> True,
                "typeInner" -> "RGBA",
                "typeOuter" -> "SquareEdgeColor"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the with of a line, default width is 1.0",
                "field" -> "square_edge_width",
                "isCopy" -> True,
                "typeInner" -> "f32",
                "typeOuter" -> "SquareEdgeWidth"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the with of a line, default width is 1.0",
                "field" -> "square_edge_width",
                "isCopy" -> True,
                "typeInner" -> "f32",
                "typeOuter" -> "SquareEdgeWidth"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the with of a line, default width is 1.0",
                "field" -> "square_edge_width",
                "isCopy" -> True,
                "typeInner" -> "f32",
                "typeOuter" -> "SquareEdgeWidth"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the color of a line, default color is black",
                "field" -> "square_fill_color",
                "isCopy" -> True,
                "typeInner" -> "RGBA",
                "typeOuter" -> "SquareFillColor"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the color of a line, default color is black",
                "field" -> "square_fill_color",
                "isCopy" -> True,
                "typeInner" -> "RGBA",
                "typeOuter" -> "SquareFillColor"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the color of a line, default color is black",
                "field" -> "square_fill_color",
                "isCopy" -> True,
                "typeInner" -> "RGBA",
                "typeOuter" -> "SquareFillColor"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the color of a line, default color is black",
                "field" -> "square_fill_color",
                "isCopy" -> True,
                "typeInner" -> "RGBA",
                "typeOuter" -> "SquareFillColor"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the color of a line, default color is black",
                "field" -> "text_color",
                "isCopy" -> True,
                "typeInner" -> "RGBA",
                "typeOuter" -> "TextColor"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the color of a line, default color is black",
                "field" -> "text_font",
                "isCopy" -> True,
                "typeInner" -> "f32",
                "typeOuter" -> "TextFont"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the with of a line, default width is 1.0",
                "field" -> "text_size",
                "isCopy" -> True,
                "typeInner" -> "f32",
                "typeOuter" -> "TextSize"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the color of a line, default color is black",
                "field" -> "triangle_edge_color",
                "isCopy" -> True,
                "typeInner" -> "RGBA",
                "typeOuter" -> "TriangleEdgeColor"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the with of a line, default width is 1.0",
                "field" -> "triangle_edge_width",
                "isCopy" -> True,
                "typeInner" -> "f32",
                "typeOuter" -> "TriangleEdgeWidth"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the with of a line, default width is 1.0",
                "field" -> "triangle_edge_width",
                "isCopy" -> True,
                "typeInner" -> "f32",
                "typeOuter" -> "TriangleEdgeWidth"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the color of a line, default color is black",
                "field" -> "triangle_fill_color",
                "isCopy" -> True,
                "typeInner" -> "RGBA",
                "typeOuter" -> "TriangleFillColor"
            |>,
            <|
                "derive" -> "Clone, Debug, Default, PartialEq",
                "details" -> "",
                "docs" -> "Represent the color of a line, default color is black",
                "field" -> "triangle_fill_color",
                "isCopy" -> True,
                "typeInner" -> "RGBA",
                "typeOuter" -> "TriangleFillColor"
            |>
        }
|>
