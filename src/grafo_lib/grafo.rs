use std::collections::{BTreeMap, HashSet};

pub struct Grafo{
    // Struct de um Grafo que recebe inteiros como vertices e arestas como tupla
    // inteiros representando as duas arestas da conexao.

    // O atributo arestas é a implementação de lista de adjacencias
    // O atributo matriz é a implementacao de uma matriz de adjacencia
    // O atributo matriz_acess é apenas para realizar o acesso na listas de forma correta
    // O atributo direcionado define se o grafo é direcionada ou não
    pub vertices: Vec<String>,
    pub arestas: BTreeMap<String, HashSet<String>>,
    pub matriz: Vec<Vec<bool>>,
    pub matriz_acess: BTreeMap<String, usize>,
    pub direcionado: bool
}

impl Grafo  {
    // Função constructor, aloca na memória espaco
    pub fn new(dir: bool) -> Self{
        Grafo {
            vertices: Vec::new(), 
            matriz: Vec::new(),
            arestas: BTreeMap::new(),
            matriz_acess: BTreeMap::new(),
            direcionado: dir
        }
    }
    
    // Função insere vertice com valor inteiro na lista de vertices e na de arestas
    pub fn inserir_vertice(&mut self, vert: String){
        self.vertices.push(vert.clone());
        self.arestas.insert(vert.clone(), HashSet::new());
    }

    // Funcao insere aresta no hashmap de arestas
    pub fn inserir_aresta(&mut self, aresta: (String, String)){
        // Bloco abaixo checa se os dois vertices já foram inseridos na lista de vertices
        // caso contrário, irá adicionar os que faltam.
        if !self.vertices.contains(&aresta.1.clone()){
            println!("Vertice {} foi inserido para criar a aresta.", aresta.1);
            self.inserir_vertice(aresta.1.clone());
        }
        if !self.vertices.contains(&aresta.0){
            println!("Vertice {} foi inserido para criar a aresta.", aresta.0);
            self.inserir_vertice(aresta.0.clone());
        }

        // Verifica se a aresta 0 já está dentro da lista de arestas, caso não esteja
        // irá inseri-la
        let lista_arestas = self.arestas
        .entry(aresta.0.clone())
        .or_insert_with(HashSet::new);


        // Verifica se a lista de arestas contem a segunda aresta, caso já tenha, irá apenas
        // avisar que já tem
        if lista_arestas.contains(&aresta.1) {
            println!("O vertice {} já tem conexão com o vertice {}", aresta.0, aresta.1);
        }else{
            lista_arestas.insert(aresta.1.clone());

            // Se o grafo for não direcionado irá adicionar a conexão na lista dos dois vértices
            if self.direcionado == false {
                let lista_arestas_v2 = self.arestas
                .entry(aresta.1.clone())
                .or_insert_with(HashSet::new);

                lista_arestas_v2.insert(aresta.0.clone());
            }
        }
    }

    // Cria um vetor de vetores contendo bool, true representa se a aresta existe
    // false a aresta não existe. A função printa a posição de cada vertice nas listas
    pub fn criar_matriz(&mut self) {

        let mut cont: usize = 0;
        // Cria um hashmap pra identificar a posicao dos vertices
        for item in self.arestas.iter(){
            self.matriz_acess.insert(item.0.to_string(), cont);
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