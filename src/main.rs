pub mod grafo_lib;
// pub mod produto_cartesiano;
pub mod dfs;

use grafo_lib::grafo::Grafo;


fn main() {
  let mut grafo = Grafo::new(true);

  grafo.inserir_vertice("A".to_string());
  grafo.inserir_vertice("B".to_string());
  grafo.inserir_vertice("C".to_string());
  grafo.inserir_vertice("D".to_string());
  grafo.inserir_vertice("E".to_string());

  grafo.inserir_aresta(("A".to_string(), "C".to_string()));
  grafo.inserir_aresta(("B".to_string(), "C".to_string()));
  grafo.inserir_aresta(("C".to_string(), "D".to_string()));
  grafo.inserir_aresta(("D".to_string(), "E".to_string()));

  let ordem_topologica = grafo.ordenacao_topologica();
  grafo.dfs();

  println!("Ordenação Topológica: {:?}", ordem_topologica);
}