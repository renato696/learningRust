fn main(){

    condicionais(); //no rust, a funcao pode ser declarada depois ou antes do main
    
}

fn condicionais(){
    let idade:u8 = 17;
    let responsavel_autorizou:bool = true;
    let eh_maior = idade >= 18;

//    if idade > 18 || idade > 16 && responsavel_autorizou {
//        println!("Pode entrar na balada!");
//    } else {
//        println!("Nao pode entrar na balada!");
//    }

if idade >= 18 {
    println!("Pode entrar na balada");
} else if idade > 16 && responsavel_autorizou {
    println!("Pode entrar, responsavel autorizou");
} else {
    println!("Nao pode entrar na balada");
}

let condicao = if eh_maior { "maior" } else { "menor"};
//a expressao acima é o mesmo que:
//let condicao;
//if idade eh_maior {
//    condicao = "maior";
//     } else {
//        condicao = menor;}    
} 

