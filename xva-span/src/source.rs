use ariadne::Span;

use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    fmt::Display,
    io::Read,
    ops::Range,
    path::PathBuf,
    sync::{Arc, RwLock},
};

/// Alias for [`chumsky::span::SimpleSpan`]
pub type TokenSpan = chumsky::span::SimpleSpan<usize>;

/// An alternative to [`std::ops::Range`] that can be cheaply copied.
#[derive(Clone, Copy)]
pub struct CheapRange<Idx>(Idx, Idx);

impl<Idx> CheapRange<Idx> {
    pub const fn new(start: Idx, end: Idx) -> Self {
        Self(start, end)
    }
}

impl<Idx> std::fmt::Display for CheapRange<Idx>
where
    Idx: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(start, end) = self;
        write!(f, "{}..{}", start, end)
    }
}

impl<Idx> CheapRange<Idx>
where
    Idx: Copy,
{
    pub fn start(&self) -> Idx {
        let Self(start, _) = self;
        *start
    }

    pub fn end(&self) -> Idx {
        let Self(_, end) = self;
        *end
    }
}

impl<Idx> From<Range<Idx>> for CheapRange<Idx> {
    fn from(value: Range<Idx>) -> Self {
        let Range { start, end } = value;
        Self(start, end)
    }
}

impl<Idx> PartialEq for CheapRange<Idx>
where
    Idx: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl<Idx> Eq for CheapRange<Idx> where Idx: Eq {}

impl<Idx> std::fmt::Debug for CheapRange<Idx>
where
    Idx: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let CheapRange(start, end) = self;
        write!(f, "{start}..{end}")
    }
}

/// A unique identifier for a source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceId(u32);

impl From<u32> for SourceId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for SourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(value) = self;
        write!(f, "{value}")
    }
}

/// Associates a range of source text with the source file that it came from.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SourceSpan {
    src: SourceId,
    range: CheapRange<usize>,
}

impl std::fmt::Debug for SourceSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} in {}", self.range, self.src)
    }
}

impl SourceSpan {
    pub const fn new(src: SourceId, range: CheapRange<usize>) -> Self {
        Self { src, range }
    }

    pub fn src(&self) -> SourceId {
        self.src
    }

    pub fn range(&self) -> CheapRange<usize> {
        self.range
    }

    /// Copies this span but replaces the end value with the specified value.
    pub fn copy_from_ending_at(&self, end: usize) -> Self {
        Self {
            src: self.src,
            range: {
                let start = self.start();
                CheapRange::new(start, end)
            },
        }
    }

    pub fn copy_from_starting_at(&self, start: usize) -> Self {
        Self {
            src: self.src,
            range: {
                let end = self.end();
                CheapRange::new(start, end)
            },
        }
    }

    pub fn from_start_end(start: Self, end: Self) -> Self {
        let (src, s) = (start.src(), start.start());
        let e = end.end();
        Self {
            src,
            range: CheapRange(s, e),
        }
    }
}

impl ariadne::Span for SourceSpan {
    type SourceId = SourceId;

    fn source(&self) -> &Self::SourceId {
        &self.src
    }

    fn start(&self) -> usize {
        let CheapRange(start, _) = self.range;
        start
    }

    fn end(&self) -> usize {
        let CheapRange(_, end) = self.range;
        end
    }
}

impl chumsky::span::Span for SourceSpan {
    type Context = SourceId;
    type Offset = usize;

    fn new(context: Self::Context, range: Range<Self::Offset>) -> Self {
        Self {
            src: context,
            range: {
                let Range { start, end } = range;
                CheapRange(start, end)
            },
        }
    }

    fn context(&self) -> Self::Context {
        self.src.clone()
    }

    fn start(&self) -> Self::Offset {
        let CheapRange(start, _) = self.range;
        start
    }

    fn end(&self) -> Self::Offset {
        let CheapRange(_, end) = self.range;
        end
    }
}

#[derive(Debug)]
pub enum LineEnding {
    /// LF, or `\n`
    Unix,

