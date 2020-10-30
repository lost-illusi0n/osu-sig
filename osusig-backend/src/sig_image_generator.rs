use crate::sig_models::SigUserData;
use image::{RgbImage, Rgb, Pixel};
use imageproc::drawing::draw_text_mut;
use rusttype::{Scale, Font};
use colors_transform::Color;
use image::imageops::FilterType;

pub struct SigImageGenerator<'a> {
    pub color: String,
    pub data: &'a SigUserData
}

impl SigImageGenerator<'_> {
    const MAGIC: u32 = 6;
    const WHITE: Rgb<u8> = Rgb([255, 255, 255]);
    const GRAY: Rgb<u8> = Rgb([85, 85, 85]);
    const TEXT_X: u32 = SigImageGenerator::MAGIC*2+82;
    const TEXT_RIGHT_X: u32 = 338-SigImageGenerator::MAGIC*2;
    const BOLD: Scale = Scale { x: 20.0, y: 20.0 };
    const SUB: Scale = Scale { x: 18.0, y: 18.0 };

    pub async fn generate(&self) -> Option<RgbImage> {
        let mut buffer: RgbImage = RgbImage::new(338, 94);
        let parsed_color_string = if self.color.starts_with("#") { &self.color[1..] } else { &self.color };

        let color: (u8, u8, u8) = match colors_transform::Rgb::from_hex_str(parsed_color_string) {
            Ok(color) => (color.get_red() as u8, color.get_green() as u8, color.get_blue() as u8),
            Err(_e) => return None
        };

        for x in 0..338 {
            for y in 0..94 {
                if x >= 88 && x <= 332 && y >= 34 && y <= 87 {
                    *buffer.get_pixel_mut(x, y) = *&SigImageGenerator::WHITE;
                    continue
                }
                *buffer.get_pixel_mut(x, y) = Rgb::<u8>::from([color.0, color.1, color.2])
            }
        }

        let font: Font<'static> = Font::try_from_vec(Vec::from(include_bytes!("fonts/Aller_Bd.ttf") as &[u8])).unwrap();
        let pfp = SigImageGenerator::download_image(&self.data.avatar_url).await;
        let pfp: RgbImage = image::imageops::resize(&pfp, 82, 82, FilterType::Nearest);
        let flag = SigImageGenerator::download_image(&self.data.country_url).await;
        let flag: RgbImage = image::imageops::resize(&flag, 26, 16, FilterType::Nearest);
        let flax_x = 338-(SigImageGenerator::get_width_of_text(&font, SigImageGenerator::BOLD, &format!("#{}", &self.data.ranking)) + flag.width() + SigImageGenerator::MAGIC+SigImageGenerator::MAGIC/2);

        for x in 0..flag.width() {
            for y in 0..flag.height() {
                *buffer.get_pixel_mut(flax_x + x, SigImageGenerator::MAGIC + 6 + y) = flag.get_pixel(x, y).to_rgb();
            }
        }

        for x in 0..pfp.width() {
            for y in 0..pfp.height() {
                *buffer.get_pixel_mut(SigImageGenerator::MAGIC + x, SigImageGenerator::MAGIC + y) = *pfp.get_pixel(x, y);
            }
        }

        self.draw_accuracy(&font, &mut buffer);
        self.draw_play_count(&font, &mut buffer);
        self.draw_ranking(&font, &mut buffer);

        let mut scale = -1 * (self.data.name.len() as i16 - 20) + 13;
        if scale > 25 { scale = 25 }

        draw_text_mut(
            &mut buffer,
            SigImageGenerator::WHITE,
            SigImageGenerator::TEXT_X, SigImageGenerator::MAGIC+3,
            Scale { x: scale as f32, y: scale as f32 },
            &font,
            self.data.name.as_str()
        );

        return Some(buffer)
    }

    async fn download_image(url: &String) -> RgbImage {
        let resp = reqwest::get(url).await.unwrap();
        let bytes = resp.bytes().await.unwrap();
        let dynimg = image::load_from_memory(&bytes).unwrap();
        let img = dynimg.to_rgb();
        img.clone()
    }

    fn get_width_of_text(font: &Font, scale: Scale, text: &String) -> u32 {
        let glyphs = text.chars().map(|char| font.glyph(char));
        let mut glyphs = glyphs.map(|glyph| glyph.scaled(scale).h_metrics().advance_width);
        let first = glyphs.next().unwrap();
        return glyphs.fold(first, |a, b| a + b) as u32;
    }

    fn draw_ranking(&self, font: &Font, buffer: &mut RgbImage) {
        let ranking = format!("#{}", self.data.ranking);

        draw_text_mut(
            buffer,
            SigImageGenerator::WHITE,
            SigImageGenerator::TEXT_RIGHT_X-SigImageGenerator::get_width_of_text(font, SigImageGenerator::BOLD, &ranking)+7, SigImageGenerator::MAGIC+5,
            SigImageGenerator::SUB,
            font,
            ranking.as_str()
        );
    }

    fn draw_accuracy(&self, font: &Font, buffer: &mut RgbImage) {
        draw_text_mut(buffer, SigImageGenerator::GRAY, SigImageGenerator::TEXT_X, SigImageGenerator::MAGIC+35, SigImageGenerator::SUB, font, "accuracy");

        let accuracy = format!("{:.2}%", self.data.accuracy);

        draw_text_mut(
            buffer,
            SigImageGenerator::GRAY,
            SigImageGenerator::TEXT_RIGHT_X-SigImageGenerator::get_width_of_text(font, SigImageGenerator::BOLD, &accuracy), SigImageGenerator::MAGIC+33,
            SigImageGenerator::BOLD,
            font,
            accuracy.as_str()
        );
    }

    fn draw_play_count(&self, font: &Font, buffer: &mut RgbImage) {
        draw_text_mut(buffer, SigImageGenerator::GRAY, SigImageGenerator::TEXT_X, SigImageGenerator::MAGIC + 57, SigImageGenerator::SUB, font, "playcount");

        let playcount = format!("{}(lvl {})", self.data.play_count, self.data.level);

        draw_text_mut(
            buffer,
            SigImageGenerator::GRAY,
            SigImageGenerator::TEXT_RIGHT_X-SigImageGenerator::get_width_of_text(font, SigImageGenerator::BOLD, &playcount), SigImageGenerator::MAGIC+55,
            SigImageGenerator::BOLD,
            font,
            playcount.as_str()
        );
    }
}