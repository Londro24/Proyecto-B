use std::default;
use std::io::stdin;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::fs::OpenOptions;
use std::io::Write;
use chrono::prelude::*;


#[derive(Default)]
struct Producto{
    codigo: String,
    nombre: String,
    costo: u32,
    venta: u32,
    stock: i32
}


fn is_entero_positivo(numero: &str) -> bool {
    for digit in numero.to_string().trim().chars(){
        if digit.is_numeric(){
            continue
        } else {
            return false
        }
    }

    return true
}

fn is_entero(numero: &str) -> bool {
    for digit in numero.to_string().trim().chars(){
        if digit.is_numeric(){
            continue
        } else if digit == '-' {
            continue
        } else {
            return false
        }
    }

    return true
}


fn open_file_to_write(path: &Path) -> File{
    open_file(path);
    let mut binding = OpenOptions::new();
    let binding = binding.write(true);
    let file: File = match binding.open(path){
        Err(_why) => panic!("No se puede abrir el archivo"),
        Ok(file) => file,
    };

    return file
}


fn open_file_to_append(path: &Path) -> File{
    open_file(path);
    let mut binding = OpenOptions::new();
    let binding = binding.append(true);
    let file: File = match binding.open(path){
        Err(_why) => panic!("No se puede abrir el archivo"),
        Ok(file) => file,
    };

    return file
}


fn read_file(mut file: &File) -> String {
    let mut text: String = String::new();
    file.read_to_string(&mut text).unwrap();
    return text
}


fn create_blank_file(path: &Path){
    let _file: File = File::create(path).expect("El archivo no pudo crearse");
    let finanzas: &Path = Path::new("finanzas.csv");
    let inventario: &Path = Path::new("inventario.csv");

    if path == finanzas {
        let mut file_finanzas = open_file_to_append(path);
        file_finanzas.write_all(b"Fecha,Costos,Ingresos\n").unwrap();
    }
    if path == inventario {
        println!("AAAA");
        let mut file_inventario = open_file_to_append(path);
        file_inventario.write_all(b"Codigo,Nombre,Costo,Venta,Stock\n").unwrap();
    }
}


fn open_file(path: &Path) -> String{
    let mut text: String = "".to_string();

    if Path::new(path).exists(){
        let file: File = match File::open(&path){
            Err(_why) => panic!("El archivo no se puede abrir..."),
            Ok(file) => file,
        };
        text = read_file(&file);
    } else {
        create_blank_file(path);
    }

    return text
}


fn cambiar_stock(inventario: &Path, entrada: &str, cambio: i32) -> Producto {
    let mut producto: Producto = Default::default();
    let text_inventario: String = open_file(inventario);
    let mut file_inventario: File = open_file_to_write(inventario);
    let mut linea: String = "".to_string();

    for a in text_inventario.split("\n") {
        let mut contador: i32 = 0;
        if a == "" {
            break
        }
        for b in a.split(",") {
            
            if entrada.trim() != b && contador == 0 {
                linea = linea + a;
                break
            }
            let mut temp:i32 = 0;
            if is_entero(b.trim()) {
                temp = b.trim().parse::<i32>().unwrap();
                temp = temp + cambio;
            }
            match contador {
                0 => producto.codigo = b.to_string(),
                1 => producto.nombre = b.to_string(),
                2 => producto.costo = b.trim().parse::<u32>().unwrap(),
                3 => producto.venta = b.trim().parse::<u32>().unwrap(),
                4 => producto.stock = temp,
                _ => continue
            }
            if contador != 0 && contador != 4 {
                linea = linea + "," + b;
            } else if contador == 4 {
                linea = linea + "," + &temp.to_string();
            } else {
                linea = linea + b;
            }
            contador += 1;
        }
        linea = linea + "\n";
    }
    if producto.codigo == "".to_string() {
        producto.venta = 0;
    }
    
    file_inventario.write_all(linea.as_bytes()).unwrap();

    return producto;
}


