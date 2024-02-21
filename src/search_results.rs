pub struct SearchResult{
    pub path: String,
    pub score: i32,
}

impl Clone for SearchResult{
    fn clone(&self) -> Self{
        SearchResult{
            path: self.path.clone(),
            score: self.score
        }
    }
}

pub fn print_search_result(results: &Vec<SearchResult>){
    println!("Search Results");
    for (index,elem) in results.iter().enumerate(){
        println!("--- {}. Abstand: {}, Pfad: {}", results.len()-index, elem.score, elem.path);
    }
}

pub fn check_insert(result_to_check: SearchResult, ordered_results: &mut Vec<SearchResult>){
    let cloned_ordered_results = ordered_results.clone();
    if ordered_results[0].score > result_to_check.score {
            let mut append_in_front = true;
           for (index, elem) in cloned_ordered_results.iter().enumerate(){
               if elem.score <= result_to_check.score{
                   ordered_results.insert(index,result_to_check.clone());
                   append_in_front = false;
                   break;
               }
           }
            if append_in_front == true{
               ordered_results.push(result_to_check);
            }
            ordered_results.remove(0);
        }
}
