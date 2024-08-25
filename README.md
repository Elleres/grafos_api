<h1>API de grafos<h1/>

<p>Esse projeto foi criado para a classe de Grafos 2024 ministrada pelo o professor Samarone. A linguagem escolhida foi o rust por ser uma linguagem de baixo nível e pelo meu interesse em aprender mais sobre ela.</p>

<h2>main.rs<h2/>
<p>Arquivo principal, dentro dele devem ser feitas as operações com o grafo para testar a API.</p>

<h2>grafo_lib.rs</h2>
<p>Arquivo que contém a importação da classe grafo que é necessária para fazer as operações de grafo.</p>

<h2>grafo_lib/grafo.rs</h2>
<p>Arquivo que contém a classe grafo. Dentro da classe existe o atributo matriz, que representa a implementação de uma matriz de adjacencias e o atributo arestas que representa a implementação de uma lista de adjacencias</p>

<h2>Tabela de métodos</h2>

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
            <td>Constructor da classe. Aloca espaço na memória para dois vetores e duas BTreeMap.</td>
        </tr>
        <tr>
            <td><code>inserir_vertice()</code></td>
            <td>Insere um vertice no vetor de vertices da instancia.</td>
        </tr>
        <tr>
            <td><code>inserir_aresta()</code></td>
            <td>Insere uma aresta no hashmap de arestas da instancia.</td>
        </tr>
        <tr>
            <td><code>criar_matriz()</code></td>
            <td>Cria o vetor matriz da instancia. Deve ser executada somente após inserir todos os vértices e arestas.</td>
        </tr>
        <tr>
            <td><code>tamanho()</code></td>
            <td>Retorna o número de vertices.</td>
        </tr>
        <tr>
            <td><code>print_grafo_matriz()</code></td>
            <td>Printa no terminal a matriz de adjacencias.</td>
        </tr>
        <tr>
            <td><code>print_arestas()</code></td>
            <td>Printa no terminal as arestas do grafo. É o método responsável por exibir a lista de adjacencias</td>
        </tr>
        <tr>
            <td><code>print_vertices()</code></td>
            <td>Printa no terminal os vertices do grafo.</td>
        </tr>
    </tbody>
</table>
