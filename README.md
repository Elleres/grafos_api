

<h1>API de Grafos</h1>

<p>Esse projeto foi criado para a classe de Grafos 2024 ministrada pelo professor Samarone. A linguagem escolhida foi o Rust por ser uma linguagem de baixo nível e pelo meu interesse em aprender mais sobre ela.</p>

<h2>main.rs</h2>
<p>Arquivo principal. Dentro dele, devem ser feitas as operações com o grafo para testar a API.</p>

<h2>grafo_lib.rs</h2>
<p>Arquivo que contém a importação da classe grafo, necessária para fazer as operações de grafo.</p>

<h2>grafo_lib/grafo.rs</h2>
<p>Arquivo que contém a classe grafo. Dentro da classe, existe o atributo matriz, que representa a implementação de uma matriz de adjacências, e o atributo arestas, que representa a implementação de uma lista de adjacências.</p>

<h2>Tabela de métodos da classe grafo</h2>

<table>
    <thead>
        <tr>
            <th>Nome do método</th>
            <th>Descrição</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td><code>new()</code></td>
            <td>Construtor da classe. Aloca espaço na memória para dois vetores e duas BTreeMap.</td>
        </tr>
        <tr>
            <td><code>inserir_vertice()</code></td>
            <td>Insere um vértice no vetor de vértices da instância.</td>
        </tr>
        <tr>
            <td><code>inserir_aresta()</code></td>
            <td>Insere uma aresta no hashmap de arestas da instância.</td>
        </tr>
        <tr>
            <td><code>criar_matriz()</code></td>
            <td>Cria o vetor matriz da instância. Deve ser executada somente após inserir todos os vértices e arestas.</td>
        </tr>
        <tr>
            <td><code>tamanho()</code></td>
            <td>Retorna o número de vértices.</td>
        </tr>
        <tr>
            <td><code>print_grafo_matriz()</code></td>
            <td>Exibe no terminal a matriz de adjacências.</td>
        </tr>
        <tr>
            <td><code>print_arestas()</code></td>
            <td>Exibe no terminal as arestas do grafo. Responsável por exibir a lista de adjacências.</td>
        </tr>
        <tr>
            <td><code>print_vertices()</code></td>
            <td>Exibe no terminal os vértices do grafo.</td>
        </tr>
    </tbody>
</table>

</body>
</html>
