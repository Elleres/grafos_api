pub mod grafo_lib;

use grafo_lib::grafo::Grafo;

fn main() { 
  // Cria instancia grafo
  let mut grafo_matriz_instancia = Grafo::new(false);

  // insere vertices
  grafo_matriz_instancia.inserir_vertice(1);
  grafo_matriz_instancia.inserir_vertice(2);
  grafo_matriz_instancia.inserir_vertice(3);

  // insere arestas
  grafo_matriz_instancia.inserir_aresta((1,3));

  grafo_matriz_instancia.inserir_aresta((2,2));
  grafo_matriz_instancia.inserir_aresta((2,1));
  grafo_matriz_instancia.inserir_aresta((2,3));
  
  
  grafo_matriz_instancia.inserir_aresta((3,2));
  grafo_matriz_instancia.inserir_aresta((3,3));


  // printa lista de adjacencias
  grafo_matriz_instancia.print_arestas();

  // cria matriz de adjacencias
  grafo_matriz_instancia.criar_matriz();
  
  // printa matriz
  grafo_matriz_instancia.print_grafo_matriz();

}