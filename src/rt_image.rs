pub struct RTImage {
    pub data: Vec<u8>,
    width: u32,
    height: u32,
    bytes_per_scanline: u32,
}

impl RTImage {
    const BYTES_PER_PIXEL: u32 = 3;

    pub fn new(path: &std::path::Path) -> Self {
        let img = image::open(&path).expect("image not found");

        let img = img.to_rgb8();
        let width = img.width();
        let height = img.height();
        let bytes_per_scanline = width * Self::BYTES_PER_PIXEL;

        Self {
            data: img.into_raw(),
            width,
            height,
            bytes_per_scanline,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
