use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;

use reqwest;
use reqwest::Url;

use errors::*;
use fetch::fetch;
use fetch::Download;
use signing::verify_clearsigned;

#[derive(PartialOrd, Ord, Hash, PartialEq, Eq)]
pub struct RequestedRelease {
    mirror: Url,
    /// This can also be called "suite" in some places,
    /// e.g. "unstable" (suite) == "sid" (codename)
    codename: String,
}

impl RequestedRelease {
    pub fn dists(&self) -> Result<Url> {
        Ok(self.mirror.join("dists/")?.join(
            &format!("{}/", self.codename),
        )?)
    }

    pub fn filesystem_safe(&self) -> String {
        let u = &self.mirror;
        let underscore_path = u.path_segments()
            .map(|parts| parts.collect::<Vec<&str>>().join("_"))
            .unwrap_or_else(|| String::new());
        format!(
            "{}_{}_{}_{}_{}_{}",
            u.scheme(),
            u.username(),
            u.host_str().unwrap_or(""),
            u.port().unwrap_or(0),
            underscore_path,
            self.codename
        )
    }
}

pub fn releases(sources_list: &[::classic_sources_list::Entry]) -> Result<Vec<RequestedRelease>> {
    let mut ret = HashSet::with_capacity(sources_list.len() / 2);

    for entry in sources_list {
        ret.insert(RequestedRelease {
            // TODO: urls without trailing slashes?
            mirror: Url::parse(&entry.url)?,
            codename: entry.suite_codename.to_string(),
        });
    }

    Ok(ret.into_iter().collect())
}

pub fn download_releases<P: AsRef<Path>>(
    lists_dir: P,
    releases: &[RequestedRelease],
) -> Result<Vec<PathBuf>> {
    let lists_dir = lists_dir.as_ref();

    let mut downloads = Vec::with_capacity(releases.len());

    for release in releases {
        let url = release.dists()?.join("InRelease")?;
        let dest = format!("{}_InRelease", release.filesystem_safe());
        downloads.push(Download::from_to(url, lists_dir.join(dest)));
    }

    let client = reqwest::Client::new();
    fetch(&client, &downloads)?;

    let mut ret = Vec::with_capacity(releases.len());

    for release in releases {
        let downloaded = lists_dir.join(format!("{}_InRelease", release.filesystem_safe()));
        let verified = lists_dir.join(format!("{}_Verified", release.filesystem_safe()));
        verify_clearsigned(downloaded, &verified)?;
        ret.push(verified);
    }
    Ok(ret)
}