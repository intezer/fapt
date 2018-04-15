use std::collections::HashMap;

use errors::*;
use rfc822;

// Everything deals with streams of Items.

pub enum Item {
    End,
    Raw(RawPackage),
    Index(RawIndex),
    Package(Package),
}

// An unparsed, raw package

pub struct RawPackage {
    type_: RawPackageType,
    entries: Vec<Entry>,
}

pub enum RawPackageType {
    Source,
    Binary,
}

pub struct Entry {
    key: String,
    value: String,
}

// An unparsed, raw index

pub struct RawIndex {
    archive: String,
    version: String,
    origin: String,
    codename: String,
    label: String,
    site: String,
    component: String,
    arch: String,
    type_: String,
}

// The parsed top-level types for package

pub enum PackageType {
    Source(Source),
    Binary(Binary),
}

pub struct Package {
    name: String,
    version: String,
    priority: Priority,
    arch: Vec<String>,

    maintainer: Vec<Identity>,
    original_maintainer: Vec<Identity>,

    unparsed: HashMap<String, Vec<String>>,

    style: PackageType,
}

pub struct Source {
    format: SourceFormat,

    binaries: Vec<SourceBinary>,
    files: Vec<File>,
    vcs: Vec<Vcs>,

    build_dep: Vec<Dependency>,
    build_dep_arch: Vec<Dependency>,
    build_dep_indep: Vec<Dependency>,
    build_conflict: Vec<Dependency>,
    build_conflict_arch: Vec<Dependency>,
    build_conflict_indep: Vec<Dependency>,

    uploaders: Vec<Identity>,
}

pub struct Binary {
    // "File" is missing in e.g. dpkg/status, but never in Packages as far as I've seen
    file: Option<File>,

    essential: bool,
    build_essential: bool,

    installed_size: u64,

    description: String,

    depends: Vec<Dependency>,
    recommends: Vec<Dependency>,
    suggests: Vec<Dependency>,
    enhances: Vec<Dependency>,
    pre_depends: Vec<Dependency>,

    breaks: Vec<Dependency>,
    conflicts: Vec<Dependency>,
    replaces: Vec<Dependency>,

    provides: Vec<Dependency>,
}

// The dependency chain types

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dependency {
    pub alternate: Vec<SingleDependency>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SingleDependency {
    pub package: String,
    pub arch: Option<String>,
    /// Note: It's possible Debian only supports a single version constraint.
    pub version_constraints: Vec<Constraint>,
    pub arch_filter: Vec<String>,
    pub stage_filter: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Constraint {
    version: String,
    operator: ConstraintOperator,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConstraintOperator {
    Ge,
    Eq,
    Le,
    Gt,
    Lt,
}

// Other types

pub struct File {
    name: String,
    size: u64,
    md5: String,
    sha1: String,
    sha256: String,
    sha512: String,
}

pub struct Vcs {
    description: String,
    type_: VcsType,
    tag: VcsTag,
}

#[derive(Copy, Clone)]
pub enum VcsType {
    Browser,
    Arch,
    Bzr,
    Cvs,
    Darcs,
    Git,
    Hg,
    Mtn,
    Svn,
}

#[derive(Copy, Clone)]
pub enum VcsTag {
    Vcs,
    Orig,
    Debian,
    Upstream,
}

pub struct SourceBinary {
    name: String,
    style: String,
    section: String,

    priority: Priority,
    extras: Vec<String>,
}

// https://www.debian.org/doc/debian-policy/#priorities
pub enum Priority {
    Unknown,
    Required,
    Important,
    Standard,
    Optional,
    Extra,
    Source,
}

pub struct Description {
    locale: String,
    value: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identity {
    pub name: String,
    pub email: String,
}

pub enum SourceFormat {
    Unknown,
    Original,
    Quilt3dot0,
    Native3dot0,
    Git3dot0,
}

impl Package {
    pub fn parse_bin<'i, I: Iterator<Item = Result<rfc822::Line<'i>>>>(it: I) -> Result<Package> {
        use rfc822::one_line;
        use rfc822::joined;

        // Package
        let mut name = None;
        let mut version = None;
        let mut priority = None;
        let mut arch = None;
        let mut maintainer = Vec::new();
        let mut original_maintainer = Vec::new();

        // Binary
        let mut file = None;
        let mut essential = None;
        let mut build_essential = None;
        let mut installed_size = None;
        let mut description = None;
        let mut depends = Vec::new();
        let mut recommends = Vec::new();
        let mut suggests = Vec::new();
        let mut enhances = Vec::new();
        let mut pre_depends = Vec::new();
        let mut breaks = Vec::new();
        let mut conflicts = Vec::new();
        let mut replaces = Vec::new();
        let mut provides = Vec::new();

        let mut unparsed = HashMap::new();

        for res in it {
            let (key, values) = res?;
            match key {
                "Package" => name = Some(one_line(&values)?),
                "Version" => version = Some(one_line(&values)?),
                "Architecture" => {
                    arch = Some(
                        one_line(&values)?
                            // TODO: alternate splitting rules?
                            .split_whitespace()
                            .map(|s| s.to_string())
                            .collect(),
                    )
                }

                "Essential" => essential = Some(::yes_no(one_line(&values)?)?),
                "Build-Essential" => build_essential = Some(::yes_no(one_line(&values)?)?),
                "Priority" => priority = Some(::parse_priority(one_line(&values)?)?),
                "Maintainer" => maintainer.extend(::ident::read(one_line(&values)?)?),
                "Installed-Size" => installed_size = Some(one_line(&values)?.parse()?),
                "Description" => description = Some(joined(&values)),

                "Depends" => depends.extend(parse_dep(&values)?),
                "Recomends" => recommends.extend(parse_dep(&values)?),
                "Suggests" => suggests.extend(parse_dep(&values)?),
                "Enhances" => enhances.extend(parse_dep(&values)?),
                "Pre-Depends" => pre_depends.extend(parse_dep(&values)?),
                "Breaks" => breaks.extend(parse_dep(&values)?),
                "Conflicts" => conflicts.extend(parse_dep(&values)?),
                "Replaces" => replaces.extend(parse_dep(&values)?),
                "Provides" => provides.extend(parse_dep(&values)?),

                other => {
                    unparsed.insert(
                        key.to_string(),
                        values.iter().map(|s| s.to_string()).collect(),
                    );
                }
            }
        }

        Ok(Package {
            name: name.ok_or("missing name")?.to_string(),
            version: version.ok_or("missing version")?.to_string(),
            priority: priority.ok_or("missing priority")?,
            arch: arch.ok_or("missing arch")?,
            maintainer,
            original_maintainer,
            style: PackageType::Binary(Binary {
                file,
                essential: essential.unwrap_or(false),
                build_essential: build_essential.unwrap_or(false),
                installed_size: installed_size.ok_or("missing installed_size")?,
                description: description.ok_or("missing description")?,
                depends,
                recommends,
                suggests,
                enhances,
                pre_depends,
                breaks,
                conflicts,
                replaces,
                provides,
            }),
            unparsed,
        })
    }
}

fn parse_dep(multi_str: &[&str]) -> Result<Vec<Dependency>> {
    ::deps::read(&::rfc822::joined(multi_str))
}

impl Constraint {
    pub fn new(operator: ConstraintOperator, version: &str) -> Self {
        Constraint {
            operator,
            version: version.to_string(),
        }
    }
}