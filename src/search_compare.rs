use std::sync::{Arc, Mutex};
use std::thread;
use crate::interact_filesystem::{
    get_directory_content,
    check_directory,
};

use crate::compare_paths::compare;
use crate::search_results::{
    SearchResult,
    check_insert,
    print_search_result
};

pub fn search_compare(s_string: &'static str, start_directory:&'static str, amount_res:usize){
    let init_search_result : SearchResult = SearchResult{
        path:"".to_string(),
        score: 100000,
    };
    let mut search_result :Arc<Mutex<Vec<SearchResult>>>= Arc::new(Mutex::new(vec![init_search_result;amount_res]));
    if check_directory(start_directory){
        let cloned_search_result = Arc::clone(&search_result);
        thread::spawn(move || {
            search_compare_rek(s_string.to_string(), start_directory.to_string(), cloned_search_result)
        }).join().unwrap();
        let finished_vector = search_result.lock().unwrap();
        print_search_result(& *finished_vector);
        println!("Finished");
    } else{
        println!("no dic")
    }

}

fn search_compare_rek(s_string: String,search_path:String,search_result:Arc<Mutex<Vec<SearchResult>>>){
    match get_directory_content(&search_path) {
        Ok(directory_content) => {
            let mut handle_vec = vec![];
            for directory in &directory_content.directories {
                let dir_path: String = directory_content.path.clone() + "/" + directory;
                let cloned_search_result = Arc::clone(&search_result);
                let cloned_s_string = s_string.clone();
                let handle = thread::spawn(move || {
                    search_compare_rek(cloned_s_string, dir_path, cloned_search_result);
                });
                handle_vec.push(handle);
            }

            //Vergleich des Inhalt
            for files in &directory_content.files{
                let filename = String::from(search_path.clone() + "/" + files);
                let current_result = SearchResult{
                    path: filename.clone(),
                    score:compare(&s_string, &filename)
                };
                let mut test = search_result.lock().unwrap();
                check_insert(current_result, &mut *test);
            }


            handle_vec.into_iter().for_each(|handle| handle.join().unwrap()); // call join on all handles
        }
        Err(_err)=>{
            eprintln!("Could not get directory content {}", search_path);
        }
    }
    //println!("Thread  {} beendet", search_path);
}