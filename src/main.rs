pub mod grafo_lib;
// pub mod produto_cartesiano;
pub mod dfs;

use grafo_lib::grafo::Grafo;


fn main() {
  let mut grafo = Grafo::new(true);  // Grafo direcionado

  grafo.inserir_vertice("A".to_string());
  grafo.inserir_vertice("B".to_string());
  grafo.inserir_vertice("C".to_string());
  grafo.inserir_vertice("D".to_string());

  grafo.inserir_aresta(("A".to_string(), "B".to_string()));
  grafo.inserir_aresta(("B".to_string(), "C".to_string()));
  grafo.inserir_aresta(("C".to_string(), "A".to_string()));  // Cria um ciclo
  grafo.inserir_aresta(("C".to_string(), "D".to_string()));

  println!("Grafo contém ciclo? {}", grafo.contem_ciclo());
}