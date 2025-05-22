use std::io::stdin;
use std::io::stdout;
use std::vec::Vec;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;

fn main() {
    println!("Tämä ohjelma demonstroi erilaisia tietorakenteita Rustissa.");

    loop {
        println!("Valitse tietorakenne:");
        println!("1) Vektori");
        println!("2) Hajautustaulu");
        println!("3) Muu");
        println!("0) Lopeta\n");

        print!("Anna valintasi: "); let _ = stdout().flush();
        let mut syote = String::new();
        stdin().read_line(&mut syote).expect("Virhe syötteessä");
        let valinta: i8 = syote.trim().parse().expect("Virhe syötteen muunnoksessa");

        match valinta {
            1 => vektori(),
            2 => hajautustaulu(),
            3 => muu(),
            0 => {
                println!("Lopetetaan.");
                break;
            }
            _ => { println!("Tuntematon valinta, yritä uudelleen."); }
        }
        println!();
    }

    println!("Kiitos ohjelman käytöstä.");
}

fn vektori() {
    let mut vec: Vec<String> = Vec::new();

    loop {
        println!("Valitse toiminta vektorille:");
        println!("1) Lisää alkio");
        println!("2) Poista alkio");
        println!("3) Tulosta alkiot");
        println!("4) Kirjoita tiedostoon");
        println!("5) Muu");
        println!("0) Palaa\n");

        print!("Anna valintasi: "); let _ = stdout().flush();
        let mut syote = String::new();
        stdin().read_line(&mut syote).expect("Virhe syötteessä");
        let valinta: i8 = syote.trim().parse().expect("Virhe syötteen muunnoksessa");

        match valinta {
            1 => {
                print!("Anna merkkijono: "); let _ = stdout().flush();
                let mut alkio = String::new();
                stdin().read_line(&mut alkio).expect("Virhe syötteessä");
                alkio = alkio.trim().to_string();

                if alkio.is_empty() {
                    println!("Alkion on oltava epätyhjä merkkijono.");
                } else {
                    vec.push(alkio.clone());
                    println!("Alkio '{}' lisätty vektoriin.", alkio);
                }
            }

            2 => {
                print!("Anna indeksi: "); let _ = stdout().flush();
                let mut syote2 = String::new();
                stdin().read_line(&mut syote2).expect("Virhe syötteessä");
                let indeksi: usize = syote2.trim().parse().expect("Virhe syötteen muunnoksessa");

                if indeksi >= vec.len() {
                    println!("Indeksin on oltava pienempi kuin {}.", vec.len());
                } else {
                    println!("Alkio '{}' poistettu.", vec.remove(indeksi));
                }
            }

            3 => {
                println!("Vektorissa on {} alkiota.", vec.len());
                let mut i = 0;
                for alkio in &vec {
                    println!("{}: {}", i, alkio);
                    i += 1;
                }
            }

            4 => {
                print!("Anna kirjoitettavan tiedoston nimi: "); let _ = stdout().flush();
                let mut nimi = String::new();
                stdin().read_line(&mut nimi).expect("Virhe syötteessä");
                nimi = nimi.trim().to_string();

                if nimi.is_empty() {
                    println!("Tiedoston nimen on oltava epätyhjä merkkijono.");
                } else {
                    let mut tiedosto = File::create(&nimi).expect("Virhe tiedoston avaamisessa");
                    let mut i = 0;
                    for alkio in &vec {
                        tiedosto.write_all(format!("{}: {}\n", i, alkio).as_bytes()).expect("Virhe kirjoituksessa");
                        i += 1;
                    }
                    println!("Alkiot kirjoitettu tiedostoon '{}'.", nimi);
                }
            }

            5 => muu(),
            
            0 => {
                println!("Palataan.");
                break;
            }
            
            _ => { println!("Tuntematon valinta, yritä uudelleen."); }
        }
        println!();
    }
}

fn hajautustaulu() {
    let mut taulu: HashMap<String, i32> = HashMap::new();

    loop {
        println!("Valitse toiminta hajautustaululle:");
        println!("1) Lisää alkio");
        println!("2) Poista alkio");
        println!("3) Tulosta alkiot");
        println!("4) Kirjoita tiedostoon");
        println!("5) Muu");
        println!("0) Palaa\n");

        print!("Anna valintasi: "); let _ = stdout().flush();
        let mut syote = String::new();
        stdin().read_line(&mut syote).expect("Virhe syötteessä");
        let valinta: i8 = syote.trim().parse().expect("Virhe syötteen muunnoksessa");

        match valinta {
            1 => {
                print!("Anna merkkijono: "); let _ = stdout().flush();
                let mut avain = String::new();
                stdin().read_line(&mut avain).expect("Virhe syötteessä");
                avain = avain.trim().to_string();

                if avain.is_empty() {
                    println!("Alkion avain on oltava epätyhjä merkkijono.");
                } else {
                    print!("Anna kokonaisluku: "); let _ = stdout().flush();
                    let mut syote2 = String::new();
                    stdin().read_line(&mut syote2).expect("Virhe syötteessä");
                    let arvo = syote2.trim().parse().expect("Virhe syötteen muunnoksessa");

                    taulu.insert(avain.clone(), arvo);
                    println!("Alkio '{}' lisätty hajautustauluun arvolla {}.", avain, arvo);
                }
            }

            2 => {
                print!("Anna merkkijono: "); let _ = stdout().flush();
                let mut avain = String::new();
                stdin().read_line(&mut avain).expect("Virhe syötteessä");
                avain = avain.trim().to_string();

                if taulu.remove(&avain) == None {
                    println!("Ei löydetty alkiota avaimella '{}'.", avain);
                } else {
                    println!("Alkio '{}' poistettu.", avain);
                }
            }

            3 => {
                println!("Hajautustaulussa on {} alkiota.", taulu.len());
                for (avain, arvo) in &taulu {
                    println!("{}: {}", avain, arvo);
                }
            }

            4 => {
                print!("Anna kirjoitettavan tiedoston nimi: "); let _ = stdout().flush();
                let mut nimi = String::new();
                stdin().read_line(&mut nimi).expect("Virhe syötteessä");
                nimi = nimi.trim().to_string();

                if nimi.is_empty() {
                    println!("Tiedoston nimen on oltava epätyhjä merkkijono.");
                } else {
                    let mut tiedosto = File::create(&nimi).expect("Virhe tiedoston avaamisessa");
                    for (avain, arvo) in &taulu {
                        tiedosto.write_all(format!("{}: {}\n", avain, arvo).as_bytes()).expect("Virhe kirjoituksessa");
                    }
                    println!("Alkiot kirjoitettu tiedostoon '{}'.", nimi);
                }
            }

            5 => muu(),
            
            0 => {
                println!("Palataan.");
                break;
            }
            
            _ => { println!("Tuntematon valinta, yritä uudelleen."); }
        }
        println!();
    }
}

fn muu() {
    println!("Ei vielä toteutettu.");
}

#[cfg(test)]
mod tests {
    // Lataa kirjastoja yksikkötestien ulkopuolelta.
    use super::*;

    #[test]
    fn test_lisaa() {
        assert_eq!(1 + 2, 3);
    }
}