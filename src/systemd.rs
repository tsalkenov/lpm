use ini::Ini;
use std::env;
use std::fs;
use std::process::Command;

pub struct Unit {
    pub unit: Vec<(String, String)>,
    pub service: Vec<(String, String)>,
    pub install: Vec<(String, String)>,
}

pub struct Systemd {
    default_args: Vec<&'static str>,
    services_path: String,  // without trailing slash
    pub default_target: String,
}

impl Systemd {
    pub fn new(user_mode: bool) -> Self {
        let mut default_args = vec![];
        if user_mode {
            default_args.push("--user");
        }

        let services_path = if user_mode {
            format!("{}/.config/systemd/user", env::var("HOME").unwrap())
        } else {
            String::from("/etc/systemd/system")
        };

        Systemd {
            default_args,
            services_path,
            default_target: if user_mode {
                "default.target".to_string()
            } else {
                "multi-user.target".to_string()
            }
        }
    }

    pub fn init(&self) {
        // Creates a services directory if it doesn't exist
        fs::create_dir_all(&self.services_path).expect("Failed to create services directory");
    }

    #[inline(always)]
    fn systemctl(&self, args: Vec<&str>) -> Command {
        let mut command = Command::new("systemctl");
        for arg in &self.default_args {
            command.arg(arg);
        }
        for arg in args {
            command.arg(arg);
        }
        command
    }
}

impl Systemd {
    pub fn start(&self, service: &str) -> Command {
        self.systemctl(vec!["start", service])
    }

    pub fn stop(&self, service: &str) -> Command {
        self.systemctl(vec!["stop", service])
    }

    pub fn restart(&self, service: &str) -> Command {
        self.systemctl(vec!["restart", service])
    }

    pub fn reload(&self, service: &str) -> Command {
        self.systemctl(vec!["reload", service])
    }

    pub fn enable(&self, service: &str) -> Command {
        self.systemctl(vec!["enable", service])
    }

    pub fn disable(&self, service: &str) -> Command {
        self.systemctl(vec!["disable", service])
    }

    pub fn status(&self, service: &str) -> Command {
        self.systemctl(vec!["status", service])
    }

    pub fn daemon_reload(&self) -> Command {
        self.systemctl(vec!["daemon-reload"])
    }
}

impl Systemd {
    pub fn install_service(&self, service: &str, unit: &Unit) {
        let unit_path = format!("{}/{}.service", self.services_path, service);
        let mut ini = Ini::new();

        for (section_name, section_data) in [
            ("Unit", &unit.unit),
            ("Service", &unit.service),
            ("Install", &unit.install),
        ] {
            for (key, value) in section_data {
                ini.with_section(Some(section_name.to_string()))
                    .add(key, value);
            }
        }

        ini.write_to_file(unit_path)
            .expect("Failed to write unit file");
    }

    pub fn uninstall_service(&self, service: &str) {
        let unit_path = format!("{}/{}.service", self.services_path, service);
        fs::remove_file(unit_path).expect("Failed to remove unit file");
    }
}
