use anyhow::Result;
use clap::Parser;
use ini::Ini;
use quiet_panics::set_panic_hook;
use std::path::{Path, PathBuf};
use strum::{EnumIter, IntoEnumIterator};

macro_rules! return_if_ok {
    ($result:expr) => {{
        if let Ok(x) = $result {
            return x;
        }
    }};
}

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(short)]
    target: PathBuf,

    #[arg(required = true)]
    patches: Vec<PathBuf>,
}

struct File {
    path: PathBuf,
    data: Data,
}

enum Data {
    Ini(Ini),
}

#[derive(PartialEq, EnumIter)]
enum Format {
    Ini,
}

impl File {
    fn load(path: &Path) -> Self {
        for format in Format::iter() {
            let result = Self::load_as(path, &format);
            return_if_ok!(result);
        }

        panic!("{} format is unsupported", path.display());
    }

    fn load_as(path: &Path, format: &Format) -> Result<Self> {
        let data = match format {
            Format::Ini => Data::Ini(Ini::load_from_file(path)?),
        };

        let path = path.to_path_buf();
        Ok(Self { path, data })
    }

    fn compatible(&self, x: &Self) -> bool {
        self.data.compatible(&x.data)
    }

    fn apply(&mut self, patch: &Self) {
        self.data.apply(&patch.data);
    }

    fn save(self) {
        self.data.save(self.path);
    }
}

impl Data {
    const fn format(&self) -> Format {
        match self {
            Self::Ini(_) => Format::Ini,
        }
    }

    fn compatible(&self, x: &Self) -> bool {
        self.format() == x.format()
    }

    fn apply(&mut self, patch: &Self) {
        match (self, patch) {
            (Self::Ini(target), Self::Ini(patch)) => Self::apply_ini(target, patch),
        }
    }

    fn apply_ini(target: &mut Ini, patch: &Ini) {
        for (sec, prop) in patch {
            for (k, v) in prop {
                target.with_section(sec).set(k, v);
            }
        }
    }

    fn save(self, path: PathBuf) {
        let result = match self {
            Self::Ini(ini) => ini.write_to_file(path),
        };

        result.expect("Failed to save target");
    }
}

fn main() {
    set_panic_hook();

    let args = Args::parse();

    if !args.target.exists() {
        std::fs::File::create(&args.target).expect("Failed to create target file");
    }

    for patch in &args.patches {
        assert!(patch.exists(), "{} doesn't exist", patch.display());
        assert!(patch.is_file(), "{} is not a file", patch.display());
    }

    let patches: Vec<File> = args.patches.iter().map(|x| File::load(x)).collect();
    let patch_format = get_patch_format(&patches);

    let target = File::load_as(&args.target, &patch_format);
    let mut target = target.expect("Failed to load target");

    for patch in &patches {
        let compatible = target.compatible(patch);

        let patch_path = patch.path.display();
        assert!(compatible, "{patch_path} is not valid");
    }

    for patch in patches {
        target.apply(&patch);
    }

    target.save();
}

#[allow(clippy::indexing_slicing)]
fn get_patch_format(items: &[File]) -> Format {
    let first = items[0].data.format();

    let are_same = items.iter().all(|x| x.data.format() == first);
    assert!(are_same, "Patches have different file formats");

    first
}
