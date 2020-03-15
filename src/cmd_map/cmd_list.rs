use super::*;

pub fn get_cmd_list() -> Vec<CommandInfo> {
    vec![
        linux_essentials::SH,
        linux_essentials::BASH,
        linux_essentials::ZSH,
        linux_essentials::FISH,
        linux_essentials::CURL,
        linux_essentials::WGET,
        linux_essentials::SCREEN,
        linux_essentials::OPENSSL,
        dev_essentials::GIT,
        dev_essentials::GCC,
        dev_essentials::GPP,
        dev_essentials::CLANG,
        dev_essentials::MAKE,
        dev_essentials::CMAKE,
        editor::NANO,
        editor::VI,
        editor::VIM,
        media::FFMPEG,
        media::IMAGEMAGIC_CONVERT,
        media::IMAGEMAGIC_COMPOSITE,
        rust::RUSTC,
        rust::RUSTFMT,
        rust::RUSTDOC,
        rust::CARGO,
        go::GO,
        go::GOENV,
        nodejs::NODE,
        nodejs::NODEJS,
        nodejs::NPM,
        nodejs::NPX,
        nodejs::YARN,
        alt_js::TSC,
        alt_js::ELM,
        python::PYTHON,
        python::PYTHON2,
        python::PYTHON3,
        python::PIP,
        python::PIP2,
        python::PIP3,
        ruby::RUBY,
        ruby::GEM,
        ruby::BUNDLE,
        jvm::JAVA,
        jvm::JAVAC,
        jvm::KOTLINC,
        jvm::SCALAC,
        container::DOCKER,
        container::DOCKER_COMPOSE,
        container::KUBECTL,
        database::MYSQL,
        database::PSQL,
        database::SQLITE3,
        database::MONGO,
        database::REDIS,
        gcloud::GCLOUD,
        gcloud::BQ,
        gcloud::GSUTIL,
        firebase::FIREBASE,
    ]
}
