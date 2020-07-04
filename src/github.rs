//! I plan to make my own git server that keeps track of all my projects, but for now I'll just pull
//! my stuff from their existing GitHub repositories.

use std::str::FromStr;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct GithubProject {
    pub name: String,
    pub repo: String,
    pub ns: Option<String>,
    pub lang: Lang,
    pub description: String,
}

impl GithubProject {
    const API: &'static str = "https://api.github.com/";

    /// Fetches a single project's data from the GH API
    pub fn get(name: &str, repo: &str) -> Result<Self, String> {
        ureq::get(&format!("{}repos/periodicaidan/{}", Self::API, repo))
            .call()
            .into_json()
            .map(|json| {
                Self::from_json(name, None, &json)
            })
            .map_err(|e| e.to_string())
    }

    /// Fetches several projects from the GH API
    /// [`projects`] is repo_name => project_name
    pub fn get_plural(projects: &HashMap<&str, &str>) -> Vec<Self> {
        ureq::get(&format!("{}users/periodicaidan/repos", Self::API))
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

    /// Turns a JSON object into a [`GithubProject`]
    fn from_json(name: &str, ns: Option<&str>, json: &Value) -> Self {
        Self {
            name: name.to_owned(),
            repo: json.get("name")
                .and_then(Value::as_str)
                .unwrap().to_owned(),
            ns: ns.map(str::to_owned),
            lang: json.get("language")
                .and_then(Value::as_str)
                .map(|s| s.parse().unwrap())
                .unwrap(),
            description: json.get("description")
                .and_then(|v| v.as_str())
                .unwrap().to_owned()
        }
    }
}

#[derive(Serialize, Deserialize)]
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