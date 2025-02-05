#[derive(Debug)]
pub enum Language {
    Rust,
    Java,
    Perl,
}

#[derive(Clone, Debug)]
pub struct Dependency {
    name: String,
    version_expression: String,
}

#[derive(Debug)]
pub struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
    dependencies: Vec<Dependency>,
    language: Option<Language>,
}

impl Package {
    pub fn as_dependency(&self) -> Dependency {
        Dependency {
            name: self.name.clone(),
            version_expression: self.version.clone(),
        }
    }
}

pub struct PackageBuilder(Package);

impl PackageBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self(Package {
            name: name.into(),
            version: "0.1".into(),
            authors: Vec::new(),
            dependencies: Vec::new(),
            language: None,
        })
    }

    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.0.version = version.into();
        self
    }

    pub fn authors(mut self, authors: Vec<String>) -> Self {
        self.0.authors = authors;
        self
    }

    pub fn dependency(mut self, dependency: Dependency) -> Self {
        self.0.dependencies.push(dependency);
        self
    }

    pub fn language(mut self, language: Language) -> Self {
        self.0.language = Some(language);
        self
    }

    pub fn build(self) -> Package {
        self.0
    }
}
