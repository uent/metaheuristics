use clap::{Parser, Subcommand};

mod problems;

#[derive(Parser)]
#[command(name = "metaheuristics")]
#[command(about = "Ejemplo de uso de metaheuristicas", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Polinomio {
    },
    PolinomioProblemaReal {
    },
    OptimizaEspacioDosD {
        #[arg(short, long)]
        numero: i32,
    },
}


fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Polinomio { }) => {
            println!("Buscando mejor solucion al polinomio {}", problems::polynomial::polynomial_text());
            problems::polynomial::search_optimal();
        }
        Some(Commands::PolinomioProblemaReal { }) => {
            println!("Buscando mejor solucion al polinomio {}", problems::polynomial_real_problem::polynomial_text());
            println!("restricciones: x pertenece al conjunto {} e y pertenece a {}", problems::polynomial_real_problem::x_range_text(), problems::polynomial_real_problem::y_range_text());
            problems::polynomial_real_problem::search_optimal();
        }
        Some(Commands::OptimizaEspacioDosD { numero }) => {
            println!("buscando como optimizar el espacio para {} figuras geometricas 2d.", numero);
            problems::optimize_space_2d::search_optimal();
        }
        None => {
            println!("Error: No se reconoció el comando");
            std::process::exit(1);
        }
    }
}