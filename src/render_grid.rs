use std::{fmt::Display, fs::File, io::Write, path::Path};

pub struct GridRenderer<C> {
    tiles: Vec<(C, C, Option<String>)>,
    rects: Vec<(C, C, C, C, Option<String>)>,
}

impl<C> GridRenderer<C>
where
    C: Display,
{
    pub fn new() -> Self {
        GridRenderer { tiles: Vec::new(), rects: Vec::new() }
    }

    pub fn add_colored_grid_tile(&mut self, y: C, x: C, color: String) {
        self.tiles.push((y, x, Some(color)));
    }

    pub fn add_colored_rect(&mut self, y: C, x: C, h: C, w: C, color: String) {
        self.rects.push((y, x, w, h, Some(color)));
    }

    pub fn add_grid_tile(&mut self, y: C, x: C) {
        self.tiles.push((y, x, None));
    }

    pub fn extend<I>(&mut self, iter: I)
    where
        I: Iterator<Item = (C, C)>,
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
        for (y, x, maybe_color) in &self.tiles {
            writeln!(
                file,
                r#"<rect width="1" height="1" x="{}" y="{}" fill="{}"/>"#,
                x,
                y,
                maybe_color
                    .as_ref()
                    .map(|s| s.to_owned())
                    .unwrap_or_else(|| "black".to_owned())
            )
            .unwrap();
        }
        for (y, x, w, h, maybe_color) in &self.rects {
            writeln!(
                file,
                r#"<rect width="{}" height="{}" x="{}" y="{}" fill="{}"/>"#,
                w,
                h,
                x,
                y,
                maybe_color
                    .as_ref()
                    .map(|s| s.to_owned())
                    .unwrap_or_else(|| "black".to_owned())
            )
            .unwrap();
        }
        file.write_all(b"</svg>").unwrap();
    }
}