fn fin_venta(suma: u32) {
    println!("Total: ${}", suma);
    let mut monto: String = String::new();
    let mut monto_u32: u32 = 0;

    loop {
        println!("Ingrese el monto:");
        stdin().read_line(&mut monto).unwrap();
        if !is_entero_positivo(&monto) || monto.trim() == "".to_string() {
            println!("\nMonto no válido\n");
            monto = "".to_string();
            continue
        }
        monto_u32 = monto.trim().parse().unwrap();
        if monto_u32 < suma {
            println!("\nMonto insuficiente\n");
            monto = "".to_string();
            continue
        }
        break
    }
    let vuelto: u32 = monto_u32 - suma;

    println!("------------------------------");
    println!("Monto: ${}", suma);
    println!("Pago: ${}", monto_u32);
    println!("Vuelto: ${}", vuelto);
    println!("------------------------------");
    println!("(ENTER para continuar)");
}


fn vender(finanzas: &Path, inventario: &Path, fecha: &str) {
    let mut suma: u32 = 0;
    let mut error: bool = false;

    loop {
        println!("Total: ${}", suma);
        println!("------------------------------");
        let mut entrada: String = String::new();
        stdin().read_line(&mut entrada).unwrap();

        if entrada.trim() == "0" {
            fin_venta(suma);
            if error{
                println!("\nERROR DE INVENTARIO, REVISAR")
            }
            let mut pausa: String = String::new();
            stdin().read_line(&mut pausa).unwrap();
            break
        }
        let producto: Producto = cambiar_stock(inventario, &entrada, -1);
        if producto.codigo == "".to_string() {
            loop {
                println!("Producto no válido presiones 1 para continuar");
                let mut entrada2: String = String::new();
                stdin().read_line(&mut entrada2).unwrap();
                if entrada2.trim() == "1".to_string(){
                    break
                }
            }
            continue;
        }

        println!("------------------------------");
        println!("{}: ${}", producto.nombre, producto.venta);

        let venta: u32 = producto.venta;
        let stock: i32 = producto.stock;

        if stock < 0 {
            error = true
        }
        suma = suma + venta;

        let text_finanzas: String =  open_file(finanzas);
        let mut file_finanzas: File = open_file_to_write(finanzas);
        let mut cadena: String = String::new();
        let mut existe: bool = false;

        for a in text_finanzas.split("\n") {
            if a == "" {
                break
            }
            let mut contador: i32 = 0;
            for b in a.split(",") {
                if b != fecha && contador == 0 {
                    cadena = cadena + a + "\n";
                    break;
                } else if contador != 2 {
                    existe = true;
                    cadena = cadena + b + ",";
                } else {
                    let ganancias: i32 = b.trim().parse::<i32>().unwrap() + venta as i32;
                    cadena = cadena + &format!("{}", ganancias) +"\n";
                }
                contador += 1;
            }
        }
        if !existe {
            let mut file_finanzas: File = open_file_to_append(finanzas);
            let cadena: String = format!("{},0,{}\n", fecha, venta);
            file_finanzas.write_all(cadena.as_bytes()).unwrap();
            continue
        }
        file_finanzas.write_all(cadena.as_bytes()).unwrap();
    }
}


