use std::collections::HashMap;

pub struct Recomendacao {
    grafo: HashMap<String, Vec<String>>,
}

impl Recomendacao {
    pub fn new() -> Self {
        Recomendacao {
            grafo: HashMap::new(),
        }
    }

    pub fn adicionar(&mut self, produto: &str, relacionado: &str) {
        self.grafo.entry(produto.to_string())
            .or_insert(Vec::new())
            .push(relacionado.to_string());
    }

    pub fn obter(&self, produto: &str) -> Option<&Vec<String>> {
        self.grafo.get(produto)
    }
}