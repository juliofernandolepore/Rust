/*trabajando con valores INGRESADOS POR EL USUARIO EN CONSOLA */

fn main() {
    /*variable String , mut porque sera modificada */
    let mut cod_puesto = String::new();

    println!("introduci el codigo en mayuscula: C , A , V");
    /*paquete metodo INPUT y puntero referencia*/
    std::io::stdin().read_line(&mut cod_puesto).expect("");
    /*le aplico el metodo trim PARA QUE SE PUEDA TIPEAR EN UNA
    NUEVA LINEA DE CONSOLA */
    let cod_puesto = cod_puesto.trim();

    /* ahora se utiliza la estructura MATCH */

    let oficinas = match cod_puesto {
        "C" => {
            println!("comercial");
            "comercial"
        }
        "V" => "ventas",
        "A" => "atencion al cliente",
        _ => panic!("codigo ingresado invalido"),
    };
    println!("departamento: {}", oficinas);
    println!("******************************************************");
}
