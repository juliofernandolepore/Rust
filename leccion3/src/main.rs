// - OPERACIONES MATEMATICAS

fn main() {
    let diez = 10;
    let resultado_suma = diez + diez;
    println!("el resultado de la suma almacenada es: {}", resultado_suma);
    println!("la resta es: {}", 10 - 8);
    println!("la multiplicacion es: {}", 5 * 40);
    println!("esta divisio es: {}", 60 / 4);
    println!("el modulo o residuo es: {}", 8 % 3);

    println("*********************************************************");

    /*  METODOS MATEMATICOS */
    println!("metodo absoluto {}", -5i32.abs());
    println!(" metodo de potencia {}", 3u32.pow());
    println!("metodo raiz cuadrada {}", 36f64.sqrt());
    println!("metodo raiz cubica {}", 9f64.cbrt());
    println!("metodo de redondeo {}", 1.45f64.round());
    println!("metodo redondeo floor {}", 1.45f64.floor());
}
