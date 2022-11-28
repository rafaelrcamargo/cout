use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Profile {
    pub patterns: Vec<Vec<String>>,
}

pub fn get_profile(profile: &str) -> Profile {
    if profile == "default" {
        toml::from_str::<Profile>(
            r#"patterns=[['''(https?://|www.)[^\s][^\)|\s]+''',"blue"],["-+","black"],["[0-9]?[0-9]/([1][0-2])?[1-9]?/[0-9]?[0-9][0-9][0-9]","yellow"],['''[^0-9a-zA-Z=:]?ms\.?''',"cyan"],["\b(?:error|Error|ERROR|Err|ERR|err)\b","red"],["\b(?:warn|Warn|WARN|Warning|WARNING|warning)\b","yellow"],["\b(?:info|Info|INFO|Information|INFORMATION|information)\b","green"],["\b(?:debug|Debug|DEBUG|Debugging|DEBUGGING|debugging)\b","blue"],["\b[A-Z_][A-Z_]*\b","yellow"],['''["|'][^"|']*["|']''',"yellow"],['''\[[^\]]*\]''',"brightcyan"],['''\{[^\}]*\}''',"brightMagenta"],[".{2,}:[\n| ]","Green"]]"#,
        ).unwrap()
    } else {
        let config = dirs::config_dir()
            .expect("Failed to get config dir.")
            .join("cout");
        match std::fs::read(config.join([profile, ".toml"].concat())) {
            Ok(data) => toml::from_str::<Profile>(
                &String::from_utf8(data).expect("Failed to read config data."),
            )
            .expect("Failed to parse config data."),
            Err(_) => get_profile("default"),
        }
    }
}
