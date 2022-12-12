use std::default;
use std::io::stdin;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::fs::OpenOptions;
use std::io::Write;
use chrono::prelude::*;
use std::io::BufWriter;



#[derive(Default)]
struct Producto{
    codigo: String,
    nombre: String,
    costo: u32,
    venta: u32,
    stock: i32
}


#[derive(Default)]
struct Finanza{
    fecha: String,
    costo: u32,
    ventas: u32
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


fn borrar_file(path: &Path) -> std::io::Result<()> {
    let mut buffer = BufWriter::new(File::create(path)?);
    buffer.flush()?;
    Ok(())
}


fn open_file_to_write(path: &Path) -> File {
    open_file(path);
    borrar_file(path);
    let file: File = open_file_to_append(path);

    return file;
}
 

fn open_file_to_append(path: &Path) -> File {
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
        let mut file_finanzas: File = open_file_to_append(path);
        file_finanzas.write_all(b"Fecha,Costos,Ingresos\n").unwrap();
    }

    if path == inventario {
        let mut file_inventario: File = open_file_to_append(path);
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
            let mut temp:i32 = 0;
            
            if entrada.trim() != b && contador == 0 {
                linea = linea + a;
                break
            }
            
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
    println!("\x1b[1;33mTotal: ${}\x1b[0m", suma);
    let mut monto: String = String::new();
    let mut _monto_u32: u32 = 0;
    
    loop {
        println!("\x1b[1;34mIngrese el monto:\x1b[0m");
        stdin().read_line(&mut monto).unwrap();
        
        if !is_entero_positivo(&monto) || monto.trim() == "".to_string() {
            println!("\n\x1b[1;31mMonto no válido\x1b[0m\n");
            monto = "".to_string();
            continue
        }
        
        _monto_u32 = monto.trim().parse().unwrap();
        
        if _monto_u32 < suma {
            println!("\n\x1b[1;31mMonto Insuficiente\x1b[0m\n");
            monto = "".to_string();
            continue
        }
        
        break
    }
    
    let vuelto: u32 = _monto_u32 - suma;
    println!("------------------------------");
    println!("Monto: ${}", suma);
    println!("Pago: ${}", _monto_u32);
    println!("Vuelto: ${}", vuelto);
    println!("------------------------------");
}


fn vender(finanzas: &Path, inventario: &Path, fecha: &str) {
    let mut suma: u32 = 0;
    let mut error: bool = false;
    
    loop {
        println!("\x1b[1;33mTotal: ${}\x1b[0m", suma);
        println!("------------------------------");
        let mut entrada: String = String::new();
        stdin().read_line(&mut entrada).unwrap();
        
        if entrada.trim() == "0" {
            fin_venta(suma);
            
            if error{
                println!("\n\x1b[1;31;1;43mERROR DE INVENTARIO, REVISAR\x1b[0m")
            }

            println!("\n\x1b[1;34m(ENTER) Para continuar\x1b[0m");
            let mut pausa: String = String::new();
            stdin().read_line(&mut pausa).unwrap();
            break
        }
        
        let producto: Producto = cambiar_stock(inventario, &entrada, -1);
        
        if producto.codigo == "".to_string() {
            loop {
                println!("\x1b[1;31mProducto no válido presiones 1 para continuar\x1b[0m");
                let mut entrada2: String = String::new();
                stdin().read_line(&mut entrada2).unwrap();
                
                if entrada2.trim() == "1".to_string(){
                    break
                }
            }
            
            continue;
        }
        
        println!("------------------------------");
        println!("\x1b[1;36m{}: ${}\x1b[0m", producto.nombre, producto.venta);
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


fn agregar_costos_inventario(fecha: &str, costo: i32, finanzas: &Path) {
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
            } else if contador == 0 {
                existe = true;
                cadena = cadena + b + ",";
            } else if contador == 2 {
                cadena = cadena + b + "\n";
            } else {
                let temp: i32 = b.trim().parse().unwrap();
                cadena = cadena + &format!("{},", costo as i32 + temp);
            }
            
            contador += 1;
        }
    }
    
    if !existe {
        let mut file_finanzas: File = open_file_to_append(finanzas);
        let cadena: String = format!("{},{},0\n", fecha, costo);
        file_finanzas.write_all(cadena.as_bytes()).unwrap();
        return
    }

    file_finanzas.write_all(cadena.as_bytes()).unwrap();
}


