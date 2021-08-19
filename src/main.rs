#[macro_use]
extern crate clap;
mod ast;
mod dot;
mod graph;
mod parser;

// #[cfg_attr(any(target_arch = "wasm32", target_arch="wasm64"), path = "tree_sitter_wasm.rs")]
#[path = "tree_sitter_native.rs"]
mod treesitter;

fn main() -> anyhow::Result<()> {
    let matches = clap_app!(cxx2flow =>
        (version: "0.1.3")
        (author: "mgt. <mgt@oi-wiki.org>")
        (about: "Convert your C/C++ code to control flow chart")
        (@arg OUTPUT: -o --output +takes_value "Sets the output file.
If not specified, result will be directed to stdout.
e.g. graph.dot")
        (@arg curved: -c --curved "Sets the style of the flow chart.
If specified, output flow chart will have curved connection line.")
        (@arg INPUT: +required "Sets the input file. e.g. test.cpp")
        (@arg FUNCTION: "The function you want to convert. e.g. main")
    )
    .after_help("Note that you need to manually compile the dot file using graphviz to get SVG or PNG files.
EXAMPLES:
    cxx2flow test.cpp | dot -Tpng -o test.png
    cxx2flow main.cpp my_custom_func | dot -Tsvg -o test.svg")
    .setting(clap::AppSettings::ColoredHelp)
    .get_matches();
    let path = matches.value_of("INPUT").unwrap();
    let func = matches.value_of("FUNCTION").map(|x| x.to_string());
    let output = matches.value_of("OUTPUT");
    let curved = matches.is_present("curved");
    let content = std::fs::read(path)?;
    let ast_vec = parser::parse(&content, func)?;
    let graph = graph::from_ast(ast_vec)?;
    let dot = dot::from_graph(&graph, curved)?;
    if let Some(output) = output {
        std::fs::write(output, dot)?;
    } else {
        print!("{}", dot);
    }
    Ok(())
}
