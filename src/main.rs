use std::io::stdin;
use std::io::stdout;
use std::vec::Vec;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;

fn kysy_merkkijono(prompti: &'static str) -> String {
    print!("{}", prompti); let _ = stdout().flush();
    let mut syote = String::new();
    stdin().read_line(&mut syote).expect("Virhe syötteessä");

    return syote.trim().to_string();
}

fn kysy_luku(prompti: &'static str) -> i32 {
    return kysy_merkkijono(prompti).parse().expect("Virhe syötteen muunnoksessa");
}

fn kysy_indeksi(prompti: &'static str) -> usize {
    return kysy_merkkijono(prompti).parse().expect("Virhe syötteen muunnoksessa");
}

fn main() {
    println!("Tämä ohjelma demonstroi erilaisia tietorakenteita Rustissa.");

    loop {
        println!("Valitse tietorakenne:");
        println!("1) Vektori");
        println!("2) Hajautustaulu");
        println!("3) Muu");
        println!("0) Lopeta\n");

        match kysy_luku("Anna valintasi: ") {
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

// Vektorin toiminnot
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

        match kysy_luku("Anna valintasi: ") {
            1 => lisaa_vektoriin(&mut vec, kysy_merkkijono("Anna merkkijono: ")),
            2 => poista_vektorista(&mut vec, kysy_indeksi("Anna indeksi: ")),
            3 => tulosta_vektori(&mut vec),
            4 => tallenna_vektori(&mut vec, kysy_merkkijono("Anna kirjoitettavan tiedoston nimi: ")),
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

fn lisaa_vektoriin(vec: &mut Vec<String>, alkio: String) {
    if alkio.is_empty() {
        println!("Alkion on oltava epätyhjä merkkijono.");
    } else {
        vec.push(alkio.clone());
        println!("Alkio '{}' lisätty vektoriin.", alkio);
    }
}

fn poista_vektorista(vec: &mut Vec<String>, indeksi: usize) {
    if indeksi >= vec.len() {
        println!("Indeksin on oltava pienempi kuin {}.", vec.len());
    } else {
        println!("Alkio '{}' poistettu.", vec.remove(indeksi));
    }
}

fn tulosta_vektori(vec: &mut Vec<String>) {
    println!("Vektorissa on {} alkiota.", vec.len());
    let mut i = 0;
    for alkio in vec {
        println!("{}: {}", i, alkio);
        i += 1;
    }
}

fn tallenna_vektori(vec: &mut Vec<String>, tiedoston_nimi: String) {
    if tiedoston_nimi.is_empty() {
        println!("Tiedoston nimen on oltava epätyhjä merkkijono.");
    } else {
        let mut tiedosto = File::create(&tiedoston_nimi).expect("Virhe tiedoston avaamisessa");
        let mut i = 0;
        for alkio in vec {
            tiedosto.write_all(format!("{}: {}\n", i, alkio).as_bytes()).expect("Virhe kirjoituksessa");
            i += 1;
        }
        println!("Alkiot kirjoitettu tiedostoon '{}'.", tiedoston_nimi);
    }
}

// Hajautustaulun toiminnot
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

        match kysy_luku("Anna valintasi: ") {
            1 => lisaa_hajautustauluun(&mut taulu, kysy_merkkijono("Anna merkkijono: "), kysy_luku("Anna kokonaisluku: ")),
            2 => poista_hajautustaulusta(&mut taulu, kysy_merkkijono("Anna merkkijono: ")),
            3 => tulosta_hajautustaulu(&mut taulu),
            4 => tallenna_hajautustaulu(&mut taulu, kysy_merkkijono("Anna kirjoitettavan tiedoston nimi: ")),
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

fn lisaa_hajautustauluun(taulu: &mut HashMap<String, i32>, avain: String, arvo: i32) {
    if avain.is_empty() {
        println!("Alkion avain on oltava epätyhjä merkkijono.");
    } else {
        taulu.insert(avain.clone(), arvo);
        println!("Alkio '{}' lisätty hajautustauluun arvolla {}.", avain, arvo);
    }
}

fn poista_hajautustaulusta(taulu: &mut HashMap<String, i32>, avain: String) {
    if taulu.remove(&avain) == None {
        println!("Ei löydetty alkiota avaimella '{}'.", avain);
    } else {
        println!("Alkio '{}' poistettu.", avain);
    }
}

fn tulosta_hajautustaulu(taulu: &mut HashMap<String, i32>) {
    println!("Hajautustaulussa on {} alkiota.", taulu.len());
    for (avain, arvo) in taulu {
        println!("{}: {}", avain, arvo);
    }
}

fn tallenna_hajautustaulu(taulu: &mut HashMap<String, i32>, tiedoston_nimi: String) {
    if tiedoston_nimi.is_empty() {
        println!("Tiedoston nimen on oltava epätyhjä merkkijono.");
    } else {
        let mut tiedosto = File::create(&tiedoston_nimi).expect("Virhe tiedoston avaamisessa");
        for (avain, arvo) in taulu {
            tiedosto.write_all(format!("{}: {}\n", avain, arvo).as_bytes()).expect("Virhe kirjoituksessa");
        }
        println!("Alkiot kirjoitettu tiedostoon '{}'.", tiedoston_nimi);
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
    fn testaa_vektorin_lisays() {
        let mut vec: Vec<String> = Vec::new();

        lisaa_vektoriin(&mut vec, "Joonatan".to_string());
        lisaa_vektoriin(&mut vec, "Ismo".to_string());
        lisaa_vektoriin(&mut vec, "Tuomas".to_string());
        assert_eq!(vec.get(2).unwrap(), "Tuomas");
    }
}