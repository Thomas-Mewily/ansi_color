A minimal package for using the Ansi Color :

Provides optional support for [Serde](https://docs.rs/serde/latest/serde/) (serialization / deserialization) when the "serde" feature is enabled.

```rust
use minimal_ansi_color::*;

println!("{}I'm green{}", AnsiColor::GREEN, AnsiColor::RESET);
println!("{}I'm red{}", AnsiColor::new_foreground(AnsiColorKind::Red), AnsiColor::RESET);
println!("{}White on magenta background{}", AnsiColor::new(AnsiColorKind::Magenta, AnsiColorLayer::Background), AnsiColor::RESET);
```
![image](https://github.com/user-attachments/assets/c1bd8fd3-f10f-4c92-b2cd-e012a4c61b33)

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
