# scrap

Scrap records your screen! At least it does if you're on Windows, macOS, or Linux.

## Usage

```toml
[dependencies]
scrap = "0.1"
```

Its API is as simple as it gets!

```rust
struct Display; /// A screen.
struct Frame; /// Just a byte array.
struct Capturer; /// A recording instance.

impl Capturer {
    /// Begin recording.
    pub fn new(display: Display) -> io::Result<Capturer>;
    /// Get a frame!
    pub fn frame<'a>(&'a mut self) -> io::Result<Frame<'a>>;

    pub fn width(&self) -> usize;
    pub fn height(&self) -> usize;
    pub fn format(&self) -> PixelFormat; // Almost always ARGB8888.
}

impl Display {
    /// The main screen.
    pub fn main() -> io::Result<Display>;
    /// All the screens.
    pub fn all() -> io::Result<Vec<Display>>;

    pub fn width(&self) -> usize;
    pub fn height(&self) -> usize;
}
```

## Contributing

Please contribute! Here's some stuff that needs doing:

- Supporting non-mappable Windows devices.
- Examples, and lots of them!
- Avoiding an `Arc<Mutex<Option<T>>>` in the macOS implementation.
- Android support (a stretch.)

For minor things that need doing, `rg` or `grep` for `TODO` in the source directory.
