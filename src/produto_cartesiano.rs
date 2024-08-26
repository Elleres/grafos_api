use crate::grafo_lib::grafo::Grafo;
use std::collections::BTreeMap;

pub fn produto_cartesiano(g1: Grafo, g2: Grafo) -> Grafo{
    let mut grafo_resultante = Grafo::new(false);
    let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for vertice_1 in g1.vertices.iter(){
        map.insert(format!("s{vertice_1}"), Vec::new());
        map.insert(format!("e{vertice_1}"), Vec::new());
        for vertice_2 in g2.vertices.iter(){
            grafo_resultante.inserir_vertice(format!("({vertice_1}, {vertice_2})"))
        }
    }

    return grafo_resultante
}