fn ingresar(finanzas: &Path, inventario: &Path, fecha: &str) {
    loop {
        let mut codigo: String = String::new();
        let mut cantidad: String = String::new();

        println!("\n\x1b[1;34mIngrese el código:    Ingrese (0) para salir\x1b[0m");
        stdin().read_line(&mut codigo).unwrap();
        
        if codigo.trim() == "0" {
            break
        } else if codigo.trim() == "" {
            println!("\x1b[1;31mEste campo no puede estar vacío\x1b[0m");
            continue;
        }
        
        loop{
            println!("\x1b[1;34mIngrese la cantidad\x1b[0m");
            stdin().read_line(&mut cantidad).unwrap();
            
            if is_entero(&cantidad) {
                break
            }
            
            cantidad = "".to_string();
        }
        
        let num_cantidad: i32 = cantidad.trim().parse().unwrap();
        let producto: Producto = cambiar_stock(inventario, &codigo, num_cantidad);
        
        if producto.codigo == "".to_string() {
            println!("\x1b[1;31mProducto no existe en el invetario, presione 1 para continuar\x1b[0m");
            continue;
        }
        
        let precio_unidad: u32 = producto.costo;
        let costo: i32 =  precio_unidad as i32 * num_cantidad;
        
        agregar_costos_inventario(fecha, costo, finanzas);

        println!("------------------------------");
        print!("\x1b[1;33mSe ingresaron: {} x{}\x1b[0m", producto.nombre, cantidad);
        println!("------------------------------");
    }
}


fn consultar(inventario: &Path) {
    loop {
        let mut codigo: String = String::new();
        
        println!("\x1b[1;34mIngrese el código:    Ingrese (0) para salir\x1b[0m");
        stdin().read_line(&mut codigo).unwrap();
        
        if codigo.trim() == "0" {
            break
        }
        
        let producto = cambiar_stock(inventario, &codigo, 0);
        
        if producto.codigo == "".to_string() {
            println!("\x1b[1;31mProducto no encontrado\x1b[0m");
            continue
        }
        
        println!("------------------------------");
        println!("\x1b[1;33mProducto: {}\x1b[0m", producto.nombre);
        println!("\x1b[1;33mPrecio: ${}\x1b[0m", producto.venta);
        println!("\x1b[1;33mCosto: ${}\x1b[0m", producto.costo);
        println!("\x1b[1;33mStock: {}\x1b[0m", producto.stock);
        println!("------------------------------");
    }
}


fn cambiar_inventario(producto: Producto, inventario: &Path) -> Producto {
    let text: String = open_file(inventario);
    let mut file: File = open_file_to_write(inventario);
    let mut linea: String = "".to_string();
    
    for a in text.split("\n") {
        let mut contador = 0;
        if a.trim() == "" {
            break
        }
        
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
        println!("\x1b[1;34mIngrese el código:    Ingrese (0) para salir\x1b[0m");
        stdin().read_line(&mut codigo).unwrap();
        
        if codigo.trim() == "0" {
            break
        }
        
        let mut producto = cambiar_stock(inventario, &codigo, 0);
        
        if producto.codigo == "".to_string() {
            println!("\x1b[1;31mProducto no encontrado\x1b[0m");
            continue
        }
        
        println!("\x1b[1;36mCarcterísticas:");
        println!("Nombre: {}", producto.nombre);
        println!("Costo: {}", producto.costo);
        println!("Precio: {}\x1b[0m\n", producto.venta);
        println!("\x1b[1;34mEscriba las nuevas caraterísticas\x1b[0m");
        
        for a in 0..3 {
            let mut entrada: String = String::new();
            
            match a{
                0 => println!("\x1b[1;34mNombre:\x1b[0m"),
                1 => println!("\x1b[1;34mCosto de compra:\x1b[0m"),
                _ => println!("\x1b[1;34mPrecio de venta:\x1b[0m")
            }
            
            if a != 0{
                loop {
                    stdin().read_line(&mut entrada).unwrap();
                    
                    if is_entero_positivo(entrada.trim()){
                        break
                    } else {
                        entrada = "".to_string();
                        println!("\x1b[1;31mNo es un número válido, vuelva a intertarlo\x1b[0m")
                    }
                }
            } else {
                stdin().read_line(&mut entrada).unwrap();
            }
            
            match a {
                0 => producto.nombre = entrada.trim().to_string(),
                1 => producto.costo = entrada.trim().parse().unwrap(),
                _ => producto.venta =  entrada.trim().parse().unwrap()
            }
        }
        
        producto = cambiar_inventario(producto, inventario);
        println!("\x1b[1;33mNuevas características:");
        println!("Nombre: {}", producto.nombre);
        println!("Costo: {}", producto.costo);
        println!("Precio: {}\x1b[0m\n", producto.venta);
    }
}


