/* CALCULO DE INDICE DE MASA CORPORAL CON INPUT DE USUARIO EN CONSOLA */
use std::io::stdin;

fn main() {
    let mut kilos = String::new();
    println!("introduci tu peso en Kilos");

    stdin().read_line(&mut kilos).expect("");
    /*al valor recibido en la variable kilos se le aplican varios
    metodos en una sola linea de codifo y se almacena en otra variable */
    let usuario_input: f32 = kilos.trim().parse().unwrap();

    /* ahora creo la otra variable que sera parametro de la funcion IMC */
    let mut altura = String::new();
    println!("introduci tu altura en metros (ej: 1.85)");
    /* se repiten los pasos con la otra variable  */
    stdin().read_line(&mut altura).expect("");
    let usuario_input2: f32 = altura.trim().parse().unwrap();

    let imc = usuario_input / (usuario_input2 * usuario_input2);

    println!(
        "tu peso es: {} y tu altura es: {}, da como resultado que tu indice de masa corporal es {}",
        usuario_input, usuario_input2, imc
    );
}
