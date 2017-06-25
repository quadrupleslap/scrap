# scrap

Scrap records your screen! At least it does if you're on Windows, macOS, or Linux.

## Usage

```toml
[dependencies]
scrap = "0.1.2"
```

Its API is as simple as it gets!

```rust
struct Display; /// A screen.
struct Frame; /// Just a byte array.
struct Capturer; /// A recording instance.

impl Capturer {
    /// Begin recording.
    pub fn new(display: Display) -> io::Result<Capturer>;
    /// Try to get a frame!
    /// Returns WouldBlock if it would block.
    pub fn frame<'a>(&'a mut self) -> io::Result<Frame<'a>>;

    pub fn width(&self) -> usize;
    pub fn height(&self) -> usize;
    pub fn format(&self) -> PixelFormat; // Almost always ARGB8888.
}

impl Display {
    /// The primary screen.
    pub fn primary() -> io::Result<Display>;
    /// All the screens.
    pub fn all() -> io::Result<Vec<Display>>;

    pub fn width(&self) -> usize;
    pub fn height(&self) -> usize;
}
```

## Contributing

Please contribute! See the [issue tracker](https://github.com/quadrupleslap/scrap/issues) for stuff that needs doing.
