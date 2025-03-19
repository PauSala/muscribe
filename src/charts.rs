use plotters::{
    chart::ChartBuilder,
    prelude::{BitMapBackend, IntoDrawingArea, Rectangle},
    style::{BLUE, Color, WHITE},
};

pub fn plot(data: &[f32], name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut path = String::from("charts/");
    path.push_str(name);
    path.push_str(".png");

    let root = BitMapBackend::new(&path, (1200, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_index = data.len() as f32;
    let max = data
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    let min = data
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption(name, ("sans-serif", 40))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(0f32..max_index, *min..*max)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(data.iter().enumerate().map(|(i, &value)| {
        let x = i as f32;
        let y = value;
        let bar = Rectangle::new([(x, 0.0), (x + 0.8, y)], BLUE.filled());
        bar
    }))?;
    Ok(())
}

pub fn print_chart(data: &[f32]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("chart.png", (1200, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_index = data.len() as f32;

    let mut chart = ChartBuilder::on(&root)
        .caption("Normalized Data Chart", ("sans-serif", 40))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(0f32..max_index, 0f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(data.iter().enumerate().map(|(i, &value)| {
        let x = i as f32;
        let bar = Rectangle::new([(x, 0.0), (x + 0.8, value)], BLUE.filled());
        bar
    }))?;
    Ok(())
}

pub fn print_frequencies(data: &[(f32, bool)]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("charts/frequencies.png", (1200, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_index = data.len() as f32;
    let max = data
        .iter()
        .filter(|(_, b)| b == &true)
        .map(|(f, _)| f)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Frequencies", ("sans-serif", 40))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(0f32..max_index, 0f32..*max)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(data.iter().enumerate().map(|(i, &value)| {
        let x = i as f32;
        let y = if value.1 { value.0 } else { 0.0 };
        let bar = Rectangle::new([(x, 0.0), (x + 0.8, y)], BLUE.filled());
        bar
    }))?;
    Ok(())
}
