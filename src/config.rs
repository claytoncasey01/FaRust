use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum IconStyle {
    Solid,
    Regular,
    Light,
    Thin,
    Duotone,
    Brands,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum IconType {
    Pro,
    Free,
}

/// The configuration for an individual icon
#[derive(Deserialize, Debug)]
pub struct Icon {
    /// The FontAwesome icon name: ex. "faGithub"
    pub name: String,
    /// The name of the component to be generated: ex. "GithubIcon"
    pub component_name: String,

    /// The style of icon to generate: ex. "Solid", "Regular", "Light", "Thin", "Duotone", "Brands"
    pub style: IconStyle,

    /// The type of icon to generate: ex. "Pro", "Free"
    pub icon_type: IconType,
}

/// The configuration file for the application
#[derive(Deserialize, Debug)]
pub struct Config {
    /// The list of icons to generate components for
    pub icons: Vec<Icon>,
    /// The path to the directory to output the generated components
    pub output: String,
}
