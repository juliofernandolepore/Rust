/*LOOPS (BUCLES) y continue */
fn main() {
    /* del 1 al 29 */
    for var_iteradora in 1..30 {
        if var_iteradora % 2 != 0 {
            continue;
        }
        println!("en el bucle el resultado es {}", var_iteradora);
    }

    println!("*********************************************");
    /*BUCLE WHILE */
    let mut iterador_mutable = 0;
    while iterador_mutable < 15 {
        iterador_mutable += 1;
        println!(
            "valor de variable en esta iteracion {} aun en el bucle",
            iterador_mutable
        );
        /* el break permite detener el bucle donde coloque un break
        y ya no sigue iterando los ciclos restantes */
        if iterador_mutable == 10 {
            break;
        };
    }
    println!("listo, fuera del bucle");
    println!("*************************************");
}
