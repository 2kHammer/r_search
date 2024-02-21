mod interact_filesystem;
mod search_compare;
mod compare_paths;
mod search_results;

use std::time::Instant;

fn main() {
    let now = Instant::now();
    let search_string ="plan";
    let search_directory = "/home/alex/";
    search_compare::search_compare(search_string,search_directory,6);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}


/* To DO
    - nur nach verzeichnissen, oder bestimmten Dateitypen suchen
    - nicht nur nach Endverzeichnissen suchen
    - Fehler bei zu großen Suchverzeichnis
        - Anzahl von Threads begrenzen
    - durchsucht aktuell nur dateien
    - levenstein distance ist nicht optimal
  */

