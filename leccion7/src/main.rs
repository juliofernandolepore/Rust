/* ARRAYS O ARREGLOS */

fn main() {
    let arreglo = ["juan", "pedrito", "tomas", "lucas", "manuelito"];

    println!(
        "el nombre de {} en arreglo de {} indices",
        arreglo[1],
        arreglo.len()
    );
    /* ahora print de arreglo con formato, lo que hace es imprimir visualmente
    tal cual es la estructura del arreglo con los indices */
    println!("{:?}", &arreglo);
    /*ahora voy a imprimir del 1 al 2 inclusive omitiendo el 0 y al 3,4,5 */
    println!("{:?}", &arreglo[1..3]);

    /*ahora a iterar el arreglo con FOR IN */

    for var_para_iterar in &arreglo {
        println!("hola señor {}", var_para_iterar);
    }
    println!("**************************************");
    /* array de pesos en float 32 dimension 3 */
    let peso: [f32; 3] = [70.0, 90.200, 101.600];
    /* un array con 3 alturas */
    let alto = [1.6, 1.7, 1.8];
    /*ahora creamos 3 iteraciones que equivale el numero de indices , y segun el
    scope el ciclo for tiene accedo directo a los arreglos */
    for iterador in 0..3 {
        println!(
            "indice masa corporal es: {}",
            peso[iterador] / (alto[iterador] * alto[iterador])
        );
    }
    println!("*********************************************************");

    /* TUPLAS o TUPLE */
    let tup_1 = ("fernando", 39);
    /*otra forma de declarar tuplas con tipos de datos */
    let tup_2: (&str, i8) = ("juan", 60);
    println!("nombre en tupla 1 es: {}", tup_1.1); //1.0 da acceso al indice 0, nombre.
    println!("edad en tuplas 2 es: {}", tup_2.1); // 1.1 da acceso al indice 1, edad.

    /*VECTORES (ARRAYS QUE PUEDEN AUMENTAR O CRECER)- ver matrices */

    let mut vec1 = vec![1, 2, 3, 4, 5, 6, 7];
    println!("item 2/ indice 1 = {}", vec1[1]);

    /* ITERAR VECTORES */

    for i in &vec1 {
        println!("{}", i);
    }
    println!("{:?}", &vec1); //imprimir con FORMAT (se imprimen horizontalmente)

    /*metodo PUSH (introduce un valor al final del arreglo) */
    vec1.push(10);
    println!("{:?}", &vec1);

    /*metodo POP (elimina un indice del arreglo) */
    vec1.pop();
    println!("{:?}", &vec1);
}
