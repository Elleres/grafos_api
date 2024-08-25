use std::collections::{BTreeMap, HashSet};


pub struct Grafo{
    // Struct de um Grafo que recebe inteiros como vertices e arestas como tupla
    // inteiros representando as duas arestas da conexao.

    // O atributo arestas é a implementação de lista de adjacencias
    // O atributo matriz é a implementacao de uma matriz de adjacencia
    // O atributo matriz_acess é apenas para realizar o acesso na listas de forma correta
    pub vertices: Vec<i32>,
    pub arestas: BTreeMap<i32, HashSet<i32>>,
    pub matriz: Vec<Vec<bool>>,
    pub matriz_acess: BTreeMap<i32, i32>
}

impl Grafo  {
    // Função constructor, aloca na memória espaco
    pub fn new() -> Self{
        Grafo {
            vertices: Vec::new(), 
            matriz: Vec::new(),
            arestas: BTreeMap::new(),
            matriz_acess: BTreeMap::new()
        }
    }
    
    // Função insere vertice com valor inteiro na lista de vertices e na de arestas
    pub fn inserir_vertice(&mut self, vert: i32){
        self.vertices.push(vert);
        self.arestas.insert(vert, HashSet::new());
    }

    // Funcao insere aresta no hashmap de arestas
    pub fn inserir_aresta(&mut self, aresta: (i32, i32)){
        // Bloco abaixo checa se os dois vertices já foram inseridos na lista de vertices
        // caso contrário, irá adicionar os que faltam.
        if !self.vertices.contains(&aresta.1){
            println!("Vertice {} foi inserido para criar a aresta.", aresta.1);
            self.inserir_vertice(aresta.1);
        }
        if !self.vertices.contains(&aresta.0){
            println!("Vertice {} foi inserido para criar a aresta.", aresta.0);
            self.inserir_vertice(aresta.0);
        }

        // Verifica se a aresta 0 já está dentro da lista de arestas, caso não esteja
        // irá inseri-la
        let lista_arestas = self.arestas
        .entry(aresta.0)
        .or_insert_with(HashSet::new);


        // Verifica se a lista de arestas contem a segunda aresta, caso já tenha, irá apenas
        // avisar que já tem
        if lista_arestas.contains(&aresta.1) {
            println!("O vertice {} já tem conexão com o vertice {}", aresta.0, aresta.1);
        }else {
            lista_arestas.insert(aresta.1);
        }
    }

    // Cria um vetor de vetores contendo bool, true representa se a aresta existe
    // false a aresta não existe. A função printa a posição de cada vertice nas listas
    pub fn criar_matriz(&mut self) {

        let mut cont: i32 = 0;
        // Cria um hashmap pra identificar a posicao dos vertices
        for item in self.arestas.iter(){
            self.matriz_acess.insert(*item.0, cont);
            cont += 1;
        }

        // Se o vertice2 estiver na lista de arestas do vetor1 irá adicionar na matriz
        // o valor true
        for item in self.arestas.iter(){
            let mut linha= vec![false; self.vertices.len()];
            for vertice2 in item.1.iter(){
                let index_atual = self.matriz_acess[vertice2];
                linha[index_atual as usize] = true;
            }
            self.matriz.push(linha);
        }

        // Printa a posicao dos vertices
        for i in self.matriz_acess.iter(){
            println!("O elemento {} esta na linha e coluna: {}", i.0, i.1);
        }
    }

    // Retorna o número de vértices do grafo
    pub fn tamanho(&mut self) -> usize{
        let numero_vertices: usize = self.vertices.len();
        return numero_vertices;
    }

    // Printa o grafo em formato de matriz
    pub fn print_grafo_matriz(&mut self){
        for row in self.matriz.iter(){
            for col in row{
                print!("{col} ")
            }
        println!()
        }
    }

    // Printa a lista encadeada contendo os vertices e suas arestas
    pub fn print_arestas(&mut self){
        for aresta in self.arestas.iter(){
            print!("{} -> ", aresta.0);
            for aresta_2 in aresta.1.iter(){
                print!("{aresta_2} -> ")
            }
            println!("None")
        }
    }

    // Printa o número de vértices
    pub fn print_vertices(&mut self){
        print!("Lista de vertices: ");
        for vertice in self.vertices.iter(){
            print!("{vertice} ")
        }
        println!()
    }
}