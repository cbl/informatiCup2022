use crate::tabu::TabuSearch;
use crate::types::TimeDiff;
use plotters::prelude::*;
use std::path::Path;

pub struct Plotter {
    pub path: &'static str,
}

impl Plotter {
    /// Gets the file name by the given base_name.
    fn file_name(&self, base_name: &str) -> String {
        let mut file_name = format!("{}/{}.png", self.path, base_name);

        for n in 0.. {
            if !Path::new(&file_name).is_file() {
                break;
            }

            file_name = format!("{}/{}_{}.png", self.path, base_name, n);
        }

        file_name
    }

    pub fn plot_fitness(&self, tabu: &TabuSearch) -> Result<(), Box<dyn std::error::Error>> {
        let file_name = self.file_name("fitness");
        let root = BitMapBackend::new(&file_name, (1024, 800)).into_drawing_area();

        root.fill(&WHITE)?;

        println!("{}", tabu.fitness.len());

        let min = tabu
            .fitness
            .iter()
            .fold(TimeDiff::MAX, |a, &b| a.min(b))
            .min(-0);

        let max = tabu.fitness.iter().fold(0, |a, &b| match a > b {
            true => a,
            false => b,
        });

        let mut chart = ChartBuilder::on(&root)
            .set_label_area_size(LabelAreaPosition::Left, 50)
            .set_label_area_size(LabelAreaPosition::Bottom, 50)
            .caption("Fitness By Iteration", ("sans-serif", 30))
            .build_cartesian_2d(-10..(tabu.fitness.len() as i32), (min - 2)..(max + 2))
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series(tabu.fitness.iter().enumerate().map(|(x, y)| {
                Circle::new((x as i32, *y), 1, Into::<ShapeStyle>::into(&BLACK).filled())
            }))
            .unwrap();

        // To avoid the IO failure being ignored silently, we manually call the present function
        root.present().expect(
            "Unable to write result to file, please make sure 'plots' dir exists under current dir",
        );
        // println!("Result has been saved to {}", file_name);

        Ok(())
    }
}