fn agregar_nuevo(finanzas: &Path, inventario: &Path, fecha: &str) {
    loop{
        let mut codigo: String = String::new();
        let text_inventario: String = open_file(inventario);
        let mut file_inventario: File = open_file_to_append(inventario);
        let mut existe: bool = false;
        println!("\n\x1b[1;34mEscriba el código        (0) Salir\x1b[0m");
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
            println!("\x1b[1;31mEl producto ya existe\x1b[0m");
            continue
        }
        
        let mut linea = codigo.trim().to_string() + ",";
        let mut cantidad: i32 = 0;
        let mut precio_unidad: i32 = 0;
        for a in 0..4 {
            loop {
                let mut entrada: String = String::new();
                
                match a {
                    0 => println!("\x1b[1;34mEscriba el nombre\x1b[0m"),
                    1 => println!("\x1b[1;34mEscriba el costo\x1b[0m"),
                    2 => println!("\x1b[1;34mEscriba el venta\x1b[0m"),
                    3 => println!("\x1b[1;34mEscriba el stock\x1b[0m"),
                    _ => continue
                }
                
                stdin().read_line(&mut entrada).unwrap();

                if entrada.trim() == "" {
                    println!("\n\x1b[1;31mEl campo no puede estar vacío\x1b[0m\n");
                    continue;
                }
                
                if a == 0 {
                    if entrada.contains(",") {
                        println!("\n\x1b[1;31mEl nombre no puede tener comas (,)\x1b[0m\n");
                        continue
                    }
                    linea = linea + entrada.trim() + ",";
                    break
                } else if a == 3 && is_entero(entrada.trim()) {
                    linea = linea + entrada.trim() + "\n";
                    cantidad = entrada.trim().parse().unwrap();
                    break
                } else if is_entero(entrada.trim()) {
                    linea = linea + entrada.trim() + ",";
                    if a == 1 {
                        precio_unidad = entrada.trim().parse().unwrap();
                    }
                    break
                } else {
                    println!("\x1b[1;31mNo es un número válido\x1b]0m");
                    continue;
                }
            }
        }
        let costo: i32 = precio_unidad * cantidad;

        agregar_costos_inventario(fecha, costo, finanzas);

        file_inventario.write_all(linea.as_bytes()).unwrap();

        println!("\x1b[1;33mSe ingresó:\x1b[0m");
        for a in linea.split(",") {
            println!("\x1b[1;33m{}\x1b[0m", a.trim());
        }
    }
}


fn ver_finanzas(path: &Path) {
    let texto: String = open_file(path);
    let mut primera_linea: bool = false;

    for a in texto.split("\n") {
        let mut contador = 0;
        let mut finanza: Finanza = Default::default();
        if a.trim() == "" {
            break
        }
        if primera_linea {
            for b in a.split(",") {
                match contador {
                    0 => finanza.fecha = b.to_string(),
                    1 => finanza.costo = b.trim().parse().unwrap(),
                    _ => finanza.ventas = b.trim().parse().unwrap()
                }
                contador += 1;
            }
        } else {
            primera_linea = true;
            continue;
        }
        let balance: i32 =  finanza.ventas as i32 - finanza.costo as i32;
        println!("\n\x1b[1;33m{}:", finanza.fecha);
        println!("Costos:{}    Ingresos:{}\n", finanza.costo, finanza.ventas);
        println!("Balance: {}\x1b[0m\n", balance)
    }

}


fn menu() -> u32 {
    let mut entrada: String = String::new();
    
    loop {
        println!("\n\x1b[1;34mElija opción:\x1b[0m");
        println!("    (1) Vender.");
        println!("    (2) Ingresar stock.");
        println!("    (3) Consultar producto.");
        println!("    (4) Editar productos.");
        println!("    (5) Agregar un producto nuevo.");
        println!("    (6) Ver finanzas.");
        println!("    (0) Salir del programa.");
        stdin().read_line(&mut entrada).unwrap();
        
        if !is_entero_positivo(&entrada) || entrada.trim() == "".to_string() || entrada.trim().len() > 2 {
            println!("\n\x1b[1;31mNO válido intentelo denuevo\x1b[0m");
            entrada = "".to_string();
            continue
        }
        
        match entrada.trim().parse().unwrap() {
            0|1|2|3|4|5|6 => break,
            _ => println!("\n\x1b[1;31mNO válido intentelo denuevo\x1b[0m")
        }
        
        entrada = "".to_string();
        continue
    }  
    
    let num: u32 = entrada.trim().parse().unwrap();
    return num
}


fn main() {
    let finanzas: &Path = Path::new("Finanzas.csv");
    let inventario: &Path = Path::new("Inventario.csv");
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
    
    fecha = fecha + &date.year().to_string();
    println!("\x1b[1;34mFecha: {}\x1b[0m", fecha);
    
    loop {
        let opcion: u32 = menu();
        match opcion {
            1 => vender(finanzas, inventario, &fecha),
            2 => ingresar(finanzas, inventario, &fecha),
            3 => consultar(inventario),
            4 => editar(inventario),
            5 => agregar_nuevo(finanzas, inventario, &fecha),
            6 => ver_finanzas(finanzas),
            _ => break
        }
    }
}