    /// CRLF, or `\r\n`
    Windows,

    /// The file only has one line
    OneLine,
}

#[derive(Debug)]
pub enum SourceFilePath {
    /// A real file that exists on disk. The path must be absolute and canonicalized.
    Real(PathBuf),

    /// A virtual file, such as stdin, the REPL input, etc.
    Virtual(String),
}

/// A container for a source file being compiled. The actual source text is immutable.
///
/// The implementation of [`Debug`](std::fmt::Debug) is hand-written to be able to display the hash as a hex string, rather
/// than an array of bytes.

pub struct SourceFile {
    /// The path of the source file. This may be an anonymous "fake" path, e.g. `<repl>` for code
    /// entered in the REPL.
    pub path: SourceFilePath,

    /// The name of the source file. This may be an anonymous "fake" path, e.g. `<repl>` for code
    /// entered in the REPL.
    ///
    /// If the file is a real file, this name should be the path to this file, relative
    /// to either:
    /// - The file on which the compiler was invoked on, or
    /// - The working directory where the compiler was invoked.
    pub name: String,

    /// The SHA2 of the file's content.
    pub hash: [u8; 32],

    /// The original content of the file.
    // pub src: String,
    pub src: Arc<str>,

    /// Cached byte ranges of each line
    _line_ranges: HashMap<Range<usize>, usize>,

    /// The line ending sequence of the file
    _eol: LineEnding,
}

impl SourceFile {
    pub fn new(name: String, path: SourceFilePath, src: String) -> Self {
        let line_ending = get_line_ending(src.as_bytes());
        let mut current_line = 0usize;
        let mut current_byte = 0usize;
        let mut line_start_byte = 0usize;

        let line_ranges = match line_ending {
            LineEnding::Unix => {
                let mut temp = HashMap::new();

                for b in src.as_bytes() {
                    if *b == '\n' as u8 {
                        temp.insert(line_start_byte..current_byte, current_line);

                        line_start_byte = current_byte + 1;
                        current_line += 1;
                    }

                    current_byte += 1;
                }

                temp
            }
            LineEnding::Windows => todo!(),
            LineEnding::OneLine => HashMap::from([(0..src.as_bytes().len(), 0)]),
        };

        Self {
            path,
            name,
            hash: {
                let mut hasher: Sha256 = Sha256::new();
                hasher.update(src.as_bytes());
                hasher
                    .finalize()
                    .as_slice()
                    .try_into()
                    .expect("Wrong length")
            },
            src: Arc::from(src.as_str()),
            _line_ranges: line_ranges,
            _eol: line_ending,
        }
    }

    fn hash_as_hex(&self) -> String {
        self.hash
            .iter()
            .map(|h| format!("{h:02x}"))
            .collect::<String>()
    }

    pub fn content(&self) -> Arc<str> {
        self.src.clone()
    }

    pub fn bytes(&self) -> &[u8] {
        self.src.as_ref().as_bytes()
    }
}

impl std::fmt::Debug for SourceFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SourceFile")
            .field("path", &self.path)
            .field("name", &self.name)
            .field("hash", &self.hash_as_hex())
            .field("src", &self.src)
            .finish()
    }
}

fn get_line_ending(src: &[u8]) -> LineEnding {
    match src.iter().enumerate().find(|(_, b)| **b == '\n' as u8) {
        Some((index, _)) => {
            if index == 0 {
                LineEnding::Unix
            } else {
                if src[index - 1] == '\r' as u8 {
                    LineEnding::Windows
                } else {
                    LineEnding::Unix
                }
            }
        }
        None => LineEnding::OneLine,
    }
}

// This is an expensive struct bro.
#[derive(Debug)]
pub struct SourceMap {
    // /// The raw sources - for getting the entire content of the file
    raws: RwLock<HashMap<SourceId, Arc<str>>>,

    /// The cache map for Ariadne - for getting file content wrapped in an [`ariadne::Source`]
    ariadne_map: HashMap<SourceId, ariadne::Source<Arc<str>>>,

