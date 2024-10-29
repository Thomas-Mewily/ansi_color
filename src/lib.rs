//! Define some Ansi Color

use std::fmt::{Debug, Display};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AnsiColorKind
{
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Grey,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AnsiColorLayer 
{
    Foreground,
    Background,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct AnsiColor
{
    pub color : AnsiColorKind,
    pub layer : AnsiColorLayer,
}

pub type AnsiColorStr = &'static str;

impl AnsiColor
{
    pub const BLACK_FOREGROUND  : AnsiColorStr = "\x1b[30m";
    pub const RED_FOREGROUND    : AnsiColorStr = "\x1b[31m";
    pub const GREEN_FOREGROUND  : AnsiColorStr = "\x1b[32m";
    pub const YELLOW_FOREGROUND : AnsiColorStr = "\x1b[33m";
    pub const BLUE_FOREGROUND   : AnsiColorStr = "\x1b[34m";
    pub const MAGENTA_FOREGROUND: AnsiColorStr = "\x1b[35m";
    pub const CYAN_FOREGROUND   : AnsiColorStr = "\x1b[36m";
    pub const WHITE_FOREGROUND  : AnsiColorStr = "\x1b[37m";
    pub const GREY_FOREGROUND   : AnsiColorStr = "\x1b[90m";
    
    pub const BLACK_BACKGROUND  : AnsiColorStr = "\x1b[40m";
    pub const RED_BACKGROUND    : AnsiColorStr = "\x1b[41m";
    pub const GREEN_BACKGROUND  : AnsiColorStr = "\x1b[42m";
    pub const YELLOW_BACKGROUND : AnsiColorStr = "\x1b[43m";
    pub const BLUE_BACKGROUND   : AnsiColorStr = "\x1b[44m";
    pub const MAGENTA_BACKGROUND: AnsiColorStr = "\x1b[45m";
    pub const CYAN_BACKGROUND   : AnsiColorStr = "\x1b[46m";
    pub const WHITE_BACKGROUND  : AnsiColorStr = "\x1b[47m";
    pub const GREY_BACKGROUND   : AnsiColorStr = "\x1b[100m";
    
    pub const COLOR_BLACK_ON_WHITE : AnsiColorStr   = "\x1b[30m\x1b[47m";
    pub const COLOR_RESET: AnsiColorStr = "\x1b[37m\x1b[40m";

    pub fn new(color : AnsiColorKind, layer : AnsiColorLayer) -> Self { Self { color, layer }}

    pub fn color(&self) -> AnsiColorKind  { self.color }
    pub fn set_color(&mut self, color : AnsiColorKind) -> &mut Self  { self.color = color; self }
    pub fn with_color(mut self, color : AnsiColorKind) -> Self  { self.set_color(color); self }

    pub fn layer(&self) -> AnsiColorLayer  { self.layer }
    pub fn set_layer(&mut self, layer : AnsiColorLayer) -> &mut Self  { self.layer = layer; self }
    pub fn with_layer(mut self, layer : AnsiColorLayer) -> Self  { self.set_layer(layer); self }

    pub fn get_str(&self) -> AnsiColorStr
    {
        match self.layer 
        {
            AnsiColorLayer::Foreground => 
            {
                match self.color 
                {
                    AnsiColorKind::Black   => Self::BLACK_FOREGROUND,
                    AnsiColorKind::Red     => Self::RED_FOREGROUND,
                    AnsiColorKind::Green   => Self::GREEN_FOREGROUND,
                    AnsiColorKind::Yellow  => Self::YELLOW_FOREGROUND,
                    AnsiColorKind::Blue    => Self::BLUE_FOREGROUND,
                    AnsiColorKind::Magenta => Self::MAGENTA_FOREGROUND,
                    AnsiColorKind::Cyan    => Self::CYAN_FOREGROUND,
                    AnsiColorKind::White   => Self::WHITE_FOREGROUND,
                    AnsiColorKind::Grey    => Self::GREY_FOREGROUND,
                }
            }
            AnsiColorLayer::Background => 
            {
                match self.color 
                {
                    AnsiColorKind::Black   => Self::BLACK_BACKGROUND,
                    AnsiColorKind::Red     => Self::RED_BACKGROUND,
                    AnsiColorKind::Green   => Self::GREEN_BACKGROUND,
                    AnsiColorKind::Yellow  => Self::YELLOW_BACKGROUND,
                    AnsiColorKind::Blue    => Self::BLUE_BACKGROUND,
                    AnsiColorKind::Magenta => Self::MAGENTA_BACKGROUND,
                    AnsiColorKind::Cyan    => Self::CYAN_BACKGROUND,
                    AnsiColorKind::White   => Self::WHITE_BACKGROUND,
                    AnsiColorKind::Grey    => Self::GREY_BACKGROUND,
                }
            }
        }
    }
}

impl From<AnsiColor> for AnsiColorStr
{
    fn from(value: AnsiColor) -> Self { value.get_str() }
}

impl Display for AnsiColor
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    { 
        f.write_str(self.get_str()) 
    }
}