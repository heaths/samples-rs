use std::env;

#[derive(Clone, Debug)]
pub struct EnvVar(&'static str, Option<String>);

impl EnvVar {
    pub fn new(name: &'static str, value: &'static str) -> Self {
        let orig = env::var(name).ok();
        env::set_var(name, value);

        Self(name, orig)
    }
}

impl Drop for EnvVar {
    fn drop(&mut self) {
        if let Some(value) = &self.1 {
            env::set_var(self.0, value);
            return;
        }

        env::remove_var(self.0);
    }
}
