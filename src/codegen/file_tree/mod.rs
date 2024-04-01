use super::{api::API, structs::Struct};
use std::io::Write;

pub trait File {
  fn file_name(&self) -> String;
  fn write(&self) -> std::io::Result<()>;
}

pub struct TypesFile {
  pub types: Vec<Struct>,
}

pub struct APIModuleFile {
  pub name: String,
  pub apis: Vec<API>,
}

pub struct EntryModuleFile {
  pub name: String,
  pub modules: Vec<APIModuleFile>,
}

/// Generate a mod.rs file for all files in the directory
pub struct Directory {
  pub name: String,
  pub files: Vec<Box<dyn File>>,
}

pub struct FileTree {
  pub files: Vec<Box<dyn File>>,
}

impl FileTree {
  pub fn new() -> Self {
    Self { files: vec![] }
  }

  pub fn add_file<T>(&mut self, file: T)
  where
    T: File + 'static,
  {
    self.files.push(Box::new(file));
  }

  pub fn write(&self) -> std::io::Result<()> {
    for file in &self.files {
      file.write()?;
    }

    Ok(())
  }
}

impl File for TypesFile {
  fn file_name(&self) -> String {
    format!("types.rs")
  }

  fn write(&self) -> std::io::Result<()> {
    todo!("Write types.rs file")
  }
}

impl File for APIModuleFile {
  fn file_name(&self) -> String {
    format!("{}.rs", self.name)
  }

  fn write(&self) -> std::io::Result<()> {
    todo!("Write {}.rs file", self.name)
  }
}

impl File for Directory {
  fn file_name(&self) -> String {
    self.name.clone()
  }

  fn write(&self) -> std::io::Result<()> {
    let mut modules = vec![];

    for file in &self.files {
      file.write()?;
      modules.push(file.file_name());
    }

    let mod_file = format!("{}/mod.rs", self.name);

    let mut mod_file = std::fs::File::create(mod_file)?;

    for module in modules {
      writeln!(mod_file, "mod {};", module)?;
    }

    Ok(())
  }
}
