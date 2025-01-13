pub mod gemini;
pub mod titan;

pub use gemini::Gemini;
pub use titan::Titan;

pub enum Request {
    Gemini(Gemini),
    Titan(Titan),
}
