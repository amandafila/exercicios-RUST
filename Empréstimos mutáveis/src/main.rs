struct livro{
    titulo: String,
    autor: String,
    quantidade_paginas: i64,
}

impl livro {
    fn adicionarPaginas (&mut self, paginas: i64){
    if paginas > 0{
            self.quantidade_paginas += paginas;
    }else {
            println!("Número de páginas inválido")
        }
    }
}

fn main() {
    let mut memoriasPostumas = livro{
        titulo: "Memórias Póstumas de Brás Cubas".to_string(),
        autor: "Machado de Assis".to_string(),
        quantidade_paginas: 368,
    };

    println!("{}", memoriasPostumas.quantidade_paginas);

    memoriasPostumas.adicionarPaginas(100);

    println!("{}", memoriasPostumas.quantidade_paginas);

}
