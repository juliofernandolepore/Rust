//Cadena de caracteres o Strings

fn main() {
    let variable_1 = "hola mundo";
    let variable_2 = "jose,juan,javier";
    //  \n para saltos de linea, sino se amontona todo en pantalla.
    println!("{}", variable_1);
    println!("{}", variable_1.replace("mundo", "jose!")); //metodo reemplazar string
    println!("{}", variable_1.len()); // metodo len devuelve el largo de una cadena
    println!("{} sin espacios", variable_1.trim()); /* metodo que recorta los espacios
                                                    en blanco entre los caracteres del string */

    println!("*******************************************");
    println!("{}", variable_1.to_owned() + variable_2); //concatenacion
    println!("{}", variable_2.to_owned() + ",fernando");

    //EJERCICIO CON CICLO FOR PARA REEMPLAZAR STRINGS

    for variable_local_que_itera in variable_1.split_whitespace() {
        //va a iterar las palabras con separacion
        println!("{}", variable_local_que_itera);
    }
    println!("*******************************************");

    for variable_local_2_que_itera in variable_2.split(",") {
        //va a iterar con el argumento , en el metodo split
        println!("{}", variable_local_2_que_itera);
    }
    println!("*******************************************");
}
