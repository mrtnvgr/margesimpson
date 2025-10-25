use clap::Parser;
use ini::Ini;
use quiet_panics::set_panic_hook;
use std::mem::discriminant;
use std::path::PathBuf;

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

impl File {
    fn load(path: PathBuf) -> Self {
        let data: Data;

        if let Ok(ini) = Ini::load_from_file(&path) {
            data = Data::Ini(ini);
        } else {
            panic!("{} format is unsupported", path.display());
        }

        Self { path, data }
    }

    fn compatible(&self, x: &Self) -> bool {
        self.data.compatible(&x.data)
    }

    fn apply(&mut self, patch: &Self) {
        self.data.apply(&patch.data);
    }
}

impl Data {
    fn compatible(&self, x: &Self) -> bool {
        discriminant(self) == discriminant(x)
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

    let mut target = File::load(args.target);

    let mut patches: Vec<File> = vec![];
    for patch in args.patches {
        let patch = File::load(patch);

        let compatible = target.compatible(&patch);
        let patch_path = patch.path.display();
        assert!(compatible, "{patch_path} is not valid");

        patches.push(patch);
    }

    for patch in patches {
        target.apply(&patch);
    }
}
