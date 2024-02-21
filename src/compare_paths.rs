use std::cmp::min;

pub fn compare(s_string:&str, c_string:&str) -> i32{
    let s_string_converted = String::from(s_string);
    let c_string_converted = String::from(c_string);
    let relevant_part_of_path = extract_relevant_part_of_path(s_string_converted,c_string_converted);
    return lev_dist(s_string, relevant_part_of_path.as_str(), 1,1,1);
}

fn extract_relevant_part_of_path(s_string:String, c_string:String) -> String{
    let path_separator = "/";
    let depth_path = s_string.to_string().chars().filter(|&c| c.to_string() == path_separator).count()+1;
    let mut amount_passed_path_separators = 0;
    let mut index_for_substring = c_string.len()-1;
    for i in (0..c_string.len()).rev() {
        if let Some(c) = c_string.chars().nth(i){
            if c.to_string() == path_separator{
                //println!("{}",i);
            amount_passed_path_separators+=1;
                if amount_passed_path_separators == depth_path{
                    index_for_substring = i+1;
                    break;
                }
            }
        }
    }
    //println!("{}",depth_path);
    //println!("{}",c_string.len());
    return String::from(&c_string[index_for_substring..c_string.len()]);
}

fn lev_dist(s_string: &str, c_string: &str, del_cost: i32, insert_cost:i32, substitue_cost:i32) -> i32{
    let rows = s_string.len()+1;
    let columns = c_string.len()+1;
    let mut matrix: Vec<Vec<i32>> = vec![vec![0; columns]; rows];

    for row in 0..rows{
        matrix[row][0] = row as i32;
    }

    for col in 0..columns{
        matrix[0][col] = col as i32;
    }


    for col in 1..columns{
        for row in 1..rows{
            let mut cost_sub :i32 = 0;
            if s_string.chars().nth(row) != c_string.chars().nth(col){
                cost_sub = substitue_cost;
            }
            matrix[row][col] = min(matrix[row-1][col]+del_cost+del_cost,min(matrix[row][col-1]+insert_cost,matrix[row-1][col-1]+cost_sub));
        }
    }
    /*for row in &matrix {
        for &element in row {
            print!("{} ", element);
        }
        println!();
    }*/
    return matrix[rows-1][columns-1];
}