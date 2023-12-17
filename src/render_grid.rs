use std::{fs::File, io::Write, path::Path};

pub struct GridRenderer {
    tiles: Vec<(usize, usize)>,
}

impl GridRenderer {
    pub fn new() -> Self {
        GridRenderer { tiles: Vec::new() }
    }

    pub fn add_grid_tile(&mut self, y: usize, x: usize) {
        self.tiles.push((y, x));
    }

    pub fn extend<I>(&mut self, iter: I)
    where
        I: Iterator<Item = (usize, usize)>,
    {
        iter.for_each(|(y, x)| self.add_grid_tile(y, x));
    }

    pub fn store_svg<P>(&self, path: P)
    where
        P: AsRef<Path>,
    {
        let mut file = File::create(path).unwrap();
        file.write_all(br#"<svg xmlns="http://www.w3.org/2000/svg">"#)
            .unwrap();
        for (y, x) in &self.tiles {
            writeln!(file, r#"<rect width="1" height="1" x="{}" y="{}"/>"#, x, y).unwrap();
        }
        file.write_all(b"</svg>")
            .unwrap();
    }
}
