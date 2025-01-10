struct ContaBancaria{
    nome_titular: String,
    saldo: f64,
    numero_conta: u32,
 }

fn main() {
   let mut Alice = ContaBancaria{
    nome_titular: "Alice".to_string(),
    saldo: 290.0,
    numero_conta: 1234,
   };

   let mut Bob= ContaBancaria{
    nome_titular: "Bob".to_string(),
    saldo: 1300.0,
    numero_conta: 22222,
   };

   println!("Saldos antigos: Alice: {}, Bob: {}", Alice.saldo, Bob.saldo);

   let valor = 20.0;

   if Alice.saldo < valor{
    println!("A transferência não pode ser realizada!")
   }

   else {
       Alice.saldo -= valor;
       Bob.saldo += valor;
   }

   println!("Novos saldos: Alice: {}, Bob: {}", Alice.saldo, Bob.saldo)

}
