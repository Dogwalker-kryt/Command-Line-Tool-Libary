use std::path::Path; // Für Dateioperationen
use std::fs; // Für Dateioperationen
use sha2::{Sha256, Digest}; // Für SHA-256 Hash

// =============================
// Einfache Taschenrechner-Funktion für Grundrechenarten
// Fragt zwei Zahlen und eine Rechenoperation ab und gibt das Ergebnis aus
// =============================
fn rechner() {
    println!("------------------ Calculator -----------------");
    println!("Enter the first number:");
    let mut num1 = String::new();
    std::io::stdin().read_line(&mut num1).expect("[Error] Failed to read input");
    let num1: f64 = match num1.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("[Error] Invalid number!");
            return;
        }
    };

    println!("Enter the second number:");
    let mut num2 = String::new();
    std::io::stdin().read_line(&mut num2).expect("[Error] Failed to read input");
    let num2: f64 = match num2.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("[Error] Invalid number!");
            return;
        }
    };

    println!("Enter operation (+, -, *, /):");
    let mut op = String::new();
    std::io::stdin().read_line(&mut op).expect("[Error] Failed to read input");
    let op = op.trim();

    // Berechnung je nach Operator
    let result = match op {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("[Error] Division by zero!");
                return;
            } else {
                num1 / num2
            }
        },
        _ => {
            println!("[Error] Invalid operation!");
            return;
        }
    };
    println!("Result: {}", result);
}

// =============================
// SHA-256 Hash Funktion für beliebigen Text
// Fragt einen Text ab und gibt den SHA-256 Hashwert aus
// =============================
fn sha256() {
    println!("------------------ SHA-256 Hash -----------------");
    println!("Enter text to hash:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("[Error] Failed to read input");
    let input = input.trim();
    let mut hasher = Sha256::new(); // Hasher initialisieren
    hasher.update(input); // Text in den Hasher geben
    let result = hasher.finalize(); // Hash berechnen
    println!("SHA-256: {:x}", result); // Hashwert ausgeben
}

// =============================
// Funktion zum Entfernen einer Datei mit Sicherheitsabfrage
// Fragt nach dem Dateipfad und einer Bestätigung, löscht dann die Datei
// =============================
fn remove_file() {
    println!("[Remove_file] Enter the file path to remove:");
    let mut file_path = String::new();
    std::io::stdin().read_line(&mut file_path).expect("[Error] Failed to read file path or an unexpected error occurred!");
    let file_path = file_path.trim();
    println!("[Remove_file] Are you sure you want to remove the file located at '{}' ?", file_path);
    println!("Enter 'yes' to confirm or 'no' to cancel:");
    let mut confirmation = String::new();
    std::io::stdin().read_line(&mut confirmation).expect("[Error] Failed to read confirmation or an unexpected error occurred!");
    let confirmation = confirmation.trim().to_lowercase();
    if confirmation == "yes" {
        // Prüfe, ob die Datei existiert
        if Path::new(file_path).exists() {
            // Versuche, die Datei zu löschen
            match fs::remove_file(file_path) {
                Ok(_) => println!("[Remove_file] File '{}' has been removed", file_path),
                Err(_) => println!("[Remove_file] Failed to remove file '{}'", file_path),
            }
        } else {
            println!("[Error] File '{}' does not exist or an unexpected error occurred!", file_path);
        }
    } else if confirmation == "no" {
        println!("[Remove_file] File removal has been canceled");
    } else {
        println!("[Error] Invalid confirmation input or an unexpected error occurred!");
    }
    // Nach Abschluss zurück ins Hauptmenü
    main();
}

// =============================
// Zeigt Informationen über das Tool an
// Gibt Name, Version, Beschreibung, Autor und Lizenz aus
// =============================
fn info() {
    println!("Info about this library");
    println!("------------------- Info -------------------");
    println!("This library provides a collection of CLI tools.");
    println!("for various tasks.");
    println!("info about this projekt:\n");
    println!("-Name: CLI-Tool library");
    println!("-Version: 0.2.3");
    println!("-Description: A library of CLI tools for various tasks.");
    println!("-Created by: Dogwalker-kryt");
    println!("-License: MIT License\n");
}

// =============================
// Würfelfunktion: Simuliert das Würfeln mit beliebig vielen Würfeln
// Fragt die Anzahl der Würfel ab und gibt für jeden Wurf eine Zufallszahl zwischen 1 und 6 aus
// =============================
fn wuerfel() {
    use rand::Rng; // Importiere Rng für Zufallszahlen
    println!("------------------ Dice -----------------");
    println!("How many dice do you want to roll?");
    let mut anzahl = String::new();
    std::io::stdin().read_line(&mut anzahl).expect("[Error] Failed");
    let anzahl: u32 = match anzahl.trim().parse() {
        Ok(num) if num > 0 => num,
        _ => {
            println!("[Error] Please enter a valid positive number!");
            return;
        }
    };
    let mut rng = rand::thread_rng(); // Zufallsgenerator initialisieren
    println!("Results:");
    for i in 1..=anzahl {
        let wurf = rng.gen_range(1..=6); // Würfeln (1 bis 6)
        println!("Die {}: {}", i, wurf); // Ergebnis ausgeben
    }
}

// -----------------------------
// Hauptfunktion: Startet das CLI-Tool und zeigt das Menü an
// Hier werden die Benutzereingaben verarbeitet und die passenden Funktionen aufgerufen
// -----------------------------
fn main() {
    println!("Welcome to a library of CLI tools!");
    println!("You can choose from the following options:");
    println!("----------------- CLI Tools Menu -----------------");
    println!("Enter:");
    println!("'cal' for calculator"); // Startet den Taschenrechner
    println!("'sha256' for SHA-256 hash"); // Startet die SHA-256 Hashfunktion
    println!("'remove' for removing files"); // Startet das Datei-Löschtool
    println!("'info' for information"); // Zeigt Informationen über das Tool an
    println!("'wuerfel' for rolling a die"); // Startet die Würfelfunktion
    let mut menu_input = String::new();
    std::io::stdin().read_line(&mut menu_input).expect("[Error] Failed to read input");
    match menu_input.trim() {
        "cal" => rechner(),      // Rechner-Funktion
        "sha256" => sha256(),   // SHA-256 Hashfunktion
        "remove" => remove_file(), // Datei entfernen
        "info" => info(),       // Info anzeigen
        "wuerfel" => wuerfel(), // Würfelfunktion
        _ => println!("[Error] Invalid option or unexpected error occurred!"), // Fehlerausgabe
    }
}
