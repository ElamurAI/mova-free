//! `mova`: the user-facing entry for Mova's add-on modules (`mova modules …`, docs/modules.md) and for a release's
//! files (`mova fetch …`, `mova_modules::fetch`) and for Open English WordNet (`mova oewn …`, `mova_modules::oewn`).

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let r = match args.first().map(String::as_str) {
        Some("modules") => mova_modules::main_args(&args[1..]),
        Some("fetch") => mova_modules::fetch::main_args(&args[1..]),
        Some("oewn") => mova_modules::oewn::main_args(&args[1..]),
        _ => Err(anyhow::anyhow!("usage: mova modules list|info|verify|install|install-torrent|make-torrent|remove … (docs/modules.md) | mova fetch [RELEASE] [ID…|all] [--via https|torrent|magnet] … (releases/0.5-files.md) | mova oewn install|check|inventory … (Open English WordNet)")),
    };
    if let Err(e) = r {
        eprintln!("mova: {e:#}");
        std::process::exit(1);
    }
}
