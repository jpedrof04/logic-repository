struct Cacete {
    tamanho: u32,
    grossura: f32,
    cor: String,
    nome: String,
}
//& pega o endereço de memoria do cacete, é um ponteiro
// vai retornar uma string
fn nome_Cacete(caraio: &Cacete) -> String {
    caraio.nome.clone() //nao usar ';'
}
/*
aqui usamos & pra pegar emprestado o cacete
e copiamos a string nome usando clone() pra evitar problemas de ownership
*/

// o ';' significa: executa e joga o resultado fora


fn main() {
    let mut cacete1 = Cacete {
        tamanho: 100,
        grossura: 10.5,
        cor: String::from("preto"),
        nome: String::from("cacete do zé"),
    };

    println!("rodando hehe...");
    let nome_do_cacete = nome_Cacete(&cacete1);
    println!("O nome do cacete é: {}", nome_do_cacete);

    println!("modificando o nome do cacete...");
    cacete1.nome = String::from("cacete do Gallo");
    let nome_do_cacete_modificado = nome_Cacete(&cacete1);
    println!("O nome do cacete modificado é: {}", nome_do_cacete_modificado);
}