pub mod grafo_lib;
pub mod produto_cartesiano;

use grafo_lib::grafo::Grafo;
use produto_cartesiano::produto_cartesiano;

fn main() { 
  // Cria instancia grafo
  let mut grafo_1 = Grafo::new(false);
  let mut grafo_2 = Grafo::new(false);

  // insere vertices
  grafo_1.inserir_vertice(String::from("1"));  
  grafo_1.inserir_vertice(String::from("2"));  

  grafo_2.inserir_vertice(String::from("1"));  
  grafo_2.inserir_vertice(String::from("2"));  
  grafo_2.inserir_vertice(String::from("3"));  

  //insere arestas
  grafo_1.inserir_aresta((String::from("1"),String::from("2")));

  grafo_2.inserir_aresta((String::from("1"),String::from("2")));
  grafo_2.inserir_aresta((String::from("1"),String::from("3")));
  grafo_2.inserir_aresta((String::from("2"),String::from("3")));
  



  // printa lista de adjacencias
  println!("Grafo 1:");
  grafo_1.print_arestas();
  println!("Grafo 2:");
  grafo_2.print_arestas();

  // cria matriz de adjacencias
  // grafo_matriz_instancia.criar_matriz();
  
  // printa matriz
  // grafo_matriz_instancia.print_grafo_matriz();

}