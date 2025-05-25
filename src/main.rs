use rand::Rng;
use plotters::prelude::*;
use std::{io, fs, path::Path};
use tiny_http::{Server, Response, StatusCode, Header};

const OUT_FILE_NAME: &str = "histogram.png";

fn calc_normalised(min:i32, max:i32, num:i32) -> f64{
    return (num as f64 - min as f64)/(max as f64-min as f64);
}

fn main() {
    // ----- SERVER ----- //
    let server = Server::http("127.0.0.1:5001").unwrap();
    println!("Server running at http://localhost:5001");

    for request in server.incoming_requests() {
        let url = request.url();
        let path = if url == "/" {
            "static/index.html".to_string()
        } else {
            format!("static{}", url)
        };

        let file_path = Path::new(&path);

        if file_path.exists() && file_path.is_file() {
            let content = fs::read(file_path).unwrap_or_else(|_| b"Error reading file".to_vec());
            let content_type = match file_path.extension().and_then(|e| e.to_str()) {
                Some("html") => "text/html",
                Some("css") => "text/css",
                Some("js") => "application/javascript",
                Some("png") => "image/png",
                Some("jpg") | Some("jpeg") => "image/jpeg",
                Some("svg") => "image/svg+xml",
                _ => "application/octet-stream",
            };
            let response = Response::from_data(content)
                .with_header(Header::from_bytes(&b"Content-Type"[..], content_type).unwrap());
            request.respond(response).unwrap();
        } else {
            let response = Response::from_string("404 Not Found")
                .with_status_code(StatusCode(404));
            request.respond(response).unwrap();
        }
    }

    // ----- MAIN ----- //
    let mut rng = rand::rng();
    let mut count = 1;
    let mut norm_data: Vec<u32> = vec![];

    loop {    
        let mut min_str = String::new();
        let mut max_str = String::new();

        println!("");
        println!("-------- Iteration #{} --------", count);
        println!("Input minimum value:");
        io::stdin()
            .read_line(&mut min_str)
            .expect("Failed to read line");

        println!("Input max value:");
        io::stdin()
            .read_line(&mut max_str)
            .expect("Failed to read line");

        let min: i32 = min_str.trim().parse().expect("Please type a number!");
        let max: i32 = max_str.trim().parse().expect("Please type a number!");

        if min == max{
            break;
        }

        let num:i32 = rng.random_range(min..max);
        println!("random num: {}", num);

        let norm = calc_normalised(min, max, num);
        println!("normalised: {}", norm);
        let data_point:u32 = ((norm * 10.0).round()) as u32;
        norm_data.push(data_point);
        count += 1;
    }

    // ------ GRAPH ----- //
    let root = BitMapBackend::new(OUT_FILE_NAME, (640, 480)).into_drawing_area();

    root.fill(&WHITE).expect("root fill error");
    
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Density of Normalised Values", ("sans-serif", 50.0))
        .build_cartesian_2d((0u32..10u32).into_segmented(), 0u32..10u32).expect("chart assign error");

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("Bucket (should be scale of 0 to 1 but i'm working on it...)")
        .axis_desc_style(("sans-serif", 15))
        .draw().expect("chart config error");

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(norm_data.iter().map(|x: &u32| (*x, 1))),
    ).expect("chart draw error");

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

}
