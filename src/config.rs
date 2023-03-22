use serde::Deserialize;

/// The configuration for an individual icon
#[derive(Deserialize, Debug)]
pub struct Icon {
    /// The FontAwesome icon name: ex. "faGithub"
    pub name: String,
    /// The import path for the icon: ex. "@fortawesome/free-brands-svg-icons"
    pub path: String,
    /// The name of the component to be generated: ex. "GithubIcon"
    pub component_name: String,
}

/// The configuration file for the application
#[derive(Deserialize, Debug)]
pub struct Config {
    /// The list of icons to generate components for
    pub icons: Vec<Icon>,
    /// The path to the directory to output the generated components
    pub output: String,
}