fn ingresar(finanzas: &Path, inventario: &Path, fecha: &str) {
    loop {
        let mut codigo: String = String::new();
        let mut cantidad: String = String::new();
        println!("Ingrese el código:    Ingrese (0) para salir");
        stdin().read_line(&mut codigo).unwrap();

        if codigo.trim() == "0" {
            break
        }

        loop{
            println!("Ingrese la cantidad");
            stdin().read_line(&mut cantidad).unwrap();
            if is_entero(&cantidad) {
                break
            }
            cantidad = "".to_string();
        }

        let num_cantidad: i32 = cantidad.trim().parse().unwrap();
        let producto: Producto = cambiar_stock(inventario, &codigo, num_cantidad);
        
        if producto.codigo == "".to_string() {
            loop {
                println!("Producto no válido presiones 1 para continuar");
                let mut entrada: String = String::new();
                stdin().read_line(&mut entrada).unwrap();
                if entrada.trim() == "1".to_string(){
                    break
                }
            }
            continue;
        }

        let text_finanzas: String =  open_file(finanzas);
        let mut file_finanzas: File = open_file_to_write(finanzas);
        let mut cadena: String = String::new();
        let mut existe: bool = false;
        let precio_unidad: u32 = producto.costo;
        let costo: i32 =  precio_unidad as i32 * num_cantidad;

        for a in text_finanzas.split("\n") {
            if a == "" {
                break
            }
            let mut contador: i32 = 0;
            for b in a.split(",") {
                if b != fecha && contador == 0 {
                    cadena = cadena + a + "\n";
                    break;
                } else if contador == 0 {
                    existe = true;
                    cadena = cadena + b + ",";
                } else if contador == 2 {
                    cadena = cadena + b + "\n";
                }else {
                    let temp: i32 = b.trim().parse().unwrap();
                    cadena = cadena + &format!("{},", costo + temp);
                }
                contador += 1;
            }
        }
        if !existe {
            let mut file_finanzas: File = open_file_to_append(finanzas);
            let cadena: String = format!("{},{},0\n", fecha, costo);
            file_finanzas.write_all(cadena.as_bytes()).unwrap();
            continue
        }

        println!("Se ingresaron: {} x{}", producto.nombre, cantidad);
        file_finanzas.write_all(cadena.as_bytes()).unwrap();
    }
}


fn consultar(inventario: &Path) {
    loop {
        let mut codigo: String = String::new();

        println!("Ingrese el código:    Ingrese (0) para salir");
        stdin().read_line(&mut codigo).unwrap();
        if codigo.trim() == "0" {
            break
        }

        let producto = cambiar_stock(inventario, &codigo, 0);
        
        if producto.codigo == "".to_string() {
            println!("Producto no encontrado");
            continue
        }

        println!("------------------------------");
        println!("Producto: {}", producto.nombre);
        println!("Precio: ${}", producto.venta);
        println!("Costo: ${}", producto.costo);
        println!("Stock: {}", producto.stock);
        println!("------------------------------");
    }
}


fn cambiar_inventario(producto: Producto, inventario: &Path) -> Producto {
    let text: String = open_file(inventario);
    let mut file: File = open_file_to_write(inventario);
    let mut linea: String = "".to_string();

    for a in text.split("\n") {
        let mut contador = 0;
        for b in a.split(",") {
            if b == producto.codigo {
                contador = contador + 1;
            } else if contador == 0 {
                linea = linea + a + "\n";
                break
            }
            match contador {
                1 => linea = linea + b + ",",
                2 => linea = linea + &producto.nombre + ",",
                3 => linea = linea + &producto.costo.to_string() + ",",
                4 => linea = linea + &producto.venta.to_string() + ",",
                5 => linea = linea + b + "\n",
                _ => continue
            }
            contador = contador +1;
        }
    }

    file.write_all(linea.as_bytes()).unwrap();
    return producto;
}


fn editar(inventario: &Path) {
    loop {
        let mut codigo: String = String::new();
        println!("Ingrese el código:    Ingrese (0) para salir");
        stdin().read_line(&mut codigo).unwrap();

        if codigo.trim() == "0" {
            break
        }

        let mut producto = cambiar_stock(inventario, &codigo, 0);
        
        if producto.codigo == "".to_string() {
            println!("Producto no encontrado");
            continue
        }

        println!("Carcterísticas:");
        println!("Nombre: {}", producto.nombre);
        println!("Costo: {}", producto.costo);
        println!("Precio: {}\n", producto.venta);
        println!("Escriba las nuevas caraterísticas");

        for a in 0..3 {
            let mut entrada: String = String::new();
            match a{
                0 => println!("Nombre"),
                1 => println!("Costo de compra"),
                _ => println!("Precio")
            }
            if a != 0{
                loop {
                    stdin().read_line(&mut entrada).unwrap();
                    if is_entero_positivo(entrada.trim()){
                        break
                    }
                    else {
                        entrada = "".to_string();
                        println!("No es un número válido, vuelva a intertarlo")
                    }
                }
            } else {
                stdin().read_line(&mut entrada).unwrap();
            }
            match a {
                0 => producto.nombre = entrada.trim().to_string(),
                _ => producto.venta =  entrada.trim().parse().unwrap()
            }
        }
        producto = cambiar_inventario(producto, inventario);

        println!("Nuevas características;");
        println!("Nombre: {}", producto.nombre);
        println!("Costo: {}", producto.costo);
        println!("Precio: {}\n", producto.venta);
    }
}


