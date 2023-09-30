
const PI:f32 = 3.14; //constante global, em letras maiusculas
static mut VARIAVEL_GLOBAL:u8 = 1; //variavel global, em letras maiusculas, pode ser mutable

fn main(){

    println!("PI = {}", PI);

    unsafe{ // uso do unsafe caso eu não queira intervencao do compilador
    println!("variavel_global = {}", VARIAVEL_GLOBAL);
    }

    let variavel:u8 = 128;
    println!("variavel = {}", variavel);

    let variavelnd = 300;
    println!("variavelnd = {}, tamanho = {} bytes", variavelnd, std::mem::size_of_val(&variavelnd));

    let decimal:f32 = 2.5;
    println!("decimal = {}", decimal);

    let mut boolean:bool = false;
    boolean = true;
    println!("boolean = {}, tamanho boolean = {}", boolean, std::mem::size_of_val(&boolean));
    
    let letra:char = 'R';
    println!("tamanho do char ={}", std::mem::size_of_val(&letra));

}

//estou declarando uma variavel, e os dois pontos apos o nome da variavel indica o tipo da
//variavel. tipo: i8 (integer 8 bits); u8 (integer 8bits, unsigned); integers vao de 8bits ate
//128bits, para um controle fino de quanto vamos usar de memoria. tbm possivel declaram a variavel
//sem tipagem, ai o rust entende como um i32. Para numeros decimais, usar float f32 e f64.
//PS: por padrao, variaveis sao imutaveis, nao mudam... quando quiser que a variavel mude o valor,
//declaro ela com let mut (mutable).


