use {
  arguments::Arguments,
  cargo_metadata::{MetadataCommand, PackageId, camino::Utf8Path, semver::Version},
  clap::{Parser, builder::styling},
  error::Error,
  snafu::{ErrorCompat, OptionExt, ResultExt, Snafu, ensure},
  std::{
    collections::{HashMap, VecDeque},
    io::{self, IsTerminal},
    process::ExitCode,
  },
};

mod arguments;
mod error;

fn main() -> ExitCode {
  if let Err(error) = run() {
    if io::stderr().is_terminal() {
      eprintln!("\x1b[1;31merror\x1b[0m: \x1b[1m{error}\x1b[0m");
    } else {
      eprintln!("error: {error}");
    }

    let causes = error.iter_chain().skip(1).count();

    for (i, source) in error.iter_chain().skip(1).enumerate() {
      eprintln!(
        "       {}─ {source}",
        if i < causes - 1 { '├' } else { '└' }
      );
    }

    ExitCode::FAILURE
  } else {
    ExitCode::SUCCESS
  }
}

fn run() -> Result<(), Error> {
  let Arguments::Path { dependency } = Arguments::parse();

  let metadata = MetadataCommand::new().exec().context(error::Metadata)?;

  let resolve = metadata.resolve.as_ref().context(error::MissingResolve)?;

  let root = resolve.root.as_ref().context(error::MissingRoot)?;

  let nodes = resolve
    .nodes
    .iter()
    .map(|node| (&node.id, node.dependencies.as_slice()))
    .collect::<HashMap<&PackageId, &[PackageId]>>();

  let mut depths = HashMap::<&PackageId, u32>::new();
  let mut queue = VecDeque::new();

  depths.insert(root, 0);
  queue.push_back(root);

  while let Some(id) = queue.pop_front() {
    let depth = depths[id];
    if let Some(deps) = nodes.get(id) {
      for dep in *deps {
        if !depths.contains_key(dep) {
          depths.insert(dep, depth + 1);
          queue.push_back(dep);
        }
      }
    }
  }

  let mut matches = metadata
    .packages
    .iter()
    .filter(|p| p.name.as_str() == dependency)
    .filter_map(|p| {
      Some((
        depths.get(&p.id).copied()?,
        &p.version,
        p.manifest_path.parent().unwrap(),
      ))
    })
    .collect::<Vec<(u32, &Version, &Utf8Path)>>();

  ensure!(
    !matches.is_empty(),
    error::DependencyNotFound { dependency }
  );

  matches.sort();

  for (_depth, _version, path) in &matches {
    println!("{path}");
  }

  Ok(())
}
