use tabled::Table;

#[derive(clap::Parser)]
pub struct List {
    #[arg(short, long, help = "Show list in raw format", default_value = "false")]
    raw: bool,
}

impl List {
    pub fn execute(&self, systemd: crate::systemd::Systemd) {
        let services = systemd.get_services();

        if self.raw {
            for service in services {
                println!(
                    "{} {} {} {}",
                    service.name, service.is_active, service.is_enabled, service.memory,
                );
            }
        } else {
            println!("{}", Table::new(&services).to_string());
        }
    }
}
