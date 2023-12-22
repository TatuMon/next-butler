use std::ffi::OsStr;

pub trait GuessReactExtension {
    fn guess_extension(&self) -> ReactExtension;
}

pub enum ReactExtension {
    Jsx,
    Tsx,
    Js,
    Ts,
}

impl From<ReactExtension> for &'static str {
    fn from(e: ReactExtension) -> Self {
        match e {
            ReactExtension::Jsx => "jsx",
            ReactExtension::Tsx => "tsx",
            ReactExtension::Js => "js",
            ReactExtension::Ts => "ts",
        }
    }
}

impl From<&ReactExtension> for &'static str {
    fn from(e: &ReactExtension) -> Self {
        match e {
            ReactExtension::Jsx => "jsx",
            ReactExtension::Tsx => "tsx",
            ReactExtension::Js => "js",
            ReactExtension::Ts => "ts",
        }
    }
}

impl From<&str> for ReactExtension {
    fn from(e: &str) -> Self {
        match e {
            "jsx" => ReactExtension::Jsx,
            "tsx" => ReactExtension::Tsx,
            "js" => ReactExtension::Js,
            "ts" => ReactExtension::Ts,
            _ => ReactExtension::Js,
        }
    }
}

impl From<&OsStr> for ReactExtension {
    fn from(e: &OsStr) -> Self {
        match e.to_string_lossy().to_string().as_str() {
            "jsx" => ReactExtension::Jsx,
            "tsx" => ReactExtension::Tsx,
            "js" => ReactExtension::Js,
            "ts" => ReactExtension::Ts,
            _ => ReactExtension::Js,
        }
    }
}

impl ReactExtension {
    pub fn guess(
        js_flag: bool,
        tsx_flag: bool,
        ts_flag: bool,
        user_new_x_cfg: Option<impl GuessReactExtension>,
    ) -> Self {
        if js_flag {
            Self::Js
        } else if tsx_flag {
            Self::Tsx
        } else if ts_flag {
            Self::Ts
        } else if user_new_x_cfg.is_none() {
            Self::Jsx
        } else if let Some(user_cfg) = user_new_x_cfg {
            user_cfg.guess_extension()
        } else {
            Self::Jsx
        }
    }
}