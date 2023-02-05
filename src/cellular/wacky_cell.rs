use rand::random;
use crate::cellular::Cell;

#[derive(Default, Clone, Copy)]
pub struct WackyCell{
     saturation: f32,
}

impl From<f32> for WackyCell{
    fn from(r: f32) -> Self {
       Self { saturation: r } 
    }
}

impl Cell for WackyCell  {
    fn update(&self, neightbors: [&Self; 8]) -> Self{
        let (n_avrg, t_avrg) = {
            let n = neightbors.iter().fold(0.0, |init, v| init + v.saturation);
            let t = n + self.saturation;

            (n / 8.0, t / 9.0)
        };

        if self.saturation != n_avrg{
            return n_avrg.into();
        }
        if self.saturation >= 1.0{
            return (t_avrg - n_avrg).into();
        }

        return self.clone();
    }

    fn random() -> Self{
        WackyCell{ saturation: (random::<f32>() % 1.0).abs()}
    }
}

impl crate::MakeBufferCell for WackyCell{
    fn make_buffer_cell(&self) -> tui::buffer::Cell {
        let c = (self.saturation * 255.0) as u8;
        let color = tui::style::Color::Rgb(c, c, c);

        let mut c = tui::buffer::Cell::default();
        c.set_fg(color).set_bg(color);
        c
    }
}
