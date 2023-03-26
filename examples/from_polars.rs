use polars::prelude::*;
use std::path::Path;
use vega_lite_5::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // input data: a CSV reader
    let df = CsvReader::from_path("iris.csv")?
        .has_header(true)
        .finish()?;
    let df = df.lazy().filter(col("symbol").eq("GOOG")).collect()?;
    // the chart
    let chart = VegaliteBuilder::default()
        .title("Stock price")
        .description("Google's stock price over time.")
        .data(df)
        .mark(Mark::Line)
        .encoding(
            EdEncodingBuilder::default()
                .x(XClassBuilder::default()
                    .field("date")
                    .position_def_type(Type::Temporal)
                    .axis(AxisBuilder::default().title("date").build()?)
                    .build()?)
                .y(YClassBuilder::default()
                    .field("price")
                    .position_def_type(Type::Quantitative)
                    .axis(AxisBuilder::default().title("price").build()?)
                    .build()?)
                .build()?,
        )
        .build()?;

    // display the chart using `showata`
    chart.show()?;

    // print the vega lite spec
    eprint!("{}", chart.to_string()?);

    Ok(())
}
