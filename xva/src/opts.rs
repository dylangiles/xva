use std::error::Error;

use clap::Parser;
pub(super) fn parse_unstable_option(
    s: &str,
) -> Result<UnstableOption, Box<dyn Error + Send + Sync + 'static>> {
    if s.contains('=') {
        let pos = s
            .find('=')
            .ok_or_else(|| format!("Invalid KEY=value: no `=` found in `{}`", s))?;
        let comma_pos = s.find(',');
        let key = s[..pos].parse()?;
        let value: UnstableOptionValue;
        if let Some(_) = comma_pos {
            value = UnstableOptionValue::Multiple(
                s[pos + 1..]
                    .split(',')
                    .into_iter()
                    .map(|x| x.trim().to_string())
                    .collect(),
            );
        } else {
            value = UnstableOptionValue::Single(s[pos + 1..].parse()?);
        }
        Ok(UnstableOption { key, value })
    } else {
        let key = s.parse()?;
        Ok(UnstableOption {
            key,
            value: UnstableOptionValue::None,
        })
    }
}

#[derive(Debug, Clone)]
pub(super) enum UnstableOptionValue {
    None,
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug)]

pub(super) struct UnstableOption {
    key: String,
    value: UnstableOptionValue,
}

impl Clone for UnstableOption {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            value: self.value.clone(),
        }
    }
}

#[derive(Parser, Debug)]
pub(super) struct Options {
    /// Specify unstable compiler options
    #[arg(short = 'Z', value_parser = parse_unstable_option)]
    pub unstable_options: Vec<UnstableOption>,
}

impl Options {
    #[allow(dead_code)]
    pub(crate) fn get_unstable(&self, key: &str) -> Option<&UnstableOption> {
        self.unstable_options.iter().find(|x| x.key == key)
    }

    #[allow(dead_code)]
    pub(crate) fn unstable_option_is(&self, key: &str, check: Option<&str>) -> bool {
        match self.unstable_options.iter().find(|x| x.key.as_str() == key) {
            Some(ref opt) => match &opt.value {
                UnstableOptionValue::None => check.is_none(),
                UnstableOptionValue::Single(v) => match check {
                    Some(c) => c == v,
                    None => false,
                },
                _ => false,
            },
            None => false,
        }
    }

    pub(crate) fn unstable_option_contains(&self, key: &str, check: &str) -> bool {
        // println!("opts: {:#?}", self.unstable_options);
        match self.unstable_options.iter().find(|x| x.key.as_str() == key) {
            Some(ref opt) => match &opt.value {
                UnstableOptionValue::Single(v) => v == check,
                UnstableOptionValue::Multiple(v) => {
                    v.iter().map(|x| x.as_str()).find(|x| *x == check).is_some()
                }
                _ => false,
            },
            None => false,
        }
    }
}
