fn main() {
    let mover= "movida";
    let clonar= String::from ("clonada");
    println!("{mover}\n{clonar}");

    let movida = mover;
    println!("{movida}");
    let clonada = clonar.clone();
    println!("{clonada}")

}
