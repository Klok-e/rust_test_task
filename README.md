# Weather CLI

A command-line interface (CLI) application for retrieving weather data from different providers.
## Usage

To use the Weather CLI, you must have Rust installed on your machine. Once you have installed Rust, clone this repository, navigate to its root directory in a terminal window, and execute the following command to build the executable:

```bash
cargo build --release
```
This will build the weather executable in the target/release directory. You can then run the CLI by executing the following command:

```bash
./target/release/weather
```

This will display the help message for the CLI, which lists the available commands and their respective options.
## Commands
### get

Prints the weather for a specified location and date. To use this command, execute the following command:

```bash
./target/release/weather get <ADDRESS> [DATE]
```

where `<ADDRESS>` is the name of the city you want to retrieve weather data for, and `[DATE]` is an optional parameter that specifies the date of the weather data. If `[DATE]` is not provided, the command will retrieve the current weather data. The `[DATE]` parameter must have the value of either "now" or a datetime string in the format "%Y-%m-%d %H:%M:%S".
### configure

Configures the provider to be used for retrieving weather data. To use this command, execute the following command:

```bash
./target/release/weather configure <PROVIDER>
```

where `<PROVIDER>` is the name of the provider you want to configure. The possible values for `<PROVIDER>` are:

- **open-weather**: OpenWeather provider
- **weather-api**: WeatherAPI provider

## Options

The following options are available for all commands:

- **-h**, **--help**: Prints help for the specified command or the CLI as a whole.
- **-V**, **--version**: Prints the version number of the CLI.
