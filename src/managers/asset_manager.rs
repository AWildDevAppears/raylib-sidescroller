/**
* Copyright (c) AWildDevAppears
*/
use raylib::{
    RaylibHandle, RaylibThread,
    ffi::{Rectangle, Vector2},
    prelude::Font,
    text::RaylibFont,
    texture::{RaylibTexture2D, Texture2D},
};
use std::{collections::HashMap, path::PathBuf};

pub struct AssetManager {
    fonts: HashMap<String, Font>,
    textures: HashMap<String, Texture2D>,
    spritesheets: HashMap<String, SpriteSheet>,
}

impl AssetManager {
    pub fn new(
        game: &mut RaylibHandle,
        thread: &RaylibThread,
        font_refs: &[FontReference],
        texture_refs: &[TextureReference],
    ) -> Self {
        let mut fonts: HashMap<String, Font> = HashMap::with_capacity(font_refs.len());

        for font in font_refs {
            let file_data = std::fs::read(&font.path).expect("Failed to read font file.");
            let format = font.format.extension();

            let font_data = game
                .load_font_from_memory(&thread, format, &file_data, 16, None)
                .expect("Failed to load default font data");

            font_data
                .texture()
                .set_texture_filter(&thread, raylib::ffi::TextureFilter::TEXTURE_FILTER_BILINEAR);

            fonts.insert(font.name.clone(), font_data);
        }

        let mut textures: HashMap<String, Texture2D> = HashMap::with_capacity(
            texture_refs
                .iter()
                .filter(|item| !item.is_sprite_sheet)
                .count(),
        );
        let mut spritesheets: HashMap<String, SpriteSheet> = HashMap::with_capacity(
            texture_refs
                .iter()
                .filter(|item| item.is_sprite_sheet)
                .count(),
        );

        for texture in texture_refs {
            if texture.is_sprite_sheet {
                spritesheets.insert(texture.name.clone(), SpriteSheet::new(texture.size));
            }

            let asset = game
                .load_texture(thread, &texture.name)
                .expect("Failed to read image file");

            textures.insert(texture.name.clone(), asset);
        }

        Self {
            fonts,
            textures,
            spritesheets,
        }
    }

    pub fn get_font(&self, name: &str) -> &Font {
        self.fonts.get(&name.to_string()).expect("Cannot find font")
    }

    pub fn get_texture(&self, name: &String) -> &Texture2D {
        self.textures.get(name).expect("Cannot find font")
    }

    pub fn get_spritesheet(&self, name: &str) -> &SpriteSheet {
        self.spritesheets
            .get(name)
            .expect("Cannot find spritesheet")
    }

    pub fn get_sprite_at(&self, name: &str, grid_x: i32, grid_y: i32) -> Rectangle {
        let sheet = self.get_spritesheet(name);
        sheet.get_tile_rect(grid_x, grid_y)
    }
}

pub struct FontReference {
    pub name: String,
    pub path: PathBuf,
    pub format: FontFormat,
}

pub enum FontFormat {
    TTF,
}

impl FontFormat {
    pub fn extension(&self) -> &str {
        match self {
            FontFormat::TTF => ".ttf",
        }
    }
}

pub struct TextureReference {
    pub name: String,
    pub is_sprite_sheet: bool,
    pub size: Vector2,
}

impl TextureReference {
    pub fn new_image(name: String) -> Self {
        Self {
            name,
            is_sprite_sheet: false,
            size: Vector2::zero(),
        }
    }

    pub fn new_sheet(name: String, size: Vector2) -> Self {
        Self {
            name,
            is_sprite_sheet: true,
            size,
        }
    }
}

pub struct SpriteSheet {
    pub tile_size: Vector2,
}

impl SpriteSheet {
    pub fn new(tile_size: Vector2) -> Self {
        Self { tile_size }
    }

    pub fn get_tile_rect(&self, grid_x: i32, grid_y: i32) -> Rectangle {
        Rectangle::new(
            grid_x as f32 * self.tile_size.x,
            grid_y as f32 * self.tile_size.y,
            self.tile_size.x,
            self.tile_size.y,
        )
    }
}
