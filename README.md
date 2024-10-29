A minimal package for printing some Ansi Color

```rust
use ansi_color::*;

println!("{}I'm green{}", AnsiColor::GREEN_FOREGROUND, AnsiColor::RESET);
println!("{}I'm red{}", AnsiColor::new_foreground(AnsiColorKind::Red), AnsiColor::RESET);
println!("{}White on magenta background{}", AnsiColor::new(AnsiColorKind::Magenta, AnsiColorLayer::Background), AnsiColor::RESET);
```

```rust
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

pub enum AnsiColorLayer 
{
    Foreground,
    Background,
}

pub struct AnsiColor
{
    pub color : AnsiColorKind,
    pub layer : AnsiColorLayer,
}
```