fn agregar_nuevo(finanzas: &Path, inventario: &Path) {
    loop{
        let mut codigo: String = String::new();
        let text_inventario: String = open_file(inventario);
        let mut file_inventario: File = open_file_to_append(inventario);
        let mut existe: bool = false;

        println!("\nEscriba el código        (0) Salir");
        stdin().read_line(&mut codigo).unwrap();
        if codigo.trim() == "0" {
            break
        }

        for a in text_inventario.split("\n") {
            for b in a.split(",") {
                if b == codigo.trim() {
                    existe = true
                }
            }
        }

        if existe {
            println!("El producto ya existe");
            continue
        }
        
        let mut linea = codigo.trim().to_string() + ",";

        for a in 0..4 {
            loop {
                let mut entrada: String = String::new();
                match a {
                    0 => println!("Escriba el nombre"),
                    1 => println!("Escriba el costo"),
                    2 => println!("Escriba el venta"),
                    3 => println!("Escriba el stock"),
                    _ => continue
                }
                stdin().read_line(&mut entrada).unwrap();
                if a == 0 {
                    linea = linea + entrada.trim() + ",";
                    break
                } else if a == 3 && is_entero(entrada.trim()) {
                    linea = linea + entrada.trim() + "\n";
                    break
                } else if is_entero(entrada.trim()) {
                    linea = linea + entrada.trim() + ",";
                    break
                } else {
                    println!("No es un número válido");
                    continue;
                }
            }
            
            
        }
        file_inventario.write_all(linea.as_bytes()).unwrap();
    }

}


fn ver_finanzas(finanzas: &Path) {

}


fn menu() -> u32 {
    let mut entrada: String = String::new();
    loop {
        println!("\nElija opción:");
        println!("    (1) Vender.");
        println!("    (2) Ingresar stock.");
        println!("    (3) Consultar producto.");
        println!("    (4) Editar productos.");
        println!("    (5) Agregar un producto nuevo.");
        println!("    (6) Ver finanzas.");
        println!("    (0) Salir del programa.");
        stdin().read_line(&mut entrada).unwrap();

        if !is_entero_positivo(&entrada) || entrada.trim() == "".to_string() || entrada.trim().len() > 2 {
            println!("\nIntentelo denuevo\n");
            entrada = "".to_string();
            continue
        }
        match entrada.trim().parse().unwrap() {
            0|1|2|3|4|5|6 => break,
            _ => entrada = "".to_string()
        }

        println!("\nIntentelo denuevo\n");
        continue
    }  

    let num: u32 = entrada.trim().parse().unwrap();
    return num
}


fn main() {
    let finanzas: &Path = Path::new("finanzas.csv");
    let inventario: &Path = Path::new("inventario.csv");
    let date = Utc::now();
    let mut fecha: String = match date.month() {
        1 => "Enero".to_string(),
        2 => "febrero".to_string(),
        3 => "Marzo".to_string(),
        4 => "Abril".to_string(),
        5 => "Mayo".to_string(),
        6 => "Junio".to_string(),
        7 => "Julio".to_string(),
        8 => "Agosto".to_string(),
        9 => "Septimebre".to_string(),
        10 => "Octubre".to_string(),
        11 => "Noviembre".to_string(),
        12 => "Diciembre".to_string(),
        _ => panic!("")
    };

    fecha = fecha + ", " + &date.year().to_string();
    println!("Fecha: {}", fecha);

    loop {
        let opcion: u32 = menu();
        match opcion {
            1 => vender(finanzas, inventario, &fecha),
            2 => ingresar(finanzas, inventario, &fecha),
            3 => consultar(inventario),
            4 => editar(inventario),
            5 => agregar_nuevo(finanzas, inventario),
            6 => ver_finanzas(finanzas),
            _ => break
        }
    }
}