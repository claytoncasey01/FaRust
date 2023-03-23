
# FaRust

FaRust is a command-line tool that generates icon wrapper components for Font Awesome Icons, based on a configuration file. It is written in Rust and utilizes the Tera template engine to render the components.

## Requirements

-   Rust 1.54 or later
-   FontAwesome Icons packages (Pro or Free)

## Installation

Clone the repository and build the project using Cargo:

```bash
$ git clone https://github.com/yourusername/farust.git
$ cd farust
$ cargo build --release
```
## Usage

Create a JSON configuration file, specifying the icons you want to generate components for, along with the output directory. Here's an example `config.json`:
```json
{
  "icons": [
    {
      "name": "faGithub",
      "component_name": "Github",
      "style": "brands",
      "icon_type": "free"
    }
  ],
  "output": "./src/components/icons"
}
```

Run FaRust with the path to the configuration file:
```bash
$ ./target/release/farust --config config.json
```

This will generate the specified components in the output directory specified in the configuration file.

## Icon Configuration

### Name

The name of the icon in camelCase.

### Component Name
The name of both the exported component and the filename. ``Note: The word Icon will be append at the end. Given "Xmark" as the component_name in the config.json you will get XmarkIcon.``

### Style

The following values can be used for the `style` field in the `config.json`:

- `solid`: Solid style icons
- `regular`: Regular style icons
- `light`: Light style icons
- `thin`: Thin style icons
- `duotone`: Duotone style icons
- `brands`: Brand style icons

### Icon Type

The following values can be used for the `icon_type` field in the `config.json`:

- `pro`: Pro (paid) version of FontAwesome icons
- `free`: Free version of FontAwesome icons

### Output
The path to output the generated components. If the necessary folders do not already exist they will be created. Anything matching the name of the generated files will be overwritten with the new generated component.

