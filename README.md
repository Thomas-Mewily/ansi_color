Define some Ansi Color

```rust
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
```