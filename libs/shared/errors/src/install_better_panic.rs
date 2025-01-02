#[allow(dead_code)]
pub fn install_better_panic() {
    better_panic::Settings::auto()
        .most_recent_first(false)
        .verbosity(better_panic::Verbosity::Full)
        .install()
}
