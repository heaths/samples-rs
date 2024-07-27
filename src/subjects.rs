use std::{
    env::{self, VarError},
    fmt,
};

#[derive(Clone, Debug)]
pub struct CurrentUser {
    username: String,
}

impl CurrentUser {
    pub fn try_new() -> Result<Self, VarError> {
        let username = env::var("USER").or_else(|_| env::var("USERNAME"))?;
        if username.is_empty() {
            return Err(VarError::NotPresent);
        }

        Ok(Self { username })
    }
}

impl fmt::Display for CurrentUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.username)
    }
}

#[test]
fn hello_current_user() {
    let _var = crate::test::EnvVar::new("USER", "samplesaurus");
    let user = CurrentUser::try_new().expect("expected USER");
    assert_eq!("hello, samplesaurus!", &crate::say_hello(user));
}
