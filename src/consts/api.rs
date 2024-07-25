#[derive(PartialEq)]
#[allow(dead_code)] // we are LITERALLY just using this for verbosity.
pub enum Endpoints {
    UserLogin,
    UserLogout
}

const ENDPOINTS: &[(Endpoints, &str)] = &[
    (Endpoints::UserLogin, "https://blacket.org/worker/login"),
    (Endpoints::UserLogout, "https://blacket.org/worker/logout")
];

pub fn get_endpoint(endpoint: Endpoints) -> String {
    ENDPOINTS.iter().find(|(e, _)| e == &endpoint).unwrap().1.to_string()
}