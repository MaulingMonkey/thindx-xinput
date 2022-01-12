use mmrbi::fs::*;
use std::io::{self, Write};

const URLS : &'static [(&'static str, &'static str)] = &[
    ("open repository",     "https://github.com/MaulingMonkey/thindx"),
    ("open documentation",  "https://docs.rs/thindx"),
];

const COMMANDS : &'static [(&'static str, &'static str)] = &[
    ("fetch",               "cargo fetch"),
    ("check",               "cargo c"),
    ("check-file",          "cargo c ${relativeFile}"),
    ("test",                "cargo t"),
    ("build",               "cargo b"),
    ("doc",                 "cargo d"),
    ("help",                "cargo h"),
    ("vsc",                 "cargo vsc"),
    ("push github master",  "git push github master"),
];

pub fn vsc(_args: std::env::Args) {
    write_if_modified_with(".vscode/launch.json", |j| launch_json(j)).expect("unable to write .vscode/launch.json");
    write_if_modified_with(".vscode/tasks.json",  |j| tasks_json(j) ).expect("unable to write .vscode/tasks.json");
}



fn launch_json(mut j: impl Write) -> io::Result<()> {
    writeln!(j, "// WARNING: autogenerated by cargo xtask vsc, will be overwritten!")?;
    writeln!(j, "{{")?;
    writeln!(j, "    \"version\": \"0.2.0\",")?;
    writeln!(j, "    \"configurations\": [")?;
    for example in thindx_examples() {
        let external_console = external_console(&example);
        for (config, extra) in [
            ("debug",   ""),
            ("release", " --release"),
        ].iter().copied() {
            let name = format!("{} • {}", example, config);
            let task = format!("cargo b --package thindx --example {}{}", example, extra);
            writeln!(j, "        {{")?;
            writeln!(j, "            \"name\":                     {:?},", name)?;
            writeln!(j, "            \"type\":                     \"cppdbg\",")?;
            writeln!(j, "            \"request\":                  \"launch\",")?;
            if external_console {
                writeln!(j, "            \"externalConsole\":          true,")?;
            } else {
                writeln!(j, "            \"internalConsoleOptions\":   \"openOnSessionStart\",")?;
            }
            writeln!(j, "            \"preLaunchTask\":            {:?},", task)?;
            writeln!(j, "            \"program\":                  {:?},", format!("${{workspaceFolder}}/target/{}/examples/{}", config, example))?;
            writeln!(j, "            \"cwd\":                      \"${{workspaceFolder}}\",")?;
            writeln!(j, "            \"environment\":              [ {{ \"name\": \"RUST_BACKTRACE\", \"value\": \"1\" }} ],")?;
            writeln!(j, "            \"windows\": {{")?;
            writeln!(j, "                \"type\":                     \"cppvsdbg\",")?;
            writeln!(j, "                \"program\":                  \"${{workspaceFolder}}/target/{}/examples/{}.exe\",", config, example)?;
            writeln!(j, "                \"enableDebugHeap\":          {},", config == "debug")?;
            writeln!(j, "            }}")?;
            writeln!(j, "        }},")?;
        }
    }
    writeln!(j, "    ]")?;
    writeln!(j, "}}")?;
    Ok(())
}

fn tasks_json(mut j: impl Write) -> io::Result<()> {
    writeln!(j, "// WARNING: autogenerated by cargo xtask vsc, will be overwritten!")?;
    writeln!(j, "{{")?;
    writeln!(j, "    \"version\": \"2.0.0\",")?;
    writeln!(j, "    \"problemMatcher\": \"$rustc\",")?;
    writeln!(j, "    \"options\": {{")?;
    writeln!(j, "        \"env\": {{")?;
    writeln!(j, "            \"RUSTC_LOG\":    \"rustc_metadata=error\",")?;
    writeln!(j, "            \"TESTFAST\":     \"1\",")?;
    writeln!(j, "        }}")?;
    writeln!(j, "    }},")?;
    writeln!(j, "    \"type\":             \"shell\",")?;
    writeln!(j, "    \"presentation\":     {{ \"clear\": true, \"reveal\": \"always\" }},")?;
    writeln!(j, "    \"tasks\": [")?;
    writeln!(j, "        // entry points")?;
    writeln!(j, "        {{")?;
    writeln!(j, "            \"label\":            \"default-build\",")?;
    writeln!(j, "            \"dependsOrder\":     \"sequence\",")?;
    writeln!(j, "            \"dependsOn\":        [ \"build\" ],")?;
    writeln!(j, "            \"group\":            {{ \"kind\": \"build\", \"isDefault\": true }}")?;
    writeln!(j, "        }},")?;
    for (label, command) in COMMANDS.iter().copied() {
        writeln!(j, "        {{")?;
        writeln!(j, "            \"label\":   {:?},", label)?;
        writeln!(j, "            \"command\": {:?},", command)?;
        writeln!(j, "        }},")?;
    }
    for example in thindx_examples() {
        for extra in ["", " --release"].iter().copied() {
            let cmd = format!("cargo b --package thindx --example {}{}", example, extra);
            writeln!(j, "        {{")?;
            writeln!(j, "            \"label\":   {:?},", cmd)?;
            writeln!(j, "            \"command\": {:?},", cmd)?;
            writeln!(j, "        }},")?;
        }
        // TODO: renderdoc launch variants?
    }
    for (label, url) in URLS.iter().copied() {
        writeln!(j, "        {{")?;
        writeln!(j, "            \"label\":        {:?},", label)?;
        writeln!(j, "            \"windows\":      {{ \"command\": \"start \\\"\\\"    \\\"{}\\\"\" }},", url)?;
        writeln!(j, "            \"linux\":        {{ \"command\": \"xdg-open      \\\"{}\\\"\" }},", url)?;
        writeln!(j, "            \"osx\":          {{ \"command\": \"open          \\\"{}\\\"\" }},", url)?;
        writeln!(j, "            \"presentation\": {{ \"clear\": true, \"panel\": \"shared\", \"reveal\": \"silent\" }},")?;
        writeln!(j, "        }},")?;
    }
    writeln!(j, "    ]")?;
    writeln!(j, "}}")?;
    Ok(())
}

fn thindx_examples() -> impl Iterator<Item = String> {
    let mut thindx_examples : Vec<DirNameType> = collect_dir("thindx/examples").expect("unable to read thindx/examples");
    thindx_examples.retain(|f| f.file_type.is_file());
    thindx_examples.sort_by(|a, b| a.file_name.cmp(&b.file_name));
    thindx_examples.into_iter().filter_map(|e| Some(e.file_name.to_str()?.strip_suffix(".rs")?.to_string()))
}

fn external_console(example: &str) -> bool {
    example.starts_with("xinput-")
}
