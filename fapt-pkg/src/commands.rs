use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use reqwest;
use serde_json;

use classic_sources_list::Entry;
use release;
use rfc822;
use lists;

use errors::*;

pub struct System {
    lists_dir: PathBuf,
    sources_entries: Vec<Entry>,
    arches: Vec<String>,
    keyring_paths: Vec<PathBuf>,
    client: reqwest::Client,
}

impl System {
    pub fn cache_dirs_only<P: AsRef<Path>>(lists_dir: P) -> Result<Self> {
        fs::create_dir_all(lists_dir.as_ref())?;

        let client = if let Ok(proxy) = env::var("http_proxy") {
            reqwest::Client::builder()
                .proxy(reqwest::Proxy::http(&proxy)?)
                .build()?
        } else {
            reqwest::Client::new()
        };

        Ok(System {
            lists_dir: lists_dir.as_ref().to_path_buf(),
            sources_entries: Vec::new(),
            arches: Vec::new(),
            keyring_paths: Vec::new(),
            client,
        })
    }

    pub fn add_sources_entry_line(&mut self, src: &str) -> Result<()> {
        self.add_sources_entries(::classic_sources_list::read(src)?);
        Ok(())
    }

    pub fn add_sources_entries<I: IntoIterator<Item = Entry>>(&mut self, entries: I) {
        self.sources_entries.extend(entries);
    }

    pub fn set_arches(&mut self, arches: &[&str]) {
        self.arches = arches.iter().map(|x| x.to_string()).collect();
    }

    pub fn add_keyring_paths<P: AsRef<Path>, I: IntoIterator<Item = P>>(
        &mut self,
        keyrings: I,
    ) -> Result<()> {
        self.keyring_paths
            .extend(keyrings.into_iter().map(|x| x.as_ref().to_path_buf()));
        Ok(())
    }

    pub fn update(&self) -> Result<()> {
        let requested =
            release::RequestedReleases::from_sources_lists(&self.sources_entries, &self.arches)
                .chain_err(|| "parsing sources entries")?;

        requested
            .download(&self.lists_dir, &self.keyring_paths, &self.client)
            .chain_err(|| "downloading releases")?;

        let releases = requested
            .parse(&self.lists_dir)
            .chain_err(|| "parsing releases")?;

        lists::download_files(&self.client, &self.lists_dir, &releases)
            .chain_err(|| "downloading release content")?;

        Ok(())
    }

    pub fn sections<'a>(&'a self) -> Result<Box<Iterator<Item = Result<String>> + 'a>> {
        let releases =
            release::RequestedReleases::from_sources_lists(&self.sources_entries, &self.arches)
                .chain_err(|| "parsing sources entries")?
                .parse(&self.lists_dir)
                .chain_err(|| "parsing releases")?;

        let mut ret = Vec::new();

        for release in releases {
            for listing in lists::selected_listings(&release) {
                for section in lists::sections_in(&release, &listing, &self.lists_dir)? {
                    ret.push(section);
                }
            }
        }

        Ok(Box::new(ret.into_iter()))
    }

    // Oh dear oh dear, not even close.
    #[cfg(never)]
    pub fn sections<'a>(&'a self) -> Result<Box<Iterator<Item = String> + 'a>> {
        let releases =
            release::RequestedReleases::from_sources_lists(&self.sources_entries, &self.arches)
                .chain_err(|| "parsing sources entries")?
                .parse(&self.lists_dir)
                .chain_err(|| "parsing releases")?;

        Ok(Box::new(
            releases
                .iter()
                .flat_map(|release| {
                    lists::selected_listings(&release)
                        .into_iter()
                        .map(move |x| (release, x))
                })
                .flat_map(move |(release, listing)| {
                    lists::sections_in(&release, &listing, &self.lists_dir).expect("??")
                })
                .map(|section| section.expect("???")),
        ))
    }

    pub fn export(&self) -> Result<()> {
        let releases =
            release::RequestedReleases::from_sources_lists(&self.sources_entries, &self.arches)
                .chain_err(|| "parsing sources entries")?
                .parse(&self.lists_dir)
                .chain_err(|| "parsing releases")?;

        for release in releases {
            for listing in lists::selected_listings(&release) {
                for section in lists::sections_in(&release, &listing, &self.lists_dir)? {
                    let section = section?;
                    let map: HashMap<&str, String> = rfc822::map(&section)
                        .chain_err(|| format!("scanning {:?}", release))?
                        .into_iter()
                        .map(|(k, v)| (k, v.join("\n")))
                        .collect();
                    serde_json::to_writer(io::stdout(), &map)?;
                    println!();
                }
            }
        }

        Ok(())
    }

    pub fn source_ninja(&self) -> Result<()> {
        let releases =
            release::RequestedReleases::from_sources_lists(&self.sources_entries, &self.arches)
                .chain_err(|| "parsing sources entries")?
                .parse(&self.lists_dir)
                .chain_err(|| "parsing releases")?;

        for release in releases {
            for listing in lists::selected_listings(&release) {
                for section in lists::sections_in(&release, &listing, &self.lists_dir)? {
                    let section = section?;
                    let map =
                        rfc822::map(&section).chain_err(|| format!("scanning {:?}", release))?;
                    if map.contains_key("Files") {
                        print_ninja_source(&map)?;
                    } else {
                        print_ninja_binary(&map)?;
                    }
                }
            }
        }

        Ok(())
    }
}

