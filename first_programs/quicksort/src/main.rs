use std::io;

fn get_size() -> u32 {
    loop {
        println!("How many numbers?");
        let mut size = String::new();
        io::stdin().read_line(&mut size)
            .expect("Failed to read line!");
        let size = size.trim().parse();

        if let Ok(sz) = size {
           return sz; 
        }
    }
}

fn get_nums_from_stdin(size: u32) -> Vec::<i32> {
    let mut input_nums_ththat_are_being_gotten_from_stdin = Vec::<i32>::new();

    let mut instr = String::new();
    io::stdin().read_line(&mut instr)
        .expect("Failed to read line!");
    for word in instr.split(' ') {
        let word = word.trim().parse();
        if let Ok(val) = word {
            input_nums_ththat_are_being_gotten_from_stdin.push(val);
        } else {
            println!("Not a number! Try again:");
            return get_nums_from_stdin(size);
        }
    }
    input_nums_ththat_are_being_gotten_from_stdin
}

fn quick_sort(nums: &mut Vec<i32>) {
    fn actual_quick_sort(nums: &mut Vec<i32>, mut l: usize, mut r: usize) {
        if (r-1 <= l) { return; }

        let end = r;
        r -= 1;
        while r > l {
            while nums[l] < nums[end] { l += 1; }
            while nums[r] > nums[end] { r -= 1; }

            if r < l { r = l }

            let tmp = nums[l];
            nums[l] = nums[r];
            nums[r] = tmp;
        }
        let tmp = nums[end];
        nums[end] = nums[l];
        nums[l] = tmp; 
    }

    

    for num in nums {print!(" num {}", num);}
    println!();
}

fn main() { 
    let size = get_size();
    
    let mut nums = get_nums_from_stdin(size);

    quick_sort(&mut nums);
    for number in nums {
        println!("num {}", number);
    }
}
