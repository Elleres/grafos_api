pub mod grafo_lib;

use grafo_lib::grafo::Grafo;

fn main() { 


  // let mut matriz = Vector::new();

  // let mut grafo_matriz_instancia = GrafoMatriz {
  //   matriz: matriz,
  // };

  let mut grafo_matriz_instancia = Grafo::new();

  // grafo_matriz_instancia.criar_matriz(4);

  // grafo_matriz_instancia.print_grafo_matriz();

  grafo_matriz_instancia.criar_matriz();

  grafo_matriz_instancia.inserir_vertice(1);
  grafo_matriz_instancia.inserir_vertice(2);
  grafo_matriz_instancia.inserir_vertice(3);


  grafo_matriz_instancia.inserir_aresta((1,3));

  grafo_matriz_instancia.inserir_aresta((2,2));
  grafo_matriz_instancia.inserir_aresta((2,1));
  grafo_matriz_instancia.inserir_aresta((2,3));
  
  
  grafo_matriz_instancia.inserir_aresta((3,2));
  grafo_matriz_instancia.inserir_aresta((3,3));

  grafo_matriz_instancia.print_arestas();

  grafo_matriz_instancia.criar_matriz();
  
  // grafo_matriz_instancia.print_grafo_matriz();

}