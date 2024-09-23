pub mod grafo_lib;
// pub mod produto_cartesiano;
pub mod dfs;

use grafo_lib::grafo::Grafo;


fn main() {
  // Criando um grafo direcionado
  let mut grafo = Grafo::new(true);

  // Adicionando vértices
  grafo.inserir_vertice("A".to_string());
  grafo.inserir_vertice("B".to_string());
  grafo.inserir_vertice("C".to_string());
  grafo.inserir_vertice("D".to_string());
  grafo.inserir_vertice("E".to_string());

  // Adicionando arestas
  grafo.inserir_aresta(("A".to_string(), "B".to_string()));
  grafo.inserir_aresta(("A".to_string(), "C".to_string()));
  grafo.inserir_aresta(("B".to_string(), "D".to_string()));
  grafo.inserir_aresta(("C".to_string(), "D".to_string()));
  grafo.inserir_aresta(("D".to_string(), "E".to_string()));

  // Exibindo os vértices e as arestas antes da busca
  println!("Lista de vértices:");
  grafo.print_vertices();

  println!("\nLista de adjacências:");
  grafo.print_arestas();

  // Executando o DFS
  println!("\nExecutando DFS...");
  grafo.dfs();

  // Exibindo os tempos de descoberta e término após o DFS
  println!("\nTempos de descoberta e término:");
  for (nome, vertice) in &grafo.vertices {
      println!(
          "Vértice {}: Descoberta = {}, Término = {}",
          nome, vertice.tempo_descoberta, vertice.tempo_termino
      );
  }
}