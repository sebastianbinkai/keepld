use std::env;
use std::path::PathBuf;

use keepld::application::node::create_node;
use keepld::application::project::{
    create_project,
    initialize_project,
};
use keepld::application::query::{
    execute,
    Query,
    QueryDetail,
    QueryResult,
};

fn print_result(result: QueryResult, detail: QueryDetail) {
    match result {
        QueryResult::Project(project) => {
            println!("Project");
            println!("  id: {}", project.id);
            println!("  name: {}", project.name);

            if detail == QueryDetail::Verbose {
                println!("  description: {}", project.description);
                println!("  nodes: {}", project.nodes.len());
            }
        }

        QueryResult::Node(node) => {
            println!("Node");
            println!("  id: {}", node.id);
            println!("  name: {}", node.name);

            if detail == QueryDetail::Verbose {
                println!("  description: {}", node.description);
                println!("  availability: {:?}", node.availability);
                println!("  kind: {:?}", node.kind);
                println!("  state: {:?}", node.state);
            }
        }
    }
}

fn usage() {
    eprintln!("Usage:");
    eprintln!("  keepld init");
    eprintln!("  keepld create project <name>");
    eprintln!("  keepld create <name>");
    eprintln!("  keepld show");
    eprintln!("  keepld show -v");
    eprintln!("  keepld show -f <position>");
}

fn main() {
    let mut args = env::args().skip(1);

    let Some(command) = args.next() else {
        usage();
        return;
    };

    let result = match command.as_str() {
        // Inicializa el directorio actual.
        "init" => {
            if args.next().is_some() {
                eprintln!("error: 'init' does not accept arguments");
                usage();
                std::process::exit(1);
            }

            let path = PathBuf::from(".");
            let name = path
                .canonicalize()
                .ok()
                .and_then(|p| p.file_name().map(|n| n.to_string_lossy().into_owned()))
                .unwrap_or_else(|| String::from("project"));

            initialize_project(
                &path,
                1,
                name,
                String::new(),
            )
            .map(|_| ())
        }

        // create project <name>
        // Crea el directorio e inicializa el proyecto.
        "create" => {
            let Some(target) = args.next() else {
                eprintln!("error: missing create target");
                usage();
                std::process::exit(1);
            };

            if target == "project" {
                let Some(name) = args.next() else {
                    eprintln!("error: missing project name");
                    usage();
                    std::process::exit(1);
                };

                if args.next().is_some() {
                    eprintln!("error: too many arguments");
                    usage();
                    std::process::exit(1);
                }

                let path = PathBuf::from(&name);

                create_project(
                    &path,
                    1,
                    name,
                    String::new(),
                )
                .map(|_| ())
            } else {
                // create <name>
                // Crea un Node dentro del proyecto actual.
                if args.next().is_some() {
                    eprintln!("error: too many arguments");
                    usage();
                    std::process::exit(1);
                }

                let path = PathBuf::from(".");

                create_node(
                    &path,
                    1,
                    target,
                    String::new(),
                )
                .map(|_| ())
            }
        }

        "show" => {
            let mut query = Query::current();

            while let Some(argument) = args.next() {
                match argument.as_str() {
                    "-v" => {
                        query.detail = QueryDetail::Verbose;
                    }

                    "-f" => {
                        let Some(position) = args.next() else {
                            eprintln!("error: missing focus position");
                            usage();
                            std::process::exit(1);
                        };

                        let position: u8 = match position.parse() {
                            Ok(position) => position,
                            Err(_) => {
                                eprintln!("error: invalid focus position '{}'", position);
                                std::process::exit(1);
                            }
                        };

                        query.target = keepld::application::query::QueryTarget::Focus(position);
                    }

                    _ => {
                        eprintln!("error: unknown show option '{}'", argument);
                        usage();
                        std::process::exit(1);
                    }
                }
            }

            let path = PathBuf::from(".");

            execute(query.clone(), &path)
                .map(|result| {
                    print_result(result, query.detail);
                })
        }

        _ => {
            eprintln!("error: unknown command '{}'", command);
            usage();
            std::process::exit(1);
        }
    };

    if let Err(error) = result {
        eprintln!("error: {}", error);
        std::process::exit(1);
    }
}
