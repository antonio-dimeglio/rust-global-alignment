use std::borrow::Borrow;

pub fn global_alignment(first_sequence:&[u8], second_sequence:&[u8]) -> (String, String){

    let mut alignment_matrix = vec![vec![0; second_sequence.len() + 1]; first_sequence.len() + 1];
    let MATCH: i32= 1;
    let MISMATCH:i32 = -1;
    let GAP:i32 = -2;

    let mut c:char = '\0'; //used to store a character from the first sequence
    let mut k:char = '\0'; //used to store a character from the second sequence

    //Matrix filling
    for i in  0..first_sequence.len()+1{
        alignment_matrix[i][0] = (i as i32) * GAP;
    }

    for i in 0..second_sequence.len()+1{
        alignment_matrix[0][i] = (i as i32) * GAP;
    }

    for i in 1..first_sequence.len()+1{
        for j in 1..second_sequence.len() + 1{
            let mut possible_values:[i32;3] = [
                alignment_matrix[i][j-1] + GAP,
                alignment_matrix[i-1][j] + GAP,
                0
            ];


            if  c == k {
                possible_values[2] = alignment_matrix[i-1][j-1] + MATCH;
            } else{
                possible_values[2] = alignment_matrix[i-1][j-1] + MISMATCH;
            }


            alignment_matrix[i][j] = *possible_values.iter().max().unwrap();
        }
    }

    //Backtracking
    let mut first_alignment = String::from("");
    let mut second_alignment = String::from("");

    let mut i = first_sequence.len();
    let mut j = second_sequence.len();

    while i != 0 || j != 0{
        if i == 0{
            c = '-';
            k = second_sequence[j-1] as char;
            j -= 1;
        } else if j == 0{
            c = first_sequence[i-1] as char;
            k = '-';
            i -= 1;
        } else{
            let values:[i32;3] = [
                alignment_matrix[i][j-1], //left
                alignment_matrix[i-1][j], //top
                alignment_matrix[i-1][j-1], //top-left
            ];

            let max_pos = values.iter().position(
                |&x| x == *values.iter().max().unwrap()
            ).unwrap();

            if max_pos == 2 || first_sequence[i-1] == second_sequence[j-1]{
                c = first_sequence[i-1] as char;
                k = second_sequence[j-1] as char;

                i -= 1;
                j -= 1;
            } else if max_pos == 1{
                c = first_sequence[i-1] as char;
                k = '-';
                i -= 1;
            } else {
                c = '-';
                k = second_sequence[j-1] as char;
                j -= 1;
            }
        }
        first_alignment = format!("{}{}", c, first_alignment);
        second_alignment = format!("{}{}", k, second_alignment);

    }

    return (first_alignment, second_alignment);
}