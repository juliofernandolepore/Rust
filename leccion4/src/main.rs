/*OPERADORES RELACIONALES: > < >= <= == !=
Y OPERADORES LOGICOS: && ││ ! */
//alt 124 para las barras verticales
fn main() {
    println!("{}", 8 > 3 && 6 == 6); //AMBAS DEBEN SER TRUE PARA QUE DEVUELVA TRUE (&&)
    println!("{}", 8 > 3 || 1 > 3); //una de las expresiones true es suficiente
    println!("{}", !true); //este lo convierte en false

    //ALT 124 para obtener lineas verticales pipe

    //CONDICIONALES
    let maximo = 50;
    let diminuto = 5;
    let size = 1;
    if size <= maximo {
        println!("{} es pequeño en comparacion a {}", size, maximo);
    } else {
        println!("es grande");
    }

    if size <= diminuto {
        println!("{} es diminuto con respecto a {}", size, maximo);
    } else if size > diminuto && size < maximo {
        println!("{} es mediano", size);
    } else if size <= 0 {
        println!("es cero o negativo");
    } else {
        println!("{} es enorme y sobrepasa a {}", size, maximo);
    }
    println!("********************************************************");

    //trabajando con declaraciones coincidentes
    //METODO MATCH
    //se asemeja a la estructura "switch"

    let codigo_trabajo = "V";

    let puesto = match codigo_trabajo {
        "M" => {
            println!("MARKETING");
            "MARKETING"
        }
        "C" => "CONTABILIDAD",
        "V" => "VENTAS",
        _ => "OTROS",
    };
    println!("area: {}", puesto);
}
