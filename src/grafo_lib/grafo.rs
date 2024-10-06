use std::collections::{BTreeMap, HashSet};

pub struct Vertice {
    pub nome: String,           // Nome do vértice
    pub cor: String,            // Cor usada no DFS (WHITE, GRAY, BLACK)
    pub tempo_descoberta: i32,  // Tempo de descoberta
    pub tempo_termino: i32,     // Tempo de término
    pub predecessor: Option<String>, // Predecessor no DFS
}

impl Vertice {
    pub fn new(nome: String) -> Self {
        Vertice {
            nome,
            cor: "WHITE".to_string(),  // Inicialmente todos os vértices são não visitados
            tempo_descoberta: -1,
            tempo_termino: -1,
            predecessor: None,
        }
    }
}

pub struct Grafo{
    // Struct de um Grafo que recebe inteiros como vertices e arestas como tupla
    // inteiros representando as duas arestas da conexao.

    // O atributo arestas é a implementação de lista de adjacencias
    // O atributo matriz é a implementacao de uma matriz de adjacencia
    // O atributo matriz_acess é apenas para realizar o acesso na listas de forma correta
    // O atributo direcionado define se o grafo é direcionada ou não
    pub vertices: BTreeMap<String, Vertice>,
    pub arestas: BTreeMap<String, HashSet<String>>,
    pub matriz: Vec<Vec<bool>>,
    pub matriz_acess: BTreeMap<String, usize>,
    pub direcionado: bool
}

impl Grafo  {
    // Função constructor, aloca na memória espaco
    pub fn new(dir: bool) -> Self{
        Grafo {
            vertices: BTreeMap::new(), 
            matriz: Vec::new(),
            arestas: BTreeMap::new(),
            matriz_acess: BTreeMap::new(),
            direcionado: dir
        }
    }
    
    // Função insere vertice com valor inteiro na lista de vertices e na de arestas
    pub fn inserir_vertice(&mut self, vert: String){
        let vertice = Vertice::new(vert.clone());
        self.vertices.insert(vert.clone(), vertice);
        self.arestas.insert(vert.clone(), HashSet::new());
    }

    // Funcao insere aresta no hashmap de arestas
    pub fn inserir_aresta(&mut self, aresta: (String, String)){
        // Bloco abaixo checa se os dois vertices já foram inseridos na lista de vertices
        // caso contrário, irá adicionar os que faltam.
        if !self.vertices.contains_key(&aresta.1.clone()){
            println!("Vertice {} foi inserido para criar a aresta.", aresta.1);
            self.inserir_vertice(aresta.1.clone());
        }
        if !self.vertices.contains_key(&aresta.0){
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
        for vertice in self.vertices.values(){
            print!("{} ", vertice.nome)
        }
        println!()
    }
    
    pub fn dfs(&mut self) {
        let mut tempo = 0;

        // Inicializa todos os vértices como não visitados (WHITE)
        for vertice in self.vertices.values_mut() {
            vertice.cor = "WHITE".to_string();
            vertice.predecessor = None;
        }

        // Coletar as chaves dos vértices em uma lista para evitar o empréstimo imutável do mapa
        let vertices_keys: Vec<String> = self.vertices.keys().cloned().collect();

        // Para cada vértice ainda não visitado, chama DFS-Visit
        for nome in vertices_keys {
            if self.vertices[&nome].cor == "WHITE" {
                self.dfs_visit(nome, &mut tempo);
            }
        }
    }

    // Função auxiliar DFS-Visit
    fn dfs_visit(&mut self, u: String, tempo: &mut i32) {
        *tempo += 1;

        {
            let vertice_u = self.vertices.get_mut(&u).unwrap();
            vertice_u.tempo_descoberta = *tempo;
            vertice_u.cor = "GRAY".to_string();
        }

        // Clonar a lista de adjacências para evitar múltiplos empréstimos mutáveis
        let adjacentes: Vec<String> = self.arestas[&u].iter().cloned().collect();

        // Explora todos os vértices adjacentes
        for v in adjacentes {
            let cor_v;
            {
                // Faz um empréstimo imutável temporário apenas para verificar a cor do vértice v
                cor_v = self.vertices.get(&v).unwrap().cor.clone();
            }

            if cor_v == "WHITE" {
                {
                    // Faz um empréstimo mutável temporário para modificar o predecessor
                    let vertice_v = self.vertices.get_mut(&v).unwrap();
                    vertice_v.predecessor = Some(u.clone());
                }
                // Chama recursivamente o DFS-Visit
                self.dfs_visit(v, tempo);
            }
        }

        {
            let vertice_u = self.vertices.get_mut(&u).unwrap();
            vertice_u.cor = "BLACK".to_string();
            *tempo += 1;
            vertice_u.tempo_termino = *tempo;
        }
    }
    pub fn contem_ciclo(&mut self) -> bool {
        // Inicializa todos os vértices como não visitados (WHITE)
        for vertice in self.vertices.values_mut() {
            vertice.cor = "WHITE".to_string();
        }

        // Coleta os nomes dos vértices
        let vertices_keys: Vec<String> = self.vertices.keys().cloned().collect();

        // Para cada vértice ainda não visitado, chama DFS-Visit para verificar ciclos
        for nome in vertices_keys {
            if self.vertices[&nome].cor == "WHITE" {
                if self.dfs_ciclo(nome.clone()) {
                    return true;  // Se um ciclo for encontrado, retorna true
                }
            }
        }

        false  // Nenhum ciclo encontrado
    }

    fn dfs_ciclo(&mut self, u: String) -> bool {
        {
            let vertice_u = self.vertices.get_mut(&u).unwrap();
            vertice_u.cor = "GRAY".to_string();  // Marca como em exploração
        }

        // Clonar a lista de adjacências para evitar múltiplos empréstimos mutáveis
        let adjacentes: Vec<String> = self.arestas[&u].iter().cloned().collect();

        for v in adjacentes {
            let cor_v;
            {
                cor_v = self.vertices.get(&v).unwrap().cor.clone();
            }

            if cor_v == "GRAY" {
                // Encontramos um vértice sendo explorado, existe um ciclo
                return true;
            } else if cor_v == "WHITE" {
                if self.dfs_ciclo(v.clone()) {
                    return true;  // Ciclo encontrado na recursão
                }
            }
        }

        {
            let vertice_u = self.vertices.get_mut(&u).unwrap();
            vertice_u.cor = "BLACK".to_string();  // Marca como totalmente explorado
        }

        false  // Nenhum ciclo encontrado a partir deste vértice
    }

}