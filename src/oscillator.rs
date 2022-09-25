use csv::Writer;
use plotters::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::{prelude, Write};

const WAVE_TABLE_SIZE: usize = 64;
const PI: f32 = std::f32::consts::PI;

pub struct Oscillator {
    sample_rate: f32,
    wave_table: Vec<f32>,
}

impl Oscillator {
    pub fn new(sample_rate: f32) -> Oscillator {
        Oscillator {
            sample_rate,
            wave_table: Vec::with_capacity(WAVE_TABLE_SIZE),
        }
    }

    pub fn generate_table(&mut self) {
        for i in 0..WAVE_TABLE_SIZE {
            self.wave_table
                .push((2.0 * PI * i as f32 / WAVE_TABLE_SIZE as f32).sin());
        }
    }

    pub fn show_table(&mut self, samples: Vec<f32>) {
        let table = samples;
        let mut file = File::create("wave.csv").unwrap();

        file.write_all("index;index;value\n".as_bytes()).unwrap();

        table.into_iter().enumerate().for_each(|(i, j)| {
            let sample_str = format!("{};{};{}\n", i, i, j);
            file.write_all(sample_str.as_bytes()).unwrap();
        });

        
    }

    pub fn generate_samples(&mut self, frequency: f32, time: f32) -> Vec<f32> {
        let sample_amount = self.sample_rate * time;
        let mut output: Vec<f32> = Vec::with_capacity(sample_amount as usize);

        let mut index: f32 = 0.0;
        let index_increment = frequency * WAVE_TABLE_SIZE as f32 / self.sample_rate;

        for _n in 0..sample_amount as i32 {
            output.push(self.wave_table[index.floor() as usize]);
            index += index_increment;
            index %= WAVE_TABLE_SIZE as f32;
        }

        output
    }

    pub fn chart_samples(&mut self, samples: Vec<f32>) -> Result<(), Box<dyn std::error::Error>> {
        let root = SVGBackend::new("0.svg", (640*2, 480)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .caption("Wave", ("sans-serif", 50).into_font())
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)?;

        chart.configure_mesh().draw()?;

        chart
            .draw_series(LineSeries::new(
                samples.into_iter().enumerate().map(|(x, y)| (x as f32, x as f32 *0.3)),
                &BLUE,
            ).point_size(5))?;

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        root.present()?;

        Ok(())
    }
}