    /// File names
    names: HashMap<SourceId, String>,

    /// File paths
    paths: HashMap<SourceId, Option<PathBuf>>,
}

impl SourceMap {
    const FIRST_SRC_ID: SourceId = SourceId(0);

    /// Loads a real file into the source map.
    pub fn load(&mut self, path: PathBuf) -> std::io::Result<SourceId> {
        let name = path.clone().into_os_string().into_string().unwrap();

        let mut buf = String::new();
        let mut file = std::fs::OpenOptions::new().read(true).open(&path)?;
        file.read_to_string(&mut buf)?;

        Ok(self.new_file(name, Some(path.canonicalize()?), buf))
    }

    /// Loads a virtual file into the source map. A virtual file is just a named string of text.
    pub fn load_virtual(&mut self, name: String, src: String) -> SourceId {
        self.new_file(name, None, src)
    }

    fn new_file(&mut self, name: String, path: Option<PathBuf>, src: String) -> SourceId {
        // Generating the next source ID needs to happen before gaining write access to the map, because
        // generating source IDs itself needs read access to the map. It'll deadlock if we do it after gaining
        // write access.

        let src_id = self.next_id();

        // For the rest of lifetime of this function, writing to both the raw files and the ariadne map is safe,
        // because the writability into the lock is dropped at the end of this function.
        let mut files = match self.raws.write() {
            Ok(l) => l,
            Err(e) => panic!("Source map lock is poisoned: {e}"),
        };

        let arc: Arc<str> = Arc::from(src.as_str());

        // Insert an Arc to the file content into the raw map
        files.insert(src_id.clone(), arc.clone());

        // Then fill out the names, paths, and ariadne map.
        self.names.insert(src_id, name);
        self.paths.insert(src_id, path);
        self.ariadne_map
            .insert(src_id, ariadne::Source::from(arc.clone()));

        src_id
    }

    /// Retrieves the next ID for a source file. If the map is being written to, the thread will wait until
    /// it can get read access.
    fn next_id(&self) -> SourceId {
        let map = match self.raws.read() {
            Ok(m) => m,
            Err(e) => panic!("Source map lock is poisoned: {e}"),
        };

        map.iter()
            .last()
            .map_or_else(|| Self::FIRST_SRC_ID, |(k, _)| *k)
    }

    /// Locates a file in the source map and returns an `Arc` to it, if it has previously been loaded.
    pub fn get_raw(&self, id: &SourceId) -> Option<Arc<str>> {
        match self.raws.read() {
            Ok(m) => m.get(id).map(|x| x.clone()),
            Err(e) => panic!("Source map lock is poisoned: {e}"),
        }
    }

    pub fn get_name(&self, id: &SourceId) -> Option<&str> {
        match self.raws.read() {
            Ok(_) => self.names.get(id).map(|name| name.as_str()),
            Err(e) => panic!("Source map lock is poisoned: {e}"),
        }
    }
}

impl Default for SourceMap {
    fn default() -> Self {
        Self {
            raws: Default::default(),
            ariadne_map: Default::default(),
            names: Default::default(),
            paths: Default::default(),
        }
    }
}

impl ariadne::Cache<SourceId> for &SourceMap {
    type Storage = Arc<str>;

    fn fetch(
        &mut self,
        id: &SourceId,
    ) -> Result<&ariadne::Source<Self::Storage>, Box<dyn std::fmt::Debug + '_>> {
        match self.raws.read() {
            Ok(_) => Ok(match self.ariadne_map.get(id) {
                Some(ariadne_source) => ariadne_source,
                None => panic!("This is real bad: the source for the following SourceId was not found in the map.\n{id:?}")
            }),
            Err(e) => panic!("Source map lock is poisoned: {e}"),
        }
    }

    fn display<'a>(&self, id: &'a SourceId) -> Option<Box<dyn std::fmt::Display + 'a>> {
        // Expensive
        Some(Box::new(format!("{}", self.get_name(id).unwrap())))
    }
}

#[cfg(test)]
mod tests {}
