use std::path::PathBuf;

pub struct Sprite {}

pub enum SpriteError {
    OtherError(String),
}

impl Sprite {
    pub fn new() -> Self {
        Sprite {}
    }

    pub fn WithSpriteFile(&mut self, buf: PathBuf) -> &mut Sprite {
        println!("Loading {buf:?}");

        // s.spriteTexture, s.w, s.h, err = graphics.GetSprite(filename)
        // if err != nil {
        // 	return errors.Wrapf(err, "SpriteRenderer texture load '%s' Error", filename)
        // }

        // glerr := gl.GetError()
        // if glerr != gl.NO_ERROR {
        // 	fmt.Printf("SpriteRenderer '%s' GL Error: %d", filename, glerr)
        // }
        // return nil
        self
    }

    pub fn Setup(&mut self) -> Result<(), SpriteError> {
        Ok(())
    }
}
