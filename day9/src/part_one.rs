#[derive(Clone, Debug)]
struct Block {
    id: usize,
    size: usize,
    free_blocks: Vec<usize>,
    free_blocks_left: usize,
}

pub fn solve_part_one(input: &str) -> usize {
    let input_chars: Vec<usize> = input.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
    let mut files: Vec<Block> = input_chars.windows(2).step_by(2).enumerate().map(|(id, cs)| {
        return Block {
            id,
            size: cs[0],
            free_blocks: Vec::new(),
            free_blocks_left: cs[1],
        }
    }).collect();
    files.push(Block{
       id: input_chars.len() / 2,
        size: input_chars[input_chars.len() - 1],
        free_blocks: Vec::new(),
        free_blocks_left: 0
    });

    compact_files(&mut files);

    calculate_checksum(&files)
}

fn compact_files(files: &mut Vec<Block>){
    let mut current_block = 0;
    loop {
        if current_block == files.len() {
            break;
        }

        let mut block = files.pop().unwrap();
        while block.size != 0 {
            if current_block == files.len() {
                files.push(block);
                return;
            }

            while files[current_block].free_blocks_left == 0 {
                current_block += 1;
                if current_block == files.len() {
                    files.push(block);
                    return;
                }
            }

            for _ in 0..files[current_block].free_blocks_left {
                files[current_block].free_blocks.push(block.id);
                files[current_block].free_blocks_left -= 1;
                block.size -= 1;
                if block.size == 0 {
                    break
                }
            }

            if block.size > 0 {
                current_block += 1;
            }
        }
    }
}

fn calculate_checksum(files: &Vec<Block>) -> usize {
    let mut current_block = 0;
    let mut current_file_index = 0;
    let mut checksum = 0;
    loop {
        if current_block == files.len() {
            break;
        }
        for _ in 0..files[current_block].size {
            checksum += files[current_block].id * current_file_index;
            current_file_index += 1;
        }

        for n in &files[current_block].free_blocks {
            checksum += n * current_file_index;
            current_file_index += 1;
        }

        current_block += 1;
    }

    return checksum;
}