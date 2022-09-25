
mod oscillator;

use oscillator::Oscillator;

fn main() {
    let frequency = 440.0;

    let mut oscillator = Oscillator::new(44100.0);

    oscillator.generate_table();

    let sample_table = oscillator.generate_samples(frequency, 3.0);
    // oscillator.show_table(sample_table);

    oscillator.chart_samples(sample_table).unwrap();


}
