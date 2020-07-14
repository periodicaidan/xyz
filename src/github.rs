//! I plan to make my own git server that keeps track of all my projects, but for now I'll just pull
//! my stuff from their existing GitHub repositories.

use std::str::FromStr;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::fs;
use chrono::{NaiveDateTime, DateTime, Utc, Duration};

#[derive(Serialize, Deserialize)]
pub struct GithubProject {
    pub name: String,
    pub repo: String,
    pub ns: Option<String>,
    pub lang: Lang,
    pub description: String,
    pub color: String,
}

#[derive(Serialize, Deserialize)]
struct GithubProjectArgs {
    pub name: String,
    pub repo: String,
}

impl GithubProject {
    const API: &'static str = "https://api.github.com";

    /// Fetches a single project's data from the GH API
    pub fn get(name: &str, repo: &str) -> Result<Self, String> {
        ureq::get(&format!("{}/repos/periodicaidan/{}", Self::API, repo))
            .call()
            .into_json()
            .map(|json| {
                println!("{:#?}", json);
                Self::from_json(name, None, &json)
            })
            .map_err(|e| e.to_string())
    }

    /// Fetches several projects from the GH API
    /// [`projects`] is repo_name => project_name
    pub fn get_plural(projects: &HashMap<&str, &str>) -> Vec<Self> {
        ureq::get(&format!("{}/users/periodicaidan/repos", Self::API))
            .call()
            .into_json()
            .map(|v| {
                v.as_array().unwrap().clone()
            })
            .unwrap()
            .iter()
            .filter_map(|v| {
                let repo_name = v.get("name")
                    .and_then(Value::as_str)
                    .unwrap();

                projects.get(repo_name)
                    .map(|s| Self::from_json(s, None, v))
            })
            .collect()
    }

    pub fn read_from_file(filepath: impl AsRef<Path>) -> Vec<Self> {
        fs::read(filepath.as_ref())
            .ok()
            .and_then(|bs| serde_json::from_slice::<Vec<GithubProjectArgs>>(&bs).ok())
            .map(|v|
                v.iter()
                    .filter_map(|args| Self::get(&args.name, &args.repo).ok())
                    .collect()
            )
            .unwrap_or(vec![])
    }

    /// Turns a JSON object into a [`GithubProject`]
    fn from_json(name: &str, ns: Option<&str>, json: &Value) -> Self {
        let lang: Lang = json.get("language")
            .and_then(Value::as_str)
            .map(|s| s.parse().unwrap())
            .unwrap_or(Lang::Other("Other".into()));

        Self {
            name: name.to_owned(),
            repo: json.get("name")
                .and_then(Value::as_str)
                .unwrap().to_owned(),
            ns: ns.map(str::to_owned),
            lang: lang.clone(),
            description: json.get("description")
                .and_then(|v| v.as_str())
                .unwrap().to_owned(),
            color: lang.color(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Lang {
    C, CSharp, Cpp, Shell, Julia, Python, Rust, Dart, Java, Kotlin, Elm, Haskell,
    Other(String)
}

impl FromStr for Lang {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "C" => Lang::C,
            "C#" => Lang::CSharp,
            "C++" => Lang::Cpp,
            "Shell" => Lang::Shell,
            "Julia" => Lang::Julia,
            "Python" => Lang::Python,
            "Rust" => Lang::Rust,
            "Dart" => Lang::Dart,
            "Java" => Lang::Java,
            "Kotlin" => Lang::Kotlin,
            "Elm" => Lang::Elm,
            "Haskell" => Lang::Haskell,
            _ => Lang::Other(s.to_owned())
        })
    }
}

impl Lang {
    pub fn color(&self) -> String {
        use Lang::*;
        match self {
            C => "grey",
            CSharp => "green",
            Cpp => "pink",
            Shell => "olive",
            Julia => "purple",
            Python => "blue",
            Rust => "brown",
            Dart | Elm => "teal",
            Java => "yellow",
            Kotlin => "orange",
            Haskell => "violet",
            Other(_) => ""
        }.into()
    }
}

pub struct GithubProjectCache {
    projects: HashMap<String, GithubProject>,
    cache_file: PathBuf,
    index_file: PathBuf,
    last_written: DateTime<Utc>
}

impl GithubProjectCache {
    pub fn new(cache_file: impl AsRef<Path>, index_file: impl AsRef<Path>) -> Self {
        Self {
            projects: map!{},
            cache_file: cache_file.as_ref().into(),
            index_file: index_file.as_ref().into(),
            last_written: Utc::now()
        }
    }

    pub fn is_old(&self) -> bool {
        let now = Utc::now();
        now - self.last_written > Duration::days(1)
    }

    pub fn pull(&mut self) {
        for k in self.projects.keys() {

        }
    }

    pub fn reload(&mut self) {

    }

    pub fn write_to_file(&self, filepath: impl AsRef<Path>) {
        fs::write(filepath, serde_json::to_string(&self.projects).unwrap());
    }

    pub fn get(&mut self, key: &str) -> Option<&GithubProject> {
        if self.is_old() {
            self.reload();
            self.pull();
            self.last_written = Utc::now();
        }

        let maybe_project = self.projects.get(key);
        if let Some(proj) = maybe_project {
            Some(proj)
        } else {
            None
        }
    }
}