fn one_line<'a>(lines: &[&'a str]) -> Result<&'a str> {
    ensure!(1 == lines.len(), "{:?} isn't exactly one line", lines);
    Ok(lines[0])
}

// Sigh, I've already written this.
fn subdir(name: &str) -> &str {
    if name.starts_with("lib") {
        &name[..4]
    } else {
        &name[..1]
    }
}

#[cfg(never)]
struct Sections<'i> {
    lists_dir: PathBuf,
    releases: Box<Iterator<Item = release::Release> + 'i>,
    release: release::Release,
    listings: Box<Iterator<Item = lists::Listing> + 'i>,
    sections: Box<Iterator<Item = Result<String>>>,
}

#[cfg(never)]
impl<'i> Iterator for Sections<'i> {
    type Item = Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(section) = self.sections.next() {
                return Some(section);
            }

            if let Some(listing) = self.listings.next() {
                // peek() also doesn't live long enough
                self.sections = match lists::sections_in(&self.release, &listing, self.lists_dir) {
                    Ok(sections) => sections,
                    Err(e) => return Some(Err(e)),
                };
                continue;
            }
        }
    }
}

fn print_ninja_source(map: &HashMap<&str, Vec<&str>>) -> Result<()> {
    let pkg = one_line(&map["Package"])?;
    let version = one_line(&map["Version"])?.replace(':', "$:");
    let dir = one_line(&map["Directory"])?;

    let dsc = map["Files"]
        .iter()
        .filter(|line| line.ends_with(".dsc"))
        .next()
        .unwrap()
        .split_whitespace()
        .nth(2)
        .unwrap();

    let size: u64 = map["Files"]
        .iter()
        .map(|line| {
            let num: &str = line.split_whitespace().nth(1).unwrap();
            let num: u64 = num.parse().unwrap();
            num
        })
        .sum();

    let prefix = format!("{}/{}_{}", subdir(pkg), pkg, version);

    println!("build $dest/{}$suffix: process-source | $script", prefix);

    println!("  description = PS {} {}", pkg, version);
    println!("  pkg = {}", pkg);
    println!("  version = {}", version);
    println!("  url = $mirror/{}/{}", dir, dsc);
    println!("  prefix = {}", prefix);
    println!("  size = {}", size);
    if size > 250 * 1024 * 1024 {
        // ~20 packages
        println!("  pool = massive")
    } else if size > 100 * 1024 * 1024 {
        // <1%
        println!("  pool = big")
    }

    Ok(())
}

fn print_ninja_binary(map: &HashMap<&str, Vec<&str>>) -> Result<()> {
    let pkg = one_line(&map["Package"])?;
    let source = one_line(&map.get("Source").unwrap_or_else(|| &map["Package"]))?
        .split_whitespace()
        .nth(0)
        .unwrap();
    let arch = one_line(&map["Architecture"])?;
    let version = one_line(&map["Version"])?.replace(':', "$:");
    let filename = one_line(&map["Filename"])?;
    let size: u64 = one_line(&map["Size"])?.parse()?;

    let prefix = format!("{}/{}/{}_{}_{}", subdir(source), source, pkg, version, arch);

    println!("build $dest/{}$suffix: process-binary | $script", prefix);
    println!("  description = PB {} {} {} {}", source, pkg, version, arch);
    println!("  source = {}", source);
    println!("  pkg = {}", pkg);
    println!("  version = {}", version);
    println!("  arch = {}", arch);
    println!("  url = $mirror/{}", filename);
    println!("  prefix = {}", prefix);

    if size > 250 * 1024 * 1024 {
        println!("  pool = massive")
    } else if size > 100 * 1024 * 1024 {
        println!("  pool = big")
    }

    Ok(())
}
