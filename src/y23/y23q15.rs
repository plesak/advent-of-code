use std::num::Wrapping;

pub fn y23q15q1(input:Vec<String>) -> String {
    let s = &input[0];
    let mut running_total = 0;
    let mut curr_total:Wrapping<u8> = Wrapping(0);
    for c in s.chars() {
        // end of current substring, comma is ignored
        if c == ',' {
            running_total += curr_total.0 as i32;
            curr_total = Wrapping(0);
        } else {
            curr_total += Wrapping(c as u8);
            curr_total *= Wrapping(17);
            // handled by the Wrapping data type
            // curr_total %= 256;
        }
    }
    // handle last group
    running_total += curr_total.0 as i32;
    running_total.to_string()
}

pub fn y23q15q2(input:Vec<String>) -> String {
    // rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
    let mut s = input[0].clone();
    s.push(',');

    let mut boxes:Vec<Vec<(String, i32)>> = vec![Vec::new(); 256];
    // eg box[1] = [ (cm, 5), (xj, 3) ]

    let mut label:String = String::new();
    let mut hash:Wrapping<u8> = Wrapping(0);
    for c in s.chars() {
        // end of current substring, comma is ignored
        if c == '-' {
            boxes[hash.0 as usize].retain(|v| v.0 != label);
            // reset for next
            label = String::new();
            hash = Wrapping(0);
        } else if c.is_digit(10) {
            let n:i32= c.to_digit(10).unwrap() as i32;
            if let Some(lens) = boxes[hash.0 as usize]
                .iter_mut().find(|v| v.0 == label) {
                lens.1 = n;
            } else {
                boxes[hash.0 as usize].push((label, n));
            }
            // reset for next
            label = String::new();
            hash = Wrapping(0);
        } else if c != ',' && c != '=' {
            label.push(c);
            hash += Wrapping(c as u8);
            hash *= Wrapping(17);
            // handled by the Wrapping data type
            // curr_total %= 256;
        }
    }
    let mut running_total = 0;
    for (i, bx) in boxes.iter().enumerate() {
        for (j, lens) in bx.iter().enumerate() {
            running_total += (i+1)*(j+1)*lens.1 as usize;
        }
    }
    running_total.to_string()
}