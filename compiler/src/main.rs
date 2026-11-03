//! This project's rbxtsc-fw: rbxts-rs's command line (`rbxts_cli`) with Flamework (`flamework_rs`) and any other
//! plugins of the project compiled in. Flamework is on for a project whose tsconfig lists
//! `@flamework-experimental/transformer` in `compilerOptions.plugins`.

#[global_allocator]
static GLOBAL: mimalloc_safe::MiMalloc = mimalloc_safe::MiMalloc;

fn main() -> std::process::ExitCode {
    rbxts_cli::Cli::new()
        .plugin(flamework_rs::Flamework::new())
        // .plugin(my_plugin::MyPlugin::new())
        .run()
}
