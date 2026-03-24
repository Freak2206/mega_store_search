use std::collections::HashMap;

pub struct Recomendacao {
    mapa: HashMap<String, Vec<String>>,
}

impl Recomendacao {
    pub fn new() -> Self {
        Recomendacao { mapa: HashMap::new() }
    }

    pub fn adicionar(&mut self, produto: &str, recomendado: &str) {
        self.mapa.entry(produto.to_string())
            .or_insert(Vec::new())
            .push(recomendado.to_string());
    }

    /// Método padronizado para obter recomendações
    pub fn sugerir(&self, produto: &str) -> Vec<String> {
        self.mapa.get(produto).cloned().unwrap_or_default()
    }
}