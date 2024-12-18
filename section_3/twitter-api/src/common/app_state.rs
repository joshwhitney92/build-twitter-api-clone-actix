// NOTE: The appstate type is generic - so we can swap out the 
// implementation of the repository type if we want.
// It does not always have to be the DBRepo
pub struct AppState<T> {
    pub client: reqwest::Client,
    pub db_repo: T,
}
